use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 66,
        name: "Dysprosium",
        symbol: "Dy",
        mass: 162.5,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(16.9_f64, 0.3_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(35.9_f64, 0.8_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(54.4_f64, 1.2_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(90.3_f64, 0.9_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(994.0_f64, 13.0_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 140,
                mass: UncertainFloat::new(139.953_79_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 141,
                mass: UncertainFloat::new(140.951_19_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 142,
                mass: UncertainFloat::new(141.946_27_f64, 0.000_85_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 143,
                mass: UncertainFloat::new(142.943_83_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 144,
                mass: UncertainFloat::new(143.939_07_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 145,
                mass: UncertainFloat::new(144.936_95_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 146,
                mass: UncertainFloat::new(145.932_72_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 147,
                mass: UncertainFloat::new(146.930_880_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 148,
                mass: UncertainFloat::new(147.927_180_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 149,
                mass: UncertainFloat::new(148.927_334_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 150,
                mass: UncertainFloat::new(149.925_580_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 151,
                mass: UncertainFloat::new(150.926_180_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 152,
                mass: UncertainFloat::new(151.924_714_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 153,
                mass: UncertainFloat::new(152.925_761_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 154,
                mass: UncertainFloat::new(153.924_422_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 155,
                mass: UncertainFloat::new(154.925_749_f64, 0.000_013_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 156,
                mass: UncertainFloat::new(155.924_278_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.06_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.1_f64, 0.5_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.7_f64, 0.8_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.7_f64, 0.8_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(33.0_f64, 3.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 157,
                mass: UncertainFloat::new(156.925_461_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 158,
                mass: UncertainFloat::new(157.924_405_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.10_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.0_f64, 4.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.0_f64, 6.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5._f64, 6._f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(43.0_f64, 6.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 159,
                mass: UncertainFloat::new(158.925_736_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 160,
                mass: UncertainFloat::new(159.925_194_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(2.34_f64, 0.08_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.7_f64, 0.4_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.6_f64, 0.7_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.6_f64, 0.7_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(56.0_f64, 5.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 161,
                mass: UncertainFloat::new(160.926_930_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(18.91_f64, 0.24_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.3_f64, 0.4_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(13.3_f64, 1.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(3.0_f64, 1.0_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(16.0_f64, 1.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(600.0_f64, 25.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 162,
                mass: UncertainFloat::new(161.926_795_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(25.51_f64, 0.26_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-1.4_f64, 0.5_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.25_f64, 0.18_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.25_f64, 0.18_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(194.0_f64, 10.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 163,
                mass: UncertainFloat::new(162.928_728_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(24.90_f64, 0.16_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.0_f64, 0.4_f64),
                    b_p: Some(UncertainFloat::new(6.1_f64, 0.5_f64)),
                    b_m: Some(UncertainFloat::new(3.5_f64, 0.5_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(3.1_f64, 0.5_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.21_f64, 0.19_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.3_f64, 0.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(124.0_f64, 7.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 164,
                mass: UncertainFloat::new(163.929_171_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(28.18_f64, 0.37_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(49.4_f64, 0.5_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(307.0_f64, 3.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(307.0_f64, 3.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2840.0_f64, 40.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 165,
                mass: UncertainFloat::new(164.931_700_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 166,
                mass: UncertainFloat::new(165.932_803_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 167,
                mass: UncertainFloat::new(166.935_650_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 168,
                mass: UncertainFloat::new(167.937_23_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 169,
                mass: UncertainFloat::new(168.940_30_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 170,
                mass: UncertainFloat::new(169.942_67_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 171,
                mass: UncertainFloat::new(170.946_48_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 172,
                mass: UncertainFloat::new(171.949_11_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 173,
                mass: UncertainFloat::new(172.953_44_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
