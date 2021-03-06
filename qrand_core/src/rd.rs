#![no_builtins]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

extern crate core;
use core::result::{Result, Result::Err, Result::Ok};

use crate::error::Error;
use crate::low_discrepancy_sequence::LowDiscrepancySequence;

// including the generated alphas.rs
#[cfg(not(feature = "std_interface"))]
include!(concat!(env!("OUT_DIR"), "/alphas.rs"));

/// Gets the sequence defined by the chosen cargo feature.
///
/// Gets the sequence that was chosen by the set cargo feature and build into the binary.
/// The sequence is always a [`LowDiscrepancySequence`](trait.LowDiscrepancySequence.html).
///
/// # Example
/// ```rust
/// use qrand_core::{LowDiscrepancySequence,get_sequence};
///
/// let sequence = get_sequence();
///
/// assert_eq!(0.0, sequence.element(0, 0).unwrap_or(1.1));
/// ```
#[cfg(not(feature = "std_interface"))]
pub fn get_sequence() -> impl LowDiscrepancySequence {
    Rd::new(&ALPHAS)
}

#[cfg(feature = "std_interface")]
pub fn create_sequence(alphas: &[f64]) -> impl LowDiscrepancySequence + '_ {
    Rd::new(&alphas)
}

struct Rd<'a> {
    alphas: &'a [f64],
}

impl<'a> Rd<'a> {
    const fn new(alphas: &'a [f64]) -> Self {
        Rd { alphas }
    }
}

impl<'a> LowDiscrepancySequence for Rd<'a> {
    #[inline]
    fn element(&self, n: usize, dim: usize) -> Result<f64, Error> {
        calculate_element(&self.alphas, n, dim)
    }
}

#[inline]
fn calculate_element(alphas: &[f64], n: usize, dim: usize) -> Result<f64, Error> {
    if dim < alphas.len() {
        let value = n as f64 * alphas[dim];
        if value < 1.0 {
            Ok(value)
        } else {
            let integer_part = (value as u64) as f64;
            Ok(value - integer_part)
        }
    } else {
        Err(Error::create_point_element_not_existing())
    }
}

#[cfg(feature = "std_interface")]
#[inline]
pub fn rd_calculate_element(alphas: &[f64], n: usize, dim: usize) -> Result<f64, Error> {
    calculate_element(alphas, n, dim)
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
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

    #[test]
    fn test_element_not_existing_error() {
        static ALPHAS: [f64; 2] = [0.7548776662466927, 0.5698402909980532];
        let rd = Rd::new(&ALPHAS);
        let result = rd.element(3, 3);
        assert_eq!(Err(Error::create_point_element_not_existing()), result);
    }

    #[cfg(feature = "std_interface")]
    #[test]
    fn test_rd_calculate_element() {
        static ALPHAS: [f64; 2] = [0.7548776662466927, 0.5698402909980532];
        assert_eq!(0.0, rd_calculate_element(&ALPHAS, 0, 0).unwrap_or(1.1));
        assert_eq!(0.0, rd_calculate_element(&ALPHAS, 0, 1).unwrap_or(1.1));
        assert_eq!(
            0.7548776662466927,
            rd_calculate_element(&ALPHAS, 1, 0).unwrap_or(1.1)
        );
        assert_eq!(
            0.5698402909980532,
            rd_calculate_element(&ALPHAS, 1, 1).unwrap_or(1.1)
        );
        assert_eq!(
            0.5097553324933854,
            rd_calculate_element(&ALPHAS, 2, 0).unwrap_or(1.1)
        );
        assert_eq!(
            0.13968058199610645,
            rd_calculate_element(&ALPHAS, 2, 1).unwrap_or(1.1)
        );
    }
}
