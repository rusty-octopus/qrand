#![no_builtins]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

extern crate core;
use core::result::{Result, Result::Err, Result::Ok};

use crate::error::QrandCoreError;
use crate::low_discrepancy_sequence::LowDiscrepancySequence;

/// Creates a new LowDiscrepancySequence
pub fn new_sequence(dimension: usize) -> impl LowDiscrepancySequence {
    Rd::new(dimension)
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
    fn element(&self, n: usize, dim: usize) -> Result<f64, QrandCoreError> {
        if dim < self.dimension {
            let value = n as f64 * self.alpha[dim];
            if value < 1.0 {
                Ok(value)
            } else {
                let integer_part = (value as u64) as f64;
                Ok(value - integer_part)
            }
        } else {
            Err(QrandCoreError::create_point_element_not_existing())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    extern crate core;
    use core::assert_eq;
    use core::panic;

    #[test]
    fn r2_values() {
        let rd = Rd::new(2);
        assert_eq!(0.0, rd.element(0, 0).unwrap_or(1.1));
        assert_eq!(0.0, rd.element(0, 1).unwrap_or(1.1));
        assert_eq!(0.7548776662466927, rd.element(1, 0).unwrap_or(1.1));
        assert_eq!(0.5698402909980532, rd.element(1, 1).unwrap_or(1.1));
        assert_eq!(0.5097553324933854, rd.element(2, 0).unwrap_or(1.1));
        assert_eq!(0.13968058199610645, rd.element(2, 1).unwrap_or(1.1));
    }
}
