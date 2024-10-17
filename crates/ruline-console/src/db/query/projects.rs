use crate::domain::project::Project;

use super::*;

impl Database {
    pub async fn store_project(
        &self,
        project: &Project,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        _ = sqlx::query(INSERT)
            .bind(&project.id)
            .bind(&project.organization_id)
            .bind(&project.name)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    pub async fn get_project(
        &self,
        organization_id: &str,
        project_id: &str,
    ) -> Result<Option<Project>> {
        let project: Option<project::Project> = sqlx::query_as(SELECT)
            .bind(organization_id)
            .bind(project_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(project.map(Into::into))
    }

    pub async fn get_projects_by_organization_id(
        &self,
        organization_id: &str,
    ) -> Result<Vec<Project>> {
        let projects: Vec<project::Project> = sqlx::query_as(SELECT_BY_ORGANIZATION_ID)
            .bind(organization_id)
            .fetch_all(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(projects.into_iter().map(Into::into).collect())
    }
}

const INSERT: &str = r#"
    INSERT INTO projects (id, organization_id, name)
    VALUES (?, ?, ?)
"#;

const SELECT: &str = r#"
    SELECT id, organization_id, name, status, created_at, updated_at
    FROM projects
    WHERE organization_id = ? AND id = ?
"#;

const SELECT_BY_ORGANIZATION_ID: &str = r#"
    SELECT id, organization_id, name, status, created_at, updated_at
    FROM projects
    WHERE organization_id = ?
"#;
