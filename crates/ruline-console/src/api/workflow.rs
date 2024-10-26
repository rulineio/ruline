use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        member::MemberRole,
        session::Session,
        workflow::{Workflow, WorkflowVersionStatus},
    },
    error::Error,
    App, Result,
};

pub fn router() -> Router<Arc<App>> {
    let vwr = Router::new()
        .route("/", get(get_workflow_versions))
        .route("/", post(create_workflow_version))
        .route("/:version", get(get_workflow_version))
        .route("/:version", patch(update_workflow_version_status));

    let wr = Router::new()
        .route("/", get(get_workflow))
        .route("/", patch(update_workflow))
        .nest("/versions", vwr);

    let r = Router::new()
        .route("/", get(get_workflows))
        .route("/", post(create_workflow))
        .nest("/:workflow_id", wr);

    Router::new().nest("/:project_id/workflows", r)
}

async fn create_workflow(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path(project_id): Path<String>,
    Json(body): Json<CreateWorkflowRequest>,
) -> Result<impl IntoResponse> {
    let (organization_id, role) = match session {
        Session::Member {
            organization,
            member,
            ..
        } => (organization.id, member.role),
        _ => return Err(Error::Unauthorized),
    };

    if !role.is_at_least(MemberRole::Admin) {
        return Err(Error::Unauthorized);
    }

    let workflow = Workflow::builder()
        .organization_id(organization_id.to_owned())
        .project_id(project_id.to_owned())
        .name(body.name)
        .build();
    let initial_version = workflow.initial_version();

    let mut trx = app.db.begin().await?;
    app.db.insert_workflow(&workflow, &mut trx).await?;
    app.db
        .insert_workflow_version(&initial_version, &mut trx)
        .await?;
    app.db.commit(trx).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateWorkflowResponse {
            id: workflow.id,
            version: initial_version.version,
        }),
    ))
}

async fn get_workflow(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path((project_id, workflow_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    let organization_id = match session {
        Session::Member { organization, .. } => organization.id,
        _ => return Err(Error::Unauthorized),
    };

    let workflow = app
        .db
        .get_workflow(&organization_id, &project_id, &workflow_id)
        .await?;

    match workflow {
        Some(workflow) => Ok(Json(WorkflowResponse {
            id: workflow.id,
            name: workflow.name,
            status: workflow.status.to_string(),
            active_version: workflow.active_version,
        })),
        None => Err(Error::BadRequest),
    }
}

async fn get_workflows(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path(project_id): Path<String>,
) -> Result<impl IntoResponse> {
    let organization_id = match session {
        Session::Member { organization, .. } => organization.id,
        _ => return Err(Error::Unauthorized),
    };

    let workflows = app
        .db
        .get_workflows_by_project_id(&organization_id, &project_id)
        .await?;

    Ok(Json(
        workflows
            .into_iter()
            .map(|workflow| WorkflowResponse {
                id: workflow.id,
                name: workflow.name,
                status: workflow.status.to_string(),
                active_version: workflow.active_version,
            })
            .collect::<Vec<_>>(),
    ))
}

async fn update_workflow(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path((project_id, workflow_id)): Path<(String, String)>,
    Json(body): Json<UpdateWorkflowRequest>,
) -> Result<impl IntoResponse> {
    let (organization_id, role) = match session {
        Session::Member {
            organization,
            member,
            ..
        } => (organization.id, member.role),
        _ => return Err(Error::Unauthorized),
    };

    if !role.is_at_least(MemberRole::Admin) {
        return Err(Error::Unauthorized);
    }

    if body.name.is_none() && body.status.is_none() {
        return Err(Error::BadRequest);
    }

    let mut trx = app.db.begin().await?;
    if let Some(name) = body.name {
        app.db
            .set_workflow_name(&organization_id, &project_id, &workflow_id, &name, &mut trx)
            .await?;
    }
    if let Some(status) = body.status {
        app.db
            .set_workflow_status(
                &organization_id,
                &project_id,
                &workflow_id,
                status.into(),
                &mut trx,
            )
            .await?;
    }
    app.db.commit(trx).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn create_workflow_version(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path((project_id, workflow_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    let (organization_id, role) = match session {
        Session::Member {
            organization,
            member,
            ..
        } => (organization.id, member.role),
        _ => return Err(Error::Unauthorized),
    };

    if !role.is_at_least(MemberRole::Editor) {
        return Err(Error::Unauthorized);
    }

    let workflow = match app
        .db
        .get_workflow(&organization_id, &project_id, &workflow_id)
        .await?
    {
        Some(workflow) => workflow,
        None => return Err(Error::BadRequest),
    };

    let version = match app
        .db
        .get_workflow_version(
            &organization_id,
            &project_id,
            &workflow_id,
            workflow.active_version.unwrap_or(1),
        )
        .await?
    {
        Some(version) => version,
        None => return Err(Error::BadRequest),
    };

    let new_version = version.next_version();

    let mut trx = app.db.begin().await?;
    app.db
        .insert_workflow_version(&new_version, &mut trx)
        .await?;
    app.db
        .set_workflow_active_version(
            &organization_id,
            &project_id,
            &workflow_id,
            new_version.version,
            &mut trx,
        )
        .await?;
    app.db.commit(trx).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateWorkflowVersionResponse {
            version: new_version.version,
        }),
    ))
}

async fn get_workflow_version(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path((project_id, workflow_id, version)): Path<(String, String, u32)>,
) -> Result<impl IntoResponse> {
    let organization_id = match session {
        Session::Member { organization, .. } => organization.id,
        _ => return Err(Error::Unauthorized),
    };

    let version = app
        .db
        .get_workflow_version(&organization_id, &project_id, &workflow_id, version)
        .await?;

    match version {
        Some(version) => Ok(Json(WorkflowVersionResponse {
            version: version.version,
            status: version.status.to_string(),
            definition: version.definition,
        })),
        None => Err(Error::BadRequest),
    }
}

async fn get_workflow_versions(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path((project_id, workflow_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    let organization_id = match session {
        Session::Member { organization, .. } => organization.id,
        _ => return Err(Error::Unauthorized),
    };

    let versions = app
        .db
        .get_workflow_versions_by_workflow_id(&organization_id, &project_id, &workflow_id)
        .await?;

    Ok(Json(
        versions
            .into_iter()
            .map(|version| WorkflowVersionResponse {
                version: version.version,
                status: version.status.to_string(),
                definition: version.definition,
            })
            .collect::<Vec<_>>(),
    ))
}

async fn update_workflow_version_status(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path((project_id, workflow_id, version)): Path<(String, String, u32)>,
    Json(body): Json<UpdateWorkflowVersionStatusRequest>,
) -> Result<impl IntoResponse> {
    let (organization_id, role) = match session {
        Session::Member {
            organization,
            member,
            ..
        } => (organization.id, member.role),
        _ => return Err(Error::Unauthorized),
    };

    let is_admin = role.is_at_least(MemberRole::Admin);
    let is_editor = role.is_at_least(MemberRole::Editor);

    let status: WorkflowVersionStatus = body.status.into();
    match status {
        WorkflowVersionStatus::Published if !is_admin => {
            return Err(Error::Unauthorized);
        }
        WorkflowVersionStatus::InReview if !is_editor => {
            return Err(Error::Unauthorized);
        }
        _ => {}
    }

    let version = match app
        .db
        .get_workflow_version(&organization_id, &project_id, &workflow_id, version)
        .await?
    {
        Some(version) => version,
        None => return Err(Error::BadRequest),
    };

    match (version.status, status.to_owned()) {
        (x, y) if x == y => {
            return Err(Error::BadRequest);
        }
        (WorkflowVersionStatus::Published, _) => {
            return Err(Error::BadRequest);
        }
        _ => {}
    }

    let mut trx = app.db.begin().await?;
    app.db
        .set_workflow_version_status(
            &organization_id,
            &project_id,
            &workflow_id,
            version.version,
            status,
            &mut trx,
        )
        .await?;
    app.db.commit(trx).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct CreateWorkflowRequest {
    pub name: String,
}

#[derive(Serialize)]
struct CreateWorkflowResponse {
    pub id: String,
    pub version: u32,
}

#[derive(Serialize)]
struct WorkflowResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub active_version: Option<u32>,
}

#[derive(Deserialize)]
struct UpdateWorkflowRequest {
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
struct CreateWorkflowVersionResponse {
    pub version: u32,
}

#[derive(Serialize)]
struct WorkflowVersionResponse {
    pub version: u32,
    pub status: String,
    pub definition: serde_json::Value,
}

#[derive(Deserialize)]
struct UpdateWorkflowVersionStatusRequest {
    pub status: String,
}
