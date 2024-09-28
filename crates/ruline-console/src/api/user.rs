use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use serde::Serialize;

use crate::{cache::session, App, Result};

pub fn router() -> Router<Arc<App>> {
    Router::new().route("/me", get(get_user))
}

async fn get_user(sess: Extension<session::Session>) -> Result<impl IntoResponse> {
    Ok(Json(UserResponse {
        status: sess.user.status.clone(),
        email: sess.user.email.clone(),
        name: sess.user.name.clone(),
        avatar: sess.user.avatar.clone(),
    }))
}

#[derive(Serialize)]
struct UserResponse {
    pub status: String,
    pub email: String,
    pub name: String,
    pub avatar: String,
}
