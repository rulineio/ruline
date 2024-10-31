use tracing::instrument;

use crate::domain::user::{User, UserStatus};

use super::*;

impl Database {
    #[instrument(
        skip_all,
        fields(
            user.id = %user.id,
            otel.name = "INSERT users",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "users",
            db.namespace = "ruline",
            db.operation.name = "INSERT",
            db.query.text = INSERT.trim(),
        )
    )]
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

    #[instrument(
        skip_all,
        fields(
            user.id = %user.id,
            otel.name = "INSERT users",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "users",
            db.namespace = "ruline",
            db.operation.name = "INSERT",
            db.query.text = INSERT.trim()
        )
    )]
    pub async fn store_user_trx(
        &self,
        user: &User,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        _ = sqlx::query(INSERT)
            .bind(&user.id)
            .bind(&user.email)
            .bind(&user.name)
            .bind(&user.avatar)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            user.id = %id,
            otel.name = "SELECT users",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "users",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT.trim()
        )
    )]
    pub async fn get_user(&self, id: &str) -> Result<Option<User>> {
        let user: Option<user::User> = sqlx::query_as(SELECT)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(user.map(Into::into))
    }

    #[instrument(
        skip_all,
        fields(
            user.email = %email,
            otel.name = "SELECT users",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "users",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT_BY_EMAIL.trim()
        )
    )]
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user: Option<user::User> = sqlx::query_as(SELECT_BY_EMAIL)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(user.map(Into::into))
    }

    #[instrument(
        skip_all,
        fields(
            user.id = %id,
            otel.name = "UPDATE users",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "users",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_LAST_LOGIN.trim()
        )
    )]
    pub async fn set_user_last_login(
        &self,
        id: &str,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        let res = sqlx::query(SET_LAST_LOGIN)
            .bind(id)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        if res.rows_affected() == 0 {
            return Err(DatabaseError::NotFound.into());
        }

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            user.id = %id,
            user.status = %status,
            otel.name = "UPDATE users",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "users",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_STATUS.trim()
        )
    )]
    pub async fn set_user_status(
        &self,
        id: &str,
        status: UserStatus,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        let res = sqlx::query(SET_STATUS)
            .bind(status.to_string())
            .bind(id)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        if res.rows_affected() == 0 {
            return Err(DatabaseError::NotFound.into());
        }

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            user.id = %id,
            otel.name = "UPDATE users",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "users",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_AVATAR_NAME.trim()
        )
    )]
    pub async fn update_user(
        &self,
        id: &str,
        avatar: Option<&str>,
        name: Option<&str>,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        let _ = match (avatar, name) {
            (Some(avatar), Some(name)) => sqlx::query(SET_AVATAR_NAME)
                .bind(avatar)
                .bind(name)
                .bind(id)
                .execute(&mut **trx)
                .await
                .map_err(DatabaseError::Sqlx)?,
            (Some(avatar), None) => sqlx::query(SET_AVATAR)
                .bind(avatar)
                .bind(id)
                .execute(&mut **trx)
                .await
                .map_err(DatabaseError::Sqlx)?,
            (None, Some(name)) => sqlx::query(SET_NAME)
                .bind(name)
                .bind(id)
                .execute(&mut **trx)
                .await
                .map_err(DatabaseError::Sqlx)?,
            _ => return Ok(()),
        };

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

const SET_STATUS: &str = r#"
    update users
    set status = ?
    where id = ?
    "#;

const SET_AVATAR: &str = r#"
    update users
    set avatar = ?
    where id = ?
    "#;

const SET_NAME: &str = r#"
    update users
    set name = ?
    where id = ?
    "#;

const SET_AVATAR_NAME: &str = r#"
    update users
    set avatar = ?, name = ?
    where id = ?
    "#;
