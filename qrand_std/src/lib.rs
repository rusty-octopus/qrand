pub use qrand_core::LowDiscrepancySequence;
pub use qrand_core::QrandCoreError;

struct LowDiscrepancySequenceWrapper<T: LowDiscrepancySequence> {
    alphas: Box<[f64]>,
    sequence: T,
}

impl<'a, T: LowDiscrepancySequence> LowDiscrepancySequence for LowDiscrepancySequenceWrapper<T> {
    fn element(&self, n: usize, dim: usize) -> Result<f64, QrandCoreError> {
        self.sequence.element(n, dim)
    }
}

pub fn create_sequence<T: LowDiscrepancySequence>(
    dim: usize,
) -> impl LowDiscrepancySequence + 'static {
    use qrand_rd_alphas::create;

    let alphas = create(dim).into_boxed_slice();

    let sequence = qrand_core::create_sequence(&alphas);

    LowDiscrepancySequenceWrapper {
        alphas: alphas,
        sequence: sequence,
    }
}
