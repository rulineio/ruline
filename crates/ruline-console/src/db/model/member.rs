use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domain::member;

#[derive(Clone, FromRow)]
pub struct Member {
    pub organization_id: String,
    pub user_id: String,
    pub role: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<member::Member> for Member {
    fn from(member: member::Member) -> Self {
        Self {
            organization_id: member.organization_id,
            user_id: member.user_id,
            role: member.role.to_string(),
            status: member.status.to_string(),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

impl From<Member> for member::Member {
    fn from(member: Member) -> Self {
        Self {
            organization_id: member.organization_id,
            user_id: member.user_id,
            role: member.role.into(),
            status: member.status.into(),
        }
    }
}
