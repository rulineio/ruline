use crate::domain::user::{User, UserStatus};

use super::*;

impl Database {
    pub async fn store_user(&self, user: &User) -> Result<User> {
        _ = sqlx::query(INSERT)
            .bind(&user.id)
            .bind(&user.email)
            .bind(&user.name)
            .bind(&user.avatar)
            .execute(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        self.get_user(&user.id)
            .await?
            .ok_or(DatabaseError::NotFound.into())
    }

    pub async fn get_user(&self, id: &str) -> Result<Option<User>> {
        let user: Option<user::User> = sqlx::query_as(SELECT)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(user.map(Into::into))
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user: Option<user::User> = sqlx::query_as(SELECT_BY_EMAIL)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(user.map(Into::into))
    }

    pub async fn set_user_last_login(&self, id: &str) -> Result<()> {
        let res = sqlx::query(SET_LAST_LOGIN)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        if res.rows_affected() == 0 {
            return Err(DatabaseError::NotFound.into());
        }

        Ok(())
    }

    pub async fn update_user_status(&self, id: &str, status: UserStatus) -> Result<()> {
        let res = sqlx::query(UPDATE_STATUS)
            .bind(status.to_string())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        if res.rows_affected() == 0 {
            return Err(DatabaseError::NotFound.into());
        }

        Ok(())
    }
}

const INSERT: &str = r#"
    insert into users (id, email, name, avatar)
    values (?, ?, ?, ?)
"#;

const SELECT: &str = r#"
    select id, email, status, name, avatar, created_at, updated_at, last_login
    from users
    where id = ?
"#;

const SELECT_BY_EMAIL: &str = r#"
    select id, email, status, name, avatar, created_at, updated_at, last_login
    from users
    where email = ?
"#;

const SET_LAST_LOGIN: &str = r#"
    update users
    set last_login = now()
    where id = ?
"#;

const UPDATE_STATUS: &str = r#"
    update users
    set status = ?
    where id = ?
"#;
