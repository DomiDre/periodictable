use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 68,
        name: "Erbium",
        symbol: "Er",
        mass: 167.259,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(7.79_f64, 0.02_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(7.63_f64, 0.04_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(1.1_f64, 0.3_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(8.7_f64, 0.3_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(159.0_f64, 4.0_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 144,
                mass: UncertainFloat::new(143.960_59_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 145,
                mass: UncertainFloat::new(144.957_46_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 146,
                mass: UncertainFloat::new(145.952_12_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 147,
                mass: UncertainFloat::new(146.949_31_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 148,
                mass: UncertainFloat::new(147.944_44_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 149,
                mass: UncertainFloat::new(148.942_17_f64, 0.000_51_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 150,
                mass: UncertainFloat::new(149.937_76_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 151,
                mass: UncertainFloat::new(150.937_46_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 152,
                mass: UncertainFloat::new(151.935_080_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 153,
                mass: UncertainFloat::new(152.935_093_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 154,
                mass: UncertainFloat::new(153.932_777_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 155,
                mass: UncertainFloat::new(154.933_200_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 156,
                mass: UncertainFloat::new(155.931_020_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 157,
                mass: UncertainFloat::new(156.931_950_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 158,
                mass: UncertainFloat::new(157.929_91_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 159,
                mass: UncertainFloat::new(158.930_681_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 160,
                mass: UncertainFloat::new(159.929_080_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 161,
                mass: UncertainFloat::new(160.930_001_f64, 0.000_010_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 162,
                mass: UncertainFloat::new(161.928_775_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.14_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.01_f64, 0.11_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(9.7_f64, 0.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(9.7_f64, 0.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(19.0_f64, 2.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 163,
                mass: UncertainFloat::new(162.930_029_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 164,
                mass: UncertainFloat::new(163.929_197_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(1.61_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.95_f64, 0.14_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(8.4_f64, 0.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(8.4_f64, 0.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(13.0_f64, 2.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 165,
                mass: UncertainFloat::new(164.930_723_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 166,
                mass: UncertainFloat::new(165.930_290_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(33.61_f64, 0.35_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.51_f64, 0.19_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(14.1_f64, 0.5_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(14.1_f64, 0.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(19.6_f64, 1.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 167,
                mass: UncertainFloat::new(166.932_045_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(22.93_f64, 0.17_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.06_f64, 0.05_f64),
                    b_p: Some(UncertainFloat::new(5.3_f64, 0.3_f64)),
                    b_m: Some(UncertainFloat::new(0.0_f64, 0.3_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(1.1_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.13_f64, 0.06_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.2_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(659.0_f64, 16.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 168,
                mass: UncertainFloat::new(167.932_368_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(26.78_f64, 0.26_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.43_f64, 0.08_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(6.9_f64, 0.7_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(6.9_f64, 0.7_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.74_f64, 0.08_f64)),
                }),
            },
            Isotope { 
                mass_number: 169,
                mass: UncertainFloat::new(168.934_588_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 170,
                mass: UncertainFloat::new(169.935_460_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(14.93_f64, 0.27_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.61_f64, 0.06_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(11.6_f64, 1.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(11.6_f64, 1.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(5.8_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 171,
                mass: UncertainFloat::new(170.938_026_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 172,
                mass: UncertainFloat::new(171.939_352_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 173,
                mass: UncertainFloat::new(172.942_40_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 174,
                mass: UncertainFloat::new(173.944_34_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 175,
                mass: UncertainFloat::new(174.947_93_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 176,
                mass: UncertainFloat::new(175.950_29_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 177,
                mass: UncertainFloat::new(176.954_37_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
