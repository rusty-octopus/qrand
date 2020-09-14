//! Core quasi random number generation library.
//! Is #[no_std]

#![crate_type = "staticlib"]
#![no_std]
#![no_builtins]
#![no_implicit_prelude]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

mod error;
pub use error::QrandCoreError;
mod low_discrepancy_sequence;
pub use low_discrepancy_sequence::LowDiscrepancySequence;

#[cfg(feature = "rd")]
mod rd;
#[cfg(feature = "rd")]
pub use rd::get_sequence;

#[cfg(feature = "sobol")]
mod sobol;
#[cfg(feature = "sobol")]
pub use sobol::new_sequence;

// TODO: Use monomorphization instead of interface?
// fn get_sequence<T: LowDiscrepancySequence>() -> T {}
