use ruline_action::ActionError;
use ruline_condition::ConditionError;
use ruline_output::OutputError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("Dependency `{dependency}` for component `{component_id}` not found")]
    DependencyNotFound { component_id: i64, dependency: i64 },
    #[error("Dependant `{dependant}` for component `{component_id}` not found")]
    DependantNotFound { component_id: i64, dependant: i64 },

    #[error("Cycle detected")]
    CycleDetected,

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    Condition(#[from] ConditionError),
    #[error(transparent)]
    Action(#[from] ActionError),
    #[error(transparent)]
    Output(#[from] OutputError),
}
