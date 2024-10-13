use std::fmt::{self, Display, Formatter};

#[derive(Default, Clone)]
pub enum ProjectStatus {
    #[default]
    Active,
}

impl Display for ProjectStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ProjectStatus::Active => write!(f, "active"),
        }
    }
}

impl From<&str> for ProjectStatus {
    fn from(s: &str) -> Self {
        match s {
            "active" => ProjectStatus::Active,
            _ => ProjectStatus::default(),
        }
    }
}

impl From<String> for ProjectStatus {
    fn from(s: String) -> Self {
        ProjectStatus::from(s.as_str())
    }
}

#[derive(Clone)]
pub struct Project {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub status: ProjectStatus,
}

mod builder {
    use super::*;
    use ulid::Ulid;

    #[derive(Default)]
    pub struct Builder {
        pub id: Option<String>,
        pub organization_id: Option<String>,
        pub name: Option<String>,
        pub status: ProjectStatus,
    }

    impl Builder {
        #[must_use]
        pub fn organization_id(mut self, organization_id: String) -> Self {
            self.organization_id = Some(organization_id);
            self
        }

        #[must_use]
        pub fn name(mut self, name: String) -> Self {
            self.name = Some(name);
            self
        }

        pub fn id(mut self, id: String) -> Self {
            self.id = Some(id);
            self
        }

        pub fn status(mut self, status: ProjectStatus) -> Self {
            self.status = status;
            self
        }

        pub fn build(self) -> Project {
            let id = self.id.unwrap_or_else(|| format!("prj_{}", Ulid::new()));

            Project {
                id,
                organization_id: self.organization_id.expect("organization_id is required"),
                name: self.name.expect("name is required"),
                status: self.status,
            }
        }
    }
}

impl Project {
    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }
}
