use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

use crate::{App, Result};

pub fn router() -> Router<Arc<App>> {
    Router::new().route("/", get(get_settings))
}

async fn get_settings(app: State<Arc<App>>) -> Result<impl IntoResponse> {
    Ok(Json(SettingsResponse {
        google_auth_enabled: app.config.google_auth_enabled(),
        magic_link_enabled: app.config.magic_link_enabled(),
    }))
}

#[derive(Serialize)]
pub struct SettingsResponse {
    pub google_auth_enabled: bool,
    pub magic_link_enabled: bool,
}
