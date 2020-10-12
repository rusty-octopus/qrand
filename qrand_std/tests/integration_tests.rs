#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod qrand_std_integration_tests {
    use qrand_std::create_sequence;
    use qrand_std::LowDiscrepancySequence;

    #[test]
    fn test_rd_sequence_std() {
        let sequence = create_sequence(2);
        assert_eq!(0.0, sequence.element(0, 0).unwrap_or(1.1));
        assert_eq!(0.0, sequence.element(0, 1).unwrap_or(1.1));
        assert_eq!(0.7548776662466927, sequence.element(1, 0).unwrap_or(1.1));
        assert_eq!(0.5698402909980532, sequence.element(1, 1).unwrap_or(1.1));
        assert_eq!(0.5097553324933854, sequence.element(2, 0).unwrap_or(1.1));
        assert_eq!(0.13968058199610645, sequence.element(2, 1).unwrap_or(1.1));
    }

    #[test]
    fn test_error() {
        let sequence = create_sequence(2);
        assert_eq!(Err(qrand_core::QrandCoreError::PointElementNotExisting("Point element not existing. Element of point is too large for the chosen dimension.")), sequence.element(0, 5));
        assert_eq!(
            "Point element not existing. Element of point is too large for the chosen dimension.",
            format!("{}", sequence.element(0, 5).unwrap_err())
        );
    }
}
