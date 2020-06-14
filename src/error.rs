#![no_builtins]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

/// Public errors of qrand_core
pub enum QrandCoreError {
    /// Dimension too large error.
    /// Occurs when a Sobol sequence is created with a dimension larger than 21201.
    #[cfg(feature = "sobol")]
    DimensionTooLarge(&'static str),
    /// Point element not existing error.
    /// Occurs when a point element is requested that is larger than the chosen dimension of the sequence.
    PointElementNotExisting(&'static str),
}

impl QrandCoreError {
    #[cfg(feature = "sobol")]
    pub(crate) fn create_error_for_dimension_too_large() -> QrandCoreError {
        QrandCoreError::DimensionTooLarge(
            "Dimension is too large. Sobol sequence can only support a dimension up to 21201.",
        )
    }
    pub(crate) fn create_point_element_not_existing() -> QrandCoreError {
        QrandCoreError::PointElementNotExisting(
            "Point element not existing. Element of point is too large for the chosen dimension.",
        )
    }
    /// Get the description of the error.
    pub fn description(&self) -> &'static str {
        match self {
            #[cfg(feature = "sobol")]
            QrandCoreError::DimensionTooLarge(desc) => desc,
            QrandCoreError::PointElementNotExisting(desc) => desc,
        }
    }
}
