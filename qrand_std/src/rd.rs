use qrand_core::rd_calculate_element;
use qrand_core::LowDiscrepancySequence;
use qrand_core::QrandCoreError;
use qrand_rd_alphas::create;

struct Rd {
    alphas: Box<[f64]>,
}

impl Rd {
    fn new(alphas: Box<[f64]>) -> Self {
        Rd { alphas }
    }
}

impl LowDiscrepancySequence for Rd {
    fn element(&self, n: usize, dim: usize) -> Result<f64, QrandCoreError> {
        rd_calculate_element(&self.alphas, n, dim)
    }
}

pub fn create_sequence(dim: usize) -> impl LowDiscrepancySequence {
    let alphas = create(dim);
    Rd::new(alphas)
}
