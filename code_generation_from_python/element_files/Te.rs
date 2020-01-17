use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 52,
        name: "Tellurium",
        symbol: "Te",
        mass: 127.6,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(5.68_f64, 0.02_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(4.23_f64, 0.04_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.09_f64, 0.06_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(4.32_f64, 0.05_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(4.7_f64, 0.1_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 106,
                mass: UncertainFloat::new(105.937_70_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 107,
                mass: UncertainFloat::new(106.935_04_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 108,
                mass: UncertainFloat::new(107.929_49_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 109,
                mass: UncertainFloat::new(108.927_460_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 110,
                mass: UncertainFloat::new(109.922_410_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 111,
                mass: UncertainFloat::new(110.921_120_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 112,
                mass: UncertainFloat::new(111.917_06_f64, 0.000_18_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 113,
                mass: UncertainFloat::new(112.915_93_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 114,
                mass: UncertainFloat::new(113.912_06_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 115,
                mass: UncertainFloat::new(114.911_58_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 116,
                mass: UncertainFloat::new(115.908_42_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 117,
                mass: UncertainFloat::new(116.908_634_f64, 0.000_020_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 118,
                mass: UncertainFloat::new(117.905_825_f64, 0.000_017_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 119,
                mass: UncertainFloat::new(118.906_408_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 120,
                mass: UncertainFloat::new(119.904_020_f64, 0.000_011_f64),
                abundance: UncertainFloat::new(0.09_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.3_f64, 0.5_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.5_f64, 0.7_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.5_f64, 0.7_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.3_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 121,
                mass: UncertainFloat::new(120.904_930_f64, 0.000_027_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 122,
                mass: UncertainFloat::new(121.903_047_1_f64, 0.000_002_0_f64),
                abundance: UncertainFloat::new(2.55_f64, 0.12_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.8_f64, 0.2_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.8_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.8_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(3.4_f64, 0.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 123,
                mass: UncertainFloat::new(122.904_273_0_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(0.89_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-0.05_f64, 0.25_f64),
                    b_p: Some(UncertainFloat::new(-1.2_f64, 0.2_f64)),
                    b_m: Some(UncertainFloat::new(3.5_f64, 0.2_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(0.002_f64, 0.003_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.52_f64, 0.05_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.52_f64, 0.05_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(418.0_f64, 30.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 124,
                mass: UncertainFloat::new(123.902_819_5_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(4.74_f64, 0.14_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.95_f64, 0.10_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(8.0_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(8.0_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(6.8_f64, 1.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 125,
                mass: UncertainFloat::new(124.904_424_7_f64, 0.000_002_0_f64),
                abundance: UncertainFloat::new(7.07_f64, 0.15_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.01_f64, 0.08_f64),
                    b_p: Some(UncertainFloat::new(4.9_f64, 0.2_f64)),
                    b_m: Some(UncertainFloat::new(5.5_f64, 0.2_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(3.17_f64, 0.10_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.008_f64, 0.008_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.18_f64, 0.10_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.55_f64, 0.16_f64)),
                }),
            },
            Isotope { 
                mass_number: 126,
                mass: UncertainFloat::new(125.903_305_5_f64, 0.000_002_0_f64),
                abundance: UncertainFloat::new(18.84_f64, 0.25_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.55_f64, 0.07_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.88_f64, 0.10_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.88_f64, 0.10_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.04_f64, 0.15_f64)),
                }),
            },
            Isotope { 
                mass_number: 127,
                mass: UncertainFloat::new(126.905_217_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 128,
                mass: UncertainFloat::new(127.904_461_4_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(31.74_f64, 0.08_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.88_f64, 0.08_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.36_f64, 0.10_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.36_f64, 0.10_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.215_f64, 0.008_f64)),
                }),
            },
            Isotope { 
                mass_number: 129,
                mass: UncertainFloat::new(128.906_596_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 130,
                mass: UncertainFloat::new(129.906_222_8_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(34.08_f64, 0.62_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.01_f64, 0.07_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.55_f64, 0.11_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.55_f64, 0.11_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.29_f64, 0.06_f64)),
                }),
            },
            Isotope { 
                mass_number: 131,
                mass: UncertainFloat::new(130.908_521_9_f64, 0.000_002_2_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 132,
                mass: UncertainFloat::new(131.908_524_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 133,
                mass: UncertainFloat::new(132.910_940_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 134,
                mass: UncertainFloat::new(133.911_540_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 135,
                mass: UncertainFloat::new(134.916_45_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 136,
                mass: UncertainFloat::new(135.920_100_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 137,
                mass: UncertainFloat::new(136.925_32_f64, 0.000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 138,
                mass: UncertainFloat::new(137.929_22_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 139,
                mass: UncertainFloat::new(138.934_73_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 140,
                mass: UncertainFloat::new(139.938_70_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 141,
                mass: UncertainFloat::new(140.944_39_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 142,
                mass: UncertainFloat::new(141.948_50_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
