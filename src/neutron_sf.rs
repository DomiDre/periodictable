use crate::UncertainFloat;

pub struct NeutronScatteringFactor {
    pub b_c: UncertainFloat,
    pub b_p: Option<UncertainFloat>,
    pub b_m: Option<UncertainFloat>,
    pub bound_coherent_scattering_xs: Option<UncertainFloat>,
    pub bound_incoherent_scattering_xs: Option<UncertainFloat>,
    pub total_bound_scattering_xs: Option<UncertainFloat>,
    pub absorption_xs: Option<UncertainFloat>
}