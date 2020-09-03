#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod integration_tests {
    use rd_alphas::create;

    const DIMENSION: usize = 5;
    create!(2);

    #[test]
    fn test_create_alphas() {
        assert_eq!(0.7548776662466927, alphas[0]);
        assert_eq!(0.5698402909980532, alphas[1]);
    }
}
