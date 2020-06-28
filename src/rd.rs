#![no_builtins]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

extern crate core;
use core::result::{Result, Result::Err, Result::Ok};

use crate::error::QrandCoreError;
use crate::low_discrepancy_sequence::LowDiscrepancySequence;

//use crate::alphas::alphas;

//#[macro_export]
/// Adds static data necessary for the sequence
//macro_rules! add_static_seq_data {
//    ($dimension:expr) => {
//        static ALPHAS: [f64; $dimension] = [0.0; $dimension];
//    };
//}

//const fn create_alphas<const DIM: usize>() -> [f64; DIM] {
//    let mut array = [0.0; DIM];
//    let mut i = 0;
//    for &mut x in array.iter_mut() {
//        *x = i;
//        i += 1;
//    }
//}

include!(concat!(env!("OUT_DIR"), "/alphas.rs"));

#[macro_export]
macro_rules! new {
    ($dimension:expr) => {{
        //use qrand_core::create_alphas;
        //create_alphas!($dimension);
        //static ALPHAS: [f64; $dimension] = [0.0; $dimension];
        let seq = $crate::new_sequence(&ALPHAS);
        seq
    }};
}

//static ARRAY: [f64; 5] = 0..5.iter().map(|v| f64::into(v)).collect::<[f64]>();

//#[macro_export]
//macro_rules! create_alphas {
//    ($dimension:expr) => {
//        const fn c_a() -> [f64; $dimension] {
//            let mut array: [f64; $dimension] = [0.0; $dimension];
//            for x in 0..$dimension {
//                array[x] = x
//            }
//        }
//        static ALPHAS: [f64; $dimension] = c_a();
//    };
//}

//macro_rules! create_alphas {
//    ($number_of_elements:expr) => {
//        static ALPHAS: [f64; $number_of_elements] = [0.0; $number_of_elements];
//    };
//}

//#[macro_export]
/// Create a new sequence
//macro_rules! new_seq {
//    () => {
//        new_sequence(&ALPHAS)
//    };
//}

//static alphalpha: [f64; 2] = calculate_alphas(2);

/// Creates a new LowDiscrepancySequence
//pub fn new_sequence(alphas: &'static [f64]) -> impl LowDiscrepancySequence {
//    //create_alphas!(dimension);
//    Rd::new(alphas.len(), alphas)
//}

/// Creates a new LowDiscrepancySequence
pub fn new_sequence(_dim: usize) -> impl LowDiscrepancySequence {
    //create_alphas!(dimension);
    Rd::new(ALPHAS.len(), &ALPHAS)
}

//const fn calculate_alphas(dim: usize) -> [f64; dim] {
//    let mut array = [0.0; dim];
//    alphas[0] = 0.7548776662466927;
//    alphas[1] = 0.5698402909980532;
//    alphas
//}

//static ALPHAS: [f64; 2] = calculate_alphas(2);

struct Rd {
    dimension: usize,
    alphas: &'static [f64],
}

impl Rd {
    fn new(_dim: usize, alphas: &'static [f64]) -> Self {
        Rd {
            dimension: 2,
            alphas: alphas,
        }
    }
}

impl LowDiscrepancySequence for Rd {
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
        let rd = Rd::new(ALPHAS.len(), &ALPHAS);
        assert_eq!(0.0, rd.element(0, 0).unwrap_or(1.1));
        assert_eq!(0.0, rd.element(0, 1).unwrap_or(1.1));
        assert_eq!(0.7548776662466927, rd.element(1, 0).unwrap_or(1.1));
        assert_eq!(0.5698402909980532, rd.element(1, 1).unwrap_or(1.1));
        assert_eq!(0.5097553324933854, rd.element(2, 0).unwrap_or(1.1));
        assert_eq!(0.13968058199610645, rd.element(2, 1).unwrap_or(1.1));
    }
}
