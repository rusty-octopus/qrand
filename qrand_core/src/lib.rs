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
pub use error::Error;
mod low_discrepancy_sequence;
pub use low_discrepancy_sequence::LowDiscrepancySequence;

//#[cfg_attr(not(feature = "std_interface"), no_std_interface)]
//#[cfg_attr(feature = "rd", rd)]
#[cfg(feature = "rd")]
mod rd;
#[cfg(feature = "rd")]
#[cfg(feature = "std_interface")]
pub use rd::create_sequence;
#[cfg(feature = "rd")]
#[cfg(not(feature = "std_interface"))]
pub use rd::get_sequence;
#[cfg(feature = "rd")]
#[cfg(feature = "std_interface")]
pub use rd::rd_calculate_element;

#[cfg(feature = "sobol")]
mod sobol;
#[cfg(feature = "sobol")]
pub use sobol::new_sequence;
