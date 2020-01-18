use crate::Element;
use crate::elements::*;
// use crate::elements::He::He;
// // use crate::element::Element;
// // use crate::isotope::IsotopeTable;
// use crate::elements::;

pub struct PeriodicTable {
    pub elements: [Element; 119]
}

impl PeriodicTable {
    pub fn new() -> PeriodicTable {
        let elements = [
            n::load(),
            H::load(),
            He::load(),
            Li::load(),
            Be::load(),
            B::load(),
            C::load(),
            N::load(),
            O::load(),
            F::load(),
            Ne::load(),
            Na::load(),
            Mg::load(),
            Al::load(),
            Si::load(),
            P::load(),
            S::load(),
            Cl::load(),
            Ar::load(),
            K::load(),
            Ca::load(),
            Sc::load(),
            Ti::load(),
            V::load(),
            Cr::load(),
            Mn::load(),
            Fe::load(),
            Co::load(),
            Ni::load(),
            Cu::load(),
            Zn::load(),
            Ga::load(),
            Ge::load(),
            As::load(),
            Se::load(),
            Br::load(),
            Kr::load(),
            Rb::load(),
            Sr::load(),
            Y::load(),
            Zr::load(),
            Nb::load(),
            Mo::load(),
            Tc::load(),
            Ru::load(),
            Rh::load(),
            Pd::load(),
            Ag::load(),
            Cd::load(),
            In::load(),
            Sn::load(),
            Sb::load(),
            Te::load(),
            I::load(),
            Xe::load(),
            Cs::load(),
            Ba::load(),
            La::load(),
            Ce::load(),
            Pr::load(),
            Nd::load(),
            Pm::load(),
            Sm::load(),
            Eu::load(),
            Gd::load(),
            Tb::load(),
            Dy::load(),
            Ho::load(),
            Er::load(),
            Tm::load(),
            Yb::load(),
            Lu::load(),
            Hf::load(),
            Ta::load(),
            W::load(),
            Re::load(),
            Os::load(),
            Ir::load(),
            Pt::load(),
            Au::load(),
            Hg::load(),
            Tl::load(),
            Pb::load(),
            Bi::load(),
            Po::load(),
            At::load(),
            Rn::load(),
            Fr::load(),
            Ra::load(),
            Ac::load(),
            Th::load(),
            Pa::load(),
            U::load(),
            Np::load(),
            Pu::load(),
            Am::load(),
            Cm::load(),
            Bk::load(),
            Cf::load(),
            Es::load(),
            Fm::load(),
            Md::load(),
            No::load(),
            Lr::load(),
            Rf::load(),
            Db::load(),
            Sg::load(),
            Bh::load(),
            Hs::load(),
            Mt::load(),
            Ds::load(),
            Rg::load(),
            Cn::load(),
            Nh::load(),
            Fl::load(),
            Mc::load(),
            Lv::load(),
            Ts::load(),
            Og::load()
        ];
        PeriodicTable { 
            elements
        }
    }
}

//             Element::new(0,   "Neutron",         "n",  vec![],                           vec![]),
//             Element::new(1,   "Hydrogen",        "H",  vec![-1, 1],                      vec![]),
//             Element::new(2,   "Helium",          "He", vec![],                           vec![1, 2]),
//             Element::new(3,   "Lithium",         "Li", vec![1],                          vec![]),
//             Element::new(4,   "Beryllium",       "Be", vec![2],                          vec![1]),
//             Element::new(5,   "Boron",           "B",  vec![3],                          vec![-5, -1, 1, 2]),
//             Element::new(6,   "Carbon",          "C",  vec![-4, -3, -2, -1, 1, 2, 3, 4], vec![]),
//             Element::new(7,   "Nitrogen",        "N",  vec![-3, 3, 5],                   vec![-2, -1, 1, 2, 4]),
//             Element::new(8,   "Oxygen",          "O",  vec![-2],                         vec![-1, 1, 2]),
//             Element::new(9,   "Fluorine",        "F",  vec![-1],                         vec![]),
//             Element::new(10,  "Neon",            "Ne", vec![],                           vec![]),
//             Element::new(11,  "Sodium",          "Na", vec![1],                          vec![-1]),
//             Element::new(12,  "Magnesium",       "Mg", vec![2],                          vec![1]),
//             Element::new(13,  "Aluminum",        "Al", vec![3],                          vec![-2, -1, 1, 2]),
//             Element::new(14,  "Silicon",         "Si", vec![-4, 4],                      vec![-3, -2, -1, 1, 2, 3]),
//             Element::new(15,  "Phosphorus",      "P",  vec![-3, 3, 5],                   vec![-2, -1, 1, 2, 4]),
//             Element::new(16,  "Sulfur",          "S",  vec![-2, 2, 4, 6],                vec![-1, 1, 3, 5]),
//             Element::new(17,  "Chlorine",        "Cl", vec![-1, 1, 3, 5, 7],             vec![2, 4, 6]),
//             Element::new(18,  "Argon",           "Ar", vec![],                           vec![]),
//             Element::new(19,  "Potassium",       "K",  vec![1],                          vec![-1]),
//             Element::new(20,  "Calcium",         "Ca", vec![2],                          vec![1]),
//             Element::new(21,  "Scandium",        "Sc", vec![3],                          vec![1, 2]),
//             Element::new(22,  "Titanium",        "Ti", vec![4],                          vec![-2, -1, 1, 2, 3]),
//             Element::new(23,  "Vanadium",        "V",  vec![5],                          vec![-3, -1, 1, 2, 3, 4]),
//             Element::new(24,  "Chromium",        "Cr", vec![3, 6],                       vec![-4, -2, -1, 1, 2, 4, 5]),
//             Element::new(25,  "Manganese",       "Mn", vec![2, 4, 7],                    vec![-3, -2, -1, 1, 3, 5, 6]),
//             Element::new(26,  "Iron",            "Fe", vec![2, 3, 6],                    vec![-4, -2, -1, 1, 4, 5, 7]),
//             Element::new(27,  "Cobalt",          "Co", vec![2, 3],                       vec![-3, -1, 1, 4, 5]),
//             Element::new(28,  "Nickel",          "Ni", vec![2],                          vec![-2, -1, 1, 3, 4]),
//             Element::new(29,  "Copper",          "Cu", vec![2],                          vec![-2, 1, 3, 4]),
//             Element::new(30,  "Zinc",            "Zn", vec![2],                          vec![-2, 1]),
//             Element::new(31,  "Gallium",         "Ga", vec![3],                          vec![-5, -4, -2, -1, 1, 2]),
//             Element::new(32,  "Germanium",       "Ge", vec![-4, 2, 4],                   vec![-3, -2, -1, 1, 3]),
//             Element::new(33,  "Arsenic",         "As", vec![-3, 3, 5],                   vec![-2, -1, 1, 2, 4]),
//             Element::new(34,  "Selenium",        "Se", vec![-2, 2, 4, 6],                vec![-1, 1, 3, 5]),
//             Element::new(35,  "Bromine",         "Br", vec![-1, 1, 3, 5],                vec![4, 7]),
//             Element::new(36,  "Krypton",         "Kr", vec![2],                          vec![]),
//             Element::new(37,  "Rubidium",        "Rb", vec![1],                          vec![-1]),
//             Element::new(38,  "Strontium",       "Sr", vec![2],                          vec![1]),
//             Element::new(39,  "Yttrium",         "Y",  vec![3],                          vec![1, 2]),
//             Element::new(40,  "Zirconium",       "Zr", vec![4],                          vec![-2, 1, 2, 3]),
//             Element::new(41,  "Niobium",         "Nb", vec![5],                          vec![-3, -1, 1, 2, 3, 4]),
//             Element::new(42,  "Molybdenum",      "Mo", vec![4, 6],                       vec![-4, -2, -1, 1, 2, 3, 5]),
//             Element::new(43,  "Technetium",      "Tc", vec![4, 7],                       vec![-3, -1, 1, 2, 3, 5, 6]),
//             Element::new(44,  "Ruthenium",       "Ru", vec![3, 4],                       vec![-4, -2, 1, 2, 5, 6, 7, 8]),
//             Element::new(45,  "Rhodium",         "Rh", vec![3],                          vec![-3, -1, 1, 2, 4, 5, 6]),
//             Element::new(46,  "Palladium",       "Pd", vec![2, 4],                       vec![1, 3, 5, 6]),
//             Element::new(47,  "Silver",          "Ag", vec![1],                          vec![-2, -1, 2, 3, 4]),
//             Element::new(48,  "Cadmium",         "Cd", vec![2],                          vec![-2, 1]),
//             Element::new(49,  "Indium",          "In", vec![3],                          vec![-5, -2, -1, 1, 2]),
//             Element::new(50,  "Tin",             "Sn", vec![-4, 2, 4],                   vec![-3, -2, -1, 1, 3]),
//             Element::new(51,  "Antimony",        "Sb", vec![-3, 3, 5],                   vec![-2, -1, 1, 2, 4]),
//             Element::new(52,  "Tellurium",       "Te", vec![-2, 2, 4, 6],                vec![-1, 1, 3, 5]),
//             Element::new(53,  "Iodine",          "I",  vec![-1, 1, 3, 5, 7],             vec![4, 6]),
//             Element::new(54,  "Xenon",           "Xe", vec![2, 4, 6],                    vec![8]),
//             Element::new(55,  "Cesium",          "Cs", vec![1],                          vec![-1]),
//             Element::new(56,  "Barium",          "Ba", vec![2],                          vec![1]),
//             Element::new(57,  "Lanthanum",       "La", vec![3],                          vec![1, 2]),
//             Element::new(58,  "Cerium",          "Ce", vec![3, 4],                       vec![2]),
//             Element::new(59,  "Praseodymium",    "Pr", vec![3],                          vec![2, 4, 5]),
//             Element::new(60,  "Neodymium",       "Nd", vec![3],                          vec![2, 4]),
//             Element::new(61,  "Promethium",      "Pm", vec![3],                          vec![2]),
//             Element::new(62,  "Samarium",        "Sm", vec![3],                          vec![2]),
//             Element::new(63,  "Europium",        "Eu", vec![2, 3],                       vec![]),
//             Element::new(64,  "Gadolinium",      "Gd", vec![3],                          vec![1, 2]),
//             Element::new(65,  "Terbium",         "Tb", vec![3],                          vec![1, 2, 4]),
//             Element::new(66,  "Dysprosium",      "Dy", vec![3],                          vec![2, 4]),
//             Element::new(67,  "Holmium",         "Ho", vec![3],                          vec![2]),
//             Element::new(68,  "Erbium",          "Er", vec![3],                          vec![2]),
//             Element::new(69,  "Thulium",         "Tm", vec![3],                          vec![2]),
//             Element::new(70,  "Ytterbium",       "Yb", vec![3],                          vec![2]),
//             Element::new(71,  "Lutetium",        "Lu", vec![3],                          vec![2]),
//             Element::new(72,  "Hafnium",         "Hf", vec![4],                          vec![-2, 1, 2, 3]),
//             Element::new(73,  "Tantalum",        "Ta", vec![5],                          vec![-3, -1, 1, 2, 3, 4]),
//             Element::new(74,  "Tungsten",        "W",  vec![4, 6],                       vec![-4, -2, -1, 1, 2, 3, 5]),
//             Element::new(75,  "Rhenium",         "Re", vec![4],                          vec![-3, -1, 1, 2, 3, 5, 6, 7]),
//             Element::new(76,  "Osmium",          "Os", vec![4],                          vec![-4, -2, -1, 1, 2, 3, 5, 6, 7, 8]),
//             Element::new(77,  "Iridium",         "Ir", vec![3, 4],                       vec![-3, -1, 1, 2, 5, 6, 7, 8, 9]),
//             Element::new(78,  "Platinum",        "Pt", vec![2, 4],                       vec![-3, -2, -1, 1, 3, 5, 6]),
//             Element::new(79,  "Gold",            "Au", vec![3],                          vec![-3, -2, -1, 1, 2, 5]),
//             Element::new(80,  "Mercury",         "Hg", vec![1, 2],                       vec![-2, 4]),
//             Element::new(81,  "Thallium",        "Tl", vec![1, 3],                       vec![-5, -2, -1, 2]),
//             Element::new(82,  "Lead",            "Pb", vec![2, 4],                       vec![-4, -2, -1, 1, 3]),
//             Element::new(83,  "Bismuth",         "Bi", vec![3],                          vec![-3, -2, -1, 1, 2, 4, 5]),
//             Element::new(84,  "Polonium",        "Po", vec![-2, 2, 4],                   vec![5, 6]),
//             Element::new(85,  "Astatine",        "At", vec![-1, 1],                      vec![3, 5, 7]),
//             Element::new(86,  "Radon",           "Rn", vec![2],                          vec![6]),
//             Element::new(87,  "Francium",        "Fr", vec![1],                          vec![]),
//             Element::new(88,  "Radium",          "Ra", vec![2],                          vec![]),
//             Element::new(89,  "Actinium",        "Ac", vec![3],                          vec![]),
//             Element::new(90,  "Thorium",         "Th", vec![4],                          vec![1, 2, 3]),
//             Element::new(91,  "Protactinium",    "Pa", vec![5],                          vec![3, 4]),
//             Element::new(92,  "Uranium",         "U",  vec![6],                          vec![1, 2, 3, 4, 5]),
//             Element::new(93,  "Neptunium",       "Np", vec![5],                          vec![2, 3, 4, 6, 7]),
//             Element::new(94,  "Plutonium",       "Pu", vec![4],                          vec![2, 3, 5, 6, 7]),
//             Element::new(95,  "Americium",       "Am", vec![3],                          vec![2, 4, 5, 6, 7]),
//             Element::new(96,  "Curium",          "Cm", vec![3],                          vec![4, 6]),
//             Element::new(97,  "Berkelium",       "Bk", vec![3],                          vec![4]),
//             Element::new(98,  "Californium",     "Cf", vec![3],                          vec![2, 4]),
//             Element::new(99,  "Einsteinium",     "Es", vec![3],                          vec![2, 4]),
//             Element::new(100, "Fermium",         "Fm", vec![3],                          vec![2]),
//             Element::new(101, "Mendelevium",     "Md", vec![3],                          vec![2]),
//             Element::new(102, "Nobelium",        "No", vec![2],                          vec![3]),
//             Element::new(103, "Lawrencium",      "Lr", vec![3],                          vec![]),
//             Element::new(104, "Rutherfordium",   "Rf", vec![4],                          vec![]),
//             Element::new(105, "Dubnium",         "Db", vec![5],                          vec![]),
//             Element::new(106, "Seaborgium",      "Sg", vec![6],                          vec![]),
//             Element::new(107, "Bohrium",         "Bh", vec![7],                          vec![]),
//             Element::new(108, "Hassium",         "Hs", vec![8],                          vec![]),
//             Element::new(109, "Meitnerium",      "Mt", vec![],                           vec![]),
//             Element::new(110, "Darmstadtium",    "Ds", vec![],                           vec![]),
//             Element::new(111, "Roentgenium",     "Rg", vec![],                           vec![]),
//             Element::new(112, "Copernicium",     "Cn", vec![2],                          vec![]),
//             Element::new(113, "Nihonium",        "Nh", vec![],                           vec![]),
//             Element::new(114, "Flerovium",       "Fl", vec![],                           vec![]),
//             Element::new(115, "Moscovium",       "Mc", vec![],                           vec![]),
//             Element::new(116, "Livermorium",     "Lv", vec![],                           vec![]),
//             Element::new(117, "Tennessine",      "Ts", vec![],                           vec![]),
//             Element::new(118, "Oganesson",       "Og", vec![],                           vec![]),
//         ];

//         // set isotope masses
//         let isotope_table = IsotopeTable::new();
//         for (i, mass) in isotope_table.masses.iter().enumerate() {
//             elements[i].isotopes = mass.clone();
//         }
//     }
// }

// impl PeriodicTable {
//     pub fn symbol(&self, symbol: &str) -> Option<&Element> {
//         if let Some(idx) = self.elements.iter().position(|x| x.symbol == symbol) {
//             Some(&self.elements[idx])
//         } else {
//             None
//         }
//     }

//     pub fn name(&self, name: &str) -> Option<&Element> {
//         if let Some(idx) = self.elements.iter().position(|x| x.name == name) {
//             Some(&self.elements[idx])
//         } else {
//             None
//         }
//     }
// }

// #[test]
// fn create_periodic_table() {
//     let table = PeriodicTable::new();
//     let iron = &table.elements[26];
//     assert_eq!(iron.atomic_number, 26);
//     assert_eq!(iron.name, "Iron");

// }

// #[test]
// fn check_hydrogen_mass() {
//     let table = PeriodicTable::new();
//     let hydrogen = &table.elements[1];
//     let hydrogen_mass = hydrogen.mass();
//     if (hydrogen_mass.value - 1.007_9).abs() > 0.000_1 {
//         panic!("Hydrogen average mass value incorrect.");
//     }
//     if (hydrogen_mass.uncertainty - 0.000_1).abs() > 0.000_1 {
//         panic!("Hydrogen average mass uncertainty incorrect.");
//     }
// }