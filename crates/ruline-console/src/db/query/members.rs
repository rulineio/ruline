use crate::domain::member::Member;

use super::*;

impl Database {
    pub async fn store_member(&self, member: &Member) -> Result<Member> {
        sqlx::query(INSERT)
            .bind(&member.id)
            .bind(&member.organization_id)
            .bind(&member.user_id)
            .bind(member.role.to_string())
            .execute(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        self.get_member(&member.id)
            .await?
            .ok_or(DatabaseError::NotFound.into())
    }

    pub async fn get_member(&self, member_id: &str) -> Result<Option<Member>> {
        let member: Option<member::Member> = sqlx::query_as(SELECT)
            .bind(member_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(member.map(Into::into))
    }

    pub async fn get_members_by_user_id(&self, user_id: &str) -> Result<Vec<Member>> {
        let members: Vec<member::Member> = sqlx::query_as(SELECT_BY_USER_ID)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(members.into_iter().map(Into::into).collect())
    }
}

const INSERT: &str = r#"
    INSERT INTO members (id, organization_id, user_id, role)
    VALUES (?, ?, ?, ?)
"#;

const SELECT: &str = r#"
    SELECT id, organization_id, user_id, role, status, created_at, updated_at
    FROM members
    WHERE id = ?
"#;

const SELECT_BY_USER_ID: &str = r#"
    SELECT id, organization_id, user_id, role, status, created_at, updated_at
    FROM members
    WHERE user_id = ?
"#;
