pub struct AtomicScatteringFactor {
    pub energy: f64,
    pub f1: Option<f64>,
    pub f2: Option<f64>
}

pub struct XrayScatteringFactor {
    pub table: Vec<AtomicScatteringFactor> // E, f1, f2
}