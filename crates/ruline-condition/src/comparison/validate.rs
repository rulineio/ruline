#[macro_export(local_inner_macros)]
macro_rules! validate_operands {
    ($operands:expr, $expected:expr) => {
        if $operands.len() != $expected {
            return Err(ComparisonError::OperandsAmountMismatch {
                expected: $expected,
                received: $operands.len(),
            }
            .into());
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! validate_min_operands {
    ($operands:expr, $min:expr) => {
        if $operands.len() < $min {
            return Err(ComparisonError::OperandsAmountLessThanRequired {
                min_required: $min,
                received: $operands.len(),
            }
            .into());
        }
    };
}
