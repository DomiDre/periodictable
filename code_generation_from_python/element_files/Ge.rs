use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 32,
        name: "Germanium",
        symbol: "Ge",
        mass: 72.64,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(8.185_f64, 0.020_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(8.42_f64, 0.04_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.18_f64, 0.07_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(8.60_f64, 0.06_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(2.20_f64, 0.04_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 58,
                mass: UncertainFloat::new(57.991_01_f64, 0.000_34_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 59,
                mass: UncertainFloat::new(58.981_75_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 60,
                mass: UncertainFloat::new(59.970_19_f64, 0.000_25_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 61,
                mass: UncertainFloat::new(60.963_79_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 62,
                mass: UncertainFloat::new(61.954_65_f64, 0.000_15_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 63,
                mass: UncertainFloat::new(62.949_64_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 64,
                mass: UncertainFloat::new(63.941_57_f64, 0.000_27_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 65,
                mass: UncertainFloat::new(64.939_44_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 66,
                mass: UncertainFloat::new(65.933_850_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 67,
                mass: UncertainFloat::new(66.932_738_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 68,
                mass: UncertainFloat::new(67.928_097_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 69,
                mass: UncertainFloat::new(68.927_972_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 70,
                mass: UncertainFloat::new(69.924_250_4_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(20.84_f64, 0.87_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.0_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(12.6_f64, 0.3_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(12.6_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(3.0_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 71,
                mass: UncertainFloat::new(70.924_954_0_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 72,
                mass: UncertainFloat::new(71.922_076_2_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(27.54_f64, 0.34_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.51_f64, 0.10_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(9.1_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(9.1_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.8_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 73,
                mass: UncertainFloat::new(72.923_459_4_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(7.73_f64, 0.05_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.02_f64, 0.04_f64),
                    b_p: Some(UncertainFloat::new(8.1_f64, 0.4_f64)),
                    b_m: Some(UncertainFloat::new(1.2_f64, 0.4_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(3.17_f64, 0.05_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(1.5_f64, 0.3_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.7_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(15.1_f64, 0.4_f64)),
                }),
            },
            Isotope { 
                mass_number: 74,
                mass: UncertainFloat::new(73.921_178_2_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(36.28_f64, 0.73_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.58_f64, 0.10_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.2_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.2_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.4_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 75,
                mass: UncertainFloat::new(74.922_859_5_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 76,
                mass: UncertainFloat::new(75.921_402_7_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(7.61_f64, 0.38_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.2_f64, 1.5_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(8.0_f64, 3.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(8.0_f64, 3.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.16_f64, 0.02_f64)),
                }),
            },
            Isotope { 
                mass_number: 77,
                mass: UncertainFloat::new(76.923_548_5_f64, 0.000_002_0_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 78,
                mass: UncertainFloat::new(77.922_853_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 79,
                mass: UncertainFloat::new(78.925_40_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 80,
                mass: UncertainFloat::new(79.925_445_f64, 0.000_025_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 81,
                mass: UncertainFloat::new(80.928_82_f64, 0.000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 82,
                mass: UncertainFloat::new(81.929_55_f64, 0.000_26_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 83,
                mass: UncertainFloat::new(82.934_51_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 84,
                mass: UncertainFloat::new(83.937_31_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 85,
                mass: UncertainFloat::new(84.942_69_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 86,
                mass: UncertainFloat::new(85.946_27_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
