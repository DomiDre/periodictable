use crate::Element;
use crate::{UncertainFloat, Isotope};

pub fn load() -> Element {
    Element {
        atomic_number: 111,
        name: "Roentgenium",
        symbol: "Rg",
        mass: 272.0_f64,
        common_ions: vec![],
        uncommon_ions: vec![],
        xray_scattering: None,
        neutron_scattering: None,

        isotopes: vec![
            Isotope { 
                mass_number: 272,
                mass: UncertainFloat::new(272.153_48_f64, 0.000_36_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
