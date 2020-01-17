use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 13,
        name: "Aluminum",
        symbol: "Al",
        mass: 26.981538,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(3.449_f64, 0.005_f64),
            b_p: Some(UncertainFloat::new(3.67_f64, 0.02_f64)),
            b_m: Some(UncertainFloat::new(3.15_f64, 0.02_f64)),
            coherent_scattering_xs: Some(UncertainFloat::new(1.495_f64, 0.004_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.008_2_f64, 0.000_6_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(1.503_f64, 0.004_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(0.231_f64, 0.003_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 21,
                mass: UncertainFloat::new(21.028_04_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 22,
                mass: UncertainFloat::new(22.019_52_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 23,
                mass: UncertainFloat::new(23.007_265_f64, 0.000_027_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 24,
                mass: UncertainFloat::new(23.999_941_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 25,
                mass: UncertainFloat::new(24.990_428_6_f64, 0.000_000_7_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 26,
                mass: UncertainFloat::new(25.986_891_66_f64, 0.000_000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 27,
                mass: UncertainFloat::new(26.981_538_44_f64, 0.000_000_14_f64),
                abundance: UncertainFloat::new(100, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 28,
                mass: UncertainFloat::new(27.981_910_18_f64, 0.000_000_15_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 29,
                mass: UncertainFloat::new(28.980_444_8_f64, 0.000_001_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 30,
                mass: UncertainFloat::new(29.982_960_f64, 0.000_015_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 31,
                mass: UncertainFloat::new(30.983_946_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 32,
                mass: UncertainFloat::new(31.988_120_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 33,
                mass: UncertainFloat::new(32.990_870_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 34,
                mass: UncertainFloat::new(33.996_93_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 35,
                mass: UncertainFloat::new(34.999_94_f64, 0.000_15_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 36,
                mass: UncertainFloat::new(36.006_35_f64, 0.000_29_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 37,
                mass: UncertainFloat::new(37.010_31_f64, 0.000_58_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 38,
                mass: UncertainFloat::new(38.016_90_f64, 0.000_60_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 39,
                mass: UncertainFloat::new(39.021_90_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
