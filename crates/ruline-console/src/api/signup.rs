use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use axum_extra::extract::CookieJar;
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::Deserialize;

use crate::{cache::session, client::resend, db::user, error::Error, template, util, App, Result};

pub fn router() -> Router<Arc<App>> {
    Router::new().route("/", post(signup))
}

async fn signup(
    State(app): State<Arc<App>>,
    jar: CookieJar,
    Json(body): Json<SignupRequest>,
) -> Result<impl IntoResponse> {
    let resend_client = app.resend_client.as_ref().ok_or(Error::Unauthorized)?;

    if jar.get("sid").is_some() {
        return Err(Error::Unauthorized);
    }

    let pre_sess_id = BASE64_STANDARD.encode(body.email.as_bytes());

    if app.cache.get_pre_session(&pre_sess_id).await?.is_some() {
        return Ok(StatusCode::ACCEPTED);
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
                        .name(body.name)
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
            subject: "Register to Ruline".to_owned(),
            html: template.render_email(&app.template_client)?,
            text: template.render_text(),
        })
        .await?;

    Ok(StatusCode::ACCEPTED)
}

#[derive(Deserialize)]
struct SignupRequest {
    pub email: String,
    pub name: String,
}
