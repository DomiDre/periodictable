use crate::{UncertainFloat, Isotope, XrayScatteringFactor, NeutronScatteringFactor};

pub struct Element {
    pub atomic_number: u8,
    pub name: &'static str,
    pub symbol: &'static str,
    pub mass: f64,
    pub isotopes: Vec<Isotope>,
    pub xray_scattering: Option<XrayScatteringFactor>,
    pub neutron_scattering: Option<NeutronScatteringFactor>,
    pub common_ions: Vec<i8>,
    pub uncommon_ions: Vec<i8>
}

impl Element {
    /// Get the mass of the element averaged over the existing isotopes, weighted by their
    /// abundance
    /// Average_Mass = Sum(Abundance * Mass) / Sum(Abundance)
    pub fn averaged_mass(&self) -> UncertainFloat {
        let mut average_mass = 0.0;
        let mut average_uncertainty = 0.0;
        let mut summed_weights = 0.0;
        for isotope in self.isotopes.iter() {
            average_mass += isotope.abundance.value*isotope.mass.value;
            average_uncertainty += (isotope.abundance.value*isotope.mass.uncertainty).powi(2);
            summed_weights += isotope.abundance.value;
        }
        average_mass /= summed_weights;
        
        // add uncertainty of abundance values
        for isotope in self.isotopes.iter() {
            average_uncertainty += (isotope.mass.value.powi(2)/summed_weights.powi(2) + average_mass.powi(2))*isotope.abundance.uncertainty.powi(2);
        }
        average_uncertainty /= summed_weights.powi(2);
        UncertainFloat::new(average_mass, average_uncertainty.sqrt())
    }

    /// Get the isotope mass. If the mass number is not present in the data for the given
    /// element, None is returned
    pub fn isotope(&self, mass_number: u16) -> Option<&Isotope> {
        if let Some(idx) = self.isotopes.iter().position(|x| x.mass_number == mass_number) {
            Some(&self.isotopes[idx])
        } else {
            None
        }
    }

    /// Get the average bound coherence length of the element
    pub fn b_c(&self) -> Option<&UncertainFloat> {
        if let Some(nsf) = &self.neutron_scattering {
            Some(&nsf.b_c)
        } else {
            None
        }
    }

    /// Get the atomic scattering factors f1, f2 for X-ray scattering at energy E (in keV)
    pub fn atomic_scattering_factor(&self, energy: f64) -> Option<(Option<f64>, Option<f64>)> {
        if let Some(xsf) = &self.xray_scattering {
            // Some(&nsf.b_c)
            xsf.get_f_at_energy(energy)
        } else {
            None
        }
    }
}
