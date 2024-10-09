use serde::{Deserialize, Serialize};

use crate::domain::member;

#[derive(Clone, Deserialize, Serialize)]
pub struct Member {
    pub organization_id: String,
    pub user_id: String,
    pub role: String,
    pub status: String,
}

impl From<member::Member> for Member {
    fn from(member: member::Member) -> Self {
        Self {
            organization_id: member.organization_id,
            user_id: member.user_id,
            role: member.role.to_string(),
            status: member.status.to_string(),
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
