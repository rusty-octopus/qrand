//! Core quasi random number generation library.
//! Is #[no_std]

#![crate_type = "staticlib"]
#![no_std]
#![no_builtins]
#![no_implicit_prelude]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]
#![feature(const_fn)]
#![feature(const_generics)]

mod error;
pub use error::QrandCoreError;
mod low_discrepancy_sequence;
pub use low_discrepancy_sequence::LowDiscrepancySequence;

#[cfg(feature = "rd")]
mod rd;
#[cfg(feature = "rd")]
pub use rd::new_sequence;
#[cfg(feature = "rd")]
extern crate qrand_rd_alphas;
#[cfg(feature = "rd")]
pub use qrand_rd_alphas::create;
#[cfg(feature = "rd")]
pub use rd::seq_dim;
#[cfg(feature = "rd")]
pub use rd::sequence;
//#[cfg(feature = "rd")]
//pub use rd_alphas;

//#[cfg(feature = "rd")]
//mod alphas;
#[cfg(feature = "sobol")]
mod sobol;
#[cfg(feature = "sobol")]
pub use sobol::new_sequence;

// TODO: Use monomorphization instead of interface?
// fn get_sequence<T: LowDiscrepancySequence>() -> T {}

//extern crate core;
//use core::default::Default;
//
//impl Default for dyn LowDiscrepancySequence {
//    #[cfg(feature = "rd")]
//    fn default() -> Self {
//        const alphas: [f64; 1] = [0.0];
//        rd::new_sequence(&alphas)
//    }
//}
