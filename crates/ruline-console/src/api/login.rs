use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::Duration;
use serde::Deserialize;
use tracing::warn;

use crate::{
    client::{
        email::{SendEmailRecipient, SendEmailRequest},
        google::OAuthRequest,
    },
    domain::{
        session::Session,
        user::{User, UserStatus},
    },
    error::Error,
    template::{LoginTemplate, Template},
    util::{self, ResultExt},
    App, Result,
};

pub fn router() -> Router<Arc<App>> {
    Router::new()
        .route("/", post(login))
        .route("/complete", get(complete))
        .route("/google", get(google_oauth))
        .route("/google/complete", get(google_oauth_complete))
}

async fn login(
    State(app): State<Arc<App>>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let email_client = app.email_client.as_ref().ok_or(Error::Unauthorized)?;

    if jar.get("sid").is_some() {
        return Ok((jar, StatusCode::ACCEPTED));
    }

    let pre_sess_id = hex::encode(&body.email);

    if app.cache.get_session(&pre_sess_id).await?.is_some() {
        return Ok((jar, StatusCode::ACCEPTED));
    }

    if app.db.get_user_by_email(&body.email).await?.is_none() {
        return Ok((jar, StatusCode::ACCEPTED));
    }

    let state = util::random_string();
    let code = format!("{}.{}", pre_sess_id, state);

    let user = User::builder()
        .email(body.email.to_owned())
        .name(String::default())
        .build();

    app.cache
        .set_session_exp(
            &pre_sess_id,
            &Session::Unauthenticated { state, user }.into(),
            5 * 60 * 60,
        )
        .await?;

    let template = Template::Login(LoginTemplate {
        url: format!("{}/login/complete?code={}", &app.config.domain, &code),
    });

    email_client
        .send_email(&SendEmailRequest {
            to: SendEmailRecipient::Single(body.email),
            subject: "Login to Ruline".to_owned(),
            html: template.render_email(&app.template_client)?,
            text: template.render_text(),
        })
        .await?;

    Ok((jar, StatusCode::ACCEPTED))
}

async fn complete(
    State(app): State<Arc<App>>,
    Query(query): Query<LoginConfirmQuery>,
    jar: CookieJar,
) -> Result<impl IntoResponse> {
    if jar.get("sid").is_some() {
        return Ok((jar, Redirect::to("/ui")));
    }

    let (pre_sess_id, state) = query.code.split_once('.').ok_or(Error::Unauthorized)?;

    let pre_sess = app
        .cache
        .get_session(pre_sess_id)
        .await?
        .ok_or(Error::Unauthorized)?;

    let (pre_sess_state, user) = match pre_sess.into() {
        Session::Unauthenticated { state, user } => (state, user),
        _ => return Err(Error::Unauthorized),
    };

    if pre_sess_state != state {
        warn!("tried to complete login with invalid state");
        return Err(Error::Unauthorized);
    }

    app.cache.delete_session(pre_sess_id).await?;

    let new_user = User::builder().email(user.email).name(user.name).build();

    complete_auth(&app, jar, new_user).await
}

async fn google_oauth(State(app): State<Arc<App>>, jar: CookieJar) -> Result<impl IntoResponse> {
    let google_client = app.google_client.as_ref().ok_or(Error::Unauthorized)?;

    let pre_sess_id = util::random_string();
    let state = util::random_string();

    if jar.get("sid").is_some() {
        return Ok((jar, Redirect::to("/ui")));
    }

    app.cache
        .set_session(
            &pre_sess_id,
            &Session::Oauth {
                state: state.to_owned(),
            }
            .into(),
        )
        .await?;

    let auth_url = google_client.get_oauth_url(&OAuthRequest {
        state,
        redirect_uri: format!("{}/login/google/complete", &app.config.domain),
        scope: "openid email profile".to_owned(),
        access_type: "offline".to_owned(),
        login_hint: "john@mycompany.com".to_owned(),
        response_type: "code".to_owned(),
    });

    let cookie = Cookie::build(("psid", pre_sess_id))
        .same_site(SameSite::Lax)
        .path("/login/google")
        .secure(!app.config.is_dev())
        .http_only(true)
        .max_age(Duration::minutes(5).to_std().unwrap().try_into().unwrap())
        .build();

    Ok((jar.add(cookie), Redirect::to(&auth_url)))
}

async fn google_oauth_complete(
    State(app): State<Arc<App>>,
    Query(query): Query<ConfirmGoogleOAuth>,
    jar: CookieJar,
) -> Result<impl IntoResponse> {
    let google_client = app.google_client.as_ref().ok_or(Error::Unauthorized)?;

    if query.error.is_some() {
        warn!({ error = ?query.error }, "Google OAuth error");
        return Err(Error::Unauthorized);
    }

    if jar.get("sid").is_some() {
        return Ok((jar, Redirect::to("/ui")));
    }

    let pre_sess_id = jar
        .get("psid")
        .ok_or(Error::Unauthorized)?
        .value()
        .to_owned();

    let pre_sess = app
        .cache
        .get_session(&pre_sess_id)
        .await?
        .ok_or(Error::Unauthorized)?;

    let oauth_state = match pre_sess.into() {
        Session::Oauth { state } => state,
        _ => return Err(Error::Unauthorized),
    };

    app.cache
        .delete_session(&pre_sess_id)
        .await
        .log_error("error deleting session")?;

    if oauth_state != query.state {
        warn!("tried to complete Google OAuth with invalid state");
        return Err(Error::Unauthorized);
    }

    let code = query.code.ok_or(Error::Unauthorized)?;

    let access_token = google_client
        .get_access_token(
            code,
            format!("{}/login/google/complete", &app.config.domain),
        )
        .await
        .map_err(|_| Error::Unauthorized)?;

    let user_info = google_client
        .get_user_info(access_token)
        .await
        .map_err(|_| Error::Unauthorized)?;

    let new_user = User::builder()
        .email(user_info.email)
        .name(user_info.name)
        .avatar(user_info.picture)
        .build();

    complete_auth(&app, jar, new_user).await
}

async fn complete_auth(
    app: &Arc<App>,
    jar: CookieJar,
    new_user: User,
) -> Result<(CookieJar, Redirect)> {
    let user: User = match app.db.get_user_by_email(&new_user.email).await? {
        Some(user) => {
            let (avatar, name) = match (user.avatar != new_user.avatar, user.name != new_user.name)
            {
                (true, true) => (Some(new_user.avatar), Some(new_user.name)),
                (true, false) => (Some(new_user.avatar), None),
                (false, true) => (None, Some(new_user.name)),
                (false, false) => (None, None),
            };

            let mut trx = app.db.begin().await?;
            app.db
                .update_user(&user.id, avatar.as_deref(), name.as_deref(), &mut trx)
                .await?;
            app.db.set_user_last_login(&user.id, &mut trx).await?;
            app.db.commit(trx).await?;
            user
        }
        None => app.db.store_user(&new_user).await?,
    };

    let sess = match user.status {
        UserStatus::Created => Session::builder().user(user.to_owned()).build(),
        UserStatus::Active => {
            let members = app.db.get_members_by_user_id(&user.id).await?;

            match members.is_empty() {
                true => Session::builder().user(user.to_owned()).build(),
                false => {
                    let member = members.into_iter().next().unwrap();

                    let organization = app.db.get_organization(&member.organization_id).await?;

                    Session::builder()
                        .user(user.to_owned())
                        .organization(organization)
                        .member(member)
                        .build()
                }
            }
        }
    };

    let sess_id = util::random_string();

    app.cache.set_session(&sess_id, &sess.into()).await?;

    let cookie = Cookie::build(("sid", sess_id))
        .same_site(SameSite::Lax)
        .path("/")
        .secure(!app.config.is_dev())
        .http_only(true)
        .max_age(Duration::weeks(1).to_std().unwrap().try_into().unwrap())
        .build();

    Ok((jar.add(cookie), Redirect::to("/ui")))
}

#[derive(Deserialize)]
struct LoginRequest {
    pub email: String,
}

#[derive(Deserialize)]
struct LoginConfirmQuery {
    pub code: String,
}

#[derive(Deserialize)]
struct ConfirmGoogleOAuth {
    pub code: Option<String>,
    pub state: String,
    pub error: Option<String>,
}
