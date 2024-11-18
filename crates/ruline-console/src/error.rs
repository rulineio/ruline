use axum::{http, response::IntoResponse};
use opentelemetry_semantic_conventions::trace::OTEL_STATUS_CODE;
use thiserror::Error;
use tracing::{error, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{cache, client, db, editor::VersionEditorEvent, template};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request")]
    BadRequest,

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("Cache error: {0}")]
    CacheError(#[from] cache::error::CacheError),

    #[error("Client error: {0}")]
    ClientError(#[from] client::error::ClientError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] db::error::DatabaseError),

    #[error("Template error: {0}")]
    TemplateError(#[from] template::TemplateError),

    #[error("Could not send message to version editor channel: {0}")]
    ChannelSendError(#[from] tokio::sync::broadcast::error::SendError<VersionEditorEvent>),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::Unauthorized => http::StatusCode::UNAUTHORIZED,
            Error::BadRequest => http::StatusCode::BAD_REQUEST,
            _ => {
                error!({ exception.message = %self }, "Error processing request");
                Span::current().set_attribute(OTEL_STATUS_CODE, "ERROR");
                http::StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = match status {
            http::StatusCode::UNAUTHORIZED => "Unauthorized",
            http::StatusCode::BAD_REQUEST => "Bad Request",
            _ => "Internal Server Error",
        };

        http::Response::builder()
            .status(status)
            .body(body.into())
            .unwrap()
    }
}
