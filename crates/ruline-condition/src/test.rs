#[macro_export]
macro_rules! assert_comparison {
    ($definition:expr, $expected:expr) => {
        let context = Context::new(json!({}), DashMap::new());
        let condition = Condition::try_from($definition).unwrap();
        let result = condition.evaluate(&context).unwrap();
        assert_eq!(result, $expected);
    };
}

#[macro_export]
macro_rules! assert_comparison_error {
    ($definition:expr) => {
        let context = Context::new(json!({}), DashMap::new());
        let condition = Condition::try_from($definition).unwrap();
        let result = condition.evaluate(&context);
        assert!(result.is_err());
        assert_snapshot!(result.unwrap_err().to_string());
    };
}

#[macro_export]
macro_rules! assert_condition_deserialize_error {
    ($definition:expr) => {
        let result = Condition::try_from($definition);
        assert!(result.is_err());
        assert_snapshot!(result.unwrap_err().to_string());
    };
}
