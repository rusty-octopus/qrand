pub use qrand_core::LowDiscrepancySequence;
pub use qrand_core::QrandCoreError;

struct LowDiscrepancySequenceWrapper {
    alphas: Vec<f64>,
    sequence: impl LowDiscrepancySequence,
}

impl LowDiscrepancySequence for LowDiscrepancySequenceWrapper {
    fn element(&self, n: usize, dim: usize) -> Result<f64, QrandCoreError> {
        self.sequence.element(n, dim)
    }
}

pub fn create_sequence(dim: usize) -> impl LowDiscrepancySequence {
    use qrand_core::create_sequence;

    use qrand_rd_alphas::create;

    let alphas = create(dim);

    let sequence = create_sequence(&alphas);

    let low_discrepancy_sequence = LowDiscrepancySequenceWrapper { alphas, sequence };

    low_discrepancy_sequence
}
