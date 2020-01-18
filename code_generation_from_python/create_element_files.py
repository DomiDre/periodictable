import periodictable
import nsf_to_rust
import os.path, numpy
def insert_underscores(num: str) -> str:
    """Given a number, insert underscore every 3 digit after decimal dot
    """
    # add underscore before .
    if "." in num:
        idx = num.index(".")-3
    else:
        idx = len(num) - 3
    while idx > 0:
        num = num[:idx] + "_" +num[idx:]
        idx -= 3

    # add underscores after .
    if not "." in num:
        return num
    idx = num.index('.') + 4
    if "e" in num:
        e_idx = len(num) - num.index('e')
    else:
        e_idx = 0
    while idx < len(num) - e_idx: 
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
    elif value == "100":
        value = "100.0"
    if "<" in value:
        value = "0.0"
    value = str(float(value))

    if "e-" in value:
        significand, exponent = value.split("e-")
        value = "0."
        i = 1
        while i < int(exponent):
            value += "0"
            i+=1
        for char in significand:
            if char == ".":
                continue
            value += char
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
    if print_uncertainty.endswith("."):
        print_uncertainty = print_uncertainty[:-1]
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
        " "*intendation_level + f"bound_coherent_scattering_xs: {option_uncertain_number(xs_coh)},\n"+
        " "*intendation_level + f"bound_incoherent_scattering_xs: {option_uncertain_number(xs_incoh)},\n"+
        " "*intendation_level + f"total_bound_scattering_xs: {option_uncertain_number(xs_total)},\n"+
        " "*intendation_level + f"absorption_xs: {option_uncertain_number(xs_thermal_absorption)},\n"+
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

elements_xsf = {}
_nff_path = "./xsf/"
for element in periodictable.elements:
    # read xsf datei
    filename = os.path.join(_nff_path, element.symbol.lower()+".nff")
    if element.symbol != 'n' and os.path.exists(filename):
        xsf = numpy.loadtxt(filename, skiprows=1).T
        xsf[1, xsf[1] == -9999.] = None
        xsf[0] *= 0.001  # Use keV in table rather than eV
        E, f1, f2 = xsf
        elements_xsf[element.symbol] = (E, f1, f2)

for atomic_number, element in periodictable.core.element_base.items():
    name, symbol, common_ions, uncommon_ions = element
    element = periodictable.elements.symbol(symbol)
    found_xray_info = False
    found_neutron_info = False
    found_isotopes = False

    element_file_content = ""
    
    element_file_content +="""pub fn load() -> Element {
    Element {
        atomic_number: """+str(atomic_number)+""",
        name: \"""" + name + """\",
        symbol: \"""" + symbol + """\",
        mass: """+ insert_underscores(str(float(periodictable.mass.mass(element)))) +"""_f64,
        common_ions: vec!"""+ str(common_ions) +""",
        uncommon_ions: vec!"""+ str(uncommon_ions) +""",
        xray_scattering: """
    if symbol not in elements_xsf:
        element_file_content +="None"
    else:
        found_xray_info = True
        element_file_content +="""Some(XrayScatteringFactor {
            table: vec![\n"""
        E, f1, f2 = elements_xsf[symbol]
        for i in range(len(E)):
            if numpy.isnan(f1[i]):
                print_f1 = "None"
            else:
                print_f1 = f"Some({insert_underscores(str(f1[i]))}_f64)"

            if numpy.isnan(f2[i]):
                print_f2 = "None"
            else:
                print_f2 = f"Some({insert_underscores(str(f2[i]))}_f64)"
            element_file_content +=" "*16 +"AtomicScatteringFactor { energy: "
            element_file_content +=f"{insert_underscores(str(numpy.round(E[i],8)))}_f64, f1: {print_f1}, f2: {print_f2} "+"},\n"
        element_file_content +=" "*12 + """]
        })"""
    element_file_content +=""",
        neutron_scattering: """
        
    if elements_nsf[symbol] is None:
        element_file_content +="None,\n"
    else:
        found_neutron_info = True
        b_c, b_p, b_m, xs_coh, xs_incoh, xs_total, xs_thermal_absorption, abundance, half_life = elements_nsf[symbol]
        element_file_content +=neutron_sf(*elements_nsf[symbol], 12)
    element_file_content +="""
        isotopes: vec!["""
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
        found_isotopes = True
        element_file_content +="""
            Isotope { 
                mass_number: """+str(mass_number)+""",
                mass: """+ uncertain_number(mass) +""",
                abundance: """+ uncertain_number(abundance) +""",
                xray_scattering: None,
                neutron_scattering: """ + neutron_scattering + """
            },"""
    element_file_content +="""
        ]
    }
}
"""
    element_file_content = "\n" + element_file_content
    import_line = "use crate::{UncertainFloat"
    if found_isotopes:
        import_line +=", Isotope"
    if found_xray_info:
        import_line +=", AtomicScatteringFactor, XrayScatteringFactor"
    if found_neutron_info:
        import_line += ", NeutronScatteringFactor"

    element_file_content = import_line + "};\n" + element_file_content
    element_file_content = "use crate::Element;\n" + element_file_content

    with open(f"../src/elements/{symbol}.rs", "w") as f:
        f.write(element_file_content)