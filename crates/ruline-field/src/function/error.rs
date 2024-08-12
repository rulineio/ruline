use thiserror::Error;

#[derive(Debug, Error)]
pub enum FunctionError {
    #[error("Expected {expected} arguments, got {received}")]
    ArgumentsAmountMismatch { expected: usize, received: usize },
    #[error("Expected at least {min_required} arguments, got {received}")]
    ArgumentsAmountLessThanRequired {
        min_required: usize,
        received: usize,
    },
    #[error("Argument type invalid")]
    ArgumentTypeInvalid,
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
