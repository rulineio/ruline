use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use serde::Serialize;

use crate::{domain::session::Session, error::Error, App, Result};

pub fn router() -> Router<Arc<App>> {
    Router::new().route("/", get(get_user))
}

async fn get_user(Extension(session): Extension<Session>) -> Result<impl IntoResponse> {
    let user = match session {
        Session::User { user } => user,
        Session::Member { user, .. } => user,
        _ => return Err(Error::Unauthorized),
    };

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        avatar: user.avatar,
        status: user.status.to_string(),
    }))
}

#[derive(Serialize)]
struct UserResponse {
    id: String,
    email: String,
    name: String,
    avatar: String,
    status: String,
}
