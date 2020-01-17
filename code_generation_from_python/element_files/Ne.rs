use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 10,
        name: "Neon",
        symbol: "Ne",
        mass: 20.1797,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(4.566_f64, 0.006_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(2.620_f64, 0.007_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.008_f64, 0.009_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(2.628_f64, 0.006_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(0.039_f64, 0.004_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 16,
                mass: UncertainFloat::new(16.025_757_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 17,
                mass: UncertainFloat::new(17.017_700_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 18,
                mass: UncertainFloat::new(18.005_697_1_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 19,
                mass: UncertainFloat::new(19.001_879_8_f64, 0.000_000_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 20,
                mass: UncertainFloat::new(19.992_440_175_9_f64, 0.000_000_002_0_f64),
                abundance: UncertainFloat::new(90.48_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.631_f64, 0.006_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(2.695_f64, 0.007_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(2.695_f64, 0.007_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.036_f64, 0.004_f64)),
                }),
            },
            Isotope { 
                mass_number: 21,
                mass: UncertainFloat::new(20.993_846_74_f64, 0.000_000_04_f64),
                abundance: UncertainFloat::new(0.27_f64, 0.01_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.66_f64, 0.19_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.6_f64, 0.3_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.05_f64, 0.02_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.7_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.67_f64, 0.11_f64)),
                }),
            },
            Isotope { 
                mass_number: 22,
                mass: UncertainFloat::new(21.991_385_51_f64, 0.000_000_23_f64),
                abundance: UncertainFloat::new(9.25_f64, 0.03_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.87_f64, 0.01_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.88_f64, 0.01_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.88_f64, 0.01_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.046_f64, 0.006_f64)),
                }),
            },
            Isotope { 
                mass_number: 23,
                mass: UncertainFloat::new(22.994_467_34_f64, 0.000_000_26_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 24,
                mass: UncertainFloat::new(23.993_615_f64, 0.000_011_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 25,
                mass: UncertainFloat::new(24.997_790_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 26,
                mass: UncertainFloat::new(26.000_460_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 27,
                mass: UncertainFloat::new(27.007_62_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 28,
                mass: UncertainFloat::new(28.012_11_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 29,
                mass: UncertainFloat::new(29.019_35_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 30,
                mass: UncertainFloat::new(30.023_87_f64, 0.000_88_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 31,
                mass: UncertainFloat::new(31.033_11_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 32,
                mass: UncertainFloat::new(32.039_91_f64, 0.000_94_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
