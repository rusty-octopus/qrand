#![no_builtins]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

extern crate core;
use core::result::{Result, Result::Err, Result::Ok};

use crate::error::QrandCoreError;
use crate::low_discrepancy_sequence::LowDiscrepancySequence;

// including the generated alphas.rs
#[cfg(not(feature = "std_interface"))]
include!(concat!(env!("OUT_DIR"), "/alphas.rs"));

#[cfg(not(feature = "std_interface"))]
pub fn get_sequence() -> impl LowDiscrepancySequence {
    Rd::new(&ALPHAS)
}

#[cfg(feature = "std_interface")]
pub fn create_sequence(alphas: &[f64]) -> impl LowDiscrepancySequence + '_ {
    Rd::new(&alphas)
}

struct Rd<'a> {
    dimension: usize,
    alphas: &'a [f64],
}

impl<'a> Rd<'a> {
    const fn new(alphas: &'a [f64]) -> Self {
        Rd {
            dimension: alphas.len(),
            alphas: alphas,
        }
    }
}

impl<'a> LowDiscrepancySequence for Rd<'a> {
    fn element(&self, n: usize, dim: usize) -> Result<f64, QrandCoreError> {
        if dim < self.dimension {
            let value = n as f64 * self.alphas[dim];
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
        static ALPHAS: [f64; 2] = [0.7548776662466927, 0.5698402909980532];
        let rd = Rd::new(&ALPHAS);
        assert_eq!(0.0, rd.element(0, 0).unwrap_or(1.1));
        assert_eq!(0.0, rd.element(0, 1).unwrap_or(1.1));
        assert_eq!(0.7548776662466927, rd.element(1, 0).unwrap_or(1.1));
        assert_eq!(0.5698402909980532, rd.element(1, 1).unwrap_or(1.1));
        assert_eq!(0.5097553324933854, rd.element(2, 0).unwrap_or(1.1));
        assert_eq!(0.13968058199610645, rd.element(2, 1).unwrap_or(1.1));
    }
}
