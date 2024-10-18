use std::fmt::{self, Display, Formatter};

#[derive(Default, Clone, PartialEq)]
pub enum MemberRole {
    Owner,
    Admin,
    Editor,
    Viewer,
    #[default]
    Member,
}

impl MemberRole {
    pub fn is_at_least(&self, role: MemberRole) -> bool {
        match self {
            MemberRole::Owner => true,
            MemberRole::Admin => role != MemberRole::Owner,
            MemberRole::Editor => role != MemberRole::Owner && role != MemberRole::Admin,
            MemberRole::Viewer => role == MemberRole::Viewer || role == MemberRole::Member,
            MemberRole::Member => role == MemberRole::Member,
        }
    }
}

impl Display for MemberRole {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MemberRole::Owner => write!(f, "owner"),
            MemberRole::Admin => write!(f, "admin"),
            MemberRole::Editor => write!(f, "editor"),
            MemberRole::Viewer => write!(f, "viewer"),
            MemberRole::Member => write!(f, "member"),
        }
    }
}

impl From<&str> for MemberRole {
    fn from(s: &str) -> Self {
        match s {
            "owner" => MemberRole::Owner,
            "admin" => MemberRole::Admin,
            "editor" => MemberRole::Editor,
            "viewer" => MemberRole::Viewer,
            "member" => MemberRole::Member,
            _ => MemberRole::default(),
        }
    }
}

impl From<String> for MemberRole {
    fn from(s: String) -> Self {
        MemberRole::from(s.as_str())
    }
}

#[derive(Default, Clone, PartialEq)]
pub enum MemberStatus {
    #[default]
    Active,
    Removed,
    Left,
    Invited,
    Declined,
}

impl Display for MemberStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MemberStatus::Active => write!(f, "active"),
            MemberStatus::Removed => write!(f, "removed"),
            MemberStatus::Left => write!(f, "left"),
            MemberStatus::Invited => write!(f, "invited"),
            MemberStatus::Declined => write!(f, "declined"),
        }
    }
}

impl From<&str> for MemberStatus {
    fn from(s: &str) -> Self {
        match s {
            "active" => MemberStatus::Active,
            "removed" => MemberStatus::Removed,
            "left" => MemberStatus::Left,
            "invited" => MemberStatus::Invited,
            "declined" => MemberStatus::Declined,
            _ => MemberStatus::default(),
        }
    }
}

impl From<String> for MemberStatus {
    fn from(s: String) -> Self {
        MemberStatus::from(s.as_str())
    }
}

#[derive(Clone)]
pub struct Member {
    pub id: String,
    pub user_id: String,
    pub organization_id: String,
    pub role: MemberRole,
    pub status: MemberStatus,
}

mod builder {
    use ulid::Ulid;

    use super::*;

    #[derive(Default)]
    pub struct Builder {
        pub id: Option<String>,
        pub user_id: Option<String>,
        pub organization_id: Option<String>,
        pub role: MemberRole,
        pub status: MemberStatus,
    }

    impl Builder {
        #[must_use]
        pub fn user_id(mut self, user_id: String) -> Self {
            self.user_id = Some(user_id);
            self
        }

        #[must_use]
        pub fn organization_id(mut self, organization_id: String) -> Self {
            self.organization_id = Some(organization_id);
            self
        }

        #[must_use]
        pub fn role(mut self, role: MemberRole) -> Self {
            self.role = role;
            self
        }

        #[must_use]
        pub fn status(mut self, status: MemberStatus) -> Self {
            self.status = status;
            self
        }

        pub fn id(mut self, id: String) -> Self {
            self.id = Some(id);
            self
        }

        pub fn build(self) -> Member {
            let id = self.id.unwrap_or_else(|| format!("mem_{}", Ulid::new()));

            Member {
                id,
                user_id: self.user_id.expect("user_id is required"),
                organization_id: self.organization_id.expect("organization_id is required"),
                role: self.role,
                status: self.status,
            }
        }
    }
}

impl Member {
    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }
}
