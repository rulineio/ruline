use std::fmt::{self, Display, Formatter};

#[derive(Default, Clone)]
pub enum InvitationStatus {
    #[default]
    Created,
    Accepted,
    Declined,
}

impl Display for InvitationStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InvitationStatus::Created => write!(f, "created"),
            InvitationStatus::Accepted => write!(f, "accepted"),
            InvitationStatus::Declined => write!(f, "declined"),
        }
    }
}

impl From<&str> for InvitationStatus {
    fn from(s: &str) -> Self {
        match s {
            "created" => InvitationStatus::Created,
            "accepted" => InvitationStatus::Accepted,
            "declined" => InvitationStatus::Declined,
            _ => InvitationStatus::default(),
        }
    }
}

impl From<String> for InvitationStatus {
    fn from(s: String) -> Self {
        InvitationStatus::from(s.as_str())
    }
}

#[derive(Clone)]
pub struct Invitation {
    pub id: String,
    pub user_id: String,
    pub organization_id: String,
    pub member_id: String,
    pub status: InvitationStatus,
}

mod builder {
    use super::*;
    use ulid::Ulid;

    #[derive(Default)]
    pub struct Builder {
        pub id: Option<String>,
        pub user_id: Option<String>,
        pub organization_id: Option<String>,
        pub member_id: Option<String>,
        pub status: InvitationStatus,
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
        pub fn member_id(mut self, member_id: String) -> Self {
            self.member_id = Some(member_id);
            self
        }

        pub fn id(mut self, id: String) -> Self {
            self.id = Some(id);
            self
        }

        pub fn build(self) -> Invitation {
            Invitation {
                id: self.id.unwrap_or_else(|| format!("inv_{}", Ulid::new())),
                user_id: self.user_id.expect("user_id is required"),
                organization_id: self.organization_id.expect("organization_id is required"),
                member_id: self.member_id.expect("member_id is required"),
                status: self.status,
            }
        }
    }
}

impl Invitation {
    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }
}
