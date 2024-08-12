#[macro_export]
macro_rules! assert_field_error {
    ($definition:tt) => {
        let data = json!({});
        let context = Context::new(data, DashMap::new());
        let definition = json!($definition);
        let field = Field::try_from(definition).unwrap();
        let result = field.process(&context);
        assert!(result.is_err());
        assert_snapshot!(result.unwrap_err().to_string());
    };
}

#[macro_export]
macro_rules! assert_field {
    ($ctx:expr, $definition:expr, $expected:tt) => {
        let field = Field::try_from($definition).unwrap();
        let result = field.process($ctx).unwrap();
        assert_eq!(result, json!($expected));
    };
}

#[macro_export]
macro_rules! assert_deserialize_error {
    ($definition:tt) => {
        let definition = json!($definition);
        let result = Field::try_from(definition);
        assert!(result.is_err());
        assert_snapshot!(result.unwrap_err().to_string());
    };
}

#[macro_export]
macro_rules! assert_function_error {
    ($definition:tt) => {
        let snapshot = insta::assert_snapshot!(String::from($value));
        snapshot.assert();
    };
}
