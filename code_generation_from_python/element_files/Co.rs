use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 27,
        name: "Cobalt",
        symbol: "Co",
        mass: 58.9332,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(2.49_f64, 0.02_f64),
            b_p: Some(UncertainFloat::new(-9.21_f64, 0.10_f64)),
            b_m: Some(UncertainFloat::new(3.58_f64, 0.10_f64)),
            coherent_scattering_xs: Some(UncertainFloat::new(0.779_f64, 0.013_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(4.8_f64, 0.3_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(5.6_f64, 0.3_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(37.18_f64, 0.06_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 48,
                mass: UncertainFloat::new(48.001_76_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 49,
                mass: UncertainFloat::new(48.989_72_f64, 0.000_28_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 50,
                mass: UncertainFloat::new(49.981_54_f64, 0.000_18_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 51,
                mass: UncertainFloat::new(50.970_72_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 52,
                mass: UncertainFloat::new(51.963_590_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 53,
                mass: UncertainFloat::new(52.954_225_f64, 0.000_019_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 54,
                mass: UncertainFloat::new(53.948_464_1_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 55,
                mass: UncertainFloat::new(54.942_003_1_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 56,
                mass: UncertainFloat::new(55.939_843_9_f64, 0.000_002_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 57,
                mass: UncertainFloat::new(56.936_296_2_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 58,
                mass: UncertainFloat::new(57.935_757_6_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 59,
                mass: UncertainFloat::new(58.933_200_2_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(100, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 60,
                mass: UncertainFloat::new(59.933_822_2_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 61,
                mass: UncertainFloat::new(60.932_479_4_f64, 0.000_001_7_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 62,
                mass: UncertainFloat::new(61.934_054_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 63,
                mass: UncertainFloat::new(62.933_615_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 64,
                mass: UncertainFloat::new(63.935_814_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 65,
                mass: UncertainFloat::new(64.936_485_f64, 0.000_014_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 66,
                mass: UncertainFloat::new(65.939_83_f64, 0.000_29_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 67,
                mass: UncertainFloat::new(66.940_61_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 68,
                mass: UncertainFloat::new(67.944_36_f64, 0.000_35_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 69,
                mass: UncertainFloat::new(68.945_20_f64, 0.000_40_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 70,
                mass: UncertainFloat::new(69.949_81_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 71,
                mass: UncertainFloat::new(70.951_73_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 72,
                mass: UncertainFloat::new(71.956_41_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
