use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 34,
        name: "Selenium",
        symbol: "Se",
        mass: 78.96,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(7.970_f64, 0.009_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(7.98_f64, 0.02_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.32_f64, 0.06_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(8.30_f64, 0.06_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(11.7_f64, 0.2_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 65,
                mass: UncertainFloat::new(64.964_66_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 66,
                mass: UncertainFloat::new(65.955_21_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 67,
                mass: UncertainFloat::new(66.950_09_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 68,
                mass: UncertainFloat::new(67.941_87_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 69,
                mass: UncertainFloat::new(68.939_560_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 70,
                mass: UncertainFloat::new(69.933_50_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 71,
                mass: UncertainFloat::new(70.932_27_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 72,
                mass: UncertainFloat::new(71.927_112_f64, 0.000_013_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 73,
                mass: UncertainFloat::new(72.926_767_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 74,
                mass: UncertainFloat::new(73.922_476_6_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(0.89_f64, 0.04_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.8_f64, 3.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.1_f64, 0.6_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.1_f64, 0.6_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(51.8_f64, 1.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 75,
                mass: UncertainFloat::new(74.922_523_6_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 76,
                mass: UncertainFloat::new(75.919_214_1_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(9.37_f64, 0.29_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(12.2_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(18.7_f64, 0.3_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(18.7_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(85.0_f64, 7.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 77,
                mass: UncertainFloat::new(76.919_914_6_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(7.63_f64, 0.16_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.25_f64, 0.08_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(8.6_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.05_f64, 0.25_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(8.65_f64, 0.16_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(42.0_f64, 4.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 78,
                mass: UncertainFloat::new(77.917_309_5_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(23.77_f64, 0.28_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.24_f64, 0.09_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(8.5_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(8.5_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.43_f64, 0.02_f64)),
                }),
            },
            Isotope { 
                mass_number: 79,
                mass: UncertainFloat::new(78.918_499_8_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 80,
                mass: UncertainFloat::new(79.916_521_8_f64, 0.000_002_0_f64),
                abundance: UncertainFloat::new(49.61_f64, 0.41_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.48_f64, 0.03_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.03_f64, 0.06_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.03_f64, 0.06_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.61_f64, 0.05_f64)),
                }),
            },
            Isotope { 
                mass_number: 81,
                mass: UncertainFloat::new(80.917_992_9_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 82,
                mass: UncertainFloat::new(81.916_700_0_f64, 0.000_002_2_f64),
                abundance: UncertainFloat::new(8.73_f64, 0.22_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.34_f64, 0.08_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.05_f64, 0.13_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.05_f64, 0.13_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.044_f64, 0.003_f64)),
                }),
            },
            Isotope { 
                mass_number: 83,
                mass: UncertainFloat::new(82.919_119_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 84,
                mass: UncertainFloat::new(83.918_465_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 85,
                mass: UncertainFloat::new(84.922_240_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 86,
                mass: UncertainFloat::new(85.924_271_f64, 0.000_017_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 87,
                mass: UncertainFloat::new(86.928_520_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 88,
                mass: UncertainFloat::new(87.931_420_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 89,
                mass: UncertainFloat::new(88.936_02_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 90,
                mass: UncertainFloat::new(89.939_42_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 91,
                mass: UncertainFloat::new(90.945_37_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 92,
                mass: UncertainFloat::new(91.949_33_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
