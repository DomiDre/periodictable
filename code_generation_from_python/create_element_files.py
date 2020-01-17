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

    # if no uncertainty present (no brackets) -> return 0 uncertainty
    if len(splitted_num) == 1:
        return value+", 0.0"
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

# load mass data including uncertainties
isotope_masses = {}
for line in periodictable.mass.massdata.split('\n'):
    isotope, mass, abundance, average_mass = line.split(',')
    isotope_masses[isotope] = (mass, abundance, average_mass)

# nsftable = periodictable.nsf.nsftable
print()
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

    is_plus_minus = "false"
    has_strong_energy_dependence = "false"
    if special_symbol == "+/-":
        is_plus_minus = "true"
    if special_symbol == "E":
        has_strong_energy_dependence = "true"

    if is_element:
        elements_nsf[symbol] = (b_c, bp, bm, xs_coh, xs_incoh, xs_total, xs_thermal_absorption, abundance, half_life)
    else:
        isotopes_nsf[symbol] = (b_c, bp, bm, xs_coh, xs_incoh, xs_total, xs_thermal_absorption, abundance, half_life)

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
            neutron_b_coherent: """)
            
            if elements_nsf[symbol] is None:
                f.write("None,\n")
            else:
                b_c, bp, bm, xs_coh, xs_incoh, xs_total, xs_thermal_absorption, abundance, half_life = elements_nsf[symbol]
                f.write("""NeutronScatteringFactor {
                    b_c: """ + uncertain_number(b_c) + """
                    b_p:
                    b_m:
                    coherent_scattering_xs:
                    incoherent_scattering_xs:
                    absorption_scattering_xs:
                    thermal_absorption_xs:
                }""")
            f.write("""
            isotopes:
                vec![""")
            for mass_number in element.isotopes:
                isotope = periodictable.elements.isotope(f"{mass_number}-{symbol}")
                mass = periodictable.mass.mass(isotope)
                try:
                    mass, abundance, average_mass = isotope_masses[f"{atomic_number}-{symbol}-{mass_number}"]
                except KeyError:
                    continue
                f.write("""
                    Isotope { 
                        mass_number: """+str(mass_number)+""",
                        mass: """+ uncertain_number(mass) +""",
                        abundance: """+ uncertain_number(abundance) +""",
                        b_coherent: 
                    },""")
            f.write("""
                ]
        }
    }
    """)