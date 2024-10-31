use tracing::instrument;

use crate::domain::invitation::{Invitation, InvitationStatus};

use super::*;

impl Database {
    #[instrument(
        skip_all,
        fields(
            invitation.id = %invitation.id,
            otel.name = "INSERT invitations",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "invitations",
            db.namespace = "ruline",
            db.operation.name = "INSERT",
            db.query.text = INSERT.trim()
        ),
        err
    )]
    pub async fn store_invitation(
        &self,
        invitation: &Invitation,
        trx: &mut Transaction<'_, MySql>,
    ) -> Result<()> {
        _ = sqlx::query(INSERT)
            .bind(&invitation.id)
            .bind(&invitation.organization_id)
            .bind(&invitation.user_id)
            .bind(&invitation.member_id)
            .bind(invitation.status.to_string())
            .execute(&mut **trx)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(())
    }

    #[instrument(
        skip_all,
        fields(
            invitation.id = %id,
            otel.name = "SELECT invitations",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "invitations",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT.trim()
        )
    )]
    pub async fn get_invitation(&self, id: &str) -> Result<Option<Invitation>> {
        let invitation: Option<invitation::Invitation> = sqlx::query_as(SELECT)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(invitation.map(Into::into))
    }

    #[instrument(
        skip_all,
        fields(
            user.id = %user_id,
            invitation.status = %status,
            otel.name = "SELECT invitations",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "invitations",
            db.namespace = "ruline",
            db.operation.name = "SELECT",
            db.query.text = SELECT_BY_USER_ID_STATUS.trim()
        )
    )]
    pub async fn get_invitations_by_user_id_status(
        &self,
        user_id: &str,
        status: InvitationStatus,
    ) -> Result<Vec<(String, String)>> {
        let invitations: Vec<(String, String)> = sqlx::query_as(SELECT_BY_USER_ID_STATUS)
            .bind(user_id)
            .bind(status.to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(DatabaseError::Sqlx)?;

        Ok(invitations)
    }

    #[instrument(skip_all,
        fields(
            invitation.id = %id,
            invitation.status = %status,
            otel.name = "UPDATE invitation",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.collection.name = "invitations",
            db.namespace = "ruline",
            db.operation.name = "UPDATE",
            db.query.text = SET_STATUS.trim()
        )
    )]
    pub async fn set_invitation_status(
        &self,
        id: &str,
        status: InvitationStatus,
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
}

const INSERT: &str = r#"
    INSERT INTO invitations (id, organization_id, user_id, member_id, status)
    VALUES (?, ?, ?, ?, ?)
"#;

const SELECT: &str = r#"
    SELECT id, organization_id, user_id, member_id, status, created_at, updated_at
    FROM invitations
    WHERE id = ?
"#;

const SELECT_BY_USER_ID_STATUS: &str = r#"
    SELECT i.id, o.name
    FROM invitations i
    INNER JOIN organizations o ON i.organization_id = o.id
    WHERE i.user_id = ? AND i.status = ?
"#;

const SET_STATUS: &str = r#"
    UPDATE invitations
    SET status = ?
    WHERE id = ?
"#;
