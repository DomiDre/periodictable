use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 7,
        name: "Nitrogen",
        symbol: "N",
        mass: 14.0067,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(9.36_f64, 0.02_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(11.01_f64, 0.05_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.50_f64, 0.12_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(11.51_f64, 0.11_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(1.90_f64, 0.03_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 10,
                mass: UncertainFloat::new(10.042_62_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 11,
                mass: UncertainFloat::new(11.026_80_f64, 0.000_19_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 12,
                mass: UncertainFloat::new(12.018_613_2_f64, 0.000_001_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 13,
                mass: UncertainFloat::new(13.005_738_58_f64, 0.000_000_29_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 14,
                mass: UncertainFloat::new(14.003_074_005_2_f64, 0.000_000_000_9_f64),
                abundance: UncertainFloat::new(99.632_f64, 0.007_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.37_f64, 0.02_f64),
                    b_p: Some(UncertainFloat::new(10.7_f64, 0.2_f64)),
                    b_m: Some(UncertainFloat::new(6.2_f64, 0.3_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(11.03_f64, 0.05_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.50_f64, 0.12_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(11.53_f64, 0.11_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(1.91_f64, 0.03_f64)),
                }),
            },
            Isotope { 
                mass_number: 15,
                mass: UncertainFloat::new(15.000_108_898_4_f64, 0.000_000_000_9_f64),
                abundance: UncertainFloat::new(0.368_f64, 0.007_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.44_f64, 0.03_f64),
                    b_p: Some(UncertainFloat::new(6.77_f64, 0.10_f64)),
                    b_m: Some(UncertainFloat::new(6.21_f64, 0.10_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(5.21_f64, 0.05_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.000_05_f64, 0.000_10_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.21_f64, 0.05_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.000_024_f64, 0.000_008_f64)),
                }),
            },
            Isotope { 
                mass_number: 16,
                mass: UncertainFloat::new(16.006_101_4_f64, 0.000_002_8_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 17,
                mass: UncertainFloat::new(17.008_450_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 18,
                mass: UncertainFloat::new(18.014_082_f64, 0.000_021_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 19,
                mass: UncertainFloat::new(19.017_027_f64, 0.000_018_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 20,
                mass: UncertainFloat::new(20.023_370_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 21,
                mass: UncertainFloat::new(21.027_09_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 22,
                mass: UncertainFloat::new(22.034_44_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 23,
                mass: UncertainFloat::new(23.040_51_f64, 0.000_76_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 24,
                mass: UncertainFloat::new(24.050_50_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
