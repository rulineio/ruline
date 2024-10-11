use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use serde::Serialize;

use crate::{
    domain::{organization::Organization, session::Session, user::User},
    App, Result,
};

pub fn router() -> Router<Arc<App>> {
    Router::new().route("/", get(get_session))
}

async fn get_session(sess: Extension<Session>) -> Result<impl IntoResponse> {
    Ok(Json(SessionResponse::from(sess.0)))
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum SessionResponse {
    Oauth {},
    Unauthenticated {},
    User {
        user_id: String,
        user_status: String,
    },
    Member {
        user_id: String,
        user_status: String,
        organization_id: String,
        organization_status: String,
        member_role: String,
        member_status: String,
    },
}

impl From<Session> for SessionResponse {
    fn from(session: Session) -> Self {
        match session {
            Session::Oauth { .. } => SessionResponse::Oauth {},
            Session::Unauthenticated { .. } => SessionResponse::Unauthenticated {},
            Session::User { user } => SessionResponse::User {
                user_id: user.id,
                user_status: user.status.to_string(),
            },
            Session::Member {
                user,
                organization,
                member,
            } => SessionResponse::Member {
                user_id: user.id,
                user_status: user.status.to_string(),
                organization_id: organization.id,
                organization_status: organization.status.to_string(),
                member_role: member.role.to_string(),
                member_status: member.status.to_string(),
            },
        }
    }
}

#[derive(Serialize)]
struct UserResponse {
    pub id: String,
    pub status: String,
    pub email: String,
    pub name: String,
    pub avatar: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            status: user.status.to_string(),
            email: user.email,
            name: user.name,
            avatar: user.avatar,
        }
    }
}

#[derive(Serialize)]
struct OrganizationResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub avatar: String,
}

impl From<Organization> for OrganizationResponse {
    fn from(organization: Organization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            status: organization.status.to_string(),
            avatar: organization.logo,
        }
    }
}

#[derive(Serialize)]
struct MemberResponse {
    pub organization_id: String,
    pub user_id: String,
    pub role: String,
    pub status: String,
}

impl From<crate::domain::member::Member> for MemberResponse {
    fn from(member: crate::domain::member::Member) -> Self {
        Self {
            organization_id: member.organization_id,
            user_id: member.user_id,
            role: member.role.to_string(),
            status: member.status.to_string(),
        }
    }
}
