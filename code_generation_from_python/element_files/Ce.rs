use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 58,
        name: "Cerium",
        symbol: "Ce",
        mass: 140.116,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(4.84_f64, 0.02_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(2.94_f64, 0.02_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.00_f64, 0.10_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(2.94_f64, 0.10_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(0.63_f64, 0.04_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 119,
                mass: UncertainFloat::new(118.952_76_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 120,
                mass: UncertainFloat::new(119.946_64_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 121,
                mass: UncertainFloat::new(120.943_67_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 122,
                mass: UncertainFloat::new(121.938_01_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 123,
                mass: UncertainFloat::new(122.935_51_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 124,
                mass: UncertainFloat::new(123.930_52_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 125,
                mass: UncertainFloat::new(124.928_54_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 126,
                mass: UncertainFloat::new(125.924_10_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 127,
                mass: UncertainFloat::new(126.922_75_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 128,
                mass: UncertainFloat::new(127.918_87_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 129,
                mass: UncertainFloat::new(128.918_09_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 130,
                mass: UncertainFloat::new(129.914_69_f64, 0.000_66_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 131,
                mass: UncertainFloat::new(130.914_42_f64, 0.000_44_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 132,
                mass: UncertainFloat::new(131.911_49_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 133,
                mass: UncertainFloat::new(132.911_55_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 134,
                mass: UncertainFloat::new(133.909_03_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 135,
                mass: UncertainFloat::new(134.909_146_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 136,
                mass: UncertainFloat::new(135.907_140_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.185_f64, 0.002_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.76_f64, 0.09_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.23_f64, 0.13_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.23_f64, 0.13_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(7.3_f64, 1.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 137,
                mass: UncertainFloat::new(136.907_780_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 138,
                mass: UncertainFloat::new(137.905_986_f64, 0.000_011_f64),
                abundance: UncertainFloat::new(0.251_f64, 0.002_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.65_f64, 0.09_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.64_f64, 0.15_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.64_f64, 0.15_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.1_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 139,
                mass: UncertainFloat::new(138.906_647_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 140,
                mass: UncertainFloat::new(139.905_434_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(88.450_f64, 0.051_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.81_f64, 0.09_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.94_f64, 0.11_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.94_f64, 0.11_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.57_f64, 0.04_f64)),
                }),
            },
            Isotope { 
                mass_number: 141,
                mass: UncertainFloat::new(140.908_271_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 142,
                mass: UncertainFloat::new(141.909_240_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(11.114_f64, 0.051_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.72_f64, 0.09_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.84_f64, 0.11_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.84_f64, 0.11_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.95_f64, 0.05_f64)),
                }),
            },
            Isotope { 
                mass_number: 143,
                mass: UncertainFloat::new(142.912_381_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 144,
                mass: UncertainFloat::new(143.913_643_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 145,
                mass: UncertainFloat::new(144.917_230_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 146,
                mass: UncertainFloat::new(145.918_690_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 147,
                mass: UncertainFloat::new(146.922_510_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 148,
                mass: UncertainFloat::new(147.924_39_f64, 0.000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 149,
                mass: UncertainFloat::new(148.928_290_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 150,
                mass: UncertainFloat::new(149.930_23_f64, 0.000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 151,
                mass: UncertainFloat::new(150.934_04_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 152,
                mass: UncertainFloat::new(151.936_38_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 153,
                mass: UncertainFloat::new(152.940_58_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 154,
                mass: UncertainFloat::new(153.943_32_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 155,
                mass: UncertainFloat::new(154.948_04_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 156,
                mass: UncertainFloat::new(155.951_26_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 157,
                mass: UncertainFloat::new(156.956_34_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
