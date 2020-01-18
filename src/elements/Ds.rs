use crate::Element;
use crate::{Isotope, UncertainFloat, AtomicScatteringFactor, XrayScatteringFactor, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 110,
        name: "Darmstadtium",
        symbol: "Ds",
        mass: 281.0,
        common_ions: vec![],
        uncommon_ions: vec![],
        xray_scattering: None,
        neutron_scattering: None,

        isotopes: vec![
            Isotope { 
                mass_number: 267,
                mass: UncertainFloat::new(267.143_96_f64, 0.000_41_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 268,
                mass: UncertainFloat::new(268.143_53_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 269,
                mass: UncertainFloat::new(269.145_14_f64, 0.000_31_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 270,
                mass: UncertainFloat::new(270.144_63_f64, 0.000_70_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 271,
                mass: UncertainFloat::new(271.146_08_f64, 0.000_20_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 272,
                mass: UncertainFloat::new(272.146_31_f64, 0.000_70_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 273,
                mass: UncertainFloat::new(273.149_25_f64, 0.000_47_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 281,
                mass: UncertainFloat::new(0.0, 0.0),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
