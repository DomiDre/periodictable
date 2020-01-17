use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 17,
        name: "Chlorine",
        symbol: "Cl",
        mass: 35.453,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(9.579_2_f64, 0.000_8_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(11.528_f64, 0.002_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(5.3_f64, 0.5_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(16.8_f64, 0.5_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(33.5_f64, 0.3_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 28,
                mass: UncertainFloat::new(28.028_51_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 29,
                mass: UncertainFloat::new(29.014_11_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 30,
                mass: UncertainFloat::new(30.004_77_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 31,
                mass: UncertainFloat::new(30.992_420_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 32,
                mass: UncertainFloat::new(31.985_689_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 33,
                mass: UncertainFloat::new(32.977_451_8_f64, 0.000_000_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 34,
                mass: UncertainFloat::new(33.973_761_97_f64, 0.000_000_13_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 35,
                mass: UncertainFloat::new(34.968_852_71_f64, 0.000_000_04_f64),
                abundance: UncertainFloat::new(75.78_f64, 0.04_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(11.70_f64, 0.09_f64),
                    b_p: Some(UncertainFloat::new(16.3_f64, 0.2_f64)),
                    b_m: Some(UncertainFloat::new(4.0_f64, 0.3_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(17.06_f64, 0.06_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(4.7_f64, 0.6_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(21.8_f64, 0.6_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(44.1_f64, 0.4_f64)),
                }),
            },
            Isotope { 
                mass_number: 36,
                mass: UncertainFloat::new(35.968_306_95_f64, 0.000_000_08_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 37,
                mass: UncertainFloat::new(36.965_902_60_f64, 0.000_000_05_f64),
                abundance: UncertainFloat::new(24.22_f64, 0.04_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.08_f64, 0.06_f64),
                    b_p: Some(UncertainFloat::new(3.10_f64, 0.07_f64)),
                    b_m: Some(UncertainFloat::new(3.05_f64, 0.07_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(1.19_f64, 0.05_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.001_f64, 0.003_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.19_f64, 0.05_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.433_f64, 0.006_f64)),
                }),
            },
            Isotope { 
                mass_number: 38,
                mass: UncertainFloat::new(37.968_010_55_f64, 0.000_000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 39,
                mass: UncertainFloat::new(38.968_007_7_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 40,
                mass: UncertainFloat::new(39.970_420_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 41,
                mass: UncertainFloat::new(40.970_650_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 42,
                mass: UncertainFloat::new(41.973_17_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 43,
                mass: UncertainFloat::new(42.974_20_f64, 0.000_17_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 44,
                mass: UncertainFloat::new(43.978_54_f64, 0.000_24_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 45,
                mass: UncertainFloat::new(44.979_70_f64, 0.000_70_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 46,
                mass: UncertainFloat::new(45.984_12_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 47,
                mass: UncertainFloat::new(46.987_95_f64, 0.000_64_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 48,
                mass: UncertainFloat::new(47.994_85_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 49,
                mass: UncertainFloat::new(48.999_89_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 50,
                mass: UncertainFloat::new(50.007_73_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 51,
                mass: UncertainFloat::new(51.013_53_f64, 0.001_07_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
