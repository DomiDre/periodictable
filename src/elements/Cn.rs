use crate::Element;
use crate::{UncertainFloat, Isotope};

pub fn load() -> Element {
    Element {
        atomic_number: 112,
        name: "Copernicium",
        symbol: "Cn",
        mass: 285.0_f64,
        common_ions: vec![2],
        uncommon_ions: vec![],
        xray_scattering: None,
        neutron_scattering: None,

        isotopes: vec![
            Isotope { 
                mass_number: 285,
                mass: UncertainFloat::new(0.0, 0.0),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
