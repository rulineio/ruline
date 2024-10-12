use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    client::email::{SendEmailRecipient, SendEmailRequest},
    domain::{session::Session, user::User},
    error::Error,
    template::{LoginTemplate, Template},
    util, App, Result,
};

pub fn router() -> Router<Arc<App>> {
    Router::new().route("/", post(signup))
}

async fn signup(
    State(app): State<Arc<App>>,
    jar: CookieJar,
    Json(body): Json<SignupRequest>,
) -> Result<impl IntoResponse> {
    let email_client = app.email_client.as_ref().ok_or(Error::Unauthorized)?;

    if jar.get("sid").is_some() {
        return Err(Error::Unauthorized);
    }

    let pre_sess_id = hex::encode(&body.email);

    if app.cache.get_session(&pre_sess_id).await?.is_some() {
        return Ok(StatusCode::ACCEPTED);
    }

    let state = util::random_string();
    let code = format!("{}.{}", pre_sess_id, state);

    let user = User::builder()
        .email(body.email.to_owned())
        .name(body.name.clone())
        .build();

    app.cache
        .set_session(
            &pre_sess_id,
            &Session::Unauthenticated { state, user }.into(),
        )
        .await?;

    let template = Template::Login(LoginTemplate {
        url: format!("{}/login/complete?code={}", &app.config.domain, &code),
    });

    email_client
        .send_email(&SendEmailRequest {
            to: SendEmailRecipient::Single(body.email),
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
