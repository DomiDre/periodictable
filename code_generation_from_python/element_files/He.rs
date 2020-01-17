use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 2,
        name: "Helium",
        symbol: "He",
        mass: 4.002602,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(3.26_f64, 0.03_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(1.34_f64, 0.02_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
            absorption_scattering_xs: Some(UncertainFloat::new(1.34_f64, 0.02_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(0.007_47_f64, 0.000_01_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 3,
                mass: UncertainFloat::new(3.016_029_309_7_f64, 0.000_000_000_9_f64),
                abundance: UncertainFloat::new(0.000_137_f64, 0.000_003_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.74_f64, 0.07_f64),
                    b_p: Some(UncertainFloat::new(4.7_f64, 0.5_f64)),
                    b_m: Some(UncertainFloat::new(8.8_f64, 1.4_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(4.42_f64, 0.10_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(1.6_f64, 0.4_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(6.0_f64, 0.4_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(5333.0_f64, 7.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 4,
                mass: UncertainFloat::new(4.002_603_249_7_f64, 0.000_000_001_0_f64),
                abundance: UncertainFloat::new(99.999_863_f64, 0.000_003_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.26_f64, 0.03_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(1.34_f64, 0.02_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(1.34_f64, 0.02_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.0, 0.0)),
                }),
            },
            Isotope { 
                mass_number: 5,
                mass: UncertainFloat::new(5.012_220_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 6,
                mass: UncertainFloat::new(6.018_888_1_f64, 0.000_001_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 7,
                mass: UncertainFloat::new(7.028_030_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 8,
                mass: UncertainFloat::new(8.033_922_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 9,
                mass: UncertainFloat::new(9.043_820_f64, 0.000_070_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 10,
                mass: UncertainFloat::new(10.052_400_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
