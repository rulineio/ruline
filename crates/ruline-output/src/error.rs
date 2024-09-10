use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutputError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
