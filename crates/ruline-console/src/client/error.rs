use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Unexpected status code: {0}")]
    UnexpectedStatus(reqwest::StatusCode),
    #[error(transparent)]
    LettreSmtp(#[from] lettre::transport::smtp::Error),
    #[error(transparent)]
    LettreEmail(#[from] lettre::error::Error),
    #[error("Invalid email address: {0}")]
    InvalidEmail(String),
}
