use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 70,
        name: "Ytterbium",
        symbol: "Yb",
        mass: 173.04,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(12.41_f64, 0.03_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(19.42_f64, 0.09_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(4.0_f64, 0.2_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(23.4_f64, 0.2_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(34.8_f64, 0.8_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 148,
                mass: UncertainFloat::new(147.966_76_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 149,
                mass: UncertainFloat::new(148.963_48_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 150,
                mass: UncertainFloat::new(149.957_99_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 151,
                mass: UncertainFloat::new(150.955_25_f64, 0.000_34_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 152,
                mass: UncertainFloat::new(151.950_17_f64, 0.000_38_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 153,
                mass: UncertainFloat::new(152.949_21_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 154,
                mass: UncertainFloat::new(153.946_24_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 155,
                mass: UncertainFloat::new(154.945_79_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 156,
                mass: UncertainFloat::new(155.942_850_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 157,
                mass: UncertainFloat::new(156.942_660_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 158,
                mass: UncertainFloat::new(157.939_858_f64, 0.000_011_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 159,
                mass: UncertainFloat::new(158.940_15_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 160,
                mass: UncertainFloat::new(159.937_56_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 161,
                mass: UncertainFloat::new(160.937_85_f64, 0.000_24_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 162,
                mass: UncertainFloat::new(161.935_75_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 163,
                mass: UncertainFloat::new(162.936_27_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 164,
                mass: UncertainFloat::new(163.934_52_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 165,
                mass: UncertainFloat::new(164.935_398_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 166,
                mass: UncertainFloat::new(165.933_880_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 167,
                mass: UncertainFloat::new(166.934_947_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 168,
                mass: UncertainFloat::new(167.933_894_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.13_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-4.07_f64, 0.02_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.13_f64, 0.02_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.13_f64, 0.02_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2230.0_f64, 40.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 169,
                mass: UncertainFloat::new(168.935_187_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 170,
                mass: UncertainFloat::new(169.934_759_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(3.04_f64, 0.15_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.8_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.8_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.8_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(11.4_f64, 1.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 171,
                mass: UncertainFloat::new(170.936_322_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(14.28_f64, 0.57_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.7_f64, 0.1_f64),
                    b_p: Some(UncertainFloat::new(6.5_f64, 0.2_f64)),
                    b_m: Some(UncertainFloat::new(19.4_f64, 0.4_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(11.7_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(3.9_f64, 0.2_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(15.6_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(48.6_f64, 2.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 172,
                mass: UncertainFloat::new(171.936_377_7_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(21.83_f64, 0.67_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.5_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(11.2_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(11.2_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.8_f64, 0.4_f64)),
                }),
            },
            Isotope { 
                mass_number: 173,
                mass: UncertainFloat::new(172.938_206_8_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(16.13_f64, 0.27_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.56_f64, 0.10_f64),
                    b_p: Some(UncertainFloat::new(2.5_f64, 0.2_f64)),
                    b_m: Some(UncertainFloat::new(13.3_f64, 0.3_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(11.5_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(3.5, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(15, 0.0)),
                    thermal_absorption_xs: Some(UncertainFloat::new(17.1_f64, 1.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 174,
                mass: UncertainFloat::new(173.938_858_1_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(31.83_f64, 0.92_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(19.2_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(46.8_f64, 0.5_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(46.8_f64, 0.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(69.4_f64, 5.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 175,
                mass: UncertainFloat::new(174.941_272_5_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 176,
                mass: UncertainFloat::new(175.942_568_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(12.76_f64, 0.41_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.7_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(9.6_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(9.6_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.85_f64, 0.05_f64)),
                }),
            },
            Isotope { 
                mass_number: 177,
                mass: UncertainFloat::new(176.945_257_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 178,
                mass: UncertainFloat::new(177.946_643_f64, 0.000_011_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 179,
                mass: UncertainFloat::new(178.950_17_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 180,
                mass: UncertainFloat::new(179.952_33_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 181,
                mass: UncertainFloat::new(180.956_15_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
