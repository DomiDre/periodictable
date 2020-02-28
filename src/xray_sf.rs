#[derive(Debug, Copy, Clone)]
pub struct AtomicScatteringFactor {
    pub energy: f64,
    pub f1: Option<f64>,
    pub f2: Option<f64>
}

#[derive(Debug, Clone)]
pub struct XrayScatteringFactor {
    pub table: Vec<AtomicScatteringFactor> // E, f1, f2
}

impl XrayScatteringFactor {

    /// For a given energy E, return linear interpolation of f1 and f2 from table 
    pub fn get_f_at_energy(&self, energy: f64) -> Option<AtomicScatteringFactor> {
        if let Some(idx) = self.table.iter().position(|x| x.energy > energy) {
            if idx == self.table.len() - 1  || idx == 0 {
                let edge_value = &self.table[idx];
                Some(AtomicScatteringFactor { energy:edge_value.energy, f1:edge_value.f1, f2:edge_value.f2 } )
            } else {
                let lower = &self.table[idx-1];
                let higher = &self.table[idx];
                let f1_value = if let Some(f1_lower) = lower.f1 {
                    if let Some(f1_higher) = higher.f1 {
                        Some(
                            f1_lower + 
                            (f1_higher - f1_lower)*
                            (energy - lower.energy)/(higher.energy - lower.energy)
                        )
                    } else {
                        None
                    }
                } else {
                    None
                };
                let f2_value = if let Some(f2_lower) = lower.f2 {
                    if let Some(f2_higher) = higher.f2 {
                        Some(
                            f2_lower + 
                            (f2_higher - f2_lower)*
                            (energy - lower.energy)/(higher.energy - lower.energy)
                        )
                    } else {
                        None
                    }
                } else {
                    None
                };
                Some( AtomicScatteringFactor { energy, f1: f1_value, f2: f2_value })
            }
        } else {
            None
        }
    }
}