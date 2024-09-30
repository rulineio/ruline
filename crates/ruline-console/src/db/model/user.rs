use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub status: String,
    pub name: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
}

mod builder {
    use chrono::Utc;
    use ulid::Ulid;

    #[derive(Default)]
    pub struct Builder {
        pub email: Option<String>,
        pub name: String,
        pub avatar: String,
    }

    impl Builder {
        #[must_use]
        pub fn email(mut self, email: String) -> Self {
            self.email = Some(email);
            self
        }

        #[must_use]
        pub fn name(mut self, name: String) -> Self {
            self.name = name;
            self
        }

        #[must_use]
        pub fn avatar(mut self, avatar: String) -> Self {
            self.avatar = avatar;
            self
        }

        pub fn build(self) -> super::User {
            super::User {
                id: Ulid::new().to_string(),
                email: self.email.expect("email is required"),
                name: self.name,
                avatar: self.avatar,
                status: String::default(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
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
