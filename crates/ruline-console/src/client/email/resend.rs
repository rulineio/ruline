use std::{sync::Arc, time::Duration};

use crate::{
    client::{error::ClientError, new_client},
    Result,
};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest_middleware::ClientWithMiddleware;

use super::*;

pub struct Client {
    client: Arc<ClientWithMiddleware>,
}

impl Client {
    pub fn new(api_key: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", api_key).parse().unwrap(),
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .default_headers(headers)
            .build()
            .map_err(ClientError::Reqwest)?;

        Ok(Self {
            client: Arc::new(new_client(client)),
        })
    }

    pub async fn send_email(&self, email_from: &str, request: &SendEmailRequest) -> Result<()> {
        let response = self
            .client
            .post("https://api.resend.com/emails")
            .json(&ResendSendEmailRequest {
                from: email_from.to_owned(),
                to: request.to.to_owned(),
                subject: request.subject.to_owned(),
                html: request.html.to_owned(),
                text: request.text.to_owned(),
            })
            .send()
            .await
            .map_err(ClientError::ReqwestMiddleware)?;

        if !response.status().is_success() {
            return Err(ClientError::UnexpectedStatus(response.status()).into());
        }

        Ok(())
    }
}

#[derive(Serialize)]
pub struct ResendSendEmailRequest {
    pub from: String,
    pub to: SendEmailRecipient,
    pub subject: String,
    pub html: String,
    pub text: String,
}
