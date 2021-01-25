#![no_builtins]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

extern crate core;
use core::fmt::{Debug, Display, Formatter, Result};
/// Public errors of qrand_core
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Point element not existing error.
    /// Occurs when a point element is requested that is larger than the chosen dimension of the sequence.
    PointElementNotExisting(&'static str),
}

impl Error {
    pub(crate) fn create_point_element_not_existing() -> Error {
        Error::PointElementNotExisting(
            "Point element not existing. Element of point is too large for the chosen dimension.",
        )
    }
    pub(crate) fn description(&self) -> &'static str {
        match self {
            Self::PointElementNotExisting(desc) => desc,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.description())
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    extern crate core;
    use core::assert_eq;
    use core::panic;

    extern crate std;
    use std::format;

    #[test]
    fn test_element_not_existing() {
        let error = Error::create_point_element_not_existing();
        assert_eq!(
            error,
            Error::PointElementNotExisting("Point element not existing. Element of point is too large for the chosen dimension.")
        );
        let error_string = format!("{}", error);
        assert_eq!(
            "Point element not existing. Element of point is too large for the chosen dimension.",
            error_string
        );
    }
}
