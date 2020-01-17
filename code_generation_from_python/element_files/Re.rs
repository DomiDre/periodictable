use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 75,
        name: "Rhenium",
        symbol: "Re",
        mass: 186.207,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(9.2_f64, 0.2_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(10.6_f64, 0.5_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.9_f64, 0.6_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(11.5_f64, 0.3_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(89.7_f64, 1.0_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 160,
                mass: UncertainFloat::new(159.981_49_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 161,
                mass: UncertainFloat::new(160.977_66_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 162,
                mass: UncertainFloat::new(161.975_71_f64, 0.000_55_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 163,
                mass: UncertainFloat::new(162.971_97_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 164,
                mass: UncertainFloat::new(163.970_32_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 165,
                mass: UncertainFloat::new(164.967_050_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 166,
                mass: UncertainFloat::new(165.965_80_f64, 0.000_15_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 167,
                mass: UncertainFloat::new(166.962_56_f64, 0.000_14_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 168,
                mass: UncertainFloat::new(167.961_61_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 169,
                mass: UncertainFloat::new(168.958_83_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 170,
                mass: UncertainFloat::new(169.958_16_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 171,
                mass: UncertainFloat::new(170.955_55_f64, 0.000_37_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 172,
                mass: UncertainFloat::new(171.955_29_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 173,
                mass: UncertainFloat::new(172.953_06_f64, 0.000_48_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 174,
                mass: UncertainFloat::new(173.953_11_f64, 0.000_44_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 175,
                mass: UncertainFloat::new(174.951_39_f64, 0.000_48_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 176,
                mass: UncertainFloat::new(175.951_57_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 177,
                mass: UncertainFloat::new(176.950_27_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 178,
                mass: UncertainFloat::new(177.950_85_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 179,
                mass: UncertainFloat::new(178.949_980_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 180,
                mass: UncertainFloat::new(179.950_790_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 181,
                mass: UncertainFloat::new(180.950_065_f64, 0.000_015_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 182,
                mass: UncertainFloat::new(181.951_21_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 183,
                mass: UncertainFloat::new(182.950_821_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 184,
                mass: UncertainFloat::new(183.952_524_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 185,
                mass: UncertainFloat::new(184.952_955_7_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(37.40_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.0_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(10.2_f64, 0.7_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.5_f64, 0.9_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(10.7_f64, 0.6_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(112.0_f64, 2.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 186,
                mass: UncertainFloat::new(185.954_987_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 187,
                mass: UncertainFloat::new(186.955_750_8_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(62.60_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.3_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(10.9_f64, 0.7_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(1.0_f64, 0.6_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(11.9_f64, 0.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(76.4_f64, 1.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 188,
                mass: UncertainFloat::new(187.958_112_3_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 189,
                mass: UncertainFloat::new(188.959_228_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 190,
                mass: UncertainFloat::new(189.961_82_f64, 0.000_23_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 191,
                mass: UncertainFloat::new(190.963_124_f64, 0.000_011_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 192,
                mass: UncertainFloat::new(191.965_96_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
