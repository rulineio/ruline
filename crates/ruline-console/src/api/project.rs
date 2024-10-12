use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use serde::Serialize;

use crate::{domain::session::Session, error::Error, App, Result};

pub fn router() -> Router<Arc<App>> {
    Router::new().route("/:project_id", get(get_project))
}

async fn get_project(
    State(app): State<Arc<App>>,
    Path(project_id): Path<String>,
    Extension(session): Extension<Session>,
) -> Result<impl IntoResponse> {
    let organization = match session {
        Session::Member { organization, .. } => organization,
        _ => return Err(Error::Unauthorized),
    };

    let project = app.db.get_project(&organization.id, &project_id).await?;

    match project {
        Some(project) => Ok(Json(ProjectResponse {
            name: project.name,
            status: project.status.to_string(),
        })),
        None => Err(Error::BadRequest),
    }
}

#[derive(Serialize)]
struct ProjectResponse {
    name: String,
    status: String,
}
