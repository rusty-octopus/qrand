#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod integration_tests {
    use qrand_rd_alphas::create;

    #[test]
    fn test_create_alphas() {
        let alphas = create(2);
        assert_eq!(0.7548776662466927, alphas[0]);
        assert_eq!(0.5698402909980532, alphas[1]);
    }
}
