#[macro_export(local_inner_macros)]
macro_rules! validate_args {
    ($args:expr, $expected:expr) => {
        if $args.len() != $expected {
            return Err(FunctionError::ArgumentsAmountMismatch {
                expected: $expected,
                received: $args.len(),
            }
            .into());
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! validate_min_args {
    ($args:expr, $min:expr) => {
        if $args.len() < $min {
            return Err(FunctionError::ArgumentsAmountLessThanRequired {
                min_required: $min,
                received: $args.len(),
            }
            .into());
        }
    };
}
