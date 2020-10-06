#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod qrand_core_integration_tests {
    #[cfg(feature = "rd")]
    #[cfg(not(feature = "std_interface"))]
    use qrand_core::get_sequence;

    #[cfg(all(feature = "rd", feature = "std_interface"))]
    use qrand_core::create_sequence;

    #[cfg(feature = "sobol")]
    use qrand_core::new_sequence;

    use qrand_core::LowDiscrepancySequence;
    //    #[cfg(feature = "rd")]
    //    use qrand_core::add_static_seq_data;

    // #[cfg(feature = "rd")]
    // add_static_seq_data!(2);

    #[cfg(feature = "rd")]
    #[cfg(not(feature = "std_interface"))]
    #[test]
    fn test_rd_sequence_no_std() {
        let sequence = get_sequence();
        assert_eq!(0.0, sequence.element(0, 0).unwrap_or(1.1));
        assert_eq!(0.0, sequence.element(0, 1).unwrap_or(1.1));
        assert_eq!(0.7548776662466927, sequence.element(1, 0).unwrap_or(1.1));
        assert_eq!(0.5698402909980532, sequence.element(1, 1).unwrap_or(1.1));
        assert_eq!(0.5097553324933854, sequence.element(2, 0).unwrap_or(1.1));
        assert_eq!(0.13968058199610645, sequence.element(2, 1).unwrap_or(1.1));
    }

    #[cfg(all(feature = "rd", feature = "std_interface"))]
    #[test]
    fn test_rd_sequence_std() {
        let alphas = [0.7548776662466927, 0.5698402909980532];
        let sequence = create_sequence(&alphas);
        assert_eq!(0.0, sequence.element(0, 0).unwrap_or(1.1));
        assert_eq!(0.0, sequence.element(0, 1).unwrap_or(1.1));
        assert_eq!(0.7548776662466927, sequence.element(1, 0).unwrap_or(1.1));
        assert_eq!(0.5698402909980532, sequence.element(1, 1).unwrap_or(1.1));
        assert_eq!(0.5097553324933854, sequence.element(2, 0).unwrap_or(1.1));
        assert_eq!(0.13968058199610645, sequence.element(2, 1).unwrap_or(1.1));
    }

    #[cfg(feature = "sobol")]
    #[test]
    fn test_sobol_sequence() {
        let sobol = new_sequence(2);
        assert_eq!(0.0, sobol.element(0, 0).unwrap_or(1.1));
        assert_eq!(0.0, sobol.element(0, 1).unwrap_or(1.1));
        assert_eq!(0.5, sobol.element(1, 0).unwrap_or(1.1));
        assert_eq!(0.5, sobol.element(1, 1).unwrap_or(1.1));
        assert_eq!(0.25, sobol.element(2, 0).unwrap_or(1.1));
        assert_eq!(0.75, sobol.element(2, 1).unwrap_or(1.1));
        assert_eq!(0.75, sobol.element(3, 0).unwrap_or(1.1));
        assert_eq!(0.25, sobol.element(3, 1).unwrap_or(1.1));
    }
}
