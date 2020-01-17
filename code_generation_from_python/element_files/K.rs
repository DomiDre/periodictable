use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 19,
        name: "Potassium",
        symbol: "K",
        mass: 39.0983,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(3.67_f64, 0.02_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(1.69_f64, 0.02_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.27_f64, 0.11_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(1.96_f64, 0.11_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(2.1_f64, 0.1_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 32,
                mass: UncertainFloat::new(32.021_92_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 33,
                mass: UncertainFloat::new(33.007_26_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 34,
                mass: UncertainFloat::new(33.998_41_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 35,
                mass: UncertainFloat::new(34.988_012_f64, 0.000_021_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 36,
                mass: UncertainFloat::new(35.981_293_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 37,
                mass: UncertainFloat::new(36.973_376_91_f64, 0.000_000_29_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 38,
                mass: UncertainFloat::new(37.969_080_1_f64, 0.000_000_8_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 39,
                mass: UncertainFloat::new(38.963_706_9_f64, 0.000_000_3_f64),
                abundance: UncertainFloat::new(93.258_1_f64, 0.004_4_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.79_f64, 0.02_f64),
                    b_p: Some(UncertainFloat::new(5.15, 0.0)),
                    b_m: Some(UncertainFloat::new(1.51, 0.0)),
                    coherent_scattering_xs: Some(UncertainFloat::new(1.76_f64, 0.02_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.25_f64, 0.11_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.01_f64, 0.11_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.1_f64, 0.1_f64)),
                }),
            },
            Isotope { 
                mass_number: 40,
                mass: UncertainFloat::new(39.963_998_67_f64, 0.000_000_29_f64),
                abundance: UncertainFloat::new(0.011_7_f64, 0.000_1_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.1_f64, 1.0_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.1_f64, 0.6_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.5_f64, 0.5_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.6_f64, 0.9_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(35.0_f64, 8.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 41,
                mass: UncertainFloat::new(40.961_825_97_f64, 0.000_000_28_f64),
                abundance: UncertainFloat::new(6.730_2_f64, 0.004_4_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(2.69_f64, 0.08_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.91_f64, 0.05_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.3_f64, 0.6_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.2_f64, 0.6_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.46_f64, 0.03_f64)),
                }),
            },
            Isotope { 
                mass_number: 42,
                mass: UncertainFloat::new(41.962_403_1_f64, 0.000_000_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 43,
                mass: UncertainFloat::new(42.960_716_f64, 0.000_010_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 44,
                mass: UncertainFloat::new(43.961_560_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 45,
                mass: UncertainFloat::new(44.960_700_f64, 0.000_011_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 46,
                mass: UncertainFloat::new(45.961_976_f64, 0.000_017_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 47,
                mass: UncertainFloat::new(46.961_678_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 48,
                mass: UncertainFloat::new(47.965_513_f64, 0.000_026_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 49,
                mass: UncertainFloat::new(48.967_450_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 50,
                mass: UncertainFloat::new(49.972_78_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 51,
                mass: UncertainFloat::new(50.976_38_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 52,
                mass: UncertainFloat::new(51.982_61_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 53,
                mass: UncertainFloat::new(52.987_12_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 54,
                mass: UncertainFloat::new(53.993_99_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 55,
                mass: UncertainFloat::new(54.999_39_f64, 0.001_07_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
