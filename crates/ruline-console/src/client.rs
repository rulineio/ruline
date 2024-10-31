use axum::{async_trait, http::Extensions};
use opentelemetry_semantic_conventions::trace::{HTTP_RESPONSE_STATUS_CODE, OTEL_STATUS_CODE};
use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next, Result};
use tracing::{debug, field, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub mod email;
pub mod error;
pub mod google;

pub(super) fn new_client(client: Client) -> ClientWithMiddleware {
    ClientBuilder::new(client)
        .with(OtelMiddleware::new())
        .build()
}

struct OtelMiddleware;
impl OtelMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Middleware for OtelMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let mut url_full = req.url().to_string();
        // get the full url but remove sensitive query parameters
        if req.url().query().is_some() {
            let pairs = req.url().query_pairs();
            // replace the value of the `password` query parameter with `REDACTED`
            // if it exists
            let query = pairs
                .map(|(key, value)| match key.as_ref() {
                    "state" => (key, "REDACTED".into()),
                    _ => (key, value),
                })
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<_>>()
                .join("&");

            let new_url = &mut req.url().to_owned();
            new_url.set_query(Some(&query));
            url_full = new_url.to_string();
        }

        let span = info_span!(
            "request",
            otel.name = format!("{} {}", req.method(), req.url().path()),
            otel.kind = "CLIENT",
            http.request.method = ?req.method(),
            server.address = req.url().authority(),
            url.full = url_full,
            url.scheme = req.url().scheme(),
            server.port = field::Empty,
        );

        let res = next.run(req, extensions).await;

        match res {
            Ok(res) => {
                span.set_attribute(HTTP_RESPONSE_STATUS_CODE, res.status().as_u16() as i64);
                debug!("Request successful");
                Ok(res)
            }
            Err(err) => {
                span.set_attribute(OTEL_STATUS_CODE, "ERROR");
                debug!({ exception.message = %err }, "Request failed");
                Err(err)
            }
        }
    }
}
