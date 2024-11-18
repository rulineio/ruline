use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Extension, Json, Router,
};
use futures::{
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{debug, error, info, info_span, instrument, warn, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{
    domain::{
        member::{Member, MemberRole},
        session::Session,
        user::User,
        workflow::{Workflow, WorkflowVersionStatus},
    },
    editor::{VersionEditor, VersionEditorEvent, VersionEditorMember},
    error::Error,
    util::ResultExt,
    App, Result,
};

pub fn router() -> Router<Arc<App>> {
    let vwr = Router::new()
        .route("/", get(get_workflow_versions))
        .route("/", post(create_workflow_version))
        .route("/:version", get(get_workflow_version))
        .route("/:version", patch(update_workflow_version_status))
        .route("/:version/editor", get(get_editor));

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
    Span::current().set_attribute("project.id", project_id.to_owned());
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

    info!({
        workflow.id = %workflow.id,
        workflow.name = %workflow.name,
    },
    "Created workflow"
    );

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
    Span::current().set_attribute("project.id", project_id.to_owned());
    Span::current().set_attribute("workflow.id", workflow_id.to_owned());
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
    Span::current().set_attribute("project.id", project_id.to_owned());
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
    Span::current().set_attribute("project.id", project_id.to_owned());
    Span::current().set_attribute("workflow.id", workflow_id.to_owned());
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

    info!({ workflow.id = %workflow_id },"Updated workflow");

    Ok(StatusCode::NO_CONTENT)
}

async fn create_workflow_version(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path((project_id, workflow_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    Span::current().set_attribute("project.id", project_id.to_owned());
    Span::current().set_attribute("workflow.id", workflow_id.to_owned());
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

    info!({
        workflow.id = %workflow.id,
        workflow.version = new_version.version,
    },
    "Created workflow version"
    );

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
    Span::current().set_attribute("project.id", project_id.to_owned());
    Span::current().set_attribute("workflow.id", workflow_id.to_owned());
    Span::current().set_attribute("workflow.version", i64::from(version));
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
    Span::current().set_attribute("project.id", project_id.to_owned());
    Span::current().set_attribute("workflow.id", workflow_id.to_owned());
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
    Span::current().set_attribute("project.id", project_id.to_owned());
    Span::current().set_attribute("workflow.id", workflow_id.to_owned());
    Span::current().set_attribute("workflow.version", i64::from(version));
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
            status.to_owned(),
            &mut trx,
        )
        .await?;
    app.db.commit(trx).await?;

    info!({
        workflow.id = %workflow_id,
        workflow.version = version.version,
        workflow.status = %status,
    },
    "Updated workflow version status"
    );

    Ok(StatusCode::NO_CONTENT)
}

pub async fn handle_version_ws(
    ws: WebSocketUpgrade,
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
) -> Result<impl IntoResponse> {
    let (organization_id, member, user) = match session {
        Session::Member {
            organization,
            member,
            user,
        } => (organization.id, member, user),
        _ => return Err(Error::Unauthorized),
    };

    Ok(ws.on_upgrade(move |socket| {
        handle_version_socket(socket, app, organization_id, member, user)
    }))
}

#[instrument(
    skip(socket, app, organization_id, member, user),
    name = "Handle Socket",
    fields(otel.kind = "server", member.id = %member.id, organization.id = %organization_id)
)]
async fn handle_version_socket(
    socket: WebSocket,
    app: Arc<App>,
    organization_id: String,
    member: Member,
    user: User,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut editor = None::<Arc<Mutex<VersionEditor>>>;

    while let Some(msg) = receiver.next().await {
        let _span = info_span!("Handle Message").entered();

        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                warn!({ exception.message = %e }, "Failed to receive message");
                continue;
            }
        };

        if let Message::Text(con_req) = msg {
            let version_msg: VersionMessage = match serde_json::from_str(&con_req) {
                Ok(msg) => msg,
                Err(e) => {
                    warn!({ exception.message = %e }, "Failed to parse message");
                    continue;
                }
            };

            if let VersionMessage::Connect {
                project_id,
                workflow_id,
                version,
            } = version_msg
            {
                Span::current().set_attribute("project.id", project_id.to_owned());
                Span::current().set_attribute("workflow.id", workflow_id.to_owned());
                Span::current().set_attribute("workflow.version", i64::from(version));
                Span::current().set_attribute("event", "connect");

                editor = Some(app.editor(&organization_id, &project_id, &workflow_id, version));
            }

            if editor.is_some() {
                break;
            }
        }
    }

    debug!("Connection established");

    let editor = match editor {
        Some(editor) => editor,
        None => return,
    };

    let mut rx = {
        let editor = editor.lock().await;
        editor.subscribe()
    };

    {
        let mut editor = editor.lock().await;
        let _ = editor
            .add_member(
                member.id.to_owned(),
                user.name.to_owned(),
                user.avatar.to_owned(),
            )
            .log_error("Failed to add member");
    }

    let receiver_editor = editor.clone();
    let mut receive = tokio::spawn(
        async move {
            while let Some(msg) = receiver.next().await {
                let msg = msg.unwrap();
                handle_message(msg, receiver_editor.clone()).await;
            }
        }
        .in_current_span(),
    );

    let mut send = tokio::spawn(
        async move {
            while let Ok(msg) = rx.recv().await {
                handle_event(msg, &mut sender).await;
            }
        }
        .in_current_span(),
    );

    tokio::select! {
        _ = (&mut receive) => send.abort(),
        _ = (&mut send) => receive.abort(),
    }

    let mut editor = editor.lock().await;
    let _ = editor
        .remove_member(&member.id)
        .log_error("Failed to remove member");

    debug!("Connection closed");
}

#[instrument(skip(msg, editor), name = "Handle Message")]
async fn handle_message(msg: Message, editor: Arc<Mutex<VersionEditor>>) {
    let editor = editor.lock().await;
    if let Message::Text(con_req) = msg {
        let version_msg: VersionMessage = match serde_json::from_str(&con_req) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Failed to parse message: {:?}", e);
                return;
            }
        };
        editor.send_event(version_msg.into()).unwrap()
    }
}

#[instrument(skip(event, sender), name = "Handle Event", fields(otel.kind = "consumer", event = event.name()))]
async fn handle_event(event: VersionEditorEvent, sender: &mut SplitSink<WebSocket, Message>) {
    let event = VersionMessage::from(event);
    let event = serde_json::to_string(&event).unwrap();
    let msg = Message::Text(event);
    sender
        .send(msg)
        .instrument(info_span!("Send Message"))
        .await
        .unwrap();
}

async fn get_editor(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path((project_id, workflow_id, version)): Path<(String, String, u32)>,
) -> Result<impl IntoResponse> {
    let organization_id = match session {
        Session::Member { organization, .. } => organization.id,
        _ => return Err(Error::Unauthorized),
    };

    let editor = app.editor(&organization_id, &project_id, &workflow_id, version);
    let editor = editor.lock().await;

    let members = editor.members.iter().map(Into::into).collect::<Vec<_>>();

    Ok(Json(VersionEditorResponse { members }))
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

#[derive(Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
enum VersionMessage {
    Connect {
        project_id: String,
        workflow_id: String,
        version: u32,
    },
    MemberJoined {
        member_id: String,
        name: String,
        avatar: String,
    },
    MemberLeft {
        member_id: String,
    },
}

impl From<VersionEditorEvent> for VersionMessage {
    fn from(msg: VersionEditorEvent) -> Self {
        match msg {
            VersionEditorEvent::MemberJoined {
                avatar,
                member_id,
                name,
            } => VersionMessage::MemberJoined {
                avatar,
                member_id,
                name,
            },
            VersionEditorEvent::MemberLeft { member_id } => {
                VersionMessage::MemberLeft { member_id }
            }
        }
    }
}

impl Into<VersionEditorEvent> for VersionMessage {
    fn into(self) -> VersionEditorEvent {
        match self {
            VersionMessage::MemberJoined {
                avatar,
                member_id,
                name,
            } => VersionEditorEvent::MemberJoined {
                avatar,
                member_id,
                name,
            },
            VersionMessage::MemberLeft { member_id } => {
                VersionEditorEvent::MemberLeft { member_id }
            }
            _ => panic!("Invalid message"),
        }
    }
}

#[derive(Serialize)]
struct VersionEditorMemberResponse {
    pub id: String,
    pub name: String,
    pub avatar: String,
}

impl From<&VersionEditorMember> for VersionEditorMemberResponse {
    fn from(member: &VersionEditorMember) -> Self {
        Self {
            id: member.id.to_owned(),
            name: member.name.to_owned(),
            avatar: member.avatar.to_owned(),
        }
    }
}

#[derive(Serialize)]
struct VersionEditorResponse {
    pub members: Vec<VersionEditorMemberResponse>,
}
