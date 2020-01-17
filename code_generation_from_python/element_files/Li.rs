use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 3,
        name: "Lithium",
        symbol: "Li",
        mass: 6.941,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(-1.90_f64, 0.03_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(0.454_f64, 0.010_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.92_f64, 0.03_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(1.37_f64, 0.03_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(70.5_f64, 0.3_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 4,
                mass: UncertainFloat::new(4.027_18_f64, 0.000_23_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 5,
                mass: UncertainFloat::new(5.012_540_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 6,
                mass: UncertainFloat::new(6.015_122_3_f64, 0.000_000_5_f64),
                abundance: UncertainFloat::new(7.59_f64, 0.04_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(2.0_f64, 0.1_f64),
                    b_p: Some(UncertainFloat::new(0.67_f64, 0.14_f64)),
                    b_m: Some(UncertainFloat::new(4.67_f64, 0.17_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(0.51_f64, 0.05_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.46_f64, 0.05_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(0.97_f64, 0.07_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(940.0_f64, 4.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 7,
                mass: UncertainFloat::new(7.016_004_0_f64, 0.000_000_5_f64),
                abundance: UncertainFloat::new(92.41_f64, 0.04_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-2.22_f64, 0.02_f64),
                    b_p: Some(UncertainFloat::new(-4.15_f64, 0.06_f64)),
                    b_m: Some(UncertainFloat::new(1.00_f64, 0.08_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(0.619_f64, 0.011_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.78_f64, 0.03_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.40_f64, 0.03_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.045_4_f64, 0.000_3_f64)),
                }),
            },
            Isotope { 
                mass_number: 8,
                mass: UncertainFloat::new(8.022_486_7_f64, 0.000_000_5_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 9,
                mass: UncertainFloat::new(9.026_789_1_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 10,
                mass: UncertainFloat::new(10.035_481_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 11,
                mass: UncertainFloat::new(11.043_796_f64, 0.000_029_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 12,
                mass: UncertainFloat::new(12.053_78_f64, 0.001_07_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
