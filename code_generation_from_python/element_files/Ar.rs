use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 18,
        name: "Argon",
        symbol: "Ar",
        mass: 39.948,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(1.909_f64, 0.006_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(0.458_f64, 0.003_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.225_f64, 0.005_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(0.683_f64, 0.004_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(0.675_f64, 0.009_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 30,
                mass: UncertainFloat::new(30.021_56_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 31,
                mass: UncertainFloat::new(31.012_13_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 32,
                mass: UncertainFloat::new(31.997_660_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 33,
                mass: UncertainFloat::new(32.989_930_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 34,
                mass: UncertainFloat::new(33.980_270_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 35,
                mass: UncertainFloat::new(34.975_256_7_f64, 0.000_000_8_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 36,
                mass: UncertainFloat::new(35.967_546_28_f64, 0.000_000_27_f64),
                abundance: UncertainFloat::new(0.336_5_f64, 0.003_0_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(24.9_f64, 0.7_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(77.9_f64, 0.4_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(77.9_f64, 0.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(5.2_f64, 0.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 37,
                mass: UncertainFloat::new(36.966_775_9_f64, 0.000_000_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 38,
                mass: UncertainFloat::new(37.962_732_2_f64, 0.000_000_5_f64),
                abundance: UncertainFloat::new(0.063_2_f64, 0.000_5_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.5_f64, 3.5_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.5_f64, 3.1_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.5_f64, 3.1_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.8_f64, 0.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 39,
                mass: UncertainFloat::new(38.964_313_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 40,
                mass: UncertainFloat::new(39.962_383_123_f64, 0.000_000_003_f64),
                abundance: UncertainFloat::new(99.600_3_f64, 0.003_0_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(1.7, 0.0),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(0.421_f64, 0.003_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.421_f64, 0.003_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.660_f64, 0.009_f64)),
                }),
            },
            Isotope { 
                mass_number: 41,
                mass: UncertainFloat::new(40.964_500_8_f64, 0.000_000_7_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 42,
                mass: UncertainFloat::new(41.963_050_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 43,
                mass: UncertainFloat::new(42.965_670_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 44,
                mass: UncertainFloat::new(43.965_365_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 45,
                mass: UncertainFloat::new(44.968_090_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 46,
                mass: UncertainFloat::new(45.968_090_f64, 0.000_040_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 47,
                mass: UncertainFloat::new(46.972_19_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 48,
                mass: UncertainFloat::new(47.975_07_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 49,
                mass: UncertainFloat::new(48.982_18_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 50,
                mass: UncertainFloat::new(49.985_94_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 51,
                mass: UncertainFloat::new(50.993_24_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 52,
                mass: UncertainFloat::new(51.998_17_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 53,
                mass: UncertainFloat::new(53.006_23_f64, 0.001_07_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
