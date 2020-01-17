use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 46,
        name: "Palladium",
        symbol: "Pd",
        mass: 106.42,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(5.91_f64, 0.06_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(4.39_f64, 0.09_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.093_f64, 0.009_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(4.48_f64, 0.09_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(6.9_f64, 0.4_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 91,
                mass: UncertainFloat::new(90.949_48_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 92,
                mass: UncertainFloat::new(91.940_42_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 93,
                mass: UncertainFloat::new(92.935_91_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 94,
                mass: UncertainFloat::new(93.928_77_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 95,
                mass: UncertainFloat::new(94.924_69_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 96,
                mass: UncertainFloat::new(95.918_22_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 97,
                mass: UncertainFloat::new(96.916_48_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 98,
                mass: UncertainFloat::new(97.912_721_f64, 0.000_023_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 99,
                mass: UncertainFloat::new(98.911_768_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 100,
                mass: UncertainFloat::new(99.908_505_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 101,
                mass: UncertainFloat::new(100.908_289_f64, 0.000_019_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 102,
                mass: UncertainFloat::new(101.905_608_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(1.02_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.7_f64, 0.7_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.5_f64, 1.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.5_f64, 1.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(3.4_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 103,
                mass: UncertainFloat::new(102.906_087_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 104,
                mass: UncertainFloat::new(103.904_035_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(11.14_f64, 0.08_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.7_f64, 0.7_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.5_f64, 1.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.5_f64, 1.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.6_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 105,
                mass: UncertainFloat::new(104.905_084_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(22.33_f64, 0.08_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.5_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.8_f64, 0.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.8_f64, 1.0_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.6_f64, 1.1_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(20.0_f64, 3.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 106,
                mass: UncertainFloat::new(105.903_483_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(27.33_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.4_f64, 0.4_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.1_f64, 0.6_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.1_f64, 0.6_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.304_f64, 0.029_f64)),
                }),
            },
            Isotope { 
                mass_number: 107,
                mass: UncertainFloat::new(106.905_128_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 108,
                mass: UncertainFloat::new(107.903_894_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(26.46_f64, 0.09_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.1_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.1_f64, 0.3_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.1_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(8.5_f64, 0.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 109,
                mass: UncertainFloat::new(108.905_954_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 110,
                mass: UncertainFloat::new(109.905_152_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(11.72_f64, 0.09_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.7_f64, 0.7_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.5_f64, 1.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.5_f64, 1.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.226_f64, 0.031_f64)),
                }),
            },
            Isotope { 
                mass_number: 111,
                mass: UncertainFloat::new(110.907_640_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 112,
                mass: UncertainFloat::new(111.907_313_f64, 0.000_019_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 113,
                mass: UncertainFloat::new(112.910_150_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 114,
                mass: UncertainFloat::new(113.910_365_f64, 0.000_026_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 115,
                mass: UncertainFloat::new(114.913_680_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 116,
                mass: UncertainFloat::new(115.914_160_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 117,
                mass: UncertainFloat::new(116.917_84_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 118,
                mass: UncertainFloat::new(117.918_98_f64, 0.000_23_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 119,
                mass: UncertainFloat::new(118.922_68_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 120,
                mass: UncertainFloat::new(119.924_03_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 121,
                mass: UncertainFloat::new(120.928_18_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 122,
                mass: UncertainFloat::new(121.929_80_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 123,
                mass: UncertainFloat::new(122.934_26_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
