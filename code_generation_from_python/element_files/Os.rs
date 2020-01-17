use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 76,
        name: "Osmium",
        symbol: "Os",
        mass: 190.23,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(10.7_f64, 0.2_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(14.4_f64, 0.5_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.3_f64, 0.8_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(14.7_f64, 0.6_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(16.0_f64, 4.0_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 162,
                mass: UncertainFloat::new(161.983_82_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 163,
                mass: UncertainFloat::new(162.982_05_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 164,
                mass: UncertainFloat::new(163.977_93_f64, 0.000_38_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 165,
                mass: UncertainFloat::new(164.976_48_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 166,
                mass: UncertainFloat::new(165.972_53_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 167,
                mass: UncertainFloat::new(166.971_55_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 168,
                mass: UncertainFloat::new(167.967_830_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 169,
                mass: UncertainFloat::new(168.967_080_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 170,
                mass: UncertainFloat::new(169.963_570_f64, 0.000_014_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 171,
                mass: UncertainFloat::new(170.963_04_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 172,
                mass: UncertainFloat::new(171.960_08_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 173,
                mass: UncertainFloat::new(172.959_79_f64, 0.000_33_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 174,
                mass: UncertainFloat::new(173.957_12_f64, 0.000_50_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 175,
                mass: UncertainFloat::new(174.957_08_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 176,
                mass: UncertainFloat::new(175.954_95_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 177,
                mass: UncertainFloat::new(176.955_05_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 178,
                mass: UncertainFloat::new(177.953_35_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 179,
                mass: UncertainFloat::new(178.953_95_f64, 0.000_25_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 180,
                mass: UncertainFloat::new(179.952_35_f64, 0.000_20_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 181,
                mass: UncertainFloat::new(180.953_27_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 182,
                mass: UncertainFloat::new(181.952_186_f64, 0.000_027_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 183,
                mass: UncertainFloat::new(182.953_11_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 184,
                mass: UncertainFloat::new(183.952_491_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.02_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.0_f64, 2.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(13.0_f64, 5.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(13.0_f64, 5.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(3000.0_f64, 150.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 185,
                mass: UncertainFloat::new(184.954_043_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 186,
                mass: UncertainFloat::new(185.953_838_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(1.59_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(12.0_f64, 1.7_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(17.0_f64, 5.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(17.0_f64, 5.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(80.0_f64, 13.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 187,
                mass: UncertainFloat::new(186.955_747_9_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(1.96_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.0_f64, 2.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(13.0_f64, 5.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.3_f64, 0.3_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(13.0_f64, 5.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(320.0_f64, 10.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 188,
                mass: UncertainFloat::new(187.955_836_0_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(13.24_f64, 0.08_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.8_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.3_f64, 0.6_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.3_f64, 0.6_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(4.7_f64, 0.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 189,
                mass: UncertainFloat::new(188.958_144_9_f64, 0.000_003_0_f64),
                abundance: UncertainFloat::new(16.15_f64, 0.05_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(11.0_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(14.4_f64, 0.8_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.5_f64, 0.5_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(14.9_f64, 0.9_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(25.0_f64, 4.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 190,
                mass: UncertainFloat::new(189.958_445_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(26.26_f64, 0.02_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(11.4_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(15.2_f64, 0.8_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(15.2_f64, 0.8_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(13.1_f64, 0.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 191,
                mass: UncertainFloat::new(190.960_928_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 192,
                mass: UncertainFloat::new(191.961_479_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(40.78_f64, 0.19_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(11.9_f64, 0.4_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(16.6_f64, 1.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(16.6_f64, 1.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.0_f64, 0.1_f64)),
                }),
            },
            Isotope { 
                mass_number: 193,
                mass: UncertainFloat::new(192.964_148_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 194,
                mass: UncertainFloat::new(193.965_179_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 195,
                mass: UncertainFloat::new(194.968_12_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 196,
                mass: UncertainFloat::new(195.969_620_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
