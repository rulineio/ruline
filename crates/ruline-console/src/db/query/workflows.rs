use serde_json::Value;
use tracing::instrument;

use crate::domain::workflow::{Workflow, WorkflowStatus, WorkflowVersion, WorkflowVersionStatus};

use super::*;

impl Database {
    #[instrument(
        skip_all,
        fields(
            workflow.id = %workflow.id,
            otel.name = "INSERT workflows",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflows",
            db.namespace = "ruline",
            db.operation.name = "INSERT",
            db.query.text = INSERT_WORKFLOW.trim()
        )
    )]
    pub async fn insert_workflow(
        &self,
        workflow: &Workflow,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        sqlx::query(INSERT_WORKFLOW)
            .bind(&workflow.id)
            .bind(&workflow.organization_id)
            .bind(&workflow.project_id)
            .bind(&workflow.name)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            organization.id = %organization_id,
            project.id = %project_id,
            workflow.id = %workflow_id,
            otel.name = "SELECT workflows",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflows",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT_WORKFLOW.trim()
        )
    )]
    pub async fn get_workflow(
        &self,
        organization_id: &str,
        project_id: &str,
        workflow_id: &str,
    ) -> Result<Option<Workflow>> {
        let workflow: Option<workflow::Workflow> = sqlx::query_as(SELECT_WORKFLOW)
            .bind(organization_id)
            .bind(project_id)
            .bind(workflow_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(workflow.map(Into::into))
    }

    #[instrument(
        skip_all,
        fields(
            organization.id = %organization_id,
            project.id = %project_id,
            otel.name = "SELECT workflows",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflows",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT_WORKFLOWS_BY_PROJECT_ID.trim()
        )
    )]
    pub async fn get_workflows_by_project_id(
        &self,
        organization_id: &str,
        project_id: &str,
    ) -> Result<Vec<Workflow>> {
        let workflows: Vec<workflow::Workflow> = sqlx::query_as(SELECT_WORKFLOWS_BY_PROJECT_ID)
            .bind(organization_id)
            .bind(project_id)
            .fetch_all(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(workflows.into_iter().map(Into::into).collect())
    }

    #[instrument(
        skip_all,
        fields(
            organization.id = %organization_id,
            project.id = %project_id,
            workflow.id = %workflow_id,
            workflow.version = %version,
            otel.name = "UPDATE workflows",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflows",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_WORKFLOW_ACTIVE_VERSION.trim()
        )
    )]
    pub async fn set_workflow_active_version(
        &self,
        organization_id: &str,
        project_id: &str,
        workflow_id: &str,
        version: u32,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        sqlx::query(SET_WORKFLOW_ACTIVE_VERSION)
            .bind(version)
            .bind(organization_id)
            .bind(project_id)
            .bind(workflow_id)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            organization.id = %organization_id,
            project.id = %project_id,
            workflow.id = %workflow_id,
            workflow.status = %status,
            otel.name = "UPDATE workflows",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflows",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_WORKFLOW_STATUS.trim()
        )
    )]
    pub async fn set_workflow_status(
        &self,
        organization_id: &str,
        project_id: &str,
        workflow_id: &str,
        status: WorkflowStatus,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        sqlx::query(SET_WORKFLOW_STATUS)
            .bind(status.to_string())
            .bind(organization_id)
            .bind(project_id)
            .bind(workflow_id)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            organization.id = %organization_id,
            project.id = %project_id,
            workflow.id = %workflow_id,
            workflow.name = %name,
            otel.name = "UPDATE workflows",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflows",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_WORKFLOW_NAME.trim()
        )
    )]
    pub async fn set_workflow_name(
        &self,
        organization_id: &str,
        project_id: &str,
        workflow_id: &str,
        name: &str,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        sqlx::query(SET_WORKFLOW_NAME)
            .bind(name)
            .bind(organization_id)
            .bind(project_id)
            .bind(workflow_id)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            workflow.id = %version.workflow_id,
            otel.name = "INSERT workflow_versions",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflow_versions",
            db.namespace = "ruline",
            db.operation.name = "INSERT",
            db.query.text = INSERT_WORKFLOW_VERSION.trim()
        )
    )]
    pub async fn insert_workflow_version(
        &self,
        version: &WorkflowVersion,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        sqlx::query(INSERT_WORKFLOW_VERSION)
            .bind(&version.organization_id)
            .bind(&version.project_id)
            .bind(&version.workflow_id)
            .bind(version.version)
            .bind(&version.definition)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            organization.id = %organization_id,
            project.id = %project_id,
            workflow.id = %workflow_id,
            workflow.version = %version,
            otel.name = "SELECT workflow_versions",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflow_versions",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT_WORKFLOW_VERSION.trim()
        )
    )]
    pub async fn get_workflow_version(
        &self,
        organization_id: &str,
        project_id: &str,
        workflow_id: &str,
        version: u32,
    ) -> Result<Option<WorkflowVersion>> {
        let version: Option<workflow::WorkflowVersion> = sqlx::query_as(SELECT_WORKFLOW_VERSION)
            .bind(organization_id)
            .bind(project_id)
            .bind(workflow_id)
            .bind(version)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(version.map(Into::into))
    }

    #[instrument(
        skip_all,
        fields(
            organization.id = %organization_id,
            project.id = %project_id,
            workflow.id = %workflow_id,
            otel.name = "SELECT workflow_versions",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflow_versions",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT_WORKFLOW_VERSIONS_BY_WORKFLOW_ID.trim()
        )
    )]
    pub async fn get_workflow_versions_by_workflow_id(
        &self,
        organization_id: &str,
        project_id: &str,
        workflow_id: &str,
    ) -> Result<Vec<WorkflowVersion>> {
        let versions: Vec<workflow::WorkflowVersion> =
            sqlx::query_as(SELECT_WORKFLOW_VERSIONS_BY_WORKFLOW_ID)
                .bind(organization_id)
                .bind(project_id)
                .bind(workflow_id)
                .fetch_all(&self.pool)
                .await
                .map_err(DatabaseError::Sqlx)?;

        Ok(versions.into_iter().map(Into::into).collect())
    }

    #[instrument(
        skip_all,
        fields(
            organization.id = %organization_id,
            project.id = %project_id,
            workflow.id = %workflow_id,
            workflow.version = %version,
            workflow.status = %status,
            otel.name = "UPDATE workflow_versions",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflow_versions",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_WORKFLOW_VERSION_STATUS.trim()
        )
    )]
    pub async fn set_workflow_version_status(
        &self,
        organization_id: &str,
        project_id: &str,
        workflow_id: &str,
        version: u32,
        status: WorkflowVersionStatus,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        sqlx::query(SET_WORKFLOW_VERSION_STATUS)
            .bind(status.to_string())
            .bind(organization_id)
            .bind(project_id)
            .bind(workflow_id)
            .bind(version)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            organization.id = %organization_id,
            project.id = %project_id,
            workflow.id = %workflow_id,
            workflow.version = %version,
            otel.name = "UPDATE workflow_versions",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "workflow_versions",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_WORKFLOW_VERSION_DEFINITION.trim()
        )
    )]
    pub async fn set_workflow_version_definition(
        &self,
        organization_id: &str,
        project_id: &str,
        workflow_id: &str,
        version: u32,
        definition: Value,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        sqlx::query(SET_WORKFLOW_VERSION_DEFINITION)
            .bind(definition)
            .bind(organization_id)
            .bind(project_id)
            .bind(workflow_id)
            .bind(version)
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }
}

const INSERT_WORKFLOW: &str = r#"
    INSERT INTO workflows (id, organization_id, project_id, name)
    VALUES (?, ?, ?, ?)
"#;

const SELECT_WORKFLOW: &str = r#"
    SELECT id, organization_id, project_id, name, status, active_version, created_at, updated_at
    FROM workflows
    WHERE organization_id = ? AND project_id = ? AND id = ?
"#;

const SELECT_WORKFLOWS_BY_PROJECT_ID: &str = r#"
    SELECT id, organization_id, project_id, name, status, active_version, created_at, updated_at
    FROM workflows
    WHERE organization_id = ? AND project_id = ?
    ORDER BY id DESC
"#;

const SET_WORKFLOW_ACTIVE_VERSION: &str = r#"
    UPDATE workflows
    SET active_version = ?
    WHERE organization_id = ? AND project_id = ? AND id = ?
"#;

const SET_WORKFLOW_STATUS: &str = r#"
    UPDATE workflows
    SET status = ?
    WHERE organization_id = ? AND project_id = ? AND id = ?
"#;

const SET_WORKFLOW_NAME: &str = r#"
    UPDATE workflows
    SET name = ?
    WHERE organization_id = ? AND project_id = ? AND id = ?
"#;

const INSERT_WORKFLOW_VERSION: &str = r#"
    INSERT INTO workflow_versions (organization_id, project_id, workflow_id, version, definition)
    VALUES (?, ?, ?, ?, ?)
"#;

const SELECT_WORKFLOW_VERSION: &str = r#"
    SELECT organization_id, project_id, workflow_id, version, status, definition, created_at, updated_at
    FROM workflow_versions
    WHERE organization_id = ? AND project_id = ? AND workflow_id = ? AND version = ?
"#;

const SELECT_WORKFLOW_VERSIONS_BY_WORKFLOW_ID: &str = r#"
    SELECT organization_id, project_id, workflow_id, version, status, definition, created_at, updated_at
    FROM workflow_versions
    WHERE organization_id = ? AND project_id = ? AND workflow_id = ?
    ORDER BY version DESC
"#;

const SET_WORKFLOW_VERSION_STATUS: &str = r#"
    UPDATE workflow_versions
    SET status = ?
    WHERE organization_id = ? AND project_id = ? AND workflow_id = ? AND version = ?
"#;

const SET_WORKFLOW_VERSION_DEFINITION: &str = r#"
    UPDATE workflow_versions
    SET definition = ?
    WHERE organization_id = ? AND project_id = ? AND workflow_id = ? AND version = ?
"#;
