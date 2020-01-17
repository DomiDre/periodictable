import periodictable
import nsf_to_rust
def insert_underscores(num: str) -> str:
    """Given a number, insert underscore every 3 digit after decimal dot
    """

    if not "." in num:
        return num
    idx = num.index('.') + 4
    while idx < len(num):
        num = num[:idx] + "_" +num[idx:]
        idx += 3+1
    return num

def uncertain_number(num:str) -> str:
    """Given a string with format 1.234(56), seperate into value and uncertainty containing string
    Applys insert_underscore to both and adds _f64 at end

    If string is empty returns 0.0, 0.0
    """
    if num == "":
        return "UncertainFloat::new(0.0, 0.0)"
    
    splitted_num = num.split('(')
    value = splitted_num[0]
    if value == "0":
        value = "0.0"
    if "<" in value:
        value = "0.0"
    # if no uncertainty present (no brackets) -> return 0 uncertainty
    if len(splitted_num) == 1:
        
        return f'UncertainFloat::new({value}, 0.0)'
    # else
    uncertainty = splitted_num[1].split(')')[0]

    # check if number has a decimal point -> determine number of zeros for uncertainty
    print_uncertainty = ""
    if "." in value:
        num_digits_after_dot = len(value) - value.index('.') - 1
        if not "." in uncertainty:
            print_uncertainty = "0."
            while len(print_uncertainty) - 2 < num_digits_after_dot - len(uncertainty):
                print_uncertainty += "0"
    print_uncertainty += uncertainty
    return "UncertainFloat::new(" + insert_underscores(value)+"_f64" + ", " + insert_underscores(print_uncertainty)+"_f64)"

def option_uncertain_number(num: str) -> str:
    """String can be either an uncertain number or None
    """
    if num is None:
        return "None"
    else:
        return f"Some({uncertain_number(num)})"

def neutron_sf(b_c, b_p, b_m, xs_coh, xs_incoh, xs_total, xs_thermal_absorption, abundance, half_life, intendation_level):
    return (
        "Some(NeutronScatteringFactor {\n"+
        " "*intendation_level + f"b_c: {uncertain_number(b_c)},\n"+
        " "*intendation_level + f"b_p: {option_uncertain_number(b_p)},\n"+
        " "*intendation_level + f"b_m: {option_uncertain_number(b_m)},\n"+
        " "*intendation_level + f"coherent_scattering_xs: {option_uncertain_number(xs_coh)},\n"+
        " "*intendation_level + f"incoherent_scattering_xs: {option_uncertain_number(xs_incoh)},\n"+
        " "*intendation_level + f"absorption_scattering_xs: {option_uncertain_number(xs_total)},\n"+
        " "*intendation_level + f"thermal_absorption_xs: {option_uncertain_number(xs_thermal_absorption)},\n"+
        " "*(intendation_level-4) + "}),"
    )

# load mass data including uncertainties
isotope_masses = {}
for line in periodictable.mass.massdata.split('\n'):
    isotope, mass, abundance, average_mass = line.split(',')
    isotope_masses[isotope] = (mass, abundance, average_mass)

elements_nsf = {}
isotopes_nsf = {}
atom_counter = 0
for line in nsf_to_rust.nsftable.split('\n'):
    name, abundance_or_half_life, spin, b_c, bp, bm, special_symbol, xs_coh, xs_incoh, xs_total, xs_thermal_absorption = line.strip().split(',')
    atomic_number, symbol, *mass_number = name.split("-")
    atomic_number = int(atomic_number)
    if atomic_number > atom_counter or symbol == "n":
        atom_counter = atomic_number
        is_element=True
    else:
        is_element=False
    if len(mass_number) > 0:
        mass_number = mass_number[0]

    if abundance_or_half_life.endswith("Y") or abundance_or_half_life.endswith("S"):
        abundance = "0.0"
        half_life = float(abundance_or_half_life.split(" Y")[0])*365.25*24*3600 if abundance_or_half_life.endswith("Y") else float(abundance_or_half_life.split(" S")[0])
    else:
        abundance = abundance_or_half_life
        half_life = "0.0"

    if bp == "":
        bp = None
    if bm == "":
        bm = None
    if xs_coh == "":
        xs_coh = None
    if xs_incoh == "":
        xs_incoh = None
    if xs_total == "":
        xs_total = None
    if xs_thermal_absorption == "":
        xs_thermal_absorption = None
    
    is_plus_minus = "false"
    has_strong_energy_dependence = "false"
    if special_symbol == "+/-":
        is_plus_minus = "true"
    if special_symbol == "E":
        has_strong_energy_dependence = "true"
    

    if is_element:
        elements_nsf[symbol] = (b_c, bp, bm, xs_coh, xs_incoh, xs_total, xs_thermal_absorption, abundance, half_life)
    else:
        isotopes_nsf[f'{symbol}-{mass_number}'] = (b_c, bp, bm, xs_coh, xs_incoh, xs_total, xs_thermal_absorption, abundance, half_life)

for element in periodictable.elements:
    if not element.symbol in elements_nsf:
        elements_nsf[element.symbol] = None

if True:
    for atomic_number, element in periodictable.core.element_base.items():
        name, symbol, oxidation_state, common_ions = element
        element = periodictable.elements.symbol(symbol)
        with open(f"element_files/{symbol}.rs", "w") as f:
            f.write(
    """use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: """+str(atomic_number)+""",
        name: \"""" + name + """\",
        symbol: \"""" + symbol + """\",
        mass: """+ str(float(periodictable.mass.mass(element))) +""",
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: """)
        
            if elements_nsf[symbol] is None:
                f.write("None,\n")
            else:
                b_c, b_p, b_m, xs_coh, xs_incoh, xs_total, xs_thermal_absorption, abundance, half_life = elements_nsf[symbol]
                f.write(neutron_sf(*elements_nsf[symbol], 12))
            f.write("""
        isotopes: vec![""")
            for mass_number in element.isotopes:
                isotope = periodictable.elements.isotope(f"{mass_number}-{symbol}")
                mass = periodictable.mass.mass(isotope)
                try:
                    mass, abundance, average_mass = isotope_masses[f"{atomic_number}-{symbol}-{mass_number}"]
                except KeyError:
                    continue
                try:
                    neutron_scattering = neutron_sf(*isotopes_nsf[f'{symbol}-{mass_number}'], 20)
                except KeyError:
                    neutron_scattering = "None"

                f.write("""
            Isotope { 
                mass_number: """+str(mass_number)+""",
                mass: """+ uncertain_number(mass) +""",
                abundance: """+ uncertain_number(abundance) +""",
                xray_scattering: None,
                neutron_scattering: """ + neutron_scattering + """
            },""")
            f.write("""
        ]
    }
}
""")