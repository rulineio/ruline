use axum::{http, response::IntoResponse};
use thiserror::Error;
use tracing::error;

use crate::{cache, client, db, template};

#[derive(Debug, Error)]
pub enum Error {
    #[error("unauthorized")]
    Unauthorized,

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("Cache error: {0}")]
    CacheError(#[from] cache::error::CacheError),

    #[error("Client error: {0}")]
    ClientError(#[from] client::error::ClientError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] db::error::DatabaseError),

    #[error("Email error: {0}")]
    EmailError(#[from] template::TemplateError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::Unauthorized => http::StatusCode::UNAUTHORIZED,
            _ => {
                error!({ error = %self }, "error processing request");
                http::StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = match status {
            http::StatusCode::UNAUTHORIZED => "Unauthorized",
            _ => "Internal Server Error",
        };

        http::Response::builder()
            .status(status)
            .body(body.into())
            .unwrap()
    }
}
