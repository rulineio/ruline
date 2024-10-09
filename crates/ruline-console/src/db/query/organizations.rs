use crate::domain::organization::Organization;

use super::*;

impl Database {
    pub async fn store_organization(&self, organization: &Organization) -> Result<Organization> {
        _ = sqlx::query(INSERT)
            .bind(&organization.id)
            .bind(&organization.name)
            .bind(&organization.avatar)
            .execute(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        self.get_organization(&organization.id).await
    }

    pub async fn get_organization(&self, id: &str) -> Result<Organization> {
        let organization: organization::Organization = sqlx::query_as(SELECT)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(organization.into())
    }
}

const INSERT: &str = r#"
    INSERT INTO organizations (id, name, avatar)
    VALUES (?, ?, ?)
"#;

const SELECT: &str = r#"
    SELECT id, name, status, avatar, created_at, updated_at
    FROM organizations
    WHERE id = ?
"#;
