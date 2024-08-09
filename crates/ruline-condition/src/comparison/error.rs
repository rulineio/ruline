use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComparisonError {
    #[error("Expected {expected} operands, got {received}")]
    OperandsAmountMismatch { expected: usize, received: usize },
    #[error("Expected at least {min_required} operands, got {received}")]
    OperandsAmountLessThanRequired {
        min_required: usize,
        received: usize,
    },
    #[error("Operands type invalid")]
    OperandTypeInvalid,
}
