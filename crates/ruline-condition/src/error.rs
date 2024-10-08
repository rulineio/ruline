use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConditionError {
    #[error("Cycle detected")]
    CycleDetected,
    #[error("Invalid operation condition")]
    ExpressionInvalid,
    #[error(
        "Children count for logical with id `{id}` is invalid, must be at least 2 and is {childrens_count}"
    )]
    LogicalChildrenCountInvalid { id: String, childrens_count: usize },
    #[error("Comparison with id `{0}` must not have any children")]
    ComparisonChildrenInvalid(String),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
