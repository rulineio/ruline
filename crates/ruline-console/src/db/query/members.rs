use super::*;

impl Database {
    pub async fn store_member(&self, member: &member::Member) -> Result<member::Member> {
        sqlx::query(INSERT)
            .bind(&member.organization_id)
            .bind(&member.user_id)
            .bind(&member.role)
            .execute(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        self.get_member(&member.organization_id, &member.user_id)
            .await?
            .ok_or(DatabaseError::NotFound.into())
    }

    pub async fn get_member(
        &self,
        organization_id: &str,
        user_id: &str,
    ) -> Result<Option<member::Member>> {
        let member = sqlx::query_as(SELECT)
            .bind(user_id)
            .bind(organization_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(member)
    }

    pub async fn get_members_by_user_id(&self, user_id: &str) -> Result<Vec<member::Member>> {
        let members = sqlx::query_as(SELECT_BY_USER_ID)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(members)
    }
}

const INSERT: &str = r#"
    INSERT INTO members (organization_id, user_id, role)
    VALUES ($1, $2, $3)
"#;

const SELECT: &str = r#"
    SELECT organization_id, user_id, role, status, created_at, updated_at
    FROM members
    WHERE user_id = $1 AND organization_id = $2
"#;

const SELECT_BY_USER_ID: &str = r#"
    SELECT organization_id, user_id, role, status, created_at, updated_at
    FROM members
    WHERE user_id = $1
"#;
