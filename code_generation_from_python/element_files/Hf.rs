use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 72,
        name: "Hafnium",
        symbol: "Hf",
        mass: 178.49,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(7.77_f64, 0.14_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(7.6_f64, 0.3_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(2.6_f64, 0.5_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(10.2_f64, 0.4_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(104.1_f64, 0.5_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 154,
                mass: UncertainFloat::new(153.964_25_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 155,
                mass: UncertainFloat::new(154.962_76_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 156,
                mass: UncertainFloat::new(155.959_25_f64, 0.000_38_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 157,
                mass: UncertainFloat::new(156.958_13_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 158,
                mass: UncertainFloat::new(157.954_65_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 159,
                mass: UncertainFloat::new(158.954_00_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 160,
                mass: UncertainFloat::new(159.950_710_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 161,
                mass: UncertainFloat::new(160.950_330_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 162,
                mass: UncertainFloat::new(161.947_203_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 163,
                mass: UncertainFloat::new(162.947_06_f64, 0.000_34_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 164,
                mass: UncertainFloat::new(163.944_42_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 165,
                mass: UncertainFloat::new(164.944_54_f64, 0.000_40_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 166,
                mass: UncertainFloat::new(165.942_25_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 167,
                mass: UncertainFloat::new(166.942_60_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 168,
                mass: UncertainFloat::new(167.940_63_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 169,
                mass: UncertainFloat::new(168.941_160_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 170,
                mass: UncertainFloat::new(169.939_65_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 171,
                mass: UncertainFloat::new(170.940_49_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 172,
                mass: UncertainFloat::new(171.939_460_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 173,
                mass: UncertainFloat::new(172.940_65_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 174,
                mass: UncertainFloat::new(173.940_040_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.16_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.9_f64, 1.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(15.0_f64, 3.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(15.0_f64, 3.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(561.0_f64, 35.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 175,
                mass: UncertainFloat::new(174.941_503_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 176,
                mass: UncertainFloat::new(175.941_401_8_f64, 0.000_002_9_f64),
                abundance: UncertainFloat::new(5.26_f64, 0.07_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.61_f64, 0.18_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.5_f64, 0.3_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.5_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(23.5_f64, 3.1_f64)),
                }),
            },
            Isotope { 
                mass_number: 177,
                mass: UncertainFloat::new(176.943_220_0_f64, 0.000_002_7_f64),
                abundance: UncertainFloat::new(18.60_f64, 0.09_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.8_f64, 1.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.1_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.1_f64, 0.3_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.2_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(373.0_f64, 10.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 178,
                mass: UncertainFloat::new(177.943_697_7_f64, 0.000_002_7_f64),
                abundance: UncertainFloat::new(27.28_f64, 0.07_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.9_f64, 0.2_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(4.4_f64, 0.3_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.4_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(84.0_f64, 4.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 179,
                mass: UncertainFloat::new(178.945_815_1_f64, 0.000_002_7_f64),
                abundance: UncertainFloat::new(13.62_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.46_f64, 0.16_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.0_f64, 0.3_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.14_f64, 0.02_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.1_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(41.0_f64, 3.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 180,
                mass: UncertainFloat::new(179.946_548_8_f64, 0.000_002_7_f64),
                abundance: UncertainFloat::new(35.08_f64, 0.16_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(13.2_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(21.9_f64, 1.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(21.9_f64, 1.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(13.04_f64, 0.07_f64)),
                }),
            },
            Isotope { 
                mass_number: 181,
                mass: UncertainFloat::new(180.949_099_1_f64, 0.000_002_8_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 182,
                mass: UncertainFloat::new(181.950_553_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 183,
                mass: UncertainFloat::new(182.953_530_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 184,
                mass: UncertainFloat::new(183.955_450_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 185,
                mass: UncertainFloat::new(184.958_78_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 186,
                mass: UncertainFloat::new(185.960_92_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
