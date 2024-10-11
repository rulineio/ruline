use std::fmt::Display;

#[derive(Default, Clone)]
pub enum OrganizationStatus {
    #[default]
    Active,
}

impl Display for OrganizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OrganizationStatus::Active => write!(f, "active"),
        }
    }
}

impl From<&str> for OrganizationStatus {
    fn from(s: &str) -> Self {
        match s {
            "active" => OrganizationStatus::Active,
            _ => OrganizationStatus::default(),
        }
    }
}

impl From<String> for OrganizationStatus {
    fn from(s: String) -> Self {
        OrganizationStatus::from(s.as_str())
    }
}

#[derive(Clone)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub status: OrganizationStatus,
    pub logo: String,
}

mod builder {
    use super::*;
    use ulid::Ulid;

    #[derive(Default)]
    pub struct Builder {
        pub id: Option<String>,
        pub name: Option<String>,
        pub logo: Option<String>,
        pub status: OrganizationStatus,
    }

    impl Builder {
        #[must_use]
        pub fn name(mut self, name: String) -> Self {
            self.name = Some(name);
            self
        }

        pub fn id(mut self, id: String) -> Self {
            self.id = Some(id);
            self
        }

        pub fn logo(mut self, logo: String) -> Self {
            self.logo = Some(logo);
            self
        }

        pub fn status(mut self, status: OrganizationStatus) -> Self {
            self.status = status;
            self
        }

        pub fn build(self) -> Organization {
            let id = self.id.unwrap_or_else(|| format!("org_{}", Ulid::new()));

            Organization {
                id,
                name: self.name.expect("name is required"),
                status: self.status,
                logo: self.logo.unwrap_or_default(),
            }
        }
    }
}

impl Organization {
    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }
}
