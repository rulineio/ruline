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
use tower_http::trace::TraceLayer;

use crate::{domain, error::Error, App, Result};

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

                    let server_address = request.uri().authority().map(|a| a.as_str());

                    let protocol_str = format!("{:?}", request.version());
                    let protocol_version = protocol_str.split('/').last();

                    let user_agent = request
                        .headers()
                        .get("user-agent")
                        .map(|v| v.to_str().unwrap_or_default());
                    let client_address = request
                        .headers()
                        .get("x-forwarded-for")
                        .or_else(|| request.headers().get("x-real-ip"))
                        .map(|v| v.to_str().unwrap_or_default());

                    tracing::info_span!(
                        "request",
                        otel.kind = "server",
                        otel.name = format!("{} {}", request.method(), route),
                        http.request.method = ?request.method(),
                        url.path = request.uri().path(),
                        url.scheme = request.uri().scheme_str().unwrap_or_default(),
                        http.route = route,
                        url.query = request.uri().query().unwrap_or_default(),
                        server.address = server_address.unwrap_or_default(),
                        server.port = request.uri().port_u16().unwrap_or_default(),
                        network.protocol.name = "http",
                        network.protocol.version = protocol_version.unwrap_or_default(),
                        user_agent.original = user_agent.unwrap_or_default(),
                        client.address = client_address.unwrap_or_default(),
                    )
                })
                .on_response(
                    |response: &Response<_>, latency: Duration, _: &tracing::Span| {
                        let duration_s = latency.as_micros() as f64 / 1_000_000.;
                        tracing::info!(
                            http.server.request.duration = duration_s,
                            http.response.status_code = response.status().as_u16(),
                            "finished processing request"
                        );
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
            req.extensions_mut()
                .insert(domain::session::Session::from(sess));
            Ok(next.run(req).await)
        }
        None => Err(Error::Unauthorized),
    }
}
