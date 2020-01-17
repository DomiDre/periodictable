use crate::utils::UncertainFloat;

pub struct NeutronScatteringFactor {
    pub b_c: UncertainFloat,
    pub b_p: Option<UncertainFloat>,
    pub b_m: Option<UncertainFloat>,
    pub coherent_scattering_xs: Option<UncertainFloat>,
    pub incoherent_scattering_xs: Option<UncertainFloat>,
    pub absorption_scattering_xs: Option<UncertainFloat>,
    pub thermal_absorption_xs: Option<UncertainFloat>
}