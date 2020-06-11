#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod integration_tests {
    use qrand_core::new_sequence;
    use qrand_core::LowDiscrepancySequence;

    #[cfg(feature = "rd")]
    #[test]
    fn test_rd_sequence() {
        let rd = new_sequence(2);
        assert_eq!(0.0, rd.element(0, 0).unwrap_or(1.1));
        assert_eq!(0.0, rd.element(0, 1).unwrap_or(1.1));
        assert_eq!(0.7548776662466927, rd.element(1, 0).unwrap_or(1.1));
        assert_eq!(0.5698402909980532, rd.element(1, 1).unwrap_or(1.1));
        assert_eq!(0.5097553324933854, rd.element(2, 0).unwrap_or(1.1));
        assert_eq!(0.13968058199610645, rd.element(2, 1).unwrap_or(1.1));
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
