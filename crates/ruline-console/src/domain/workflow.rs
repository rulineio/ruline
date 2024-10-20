use std::fmt::{self, Display, Formatter};

use serde_json::Value;

#[derive(Default, Clone)]
pub enum WorkflowStatus {
    #[default]
    Active,
    Archived,
}

impl Display for WorkflowStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            WorkflowStatus::Active => write!(f, "active"),
            WorkflowStatus::Archived => write!(f, "archived"),
        }
    }
}

impl From<&str> for WorkflowStatus {
    fn from(s: &str) -> Self {
        match s {
            "active" => WorkflowStatus::Active,
            "archived" => WorkflowStatus::Archived,
            _ => WorkflowStatus::default(),
        }
    }
}

impl From<String> for WorkflowStatus {
    fn from(s: String) -> Self {
        WorkflowStatus::from(s.as_str())
    }
}

#[derive(Clone)]
pub struct Workflow {
    pub id: String,
    pub organization_id: String,
    pub project_id: String,
    pub name: String,
    pub status: WorkflowStatus,
    pub active_version: Option<u32>,
}

mod builder {
    use super::*;
    use ulid::Ulid;

    #[derive(Default)]
    pub struct Builder {
        pub id: Option<String>,
        pub organization_id: Option<String>,
        pub project_id: Option<String>,
        pub name: Option<String>,
        pub status: WorkflowStatus,
        pub active_version: Option<u32>,
    }

    impl Builder {
        #[must_use]
        pub fn organization_id(mut self, organization_id: String) -> Self {
            self.organization_id = Some(organization_id);
            self
        }

        #[must_use]
        pub fn project_id(mut self, project_id: String) -> Self {
            self.project_id = Some(project_id);
            self
        }

        #[must_use]
        pub fn name(mut self, name: String) -> Self {
            self.name = Some(name);
            self
        }

        #[must_use]
        pub fn status(mut self, status: WorkflowStatus) -> Self {
            self.status = status;
            self
        }

        pub fn active_version(mut self, active_version: u32) -> Self {
            self.active_version = Some(active_version);
            self
        }

        pub fn build(self) -> Workflow {
            let id = self.id.unwrap_or_else(|| format!("wrk_{}", Ulid::new()));

            Workflow {
                id,
                organization_id: self.organization_id.expect("organization_id is required"),
                project_id: self.project_id.expect("project_id is required"),
                name: self.name.expect("name is required"),
                status: self.status,
                active_version: self.active_version,
            }
        }
    }
}

impl Workflow {
    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }

    pub fn initial_version(&self) -> WorkflowVersion {
        WorkflowVersion::builder()
            .organization_id(self.organization_id.to_owned())
            .project_id(self.project_id.to_owned())
            .workflow_id(self.id.to_owned())
            .version(1)
            .build()
    }
}

#[derive(Default, Clone, PartialEq)]
pub enum WorkflowVersionStatus {
    #[default]
    Draft,
    InReview,
    Published,
    Archived,
}

impl Display for WorkflowVersionStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            WorkflowVersionStatus::Draft => write!(f, "draft"),
            WorkflowVersionStatus::InReview => write!(f, "in_review"),
            WorkflowVersionStatus::Published => write!(f, "published"),
            WorkflowVersionStatus::Archived => write!(f, "archived"),
        }
    }
}

impl From<&str> for WorkflowVersionStatus {
    fn from(s: &str) -> Self {
        match s {
            "draft" => WorkflowVersionStatus::Draft,
            "in_review" => WorkflowVersionStatus::InReview,
            "published" => WorkflowVersionStatus::Published,
            "archived" => WorkflowVersionStatus::Archived,
            _ => WorkflowVersionStatus::default(),
        }
    }
}

impl From<String> for WorkflowVersionStatus {
    fn from(s: String) -> Self {
        WorkflowVersionStatus::from(s.as_str())
    }
}

#[derive(Clone)]
pub struct WorkflowVersion {
    pub organization_id: String,
    pub project_id: String,
    pub workflow_id: String,
    pub version: u32,
    pub status: WorkflowVersionStatus,
    pub definition: Value,
}

mod version_builder {
    use super::*;

    #[derive(Default)]
    pub struct Builder {
        pub organization_id: Option<String>,
        pub project_id: Option<String>,
        pub workflow_id: Option<String>,
        pub version: Option<u32>,
        pub status: WorkflowVersionStatus,
        pub definition: Value,
    }

    impl Builder {
        #[must_use]
        pub fn organization_id(mut self, organization_id: String) -> Self {
            self.organization_id = Some(organization_id);
            self
        }

        #[must_use]
        pub fn project_id(mut self, project_id: String) -> Self {
            self.project_id = Some(project_id);
            self
        }

        #[must_use]
        pub fn workflow_id(mut self, workflow_id: String) -> Self {
            self.workflow_id = Some(workflow_id);
            self
        }

        #[must_use]
        pub fn version(mut self, version: u32) -> Self {
            self.version = Some(version);
            self
        }

        pub fn definition(mut self, definition: Value) -> Self {
            self.definition = definition;
            self
        }

        pub fn status(mut self, status: WorkflowVersionStatus) -> Self {
            self.status = status;
            self
        }

        #[must_use]
        pub fn build(self) -> WorkflowVersion {
            WorkflowVersion {
                organization_id: self.organization_id.expect("organization_id is required"),
                project_id: self.project_id.expect("project_id is required"),
                workflow_id: self.workflow_id.expect("workflow_id is required"),
                version: self.version.expect("version is required"),
                status: self.status,
                definition: self.definition,
            }
        }
    }
}

impl WorkflowVersion {
    pub fn builder() -> version_builder::Builder {
        version_builder::Builder::default()
    }

    pub fn next_version(&self) -> WorkflowVersion {
        WorkflowVersion::builder()
            .organization_id(self.organization_id.to_owned())
            .project_id(self.project_id.to_owned())
            .workflow_id(self.workflow_id.to_owned())
            .version(self.version + 1)
            .status(WorkflowVersionStatus::Draft)
            .definition(self.definition.to_owned())
            .build()
    }
}
