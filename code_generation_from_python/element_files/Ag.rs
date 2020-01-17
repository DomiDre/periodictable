use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 47,
        name: "Silver",
        symbol: "Ag",
        mass: 107.8682,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(5.922_f64, 0.007_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(4.407_f64, 0.010_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.58_f64, 0.03_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(4.99_f64, 0.03_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(63.3_f64, 0.4_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 94,
                mass: UncertainFloat::new(93.942_78_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 95,
                mass: UncertainFloat::new(94.935_48_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 96,
                mass: UncertainFloat::new(95.930_68_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 97,
                mass: UncertainFloat::new(96.924_00_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 98,
                mass: UncertainFloat::new(97.921_76_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 99,
                mass: UncertainFloat::new(98.917_60_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 100,
                mass: UncertainFloat::new(99.916_070_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 101,
                mass: UncertainFloat::new(100.912_80_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 102,
                mass: UncertainFloat::new(101.912_000_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 103,
                mass: UncertainFloat::new(102.908_972_f64, 0.000_018_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 104,
                mass: UncertainFloat::new(103.908_628_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 105,
                mass: UncertainFloat::new(104.906_528_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 106,
                mass: UncertainFloat::new(105.906_666_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 107,
                mass: UncertainFloat::new(106.905_093_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(51.839_f64, 0.008_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.555_f64, 0.011_f64),
                    b_p: Some(UncertainFloat::new(8.14_f64, 0.09_f64)),
                    b_m: Some(UncertainFloat::new(5.8_f64, 0.3_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(7.17_f64, 0.02_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.13_f64, 0.03_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.30_f64, 0.04_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(37.6_f64, 1.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 108,
                mass: UncertainFloat::new(107.905_954_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 109,
                mass: UncertainFloat::new(108.904_756_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(48.161_f64, 0.008_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.165_f64, 0.011_f64),
                    b_p: Some(UncertainFloat::new(3.24_f64, 0.08_f64)),
                    b_m: Some(UncertainFloat::new(6.9_f64, 0.2_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(2.18_f64, 0.01_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.32_f64, 0.05_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.50_f64, 0.05_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(91.0_f64, 1.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 110,
                mass: UncertainFloat::new(109.906_110_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 111,
                mass: UncertainFloat::new(110.905_295_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 112,
                mass: UncertainFloat::new(111.907_004_f64, 0.000_018_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 113,
                mass: UncertainFloat::new(112.906_566_f64, 0.000_018_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 114,
                mass: UncertainFloat::new(113.908_808_f64, 0.000_028_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 115,
                mass: UncertainFloat::new(114.908_760_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 116,
                mass: UncertainFloat::new(115.911_360_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 117,
                mass: UncertainFloat::new(116.911_680_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 118,
                mass: UncertainFloat::new(117.914_580_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 119,
                mass: UncertainFloat::new(118.915_67_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 120,
                mass: UncertainFloat::new(119.918_790_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 121,
                mass: UncertainFloat::new(120.919_85_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 122,
                mass: UncertainFloat::new(121.923_32_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 123,
                mass: UncertainFloat::new(122.924_90_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 124,
                mass: UncertainFloat::new(123.928_53_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 125,
                mass: UncertainFloat::new(124.930_54_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 126,
                mass: UncertainFloat::new(125.934_50_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 127,
                mass: UncertainFloat::new(126.936_88_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
