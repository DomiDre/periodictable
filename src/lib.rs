// pub mod element_basic_info;
// pub mod element_neutron_scattering;
pub mod isotope;
pub mod element;
pub mod elements;
pub mod neutron_sf;
pub mod table;
pub mod utils;

use element::Element;
use isotope::Isotope;
use utils::UncertainFloat;
use neutron_sf::NeutronScatteringFactor;

// use element_basic_info::ElementBasicInfo;
// use element_neutron_scattering::ElementNeutronScattering;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
