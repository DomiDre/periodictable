use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 1,
        name: "Hydrogen",
        symbol: "H",
        mass: 1.00794,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(-3.740_9_f64, 0.001_1_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(1.756_8_f64, 0.001_0_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(80.26_f64, 0.06_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(82.02_f64, 0.06_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(0.332_6_f64, 0.000_7_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 1,
                mass: UncertainFloat::new(1.007_825_032_1_f64, 0.000_000_000_4_f64),
                abundance: UncertainFloat::new(99.988_5_f64, 0.007_0_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-3.742_3_f64, 0.001_2_f64),
                    b_p: Some(UncertainFloat::new(10.817_f64, 0.005_f64)),
                    b_m: Some(UncertainFloat::new(-47.420_f64, 0.014_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(1.758_3_f64, 0.001_0_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(80.27_f64, 0.06_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(82.03_f64, 0.06_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.332_6_f64, 0.000_7_f64)),
                }),
            },
            Isotope { 
                mass_number: 2,
                mass: UncertainFloat::new(2.014_101_778_0_f64, 0.000_000_000_4_f64),
                abundance: UncertainFloat::new(0.011_5_f64, 0.007_0_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.674_f64, 0.006_f64),
                    b_p: Some(UncertainFloat::new(9.53_f64, 0.03_f64)),
                    b_m: Some(UncertainFloat::new(0.975_f64, 0.060_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(5.592_f64, 0.007_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(2.05_f64, 0.03_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.64_f64, 0.03_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.000_519_f64, 0.000_007_f64)),
                }),
            },
            Isotope { 
                mass_number: 3,
                mass: UncertainFloat::new(3.016_049_267_5_f64, 0.000_000_001_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.792_f64, 0.027_f64),
                    b_p: Some(UncertainFloat::new(4.18_f64, 0.15_f64)),
                    b_m: Some(UncertainFloat::new(6.56_f64, 0.37_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(2.89_f64, 0.03_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.14_f64, 0.04_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(3.03_f64, 0.05_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.0, 0.0)),
                }),
            },
            Isotope { 
                mass_number: 4,
                mass: UncertainFloat::new(4.027_83_f64, 0.000_12_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 5,
                mass: UncertainFloat::new(5.039_54_f64, 0.001_02_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 6,
                mass: UncertainFloat::new(6.044_94_f64, 0.000_28_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
