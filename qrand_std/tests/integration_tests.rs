#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod qrand_std_integration_tests {
    use qrand_std::create_sequence;
    use qrand_std::LowDiscrepancySequence;

    #[cfg(all(feature = "rd", feature = "std_interface"))]
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
}
