use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::Duration;
use serde::Deserialize;

use crate::{cache::session, client::google, db::user, error::Error, util, App, Result};

pub fn router() -> Router<Arc<App>> {
    Router::new()
        .route("/google", get(google_oauth))
        .route("/google/complete", get(confirm_google_oauth))
}

async fn google_oauth(
    State(app): State<Arc<App>>,
    mut jar: CookieJar,
) -> Result<impl IntoResponse> {
    let session_id = util::random_string();
    let oauth_state = util::random_string();

    if jar.get("sid").is_some() {
        jar = jar.remove(Cookie::from("sid"));
    }

    app.cache
        .set_oauth_session(&session_id, &oauth_state)
        .await?;

    let auth_url = app.google_client.get_oauth_url(&google::OAuthRequest {
        state: oauth_state,
        redirect_uri: format!("{}/login/google/complete", &app.config.domain),
        scope: "openid email profile".to_owned(),
        access_type: "offline".to_owned(),
        login_hint: "john@mycompany.com".to_owned(),
        response_type: "code".to_owned(),
    });

    let cookie = Cookie::build(("osid", session_id))
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
    if query.error.is_some() {
        return Err(Error::Unauthorized);
    }

    let session_id = jar
        .get("osid")
        .ok_or(Error::Unauthorized)?
        .value()
        .to_owned();

    let oauth_state = app
        .cache
        .get_oauth_session(session_id.to_owned())
        .await?
        .ok_or(Error::Unauthorized)?;

    app.cache.delete_oauth_session(&session_id).await?;

    if oauth_state != query.state {
        return Err(Error::Unauthorized);
    }

    let code = query.code.ok_or(Error::Unauthorized)?;

    let access_token = app
        .google_client
        .get_access_token(
            code,
            format!("{}/login/google/complete", &app.config.domain),
        )
        .await
        .inspect_err(|e| {
            eprintln!("Error getting access token: {:?}", e);
        })
        .map_err(|_| Error::Unauthorized)?;

    let user_info = app
        .google_client
        .get_user_info(access_token)
        .await
        .inspect_err(|e| {
            eprintln!("Error getting user info: {:?}", e);
        })
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
pub struct ConfirmGoogleOAuth {
    pub code: Option<String>,
    pub state: String,
    pub error: Option<String>,
}
