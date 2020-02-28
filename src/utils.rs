use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct UncertainFloat {
    pub value: f64,
    pub uncertainty: f64
}

impl UncertainFloat {
    pub fn new(value: f64, uncertainty: f64) -> UncertainFloat {
        UncertainFloat {
            value,
            uncertainty
        }
    }
    pub fn relative_uncertainty(&self) -> f64 {
        self.uncertainty/self.value
    }
}

impl Display for UncertainFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} +/- {}", self.value, self.uncertainty)
    }
}