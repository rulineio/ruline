use std::sync::Arc;

use axum::{
    extract::State,
    http::status::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        member::{Member, MemberRole},
        organization::Organization,
        project::Project,
        session::Session,
        user::UserStatus,
    },
    error::Error,
    util, App, Result,
};

pub fn router() -> Router<Arc<App>> {
    Router::new()
        .route("/", get(get_organization))
        .route("/", post(create_organization))
}

async fn get_organization(Extension(session): Extension<Session>) -> Result<impl IntoResponse> {
    let organization = match session {
        Session::Member { organization, .. } => organization,
        _ => return Err(Error::Unauthorized),
    };

    Ok(Json(OrganizationResponse {
        id: organization.id,
        name: organization.name,
        logo: organization.logo,
        status: organization.status.to_string(),
    }))
}

async fn create_organization(
    State(app): State<Arc<App>>,
    jar: CookieJar,
    Extension(session): Extension<Session>,
    Json(body): Json<CreateOrganizationRequest>,
) -> Result<impl IntoResponse> {
    let user = match session {
        Session::User { user } => user,
        Session::Member { user, .. } => user,
        _ => return Err(Error::Unauthorized),
    };

    let organization = Organization::builder().name(body.name).build();
    let member = Member::builder()
        .organization_id(organization.id.to_owned())
        .user_id(user.id.to_owned())
        .role(MemberRole::Owner)
        .build();
    let project = Project::builder()
        .organization_id(organization.id.to_owned())
        .name("Default".to_owned())
        .build();

    app.db.store_organization(&organization).await?;
    app.db.store_member(&member).await?;
    app.db
        .update_user_status(&user.id, UserStatus::Active)
        .await?;
    app.db.store_project(&project).await?;

    let new_session = Session::Member {
        user,
        organization,
        member,
    };
    let new_session_id = util::random_string();

    app.cache
        .set_session(&new_session_id, &new_session.into())
        .await?;

    app.cache
        .delete_session(jar.get("sid").unwrap().value())
        .await?;

    let cookie = Cookie::build(("sid", new_session_id))
        .same_site(SameSite::Lax)
        .path("/")
        .secure(!app.config.is_dev())
        .http_only(true)
        .max_age(Duration::weeks(1).to_std().unwrap().try_into().unwrap())
        .build();

    Ok((
        StatusCode::CREATED,
        jar.remove("sid").add(cookie),
        Json(CreateOrganizationResponse {
            project_id: project.id,
        }),
    ))
}

#[derive(Serialize)]
struct OrganizationResponse {
    pub id: String,
    pub name: String,
    pub logo: String,
    pub status: String,
}

#[derive(Deserialize)]
struct CreateOrganizationRequest {
    pub name: String,
}

#[derive(Serialize)]
struct CreateOrganizationResponse {
    pub project_id: String,
}
