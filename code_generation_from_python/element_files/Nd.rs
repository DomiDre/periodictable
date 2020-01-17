use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 60,
        name: "Neodymium",
        symbol: "Nd",
        mass: 144.24,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(7.69_f64, 0.05_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(7.43_f64, 0.19_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(9.2_f64, 0.8_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(16.6_f64, 0.8_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(50.5_f64, 1.2_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 126,
                mass: UncertainFloat::new(125.943_07_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 127,
                mass: UncertainFloat::new(126.940_50_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 128,
                mass: UncertainFloat::new(127.935_39_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 129,
                mass: UncertainFloat::new(128.933_25_f64, 0.000_39_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 130,
                mass: UncertainFloat::new(129.928_78_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 131,
                mass: UncertainFloat::new(130.927_10_f64, 0.000_50_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 132,
                mass: UncertainFloat::new(131.923_12_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 133,
                mass: UncertainFloat::new(132.922_21_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 134,
                mass: UncertainFloat::new(133.918_65_f64, 0.000_36_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 135,
                mass: UncertainFloat::new(134.918_24_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 136,
                mass: UncertainFloat::new(135.915_020_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 137,
                mass: UncertainFloat::new(136.914_640_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 138,
                mass: UncertainFloat::new(137.911_93_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 139,
                mass: UncertainFloat::new(138.911_920_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 140,
                mass: UncertainFloat::new(139.909_310_f64, 0.000_021_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 141,
                mass: UncertainFloat::new(140.909_605_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 142,
                mass: UncertainFloat::new(141.907_719_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(27.2_f64, 0.5_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.7_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.5_f64, 0.6_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.5_f64, 0.6_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(18.7_f64, 0.7_f64)),
                }),
            },
            Isotope { 
                mass_number: 143,
                mass: UncertainFloat::new(142.909_810_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(12.2_f64, 0.2_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(14.0_f64, 2.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(25.0_f64, 7.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(55.0_f64, 7.0_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(80.0_f64, 2.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(337.0_f64, 10.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 144,
                mass: UncertainFloat::new(143.910_083_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(23.8_f64, 0.3_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(2.8_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.0_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.0_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(3.6_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 145,
                mass: UncertainFloat::new(144.912_569_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(8.3_f64, 0.1_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(14.0_f64, 2.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(25.0_f64, 7.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(5.0_f64, 5.0_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(30.0_f64, 9.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(42.0_f64, 2.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 146,
                mass: UncertainFloat::new(145.913_112_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(17.2_f64, 0.3_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.7_f64, 0.2_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(9.5_f64, 0.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(9.5_f64, 0.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.4_f64, 0.1_f64)),
                }),
            },
            Isotope { 
                mass_number: 147,
                mass: UncertainFloat::new(146.916_096_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 148,
                mass: UncertainFloat::new(147.916_889_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(5.7_f64, 0.1_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.7_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.1_f64, 0.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.1_f64, 0.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.5_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 149,
                mass: UncertainFloat::new(148.920_144_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 150,
                mass: UncertainFloat::new(149.920_887_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(5.6_f64, 0.2_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.28_f64, 0.20_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.5_f64, 0.3_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.5_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.2_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 151,
                mass: UncertainFloat::new(150.923_825_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 152,
                mass: UncertainFloat::new(151.924_680_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 153,
                mass: UncertainFloat::new(152.927_695_f64, 0.000_029_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 154,
                mass: UncertainFloat::new(153.929_48_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 155,
                mass: UncertainFloat::new(154.932_63_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 156,
                mass: UncertainFloat::new(155.935_20_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 157,
                mass: UncertainFloat::new(156.939_27_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 158,
                mass: UncertainFloat::new(157.941_87_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 159,
                mass: UncertainFloat::new(158.946_39_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 160,
                mass: UncertainFloat::new(159.949_39_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 161,
                mass: UncertainFloat::new(160.954_33_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
