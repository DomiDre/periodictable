use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 6,
        name: "Carbon",
        symbol: "C",
        mass: 12.0107,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(6.648_4_f64, 0.001_3_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(5.551_f64, 0.002_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.001_f64, 0.004_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(5.551_f64, 0.003_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(0.003_50_f64, 0.000_07_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 8,
                mass: UncertainFloat::new(8.037_675_f64, 0.000_025_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 9,
                mass: UncertainFloat::new(9.031_040_1_f64, 0.000_002_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 10,
                mass: UncertainFloat::new(10.016_853_1_f64, 0.000_000_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 11,
                mass: UncertainFloat::new(11.011_433_8_f64, 0.000_001_0_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 12,
                mass: UncertainFloat::new(12.000_000_0_f64, 0.000_000_0_f64),
                abundance: UncertainFloat::new(98.93_f64, 0.08_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.653_5_f64, 0.001_4_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(5.559_f64, 0.003_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.559_f64, 0.003_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.003_53_f64, 0.000_07_f64)),
                }),
            },
            Isotope { 
                mass_number: 13,
                mass: UncertainFloat::new(13.003_354_837_8_f64, 0.000_000_001_0_f64),
                abundance: UncertainFloat::new(1.07_f64, 0.08_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.19_f64, 0.09_f64),
                    b_p: Some(UncertainFloat::new(5.6_f64, 0.5_f64)),
                    b_m: Some(UncertainFloat::new(6.2_f64, 0.5_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(4.81_f64, 0.14_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.034_f64, 0.011_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(4.84_f64, 0.14_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.001_37_f64, 0.000_04_f64)),
                }),
            },
            Isotope { 
                mass_number: 14,
                mass: UncertainFloat::new(14.003_241_988_f64, 0.000_000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 15,
                mass: UncertainFloat::new(15.010_599_3_f64, 0.000_000_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 16,
                mass: UncertainFloat::new(16.014_701_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 17,
                mass: UncertainFloat::new(17.022_584_f64, 0.000_019_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 18,
                mass: UncertainFloat::new(18.026_760_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 19,
                mass: UncertainFloat::new(19.035_25_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 20,
                mass: UncertainFloat::new(20.040_32_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 21,
                mass: UncertainFloat::new(21.049_34_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 22,
                mass: UncertainFloat::new(22.056_45_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
