use serde::{Deserialize, Serialize};

use crate::domain::session;

use super::{member, organization, user};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Session {
    Oauth {
        state: String,
    },
    Unauthenticated {
        state: String,
        user: user::User,
    },
    User {
        user: user::User,
    },
    Member {
        user: user::User,
        organization: organization::Organization,
        member: member::Member,
    },
}

impl From<session::Session> for Session {
    fn from(session: session::Session) -> Self {
        match session {
            session::Session::Oauth { state } => Session::Oauth { state },
            session::Session::Unauthenticated { state, user } => Session::Unauthenticated {
                state,
                user: user.into(),
            },
            session::Session::User { user } => Session::User { user: user.into() },
            session::Session::Member {
                user,
                organization,
                member,
            } => Session::Member {
                user: user.into(),
                organization: organization.into(),
                member: member.into(),
            },
        }
    }
}

impl From<Session> for session::Session {
    fn from(session: Session) -> Self {
        match session {
            Session::Oauth { state } => session::Session::Oauth { state },
            Session::Unauthenticated { state, user } => session::Session::Unauthenticated {
                state,
                user: user.into(),
            },
            Session::User { user } => session::Session::User { user: user.into() },
            Session::Member {
                user,
                organization,
                member,
            } => session::Session::Member {
                user: user.into(),
                organization: organization.into(),
                member: member.into(),
            },
        }
    }
}
