use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 74,
        name: "Tungsten",
        symbol: "W",
        mass: 183.84,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(4.755_f64, 0.018_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(2.97_f64, 0.02_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(1.63_f64, 0.06_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(4.60_f64, 0.06_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(18.3_f64, 0.2_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 158,
                mass: UncertainFloat::new(157.973_94_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 159,
                mass: UncertainFloat::new(158.972_28_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 160,
                mass: UncertainFloat::new(159.968_37_f64, 0.000_38_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 161,
                mass: UncertainFloat::new(160.967_09_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 162,
                mass: UncertainFloat::new(161.963_34_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 163,
                mass: UncertainFloat::new(162.962_53_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 164,
                mass: UncertainFloat::new(163.958_980_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 165,
                mass: UncertainFloat::new(164.958_340_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 166,
                mass: UncertainFloat::new(165.955_020_f64, 0.000_013_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 167,
                mass: UncertainFloat::new(166.954_67_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 168,
                mass: UncertainFloat::new(167.951_86_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 169,
                mass: UncertainFloat::new(168.951_76_f64, 0.000_34_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 170,
                mass: UncertainFloat::new(169.949_29_f64, 0.000_51_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 171,
                mass: UncertainFloat::new(170.949_46_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 172,
                mass: UncertainFloat::new(171.947_42_f64, 0.000_29_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 173,
                mass: UncertainFloat::new(172.947_83_f64, 0.000_40_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 174,
                mass: UncertainFloat::new(173.946_16_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 175,
                mass: UncertainFloat::new(174.946_77_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 176,
                mass: UncertainFloat::new(175.945_59_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 177,
                mass: UncertainFloat::new(176.946_62_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 178,
                mass: UncertainFloat::new(177.945_85_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 179,
                mass: UncertainFloat::new(178.947_072_f64, 0.000_017_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 180,
                mass: UncertainFloat::new(179.946_706_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.12_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.0_f64, 3.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(3.0_f64, 4.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.0_f64, 4.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(30.0_f64, 20.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 181,
                mass: UncertainFloat::new(180.948_198_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 182,
                mass: UncertainFloat::new(181.948_206_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(26.50_f64, 0.16_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.04_f64, 0.04_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(6.10_f64, 0.07_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(6.10_f64, 0.07_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(20.7_f64, 0.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 183,
                mass: UncertainFloat::new(182.950_224_5_f64, 0.000_002_9_f64),
                abundance: UncertainFloat::new(14.31_f64, 0.04_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.59_f64, 0.04_f64),
                    b_p: Some(UncertainFloat::new(6.3_f64, 0.4_f64)),
                    b_m: Some(UncertainFloat::new(7.0_f64, 0.4_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(5.36_f64, 0.07_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.3_f64, 0.3_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.7_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(10.1_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 184,
                mass: UncertainFloat::new(183.950_932_6_f64, 0.000_002_9_f64),
                abundance: UncertainFloat::new(30.64_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.55_f64, 0.06_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.03_f64, 0.11_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.03_f64, 0.11_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.7_f64, 0.1_f64)),
                }),
            },
            Isotope { 
                mass_number: 185,
                mass: UncertainFloat::new(184.953_420_6_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 186,
                mass: UncertainFloat::new(185.954_362_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(28.43_f64, 0.19_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-0.73_f64, 0.04_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.065_f64, 0.007_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.065_f64, 0.007_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(37.9_f64, 0.6_f64)),
                }),
            },
            Isotope { 
                mass_number: 187,
                mass: UncertainFloat::new(186.957_158_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 188,
                mass: UncertainFloat::new(187.958_487_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 189,
                mass: UncertainFloat::new(188.961_91_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 190,
                mass: UncertainFloat::new(189.963_18_f64, 0.000_24_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
