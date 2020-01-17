use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 31,
        name: "Gallium",
        symbol: "Ga",
        mass: 69.723,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(7.288_f64, 0.002_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(6.675_f64, 0.004_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.16_f64, 0.03_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(6.83_f64, 0.03_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(2.75_f64, 0.03_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 56,
                mass: UncertainFloat::new(55.994_91_f64, 0.000_28_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 57,
                mass: UncertainFloat::new(56.982_93_f64, 0.000_28_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 58,
                mass: UncertainFloat::new(57.974_25_f64, 0.000_23_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 59,
                mass: UncertainFloat::new(58.963_37_f64, 0.000_18_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 60,
                mass: UncertainFloat::new(59.957_06_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 61,
                mass: UncertainFloat::new(60.949_17_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 62,
                mass: UncertainFloat::new(61.944_180_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 63,
                mass: UncertainFloat::new(62.939_14_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 64,
                mass: UncertainFloat::new(63.936_838_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 65,
                mass: UncertainFloat::new(64.932_739_3_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 66,
                mass: UncertainFloat::new(65.931_592_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 67,
                mass: UncertainFloat::new(66.928_204_9_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 68,
                mass: UncertainFloat::new(67.927_983_5_f64, 0.000_002_2_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 69,
                mass: UncertainFloat::new(68.925_581_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(60.108_f64, 0.009_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.043_f64, 0.016_f64),
                    b_p: Some(UncertainFloat::new(6.3_f64, 0.2_f64)),
                    b_m: Some(UncertainFloat::new(10.5_f64, 0.4_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(7.80_f64, 0.04_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.091_f64, 0.011_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.89_f64, 0.04_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.18_f64, 0.05_f64)),
                }),
            },
            Isotope { 
                mass_number: 70,
                mass: UncertainFloat::new(69.926_028_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 71,
                mass: UncertainFloat::new(70.924_705_0_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(39.892_f64, 0.009_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.170_f64, 0.011_f64),
                    b_p: Some(UncertainFloat::new(5.5_f64, 0.6_f64)),
                    b_m: Some(UncertainFloat::new(7.8_f64, 0.1_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(5.15_f64, 0.05_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.084_f64, 0.008_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.23_f64, 0.05_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(3.61_f64, 0.10_f64)),
                }),
            },
            Isotope { 
                mass_number: 72,
                mass: UncertainFloat::new(71.926_369_4_f64, 0.000_002_2_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 73,
                mass: UncertainFloat::new(72.925_170_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 74,
                mass: UncertainFloat::new(73.926_940_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 75,
                mass: UncertainFloat::new(74.926_501_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 76,
                mass: UncertainFloat::new(75.928_93_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 77,
                mass: UncertainFloat::new(76.929_280_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 78,
                mass: UncertainFloat::new(77.931_660_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 79,
                mass: UncertainFloat::new(78.932_92_f64, 0.000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 80,
                mass: UncertainFloat::new(79.936_59_f64, 0.000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 81,
                mass: UncertainFloat::new(80.937_75_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 82,
                mass: UncertainFloat::new(81.943_16_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 83,
                mass: UncertainFloat::new(82.946_87_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 84,
                mass: UncertainFloat::new(83.952_34_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
