use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domain::workflow;

#[derive(Clone, FromRow)]
pub struct Workflow {
    pub id: String,
    pub organization_id: String,
    pub project_id: String,
    pub name: String,
    pub status: String,
    pub active_version: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<workflow::Workflow> for Workflow {
    fn from(workflow: workflow::Workflow) -> Self {
        Self {
            id: workflow.id,
            organization_id: workflow.organization_id,
            project_id: workflow.project_id,
            name: workflow.name,
            status: workflow.status.to_string(),
            active_version: workflow.active_version,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

impl From<Workflow> for workflow::Workflow {
    fn from(workflow: Workflow) -> Self {
        Self {
            id: workflow.id,
            organization_id: workflow.organization_id,
            project_id: workflow.project_id,
            name: workflow.name,
            status: workflow.status.into(),
            active_version: workflow.active_version,
        }
    }
}

#[derive(Clone, FromRow)]
pub struct WorkflowVersion {
    pub organization_id: String,
    pub project_id: String,
    pub workflow_id: String,
    pub version: u32,
    pub status: String,
    pub definition: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<workflow::WorkflowVersion> for WorkflowVersion {
    fn from(version: workflow::WorkflowVersion) -> Self {
        Self {
            organization_id: version.organization_id,
            project_id: version.project_id,
            workflow_id: version.workflow_id,
            version: version.version,
            status: version.status.to_string(),
            definition: version.definition,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

impl From<WorkflowVersion> for workflow::WorkflowVersion {
    fn from(version: WorkflowVersion) -> Self {
        Self {
            organization_id: version.organization_id,
            project_id: version.project_id,
            workflow_id: version.workflow_id,
            version: version.version,
            status: version.status.into(),
            definition: version.definition,
        }
    }
}
