use crate::Element;
use crate::{UncertainFloat, Isotope};

pub fn load() -> Element {
    Element {
        atomic_number: 117,
        name: "Tennessine",
        symbol: "Ts",
        mass: 294.0_f64,
        common_ions: vec![],
        uncommon_ions: vec![],
        xray_scattering: None,
        neutron_scattering: None,

        isotopes: vec![
            Isotope { 
                mass_number: 294,
                mass: UncertainFloat::new(0.0, 0.0),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
