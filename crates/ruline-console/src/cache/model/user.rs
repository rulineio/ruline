use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::user;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub status: String,
    pub avatar: String,
    pub last_login: DateTime<Utc>,
}

impl From<user::User> for User {
    fn from(user: user::User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            status: user.status.to_string(),
            avatar: user.avatar,
            name: user.name,
            last_login: user.last_login,
        }
    }
}

impl From<User> for user::User {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            status: user.status.into(),
            avatar: user.avatar,
            name: user.name,
            last_login: user.last_login,
        }
    }
}
