use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 48,
        name: "Cadmium",
        symbol: "Cd",
        mass: 112.411,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(4.83_f64, 0.05_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(3.04_f64, 0.06_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(3.46_f64, 0.13_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(6.50_f64, 0.12_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(2520.0_f64, 50.0_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 96,
                mass: UncertainFloat::new(95.939_77_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 97,
                mass: UncertainFloat::new(96.934_94_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 98,
                mass: UncertainFloat::new(97.927_58_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 99,
                mass: UncertainFloat::new(98.925_01_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 100,
                mass: UncertainFloat::new(99.920_23_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 101,
                mass: UncertainFloat::new(100.918_68_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 102,
                mass: UncertainFloat::new(101.914_780_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 103,
                mass: UncertainFloat::new(102.913_419_f64, 0.000_017_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 104,
                mass: UncertainFloat::new(103.909_848_f64, 0.000_010_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 105,
                mass: UncertainFloat::new(104.909_468_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 106,
                mass: UncertainFloat::new(105.906_458_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(1.25_f64, 0.06_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.0_f64, 2.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.1_f64, 2.5_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.1_f64, 2.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.0_f64, 2.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 107,
                mass: UncertainFloat::new(106.906_614_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 108,
                mass: UncertainFloat::new(107.904_183_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.89_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.31_f64, 0.24_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.7_f64, 0.1_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.7_f64, 0.1_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.1_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 109,
                mass: UncertainFloat::new(108.904_986_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 110,
                mass: UncertainFloat::new(109.903_006_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(12.49_f64, 0.18_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.78_f64, 0.08_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.4_f64, 0.1_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.4_f64, 0.1_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(11.0_f64, 1.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 111,
                mass: UncertainFloat::new(110.904_182_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(12.80_f64, 0.12_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.47_f64, 0.08_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.3_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.3_f64, 0.3_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.6_f64, 0.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(24.0_f64, 5.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 112,
                mass: UncertainFloat::new(111.902_757_2_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(24.13_f64, 0.21_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.34_f64, 0.06_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.1_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.1_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.2_f64, 0.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 113,
                mass: UncertainFloat::new(112.904_400_9_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(12.22_f64, 0.12_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-8.0_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(12.1_f64, 0.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.3_f64, 0.3_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(12.4_f64, 0.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(20600.0_f64, 400.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 114,
                mass: UncertainFloat::new(113.903_358_1_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(28.73_f64, 0.42_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.48_f64, 0.05_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.1_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.1_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.34_f64, 0.02_f64)),
                }),
            },
            Isotope { 
                mass_number: 115,
                mass: UncertainFloat::new(114.905_431_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 116,
                mass: UncertainFloat::new(115.904_755_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(7.49_f64, 0.18_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.26_f64, 0.09_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.0_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.0_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.075_f64, 0.013_f64)),
                }),
            },
            Isotope { 
                mass_number: 117,
                mass: UncertainFloat::new(116.907_218_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 118,
                mass: UncertainFloat::new(117.906_914_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 119,
                mass: UncertainFloat::new(118.909_920_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 120,
                mass: UncertainFloat::new(119.909_851_f64, 0.000_020_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 121,
                mass: UncertainFloat::new(120.912_980_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 122,
                mass: UncertainFloat::new(121.913_50_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 123,
                mass: UncertainFloat::new(122.917_000_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 124,
                mass: UncertainFloat::new(123.917_650_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 125,
                mass: UncertainFloat::new(124.921_250_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 126,
                mass: UncertainFloat::new(125.922_350_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 127,
                mass: UncertainFloat::new(126.926_430_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 128,
                mass: UncertainFloat::new(127.927_76_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 129,
                mass: UncertainFloat::new(128.932_26_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 130,
                mass: UncertainFloat::new(129.933_98_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
