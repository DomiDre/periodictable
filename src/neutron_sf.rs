use crate::UncertainFloat;

#[derive(Debug, Clone, Copy)]
pub struct NeutronScatteringFactor {
    pub b_c: UncertainFloat, //bound coherent scattering length (in fm)
    pub b_p: Option<UncertainFloat>, 
    pub b_m: Option<UncertainFloat>,
    pub bound_coherent_scattering_xs: Option<UncertainFloat>, //bound coherent scattering cross section (in barn)
    pub bound_incoherent_scattering_xs: Option<UncertainFloat>, //bound incoherent scattering cross section (in barn)
    pub total_bound_scattering_xs: Option<UncertainFloat>, //total bound scattering cross section (in barn)
    pub absorption_xs: Option<UncertainFloat> //absorption cross section for 2200 m/s neutrons  (in barn)
}