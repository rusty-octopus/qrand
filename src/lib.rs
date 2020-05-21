//! Core quasi random number generation library.
//! Is #[no_std]

#![no_std]
#![crate_type = "lib"]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

/// Public errors of qrand_core
pub enum QrandCoreError {
    /// Dimension too large error.
    /// Occurs when a Sobol sequence is created with a dimension larger than 21201.
    DimensionTooLarge(&'static str),
    /// Point element not existing error.
    /// Occurs when a point element is requested that is larger than the chosen dimension of the sequence.
    PointElementNotExisting(&'static str),
}

impl QrandCoreError {
    fn create_error_for_dimension_too_large() -> QrandCoreError {
        QrandCoreError::DimensionTooLarge(
            "Dimension is too large. Sobol sequence can only support a dimension up to 21201.",
        )
    }
    fn create_point_element_not_existing() -> QrandCoreError {
        QrandCoreError::PointElementNotExisting(
            "Point element not existing. Element of point is too large for the chosen dimension.",
        )
    }
}

/// Struct to describe a point in a low discrepancy sequence
pub struct LowDiscrepancySequencePoint {
    point: [f64],
}

/// Interface of a low-discrepance sequence.
pub trait LowDiscrepancySequence {
    /// Create new low discrepancy sequence with given dimension.
    /// Returns the sequence or error if requested dimension is too large.
    //fn new<T: LowDiscrepancySequence + Sized>(dim: usize) -> Result<T, QrandCoreError>;
    /// Get the next point based on the given the current point.
    /// Is used for sequential execution of the sequence.
    /// Returns `None` when the end of the sequence was reached.
    fn next_point(current_point: &mut LowDiscrepancySequencePoint) -> Option<()>;
    /// Get the j-th element of the point of n-th element of the sequence.
    /// Is used for parallel execution instead of sequential execution.#
    /// Returns `Ok` result with an `Option<f64>` which is none, when `n` is
    /// larger than the length of the sequence.
    /// Returns `QrandCoreError` when `point_element_j` is larger than the dimension
    /// of the sequence.
    fn get_j_th_of_n_th(
        point_element_j: usize,
        seq_element_n: usize,
    ) -> Result<Option<f64>, QrandCoreError>;
}

/// Enum to describe the sequence length.
/// Finite or infinite.
pub enum SequenceLength {
    /// Sequence is of finite length.
    Finite(usize),
    /// Sequence has infinite length.
    Infinite,
}

struct Rd {
    dimension: usize,
    length: SequenceLength,
    alpha: [f64; 1],
}

impl Rd {
    fn new(_dim: usize, sequence_length: SequenceLength) -> Self {
        Rd {
            dimension: 1,
            length: sequence_length,
            alpha: [1.0 / 1.6180339887498948482],
        }
    }
}

impl LowDiscrepancySequence for Rd {
    fn next_point(_current_point: &mut LowDiscrepancySequencePoint) -> Option<()> {
        None
    }

    fn get_j_th_of_n_th(
        _point_element_j: usize,
        _seq_element_n: usize,
    ) -> Result<Option<f64>, QrandCoreError> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
