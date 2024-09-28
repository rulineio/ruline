use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::{self, Next},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::CookieJar;
use tower_http::trace::TraceLayer;

use crate::{error::Error, App, Result};

mod login;
mod user;

pub fn router(app: Arc<App>) -> Router {
    let static_dir = tower_http::services::ServeDir::new("ui/dist/static");
    let main_page = tower_http::services::ServeFile::new("ui/dist/index.html");

    Router::new()
        .route("/", get(|| async { Redirect::to("/ui") }))
        .nest("/users", user::router())
        .route_layer(middleware::from_fn_with_state(
            app.clone(),
            authenticate_user,
        ))
        .nest_service("/static", static_dir)
        .nest_service("/ui", main_page)
        .nest("/login", login::router())
        .with_state(app)
        .layer(TraceLayer::new_for_http())
}

pub async fn authenticate_user(
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
            req.extensions_mut().insert(sess);
            Ok(next.run(req).await)
        }
        None => Err(Error::Unauthorized),
    }
}
