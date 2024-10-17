use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::status::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        invitation::{Invitation, InvitationStatus},
        member::{Member, MemberRole, MemberStatus},
        session::Session,
        user::{User, UserStatus},
    },
    error::Error,
    App, Result,
};

pub fn router() -> Router<Arc<App>> {
    Router::new()
        .route("/", post(create_invitation))
        .route("/", get(get_invitations))
        .route("/{invitation_id}/accept", post(accept_invitation))
        .route("/{invitation_id}/decline", post(decline_invitation))
}

async fn create_invitation(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Json(body): Json<CreateInvitationRequest>,
) -> Result<impl IntoResponse> {
    let (role, organization_id) = match session {
        Session::Member {
            member,
            organization,
            ..
        } => (member.role, organization.id),
        _ => return Err(Error::Unauthorized),
    };

    if !role.is_at_least(MemberRole::Admin) {
        return Err(Error::Unauthorized);
    }

    let user = User::builder()
        .email(body.email.to_owned())
        .name(body.name)
        .build();
    let member = Member::builder()
        .organization_id(organization_id.to_owned())
        .role(MemberRole::from(body.role))
        .user_id(user.id.to_owned())
        .status(MemberStatus::Invited)
        .build();
    let invitation = Invitation::builder()
        .organization_id(organization_id)
        .member_id(member.id.to_owned())
        .user_id(user.id.to_owned())
        .build();

    let mut trx = app.db.begin().await?;
    app.db.store_user_trx(&user, &mut trx).await?;
    app.db.store_member(&member, &mut trx).await?;
    app.db.store_invitation(&invitation, &mut trx).await?;
    app.db.commit(trx).await?;

    Ok(StatusCode::CREATED)
}

async fn get_invitations(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
) -> Result<impl IntoResponse> {
    let user_id = match session {
        Session::User { user } => user.id,
        _ => return Err(Error::Unauthorized),
    };

    let invitations = app
        .db
        .get_invitations_by_user_id_status(&user_id, InvitationStatus::Created)
        .await?;

    Ok(Json(
        invitations
            .into_iter()
            .map(|(id, organization_name)| InvitationResponse {
                id,
                organization_name,
            })
            .collect::<Vec<_>>(),
    ))
}

async fn accept_invitation(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path(invitation_id): Path<String>,
) -> Result<impl IntoResponse> {
    let user_id = match session {
        Session::User { user } => user.id,
        _ => return Err(Error::Unauthorized),
    };

    let invitation = match app.db.get_invitation(&invitation_id).await? {
        Some(invitation) => invitation,
        None => return Err(Error::BadRequest),
    };

    if invitation.user_id != user_id {
        return Err(Error::Unauthorized);
    }

    let mut trx = app.db.begin().await?;
    app.db
        .set_invitation_status(&invitation_id, InvitationStatus::Accepted, &mut trx)
        .await?;
    app.db
        .set_member_status(&invitation.member_id, MemberStatus::Active, &mut trx)
        .await?;
    app.db
        .set_user_status(&user_id, UserStatus::Active, &mut trx)
        .await?;
    app.db.commit(trx).await?;

    Ok(StatusCode::ACCEPTED)
}

async fn decline_invitation(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path(invitation_id): Path<String>,
) -> Result<impl IntoResponse> {
    let user_id = match session {
        Session::User { user } => user.id,
        _ => return Err(Error::Unauthorized),
    };

    let invitation = match app.db.get_invitation(&invitation_id).await? {
        Some(invitation) => invitation,
        None => return Err(Error::BadRequest),
    };

    if invitation.user_id != user_id {
        return Err(Error::Unauthorized);
    }

    let mut trx = app.db.begin().await?;
    app.db
        .set_invitation_status(&invitation_id, InvitationStatus::Declined, &mut trx)
        .await?;
    app.db
        .set_member_status(&invitation.member_id, MemberStatus::Declined, &mut trx)
        .await?;
    app.db.commit(trx).await?;

    Ok(StatusCode::ACCEPTED)
}

#[derive(Deserialize)]
struct CreateInvitationRequest {
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Serialize)]
struct InvitationResponse {
    pub id: String,
    pub organization_name: String,
}
