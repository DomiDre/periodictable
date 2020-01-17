use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 44,
        name: "Ruthenium",
        symbol: "Ru",
        mass: 101.07,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(7.02_f64, 0.02_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(6.21_f64, 0.05_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.4_f64, 0.1_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(6.6_f64, 0.1_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(2.56_f64, 0.13_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 87,
                mass: UncertainFloat::new(86.949_18_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 88,
                mass: UncertainFloat::new(87.940_42_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 89,
                mass: UncertainFloat::new(88.936_11_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 90,
                mass: UncertainFloat::new(89.929_78_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 91,
                mass: UncertainFloat::new(90.926_38_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 92,
                mass: UncertainFloat::new(91.920_12_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 93,
                mass: UncertainFloat::new(92.917_050_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 94,
                mass: UncertainFloat::new(93.911_360_f64, 0.000_014_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 95,
                mass: UncertainFloat::new(94.910_413_f64, 0.000_013_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 96,
                mass: UncertainFloat::new(95.907_598_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(5.54_f64, 0.14_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.0, 0.0),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: None,
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: None,
                    thermal_absorption_xs: Some(UncertainFloat::new(0.28_f64, 0.02_f64)),
                }),
            },
            Isotope { 
                mass_number: 97,
                mass: UncertainFloat::new(96.907_555_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 98,
                mass: UncertainFloat::new(97.905_287_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(1.87_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.0, 0.0),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: None,
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: None,
                    thermal_absorption_xs: Some(UncertainFloat::new(0.0, 0.0)),
                }),
            },
            Isotope { 
                mass_number: 99,
                mass: UncertainFloat::new(98.905_939_3_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(12.76_f64, 0.14_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.0, 0.0),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: None,
                    incoherent_scattering_xs: None,
                    absorption_scattering_xs: None,
                    thermal_absorption_xs: Some(UncertainFloat::new(6.9_f64, 1.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 100,
                mass: UncertainFloat::new(99.904_219_7_f64, 0.000_002_2_f64),
                abundance: UncertainFloat::new(12.60_f64, 0.07_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.0, 0.0),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: None,
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: None,
                    thermal_absorption_xs: Some(UncertainFloat::new(4.8_f64, 0.6_f64)),
                }),
            },
            Isotope { 
                mass_number: 101,
                mass: UncertainFloat::new(100.905_582_2_f64, 0.000_002_2_f64),
                abundance: UncertainFloat::new(17.06_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.0, 0.0),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: None,
                    incoherent_scattering_xs: None,
                    absorption_scattering_xs: None,
                    thermal_absorption_xs: Some(UncertainFloat::new(3.3_f64, 0.9_f64)),
                }),
            },
            Isotope { 
                mass_number: 102,
                mass: UncertainFloat::new(101.904_349_5_f64, 0.000_002_2_f64),
                abundance: UncertainFloat::new(31.55_f64, 0.14_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.0, 0.0),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: None,
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: None,
                    thermal_absorption_xs: Some(UncertainFloat::new(1.17_f64, 0.07_f64)),
                }),
            },
            Isotope { 
                mass_number: 103,
                mass: UncertainFloat::new(102.906_323_7_f64, 0.000_002_2_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 104,
                mass: UncertainFloat::new(103.905_430_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(18.62_f64, 0.27_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.0, 0.0),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: None,
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: None,
                    thermal_absorption_xs: Some(UncertainFloat::new(0.31_f64, 0.02_f64)),
                }),
            },
            Isotope { 
                mass_number: 105,
                mass: UncertainFloat::new(104.907_750_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 106,
                mass: UncertainFloat::new(105.907_327_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 107,
                mass: UncertainFloat::new(106.909_91_f64, 0.000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 108,
                mass: UncertainFloat::new(107.910_19_f64, 0.000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 109,
                mass: UncertainFloat::new(108.913_200_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 110,
                mass: UncertainFloat::new(109.913_97_f64, 0.000_25_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 111,
                mass: UncertainFloat::new(110.917_56_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 112,
                mass: UncertainFloat::new(111.918_55_f64, 0.000_58_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 113,
                mass: UncertainFloat::new(112.922_54_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 114,
                mass: UncertainFloat::new(113.924_00_f64, 0.000_39_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 115,
                mass: UncertainFloat::new(114.928_31_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 116,
                mass: UncertainFloat::new(115.930_16_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 117,
                mass: UncertainFloat::new(116.934_79_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 118,
                mass: UncertainFloat::new(117.937_03_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
