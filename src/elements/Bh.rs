use crate::Element;
use crate::{Isotope, UncertainFloat, AtomicScatteringFactor, XrayScatteringFactor, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 107,
        name: "Bohrium",
        symbol: "Bh",
        mass: 264.0,
        common_ions: vec![7],
        uncommon_ions: vec![],
        xray_scattering: None,
        neutron_scattering: None,

        isotopes: vec![
            Isotope { 
                mass_number: 260,
                mass: UncertainFloat::new(260.121_8_f64, 0.006_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 261,
                mass: UncertainFloat::new(261.121_8_f64, 0.002_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 262,
                mass: UncertainFloat::new(262.123_01_f64, 0.000_40_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 263,
                mass: UncertainFloat::new(263.123_15_f64, 0.000_45_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 264,
                mass: UncertainFloat::new(264.124_73_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 265,
                mass: UncertainFloat::new(265.125_2_f64, 0.004_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 266,
                mass: UncertainFloat::new(266.127_01_f64, 0.000_38_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 267,
                mass: UncertainFloat::new(267.127_74_f64, 0.000_37_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
