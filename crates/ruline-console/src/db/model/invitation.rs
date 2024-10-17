use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domain::invitation;

#[derive(Clone, FromRow)]
pub struct Invitation {
    pub id: String,
    pub organization_id: String,
    pub user_id: String,
    pub member_id: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<invitation::Invitation> for Invitation {
    fn from(invitation: invitation::Invitation) -> Self {
        Self {
            id: invitation.id,
            user_id: invitation.user_id,
            organization_id: invitation.organization_id,
            member_id: invitation.member_id,
            status: invitation.status.to_string(),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

impl From<Invitation> for invitation::Invitation {
    fn from(invitation: Invitation) -> Self {
        Self {
            id: invitation.id,
            user_id: invitation.user_id,
            organization_id: invitation.organization_id,
            member_id: invitation.member_id,
            status: invitation.status.into(),
        }
    }
}
