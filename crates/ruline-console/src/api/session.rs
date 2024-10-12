use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use serde::Serialize;

use crate::{domain::session::Session, App, Result};

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
        user_status: String,
    },
    Member {
        user_status: String,
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
                user_status: user.status.to_string(),
            },
            Session::Member {
                user,
                organization,
                member,
            } => SessionResponse::Member {
                user_status: user.status.to_string(),
                organization_status: organization.status.to_string(),
                member_role: member.role.to_string(),
                member_status: member.status.to_string(),
            },
        }
    }
}
