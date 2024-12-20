use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domain::user;

#[derive(Clone, FromRow)]
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

impl From<user::User> for User {
    fn from(user: user::User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            status: user.status.to_string(),
            name: user.name,
            avatar: user.avatar,
            last_login: user.last_login,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
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
