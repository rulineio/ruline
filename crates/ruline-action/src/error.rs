use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActionError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
