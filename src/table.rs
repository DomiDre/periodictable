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

impl PeriodicTable {
    pub fn symbol(&self, symbol: &str) -> Option<&Element> {
        if let Some(idx) = self.elements.iter().position(|x| x.symbol == symbol) {
            Some(&self.elements[idx])
        } else {
            None
        }
    }

    pub fn name(&self, name: &str) -> Option<&Element> {
        if let Some(idx) = self.elements.iter().position(|x| x.name == name) {
            Some(&self.elements[idx])
        } else {
            None
        }
    }
}

#[test]
fn create_periodic_table() {
    let table = PeriodicTable::new();
    let iron = &table.elements[26];
    assert_eq!(iron.atomic_number, 26);
    assert_eq!(iron.name, "Iron");

}

#[test]
fn check_hydrogen_mass() {
    let table = PeriodicTable::new();
    let hydrogen = &table.elements[1];
    let hydrogen_mass = hydrogen.mass;
    if (hydrogen_mass - 1.007_9).abs() > 0.000_1 {
        panic!("Hydrogen average mass value incorrect.");
    }
}

#[test]
fn check_iron_scattering() {
    let table = PeriodicTable::new();
    let iron = &table.elements[26];
    let b_c = iron.b_c().unwrap();
    let xsf = iron.atomic_scattering_factor(10.0).unwrap();
    let f1 = xsf.f1.unwrap();
    let f2 = xsf.f2.unwrap();
    if (b_c.value - 9.45).abs() > 0.000_1 {
        panic!("Iron bound coherence length odd");
    }

    if (f1 - 25.975).abs() > 0.000_1 {
        panic!("Iron f1 odd: {}", f1);
    }
    if (f2 - 2.2671).abs() > 0.000_1 {
        panic!("Iron f2 odd: {}", f2);
    }
}