use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 26,
        name: "Iron",
        symbol: "Fe",
        mass: 55.845,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(9.45_f64, 0.02_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(11.22_f64, 0.05_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.40_f64, 0.11_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(11.62_f64, 0.10_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(2.56_f64, 0.03_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 45,
                mass: UncertainFloat::new(45.014_56_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 46,
                mass: UncertainFloat::new(46.000_81_f64, 0.000_38_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 47,
                mass: UncertainFloat::new(46.992_89_f64, 0.000_28_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 48,
                mass: UncertainFloat::new(47.980_56_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 49,
                mass: UncertainFloat::new(48.973_61_f64, 0.000_17_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 50,
                mass: UncertainFloat::new(49.962_990_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 51,
                mass: UncertainFloat::new(50.956_825_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 52,
                mass: UncertainFloat::new(51.948_117_f64, 0.000_011_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 53,
                mass: UncertainFloat::new(52.945_312_3_f64, 0.000_002_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 54,
                mass: UncertainFloat::new(53.939_614_8_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(5.845_f64, 0.035_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.2_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.2_f64, 0.1_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.2_f64, 0.1_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.25_f64, 0.18_f64)),
                }),
            },
            Isotope { 
                mass_number: 55,
                mass: UncertainFloat::new(54.938_298_0_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 56,
                mass: UncertainFloat::new(55.934_942_1_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(91.754_f64, 0.036_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.1_f64, 0.2_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(12.42_f64, 0.07_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(12.42_f64, 0.07_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.59_f64, 0.14_f64)),
                }),
            },
            Isotope { 
                mass_number: 57,
                mass: UncertainFloat::new(56.935_398_7_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(2.119_f64, 0.010_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(2.3_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.66_f64, 0.06_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.3_f64, 0.3_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.0_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.48_f64, 0.30_f64)),
                }),
            },
            Isotope { 
                mass_number: 58,
                mass: UncertainFloat::new(57.933_280_5_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(0.282_f64, 0.004_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(15_f64, 7_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(28.0_f64, 26.0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(28.0_f64, 26.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.28_f64, 0.05_f64)),
                }),
            },
            Isotope { 
                mass_number: 59,
                mass: UncertainFloat::new(58.934_880_5_f64, 0.000_001_5_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 60,
                mass: UncertainFloat::new(59.934_077_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 61,
                mass: UncertainFloat::new(60.936_749_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 62,
                mass: UncertainFloat::new(61.936_770_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 63,
                mass: UncertainFloat::new(62.940_12_f64, 0.000_20_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 64,
                mass: UncertainFloat::new(63.940_87_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 65,
                mass: UncertainFloat::new(64.944_94_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 66,
                mass: UncertainFloat::new(65.945_98_f64, 0.000_35_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 67,
                mass: UncertainFloat::new(66.950_00_f64, 0.000_50_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 68,
                mass: UncertainFloat::new(67.952_51_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 69,
                mass: UncertainFloat::new(68.957_70_f64, 0.000_86_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
