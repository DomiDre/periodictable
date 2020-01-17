use crate::utils::UncertainFloat;

pub struct NeutronScatteringFactor {
    b_c: UncertainFloat,
    b_p: Option<UncertainFloat>,
    b_m: Option<UncertainFloat>,
    coherent_scattering_xs: Option<UncertainFloat>,
    incoherent_scattering_xs: Option<UncertainFloat>,
    absorption_scattering_xs: Option<UncertainFloat>,
    thermal_absorption_xs: Option<UncertainFloat>
}