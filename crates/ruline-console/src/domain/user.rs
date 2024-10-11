use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, Utc};
use sha2::Digest;

#[derive(Default, Clone)]
pub enum UserStatus {
    #[default]
    Created,
    Active,
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            UserStatus::Created => write!(f, "created"),
            UserStatus::Active => write!(f, "active"),
        }
    }
}

impl From<&str> for UserStatus {
    fn from(s: &str) -> Self {
        match s {
            "created" => UserStatus::Created,
            "active" => UserStatus::Active,
            _ => UserStatus::default(),
        }
    }
}

impl From<String> for UserStatus {
    fn from(s: String) -> Self {
        UserStatus::from(s.as_str())
    }
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub status: UserStatus,
    pub name: String,
    pub avatar: String,
    pub last_login: DateTime<Utc>,
}

mod builder {
    use super::*;
    use ulid::Ulid;

    #[derive(Default)]
    pub struct Builder {
        pub id: Option<String>,
        pub email: Option<String>,
        pub name: Option<String>,
        pub avatar: Option<String>,
        pub status: UserStatus,
    }

    impl Builder {
        #[must_use]
        pub fn email(mut self, email: String) -> Self {
            self.email = Some(email);
            self
        }

        #[must_use]
        pub fn name(mut self, name: String) -> Self {
            self.name = Some(name);
            self
        }

        pub fn avatar(mut self, avatar: String) -> Self {
            self.avatar = Some(avatar);
            self
        }

        pub fn id(mut self, id: String) -> Self {
            self.id = Some(id);
            self
        }

        pub fn status(mut self, status: UserStatus) -> Self {
            self.status = status;
            self
        }

        pub fn build(self) -> super::User {
            let id = self.id.unwrap_or_else(|| format!("usr_{}", Ulid::new()));
            let email = self.email.expect("email is required");
            let avatar = self.avatar.unwrap_or_else(|| build_gravatar(&email));

            User {
                id,
                email,
                name: self.name.expect("name is required"),
                avatar,
                status: self.status,
                last_login: Utc::now(),
            }
        }
    }
}

impl User {
    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }
}

fn build_gravatar(email: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(email.to_lowercase());
    format!(
        "https://www.gravatar.com/avatar/{}",
        hex::encode(hasher.finalize())
    )
}
