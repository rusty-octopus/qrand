#![no_builtins]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

extern crate core;
use core::result::Result;

use crate::error::Error;

/// Interface of a low-discrepance sequence.
pub trait LowDiscrepancySequence {
    /// Convert sequence into an iterator through the sequence
    //fn into_iter(sequence_length: usize) -> dyn Iterator<Item = dyn Iterator<Item = f64>>;
    // Fix dynamic type
    // TODO: Blanket implementation possible?

    /// Get the n-th sequence element of the specified dimensions.
    /// Is used for parallel execution instead of sequential execution.
    /// Returns an Error when `dim` is larger than the dimension
    /// of the sequence.
    fn element(&self, n: usize, dim: usize) -> Result<f64, Error>;
}
