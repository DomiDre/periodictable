use crate::{UncertainFloat, XrayScatteringFactor, NeutronScatteringFactor};

#[derive(Debug, Clone)]
pub struct Isotope {
    pub mass_number: u16,
    pub mass: UncertainFloat,
    pub abundance: UncertainFloat,
    pub xray_scattering: Option<XrayScatteringFactor>,
    pub neutron_scattering: Option<NeutronScatteringFactor>
}
