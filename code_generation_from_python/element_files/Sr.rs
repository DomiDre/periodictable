use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 38,
        name: "Strontium",
        symbol: "Sr",
        mass: 87.62,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(7.02_f64, 0.02_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(6.19_f64, 0.04_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.06_f64, 0.11_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(6.25_f64, 0.10_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(1.28_f64, 0.06_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 73,
                mass: UncertainFloat::new(72.965_97_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 74,
                mass: UncertainFloat::new(73.956_31_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 75,
                mass: UncertainFloat::new(74.949_92_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 76,
                mass: UncertainFloat::new(75.941_61_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 77,
                mass: UncertainFloat::new(76.937_76_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 78,
                mass: UncertainFloat::new(77.932_179_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 79,
                mass: UncertainFloat::new(78.929_707_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 80,
                mass: UncertainFloat::new(79.924_525_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 81,
                mass: UncertainFloat::new(80.923_213_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 82,
                mass: UncertainFloat::new(81.918_401_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 83,
                mass: UncertainFloat::new(82.917_555_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 84,
                mass: UncertainFloat::new(83.913_425_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.56_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.0_f64, 2.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(6.0_f64, 2.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(6.0_f64, 2.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.87_f64, 0.07_f64)),
                }),
            },
            Isotope { 
                mass_number: 85,
                mass: UncertainFloat::new(84.912_933_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 86,
                mass: UncertainFloat::new(85.909_262_4_f64, 0.000_002_4_f64),
                abundance: UncertainFloat::new(9.86_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.68_f64, 0.05_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.04_f64, 0.07_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.04_f64, 0.07_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.04_f64, 0.07_f64)),
                }),
            },
            Isotope { 
                mass_number: 87,
                mass: UncertainFloat::new(86.908_879_3_f64, 0.000_002_4_f64),
                abundance: UncertainFloat::new(7.00_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.41_f64, 0.07_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(6.88_f64, 0.13_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.5_f64, 0.5_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.4_f64, 0.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(16.0_f64, 3.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 88,
                mass: UncertainFloat::new(87.905_614_3_f64, 0.000_002_4_f64),
                abundance: UncertainFloat::new(82.58_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.16_f64, 0.06_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(6.42_f64, 0.11_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(6.42_f64, 0.11_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.058_f64, 0.004_f64)),
                }),
            },
            Isotope { 
                mass_number: 89,
                mass: UncertainFloat::new(88.907_452_9_f64, 0.000_002_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 90,
                mass: UncertainFloat::new(89.907_737_6_f64, 0.000_002_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 91,
                mass: UncertainFloat::new(90.910_210_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 92,
                mass: UncertainFloat::new(91.911_030_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 93,
                mass: UncertainFloat::new(92.914_022_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 94,
                mass: UncertainFloat::new(93.915_360_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 95,
                mass: UncertainFloat::new(94.919_358_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 96,
                mass: UncertainFloat::new(95.921_680_f64, 0.000_026_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 97,
                mass: UncertainFloat::new(96.926_149_f64, 0.000_020_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 98,
                mass: UncertainFloat::new(97.928_471_f64, 0.000_027_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 99,
                mass: UncertainFloat::new(98.933_32_f64, 0.000_15_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 100,
                mass: UncertainFloat::new(99.935_35_f64, 0.000_14_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 101,
                mass: UncertainFloat::new(100.940_52_f64, 0.000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 102,
                mass: UncertainFloat::new(101.943_02_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 103,
                mass: UncertainFloat::new(102.948_95_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 104,
                mass: UncertainFloat::new(103.952_33_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
