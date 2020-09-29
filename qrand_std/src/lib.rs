pub use qrand_core::LowDiscrepancySequence;
pub use qrand_core::QrandCoreError;

struct LowDiscrepancySequenceWrapper<T: LowDiscrepancySequence> {
    alphas: Box<[f64]>,
    sequence: Option<T>,
}

impl<T: LowDiscrepancySequence> LowDiscrepancySequence for LowDiscrepancySequenceWrapper<T> {
    fn element(&self, n: usize, dim: usize) -> Result<f64, QrandCoreError> {
        self.sequence.unwrap().element(n, dim)
    }
}

pub fn create_sequence<'a, T: LowDiscrepancySequence>(
    dim: usize,
) -> impl LowDiscrepancySequence + 'a {
    use qrand_rd_alphas::create;

    let alphas = create(dim).into_boxed_slice();

    let mut wrapper = LowDiscrepancySequenceWrapper {
        alphas: alphas,
        sequence: None,
    };

    let sequence = qrand_core::create_sequence(&wrapper.alphas);

    wrapper.sequence = Some(sequence);

    wrapper
}
