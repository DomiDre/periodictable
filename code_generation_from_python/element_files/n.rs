use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 0,
        name: "Neutron",
        symbol: "n",
        mass: 1.00866491597,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(-37.0_f64, 0.6_f64),
            b_p: Some(UncertainFloat::new(0.0, 0.0)),
            b_m: Some(UncertainFloat::new(-37.0_f64, 0.6_f64)),
            coherent_scattering_xs: Some(UncertainFloat::new(43.01_f64, 0.02_f64)),
            incoherent_scattering_xs: None,
            absorption_scattering_xs: Some(UncertainFloat::new(43.01_f64, 0.02_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(0.0, 0.0)),
        }),
        isotopes: vec![
        ]
    }
}
