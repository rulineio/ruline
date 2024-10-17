use crate::domain::organization::Organization;

use super::*;

impl Database {
    pub async fn store_organization(
        &self,
        organization: &Organization,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        _ = sqlx::query(INSERT)
            .bind(&organization.id)
            .bind(&organization.name)
            .bind(&organization.logo)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    pub async fn get_organization(&self, id: &str) -> Result<Organization> {
        let organization: organization::Organization = sqlx::query_as(SELECT)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(organization.into())
    }

    pub async fn get_organization_members(
        &self,
        organization_id: &str,
    ) -> Result<Vec<organization::OrganizationMember>> {
        let organization_users = sqlx::query_as(SELECT_ORGANIZATION_MEMBERS)
            .bind(organization_id)
            .fetch_all(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(organization_users)
    }
}

const INSERT: &str = r#"
    INSERT INTO organizations (id, name, logo)
    VALUES (?, ?, ?)
"#;

const SELECT: &str = r#"
    SELECT id, name, status, logo, created_at, updated_at
    FROM organizations
    WHERE id = ?
"#;

const SELECT_ORGANIZATION_MEMBERS: &str = r#"
    SELECT u.name, u.email, u.avatar, m.role, m.status
    FROM users u
    INNER JOIN members m ON u.id = m.user_id
    WHERE m.organization_id = ?
"#;
