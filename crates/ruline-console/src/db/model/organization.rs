use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domain::organization;

#[derive(Clone, FromRow)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub status: String,
    pub logo: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<organization::Organization> for Organization {
    fn from(organization: organization::Organization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            status: organization.status.to_string(),
            logo: organization.logo,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

impl From<Organization> for organization::Organization {
    fn from(organization: Organization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            status: organization.status.into(),
            logo: organization.logo,
        }
    }
}

#[derive(Clone, FromRow)]
pub struct OrganizationMember {
    pub name: String,
    pub email: String,
    pub avatar: String,
    pub role: String,
    pub status: String,
}
