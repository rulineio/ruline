use tracing::instrument;

use crate::domain::member::{Member, MemberStatus};

use super::*;

impl Database {
    #[instrument(
        skip_all,
        fields(
            member.id = %member.id,
            otel.name = "INSERT members",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "members",
            db.namespace = "ruline",
            db.operation.name = "INSERT",
            db.query.text = INSERT.trim()
        )
    )]
    pub async fn store_member(
        &self,
        member: &Member,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        sqlx::query(INSERT)
            .bind(&member.id)
            .bind(&member.organization_id)
            .bind(&member.user_id)
            .bind(member.role.to_string())
            .bind(member.status.to_string())
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            member.id = %member_id,
            otel.name = "SELECT members",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "members",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT.trim()
        )
    )]
    pub async fn get_member(&self, member_id: &str) -> Result<Option<Member>> {
        let member: Option<member::Member> = sqlx::query_as(SELECT)
            .bind(member_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(member.map(Into::into))
    }

    #[instrument(
        skip_all,
        fields(
            user.id = %user_id,
            otel.name = "SELECT members",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "members",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT_BY_USER_ID.trim()
        )
    )]
    pub async fn get_members_by_user_id(&self, user_id: &str) -> Result<Vec<Member>> {
        let members: Vec<member::Member> = sqlx::query_as(SELECT_BY_USER_ID)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(members.into_iter().map(Into::into).collect())
    }

    #[instrument(
        skip_all,
        fields(
            member.id = %member_id,
            member.status = %status,
            otel.name = "UPDATE members",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "members",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_STATUS.trim()
        )
    )]
    pub async fn set_member_status(
        &self,
        member_id: &str,
        status: MemberStatus,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        let res = sqlx::query(SET_STATUS)
            .bind(status.to_string())
            .bind(member_id)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        if res.rows_affected() == 0 {
            return Err(DatabaseError::NotFound.into());
        }

        Ok(())
    }
}

const INSERT: &str = r#"
    INSERT INTO members (id, organization_id, user_id, role, status)
    VALUES (?, ?, ?, ?, ?)
"#;

const SELECT: &str = r#"
    SELECT id, organization_id, user_id, role, status, created_at, updated_at
    FROM members
    WHERE id = ?
"#;

const SELECT_BY_USER_ID: &str = r#"
    SELECT id, organization_id, user_id, role, status, created_at, updated_at
    FROM members
    WHERE user_id = ? AND status = 'active'
"#;

const SET_STATUS: &str = r#"
    UPDATE members
    SET status = ?
    WHERE id = ?
"#;
