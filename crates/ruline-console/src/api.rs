use std::{sync::Arc, time::Duration};

use axum::{
    body::Body,
    extract::{MatchedPath, State},
    http::{Request, Response},
    middleware::{self, Next},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::CookieJar;
use opentelemetry_semantic_conventions::trace::HTTP_RESPONSE_STATUS_CODE;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{field, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{domain::session::Session, error::Error, App, Result};

mod invitation;
mod login;
mod organization;
mod project;
mod session;
mod settings;
mod signup;
mod user;
mod workflow;

pub fn router(app: Arc<App>) -> Router {
    let static_dir = tower_http::services::ServeDir::new("ui/dist/static");
    let main_page = tower_http::services::ServeFile::new("ui/dist/index.html");

    Router::new()
        .nest("/session", session::router())
        .nest("/organizations", organization::router())
        .nest("/users", user::router())
        .nest("/projects", project::router().merge(workflow::router()))
        .nest("/invitations", invitation::router())
        .route_layer(middleware::from_fn_with_state(
            app.clone(),
            authenticate_user,
        ))
        .nest_service("/static", static_dir)
        .nest_service("/ui", main_page)
        .route("/", get(|| async { Redirect::to("/ui") }))
        .nest("/login", login::router())
        .nest("/signup", signup::router())
        .nest("/settings", settings::router())
        .with_state(app.clone())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let route = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str)
                        .unwrap_or_else(|| request.uri().path());

                    let span = tracing::info_span!(
                        "http_request",
                        otel.kind = "SERVER",
                        otel.name = format!("{} {}", request.method(), route),
                        http.request.method = ?request.method(),
                        url.path = request.uri().path(),
                        http.route = route,
                        url.scheme = field::Empty,
                        url.query = field::Empty,
                        server.address = field::Empty,
                        server.port = field::Empty,
                        user_agent.original = field::Empty,
                        client.address = field::Empty,
                        network.protocol.version = field::Empty,
                        http.response.status_code = field::Empty,
                        user.id = field::Empty,
                        organization.id = field::Empty,
                        member.id = field::Empty,
                    );

                    if let Some(scheme) = request.uri().scheme_str() {
                        span.record("url.scheme", scheme);
                    }
                    if let Some(query) = request.uri().query() {
                        span.record("url.query", query);
                    }
                    let server_address = request.uri().authority().map(|a| a.as_str());
                    if let Some(address) = server_address {
                        span.record("server.address", address);
                    }
                    if let Some(port) = request.uri().port_u16() {
                        span.record("server.port", port);
                    }
                    let protocol_str = format!("{:?}", request.version());
                    let protocol_version = protocol_str.split('/').last();
                    if let Some(protocol_version) = protocol_version {
                        span.record("network.protocol.version", protocol_version);
                    }
                    let user_agent = request.headers().get("user-agent");
                    if let Some(user_agent) = user_agent {
                        span.record("user_agent.original", user_agent.to_str().unwrap());
                    }
                    let client_address = request
                        .headers()
                        .get("x-forwarded-for")
                        .or_else(|| request.headers().get("x-real-ip"));
                    if let Some(client_address) = client_address {
                        span.record("client.address", client_address.to_str().unwrap());
                    }

                    span
                })
                .on_response(
                    |response: &Response<_>, _latency: Duration, span: &tracing::Span| {
                        span.record(HTTP_RESPONSE_STATUS_CODE, response.status().as_u16() as i64);
                        tracing::debug!("Finished processing request");
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &tracing::Span| {
                        tracing::debug!(error = %error, "Request failed");
                    },
                ),
        )
}

async fn authenticate_user(
    State(app): State<Arc<App>>,
    jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse> {
    let session_id = match jar.get("sid") {
        Some(sid) => sid,
        None => return Err(Error::Unauthorized),
    };

    let sess = app.cache.get_session(session_id.value()).await?;

    match sess {
        Some(sess) => {
            let span = Span::current();
            match sess.to_owned().into() {
                Session::User { user } => span.set_attribute("user.id", user.id),
                Session::Member {
                    user,
                    organization,
                    member,
                } => {
                    span.record("user.id", user.id)
                        .record("organization.id", organization.id)
                        .record("member.id", member.id);
                }
                _ => {}
            };
            req.extensions_mut().insert(Session::from(sess));
            Ok(next.run(req).await)
        }
        None => Err(Error::Unauthorized),
    }
}
