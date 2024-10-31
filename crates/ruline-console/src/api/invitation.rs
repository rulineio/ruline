use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::status::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{
    client::email::{SendEmailRecipient, SendEmailRequest},
    domain::{
        invitation::{Invitation, InvitationStatus},
        member::{Member, MemberRole, MemberStatus},
        session::Session,
        user::{User, UserStatus},
    },
    error::Error,
    template::{InvitationTemplate, Template},
    util, App, Result,
};

pub fn router() -> Router<Arc<App>> {
    Router::new()
        .route("/:invitation_id/accept", post(accept_invitation))
        .route("/:invitation_id/decline", post(decline_invitation))
        .route("/", post(create_invitation))
        .route("/", get(get_invitations))
}

async fn create_invitation(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Json(body): Json<CreateInvitationRequest>,
) -> Result<impl IntoResponse> {
    let (role, organization_id, organization_name) = match session {
        Session::Member {
            member,
            organization,
            ..
        } => (member.role, organization.id, organization.name),
        _ => return Err(Error::Unauthorized),
    };

    if !role.is_at_least(MemberRole::Admin) {
        return Err(Error::Unauthorized);
    }

    let mut trx = app.db.begin().await?;
    let user = match app.db.get_user_by_email(&body.email).await? {
        Some(user) => user,
        None => {
            let new_user = User::builder()
                .email(body.email.to_owned())
                .name(body.name)
                .build();
            app.db.store_user_trx(&new_user, &mut trx).await?;
            info!({
                user.email = %new_user.email,
                user.id = %new_user.id
            }, "Created user");
            new_user
        }
    };
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

    app.db.store_member(&member, &mut trx).await?;
    app.db.store_invitation(&invitation, &mut trx).await?;
    app.db.commit(trx).await?;

    info!({
        invitation.id = %invitation.id,
        user.email = %user.email,
        user.id = %user.id
    },
    "Created invitation"
    );

    let template = Template::Invitation(InvitationTemplate {
        organization: organization_name,
        url: app.config.domain.to_owned(),
    });

    if let Some(email_client) = app.email_client.as_ref() {
        email_client
            .send_email(&SendEmailRequest {
                to: SendEmailRecipient::Single(body.email),
                subject: "Invitation to Ruline".to_owned(),
                html: template.render_email(&app.template_client)?,
                text: template.render_text(),
            })
            .await?;
    }

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
    jar: CookieJar,
    Path(invitation_id): Path<String>,
) -> Result<impl IntoResponse> {
    Span::current().set_attribute("invitation.id", invitation_id.to_string());

    let user_id = match session {
        Session::User { user } => user.id,
        _ => return Err(Error::Unauthorized),
    };

    let invitation = match app.db.get_invitation(&invitation_id).await? {
        Some(invitation) => invitation,
        None => return Err(Error::BadRequest),
    };

    if invitation.user_id != user_id {
        warn!("User tried to accept invitation for another user");
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

    let member = match app.db.get_member(&invitation.member_id).await? {
        Some(member) => member,
        None => return Err(Error::BadRequest),
    };
    let user = match app.db.get_user(&user_id).await? {
        Some(user) => user,
        None => return Err(Error::BadRequest),
    };
    let organization = app.db.get_organization(&member.organization_id).await?;

    app.db.commit(trx).await?;

    info!({ invitation.id = %invitation_id }, "Accepted invitation");

    let sess = Session::builder()
        .user(user)
        .organization(organization)
        .member(member)
        .build();

    let sess_id = util::random_string();
    app.cache.set_session(&sess_id, &sess.into()).await?;

    let cookie = Cookie::build(("sid", sess_id))
        .same_site(SameSite::Lax)
        .path("/")
        .secure(!app.config.is_dev())
        .http_only(true)
        .max_age(Duration::weeks(1).to_std().unwrap().try_into().unwrap())
        .build();

    Ok((jar.add(cookie), StatusCode::NO_CONTENT))
}

async fn decline_invitation(
    State(app): State<Arc<App>>,
    Extension(session): Extension<Session>,
    Path(invitation_id): Path<String>,
) -> Result<impl IntoResponse> {
    Span::current().set_attribute("invitation.id", invitation_id.to_owned());

    let user_id = match session {
        Session::User { user } => user.id,
        _ => return Err(Error::Unauthorized),
    };

    let invitation = match app.db.get_invitation(&invitation_id).await? {
        Some(invitation) => invitation,
        None => return Err(Error::BadRequest),
    };

    if invitation.user_id != user_id {
        warn!("User tried to decline invitation for another user");
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

    info!({ invitation.id = %invitation_id }, "Declined invitation");

    Ok(StatusCode::NO_CONTENT)
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
