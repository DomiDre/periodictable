use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 20,
        name: "Calcium",
        symbol: "Ca",
        mass: 40.078,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(4.70_f64, 0.02_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(2.78_f64, 0.02_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.05_f64, 0.03_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(2.83_f64, 0.02_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(0.43_f64, 0.02_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 34,
                mass: UncertainFloat::new(34.014_12_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 35,
                mass: UncertainFloat::new(35.004_770_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 36,
                mass: UncertainFloat::new(35.993_090_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 37,
                mass: UncertainFloat::new(36.985_872_f64, 0.000_024_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 38,
                mass: UncertainFloat::new(37.976_319_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 39,
                mass: UncertainFloat::new(38.970_717_7_f64, 0.000_001_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 40,
                mass: UncertainFloat::new(39.962_591_2_f64, 0.000_000_3_f64),
                abundance: UncertainFloat::new(96.941_f64, 0.156_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.78_f64, 0.05_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.90_f64, 0.02_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.90_f64, 0.02_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.41_f64, 0.02_f64)),
                }),
            },
            Isotope { 
                mass_number: 41,
                mass: UncertainFloat::new(40.962_278_3_f64, 0.000_000_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 42,
                mass: UncertainFloat::new(41.958_618_3_f64, 0.000_000_4_f64),
                abundance: UncertainFloat::new(0.647_f64, 0.023_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.36_f64, 0.10_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.42_f64, 0.08_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.42_f64, 0.08_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.68_f64, 0.07_f64)),
                }),
            },
            Isotope { 
                mass_number: 43,
                mass: UncertainFloat::new(42.958_766_8_f64, 0.000_000_5_f64),
                abundance: UncertainFloat::new(0.135_f64, 0.010_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-1.56_f64, 0.09_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.31_f64, 0.04_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.5_f64, 0.5_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.8_f64, 0.5_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(6.2_f64, 0.6_f64)),
                }),
            },
            Isotope { 
                mass_number: 44,
                mass: UncertainFloat::new(43.955_481_1_f64, 0.000_000_9_f64),
                abundance: UncertainFloat::new(2.086_f64, 0.110_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(1.42_f64, 0.06_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.25_f64, 0.02_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.25_f64, 0.02_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.88_f64, 0.05_f64)),
                }),
            },
            Isotope { 
                mass_number: 45,
                mass: UncertainFloat::new(44.956_185_9_f64, 0.000_001_0_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 46,
                mass: UncertainFloat::new(45.953_692_8_f64, 0.000_002_5_f64),
                abundance: UncertainFloat::new(0.004_f64, 0.003_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.55_f64, 0.21_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.6_f64, 0.2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.6_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.74_f64, 0.07_f64)),
                }),
            },
            Isotope { 
                mass_number: 47,
                mass: UncertainFloat::new(46.954_546_5_f64, 0.000_002_5_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 48,
                mass: UncertainFloat::new(47.952_534_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.187_f64, 0.021_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(0.39_f64, 0.09_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.019_f64, 0.009_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.019_f64, 0.009_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.09_f64, 0.14_f64)),
                }),
            },
            Isotope { 
                mass_number: 49,
                mass: UncertainFloat::new(48.955_673_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 50,
                mass: UncertainFloat::new(49.957_518_f64, 0.000_010_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 51,
                mass: UncertainFloat::new(50.961_47_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 52,
                mass: UncertainFloat::new(51.965_10_f64, 0.000_50_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 53,
                mass: UncertainFloat::new(52.970_05_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 54,
                mass: UncertainFloat::new(53.974_68_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 55,
                mass: UncertainFloat::new(54.980_55_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 56,
                mass: UncertainFloat::new(55.985_79_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 57,
                mass: UncertainFloat::new(56.992_36_f64, 0.001_07_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}