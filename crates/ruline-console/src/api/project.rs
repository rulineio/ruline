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
    Router::new()
        .route("/", get(get_projects))
        .route("/:project_id", get(get_project))
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
            id: project.id,
            name: project.name,
            status: project.status.to_string(),
        })),
        None => Err(Error::BadRequest),
    }
}

async fn get_projects(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
) -> Result<impl IntoResponse> {
    let organization = match session {
        Session::Member { organization, .. } => organization,
        _ => return Err(Error::Unauthorized),
    };

    let projects = app
        .db
        .get_projects_by_organization_id(&organization.id)
        .await?;

    Ok(Json(
        projects
            .into_iter()
            .map(|project| ProjectResponse {
                id: project.id,
                name: project.name,
                status: project.status.to_string(),
            })
            .collect::<Vec<_>>(),
    ))
}

#[derive(Serialize)]
struct ProjectResponse {
    id: String,
    name: String,
    status: String,
}
