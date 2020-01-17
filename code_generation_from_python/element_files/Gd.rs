use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 64,
        name: "Gadolinium",
        symbol: "Gd",
        mass: 157.25,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(9.5_f64, 0.2_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(29.3_f64, 0.8_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(151.0_f64, 2.0_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(180.0_f64, 2.0_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(49700.0_f64, 125.0_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 136,
                mass: UncertainFloat::new(135.947_07_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 137,
                mass: UncertainFloat::new(136.944_65_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 138,
                mass: UncertainFloat::new(137.939_97_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 139,
                mass: UncertainFloat::new(138.938_08_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 140,
                mass: UncertainFloat::new(139.933_95_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 141,
                mass: UncertainFloat::new(140.932_21_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 142,
                mass: UncertainFloat::new(141.928_23_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 143,
                mass: UncertainFloat::new(142.926_74_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 144,
                mass: UncertainFloat::new(143.922_79_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 145,
                mass: UncertainFloat::new(144.921_690_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 146,
                mass: UncertainFloat::new(145.918_305_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 147,
                mass: UncertainFloat::new(146.919_089_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 148,
                mass: UncertainFloat::new(147.918_110_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 149,
                mass: UncertainFloat::new(148.919_336_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 150,
                mass: UncertainFloat::new(149.918_655_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 151,
                mass: UncertainFloat::new(150.920_344_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 152,
                mass: UncertainFloat::new(151.919_788_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.20_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.0_f64, 3.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(13.0_f64, 8.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(13.0_f64, 8.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(735.0_f64, 20.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 153,
                mass: UncertainFloat::new(152.921_746_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 154,
                mass: UncertainFloat::new(153.920_862_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(2.18_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.0_f64, 3.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(13.0_f64, 8.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(13.0_f64, 8.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(85.0_f64, 12.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 155,
                mass: UncertainFloat::new(154.922_619_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(14.80_f64, 0.12_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(13.8_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(40.8_f64, 0.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(25.0_f64, 6.0_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(66.0_f64, 6.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(61100.0_f64, 400.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 156,
                mass: UncertainFloat::new(155.922_120_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(20.47_f64, 0.09_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.3_f64, 0.4_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.0_f64, 0.6_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.0_f64, 0.6_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.5_f64, 1.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 157,
                mass: UncertainFloat::new(156.923_957_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(15.65_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.0_f64, 2.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(650.0_f64, 4.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(394.0_f64, 7.0_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1044.0_f64, 8.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(259000.0_f64, 700.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 158,
                mass: UncertainFloat::new(157.924_101_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(24.84_f64, 0.07_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.0_f64, 2.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(10.0_f64, 5.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(10.0_f64, 5.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.2_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 159,
                mass: UncertainFloat::new(158.926_385_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 160,
                mass: UncertainFloat::new(159.927_051_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(21.86_f64, 0.19_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.15_f64, 0.05_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(10.52_f64, 0.11_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(10.52_f64, 0.11_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.77_f64, 0.02_f64)),
                }),
            },
            Isotope { 
                mass_number: 161,
                mass: UncertainFloat::new(160.929_666_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 162,
                mass: UncertainFloat::new(161.930_981_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 163,
                mass: UncertainFloat::new(162.933_99_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 164,
                mass: UncertainFloat::new(163.935_86_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 165,
                mass: UncertainFloat::new(164.939_38_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 166,
                mass: UncertainFloat::new(165.941_60_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 167,
                mass: UncertainFloat::new(166.945_57_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 168,
                mass: UncertainFloat::new(167.948_36_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 169,
                mass: UncertainFloat::new(168.952_87_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
