use serde::{Deserialize, Serialize};

use crate::domain::organization;

#[derive(Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub status: String,
    pub logo: String,
}

impl From<organization::Organization> for Organization {
    fn from(organization: organization::Organization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            status: organization.status.to_string(),
            logo: organization.logo,
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
