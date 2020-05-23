//! Core quasi random number generation library.
//! Is #[no_std]

#![no_std]
#![crate_type = "lib"]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

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
    /// Convert sequence into an iterator through the sequence
    //fn into_iter(sequence_length: usize) -> dyn Iterator<Item = dyn Iterator<Item = f64>>;
    // Fix dynamic type
    // TODO: Blanket implementation possible?

    /// Get the j-th element of the point of n-th element of the sequence.
    /// Is used for parallel execution instead of sequential execution.
    /// Returns `QrandCoreError` when `point_element_j` is larger than the dimension
    /// of the sequence.
    fn get_j_th_of_n_th(
        &self,
        point_element_j: usize,
        seq_element_n: usize,
    ) -> Result<f64, QrandCoreError>;
}

struct Rd {
    dimension: usize,
    alpha: [f64; 2],
}

impl Rd {
    fn new(_dim: usize) -> Self {
        Rd {
            dimension: 2,
            alpha: [0.7548776662466927, 0.5698402909980532],
        }
    }
}

impl LowDiscrepancySequence for Rd {
    fn get_j_th_of_n_th(
        &self,
        point_element_j: usize,
        seq_element_n: usize,
    ) -> Result<f64, QrandCoreError> {
        if point_element_j < self.dimension {
            let value = seq_element_n as f64 * self.alpha[point_element_j];
            if value < 1.0 {
                Ok(value)
            } else {
                Ok(value - 1.0)
            }
        } else {
            Err(QrandCoreError::create_point_element_not_existing())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rd_2_values_for_2d() {
        let rd = Rd::new(2);
        assert_eq!(0.0, rd.get_j_th_of_n_th(0, 0).unwrap_or(1.1));
        assert_eq!(0.0, rd.get_j_th_of_n_th(1, 0).unwrap_or(1.1));
        assert_eq!(0.7548776662466927, rd.get_j_th_of_n_th(0, 1).unwrap_or(1.1));
        assert_eq!(0.5698402909980532, rd.get_j_th_of_n_th(1, 1).unwrap_or(1.1));
        assert_eq!(0.5097553324933854, rd.get_j_th_of_n_th(0, 2).unwrap_or(1.1));
        assert_eq!(
            0.13968058199610645,
            rd.get_j_th_of_n_th(1, 2).unwrap_or(1.1)
        );
    }
}
