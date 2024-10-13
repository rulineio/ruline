use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domain::project;

#[derive(Clone, FromRow)]
pub struct Project {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<project::Project> for Project {
    fn from(project: project::Project) -> Self {
        Self {
            id: project.id,
            organization_id: project.organization_id,
            name: project.name,
            status: project.status.to_string(),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

impl From<Project> for project::Project {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            organization_id: project.organization_id,
            name: project.name,
            status: project.status.into(),
        }
    }
}
