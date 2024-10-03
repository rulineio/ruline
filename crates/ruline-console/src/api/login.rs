use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Duration;
use serde::Deserialize;

use crate::{
    cache::session,
    client::{google, resend},
    db::user,
    error::Error,
    template, util, App, Result,
};

pub fn router() -> Router<Arc<App>> {
    Router::new()
        .route("/", post(login))
        .route("/complete", get(complete))
        .route("/google", get(google_oauth))
        .route("/google/complete", get(confirm_google_oauth))
}

async fn login(
    State(app): State<Arc<App>>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let resend_client = app.resend_client.as_ref().ok_or(Error::Unauthorized)?;

    if jar.get("sid").is_some() {
        return Err(Error::Unauthorized);
    }

    let pre_sess_id = BASE64_STANDARD.encode(body.email.as_bytes());

    if app.cache.get_pre_session(&pre_sess_id).await?.is_some() {
        return Ok((jar, StatusCode::ACCEPTED));
    }

    if app.db.get_user_by_email(&body.email).await?.is_none() {
        return Ok((jar, StatusCode::ACCEPTED));
    }

    let state = util::random_string();
    let code = format!("{}.{}", pre_sess_id, state);

    app.cache
        .set_pre_session(
            &pre_sess_id,
            &session::PreSession {
                state,
                user: Some(
                    user::User::builder()
                        .email(body.email.to_owned())
                        .name(String::default())
                        .build(),
                ),
            },
        )
        .await?;

    let template = template::Template::Login(template::LoginTemplate {
        url: format!("{}/login/complete?code={}", &app.config.domain, &code),
    });

    let _ = resend_client
        .send_email(&resend::SendEmailRequest {
            from: "Ruline <hello@ruline.io>".to_owned(),
            to: resend::SendEmailRecipient::Single(body.email),
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
        .get_pre_session(pre_sess_id)
        .await?
        .ok_or(Error::Unauthorized)?;

    if pre_sess.state != state {
        return Err(Error::Unauthorized);
    }

    app.cache.delete_pre_session(pre_sess_id).await?;

    let user = pre_sess.user.ok_or(Error::Unauthorized)?;

    let user = match app.db.get_user_by_email(&user.email).await? {
        Some(user) => {
            app.db.set_last_login(&user.id).await?;
            user
        }
        None => {
            let user = user::User::builder()
                .email(user.email)
                .name(user.name)
                .build();

            app.db.store_user(&user).await?
        }
    };

    let sess_id = util::random_string();
    let sess = session::Session::builder().user(user).build();

    app.cache.set_session(&sess_id, &sess).await?;

    let cookie = Cookie::build(("sid", sess_id))
        .same_site(SameSite::Lax)
        .path("/")
        .secure(!app.config.is_dev())
        .http_only(true)
        .max_age(Duration::days(1).to_std().unwrap().try_into().unwrap())
        .build();

    Ok((jar.add(cookie), Redirect::to("/ui")))
}

async fn google_oauth(State(app): State<Arc<App>>, jar: CookieJar) -> Result<impl IntoResponse> {
    let google_client = app.google_client.as_ref().ok_or(Error::Unauthorized)?;

    let pre_sess_id = util::random_string();
    let state = util::random_string();

    if jar.get("sid").is_some() {
        return Err(Error::Unauthorized);
    }

    app.cache
        .set_pre_session(
            &pre_sess_id,
            &session::PreSession {
                state: state.to_owned(),
                user: None,
            },
        )
        .await?;

    let auth_url = google_client.get_oauth_url(&google::OAuthRequest {
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

async fn confirm_google_oauth(
    State(app): State<Arc<App>>,
    Query(query): Query<ConfirmGoogleOAuth>,
    jar: CookieJar,
) -> Result<impl IntoResponse> {
    let google_client = app.google_client.as_ref().ok_or(Error::Unauthorized)?;

    if query.error.is_some() {
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

    let oauth_state = app
        .cache
        .get_pre_session(&pre_sess_id)
        .await?
        .ok_or(Error::Unauthorized)?
        .state;

    app.cache.delete_pre_session(&pre_sess_id).await?;

    if oauth_state != query.state {
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

    let user = match app.db.get_user_by_email(&user_info.email).await? {
        Some(user) => {
            app.db.set_last_login(&user.id).await?;
            user
        }
        None => {
            let user = user::User::builder()
                .email(user_info.email)
                .name(user_info.name)
                .avatar(user_info.picture)
                .build();

            app.db.store_user(&user).await?
        }
    };

    let sess_id = util::random_string();
    let sess = session::Session::builder().user(user).build();

    app.cache.set_session(&sess_id, &sess).await?;

    let cookie = Cookie::build(("sid", sess_id))
        .same_site(SameSite::Lax)
        .path("/")
        .secure(!app.config.is_dev())
        .http_only(true)
        .max_age(Duration::days(1).to_std().unwrap().try_into().unwrap())
        .build();

    Ok((jar.add(cookie), Redirect::to("/ui")))
}

#[derive(Deserialize)]
pub struct LoginRequest {
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
