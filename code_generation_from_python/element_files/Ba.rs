use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 56,
        name: "Barium",
        symbol: "Ba",
        mass: 137.327,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(5.07_f64, 0.03_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(3.23_f64, 0.04_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.15_f64, 0.11_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(3.38_f64, 0.10_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(1.1_f64, 0.1_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 114,
                mass: UncertainFloat::new(113.950_94_f64, 0.000_48_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 115,
                mass: UncertainFloat::new(114.947_71_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 116,
                mass: UncertainFloat::new(115.941_68_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 117,
                mass: UncertainFloat::new(116.938_86_f64, 0.000_70_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 118,
                mass: UncertainFloat::new(117.933_44_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 119,
                mass: UncertainFloat::new(118.931_05_f64, 0.001_09_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 120,
                mass: UncertainFloat::new(119.926_05_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 121,
                mass: UncertainFloat::new(120.924_49_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 122,
                mass: UncertainFloat::new(121.920_26_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 123,
                mass: UncertainFloat::new(122.918_85_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 124,
                mass: UncertainFloat::new(123.915_088_f64, 0.000_015_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 125,
                mass: UncertainFloat::new(124.914_62_f64, 0.000_27_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 126,
                mass: UncertainFloat::new(125.911_244_f64, 0.000_015_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 127,
                mass: UncertainFloat::new(126.911_12_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 128,
                mass: UncertainFloat::new(127.908_309_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 129,
                mass: UncertainFloat::new(128.908_674_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 130,
                mass: UncertainFloat::new(129.906_310_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.106_f64, 0.001_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-3.6_f64, 0.6_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.6_f64, 0.5_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.6_f64, 0.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(30.0_f64, 5.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 131,
                mass: UncertainFloat::new(130.906_931_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 132,
                mass: UncertainFloat::new(131.905_056_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.101_f64, 0.001_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.8_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.6_f64, 0.6_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.6_f64, 0.6_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(7.0_f64, 0.8_f64)),
                }),
            },
            Isotope { 
                mass_number: 133,
                mass: UncertainFloat::new(132.906_002_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 134,
                mass: UncertainFloat::new(133.904_503_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(2.417_f64, 0.018_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.7_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.08_f64, 0.14_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.08_f64, 0.14_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.0_f64, 1.6_f64)),
                }),
            },
            Isotope { 
                mass_number: 135,
                mass: UncertainFloat::new(134.905_683_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(6.592_f64, 0.012_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.66_f64, 0.10_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.74_f64, 0.12_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.5_f64, 0.5_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.2_f64, 0.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(5.8_f64, 0.9_f64)),
                }),
            },
            Isotope { 
                mass_number: 136,
                mass: UncertainFloat::new(135.904_570_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(7.854_f64, 0.024_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.90_f64, 0.08_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.03_f64, 0.10_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.03_f64, 0.10_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.68_f64, 0.17_f64)),
                }),
            },
            Isotope { 
                mass_number: 137,
                mass: UncertainFloat::new(136.905_821_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(11.232_f64, 0.024_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.82_f64, 0.10_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.86_f64, 0.17_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.5_f64, 0.5_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(6.4_f64, 0.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(3.6_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 138,
                mass: UncertainFloat::new(137.905_241_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(71.698_f64, 0.042_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.83_f64, 0.08_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.94_f64, 0.10_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.94_f64, 0.19_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.27_f64, 0.14_f64)),
                }),
            },
            Isotope { 
                mass_number: 139,
                mass: UncertainFloat::new(138.908_835_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 140,
                mass: UncertainFloat::new(139.910_599_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 141,
                mass: UncertainFloat::new(140.914_406_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 142,
                mass: UncertainFloat::new(141.916_448_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 143,
                mass: UncertainFloat::new(142.920_617_f64, 0.000_014_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 144,
                mass: UncertainFloat::new(143.922_940_f64, 0.000_015_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 145,
                mass: UncertainFloat::new(144.926_920_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 146,
                mass: UncertainFloat::new(145.930_110_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 147,
                mass: UncertainFloat::new(146.933_99_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 148,
                mass: UncertainFloat::new(147.937_68_f64, 0.000_15_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 149,
                mass: UncertainFloat::new(148.942_46_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 150,
                mass: UncertainFloat::new(149.945_62_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 151,
                mass: UncertainFloat::new(150.950_70_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 152,
                mass: UncertainFloat::new(151.954_16_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 153,
                mass: UncertainFloat::new(152.959_61_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
