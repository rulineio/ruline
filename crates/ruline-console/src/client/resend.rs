use crate::Result;
use serde::{Deserialize, Serialize};

use super::error::ClientError;

pub struct Client {
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn send_email(&self, request: &SendEmailRequest) -> Result<String> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(ClientError::Reqwest)?;

        if !response.status().is_success() {
            return Err(ClientError::UnexpectedStatus(response.status()).into());
        }

        let response: SendEmailResponse = response.json().await.map_err(ClientError::Reqwest)?;
        Ok(response.id)
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum SendEmailRecipient {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize)]
pub struct SendEmailRequest {
    pub from: String,
    pub to: SendEmailRecipient,
    pub subject: String,
    pub html: String,
    pub text: String,
}

#[derive(Deserialize)]
pub struct SendEmailResponse {
    pub id: String,
}
