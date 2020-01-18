use crate::Element;
use crate::{UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 0,
        name: "Neutron",
        symbol: "n",
        mass: 1.008_664_915_97_f64,
        common_ions: vec![],
        uncommon_ions: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(-37.0_f64, 0.6_f64),
            b_p: Some(UncertainFloat::new(0.0, 0.0)),
            b_m: Some(UncertainFloat::new(-37.0_f64, 0.6_f64)),
            bound_coherent_scattering_xs: Some(UncertainFloat::new(43.01_f64, 0.02_f64)),
            bound_incoherent_scattering_xs: None,
            total_bound_scattering_xs: Some(UncertainFloat::new(43.01_f64, 0.02_f64)),
            absorption_xs: Some(UncertainFloat::new(0.0, 0.0)),
        }),
        isotopes: vec![
        ]
    }
}
