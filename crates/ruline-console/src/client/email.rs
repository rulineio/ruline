use std::sync::Arc;

use crate::{Config, Result};

use serde::Serialize;
use tracing::warn;

mod resend;
mod smpt;

pub struct Client {
    email_from: String,
    resend_client: Option<Arc<resend::Client>>,
    smtp_client: Option<Arc<smpt::Client>>,
}

impl Client {
    pub fn new(email_from: String, config: &Config) -> Result<Self> {
        let resend_client = match config.resend_api_key.as_ref() {
            Some(api_key) => Some(Arc::new(resend::Client::new(api_key.to_owned())?)),
            None => None,
        };

        let smtp_client = match (
            config.smtp_host.as_ref(),
            config.smtp_port,
            config.smtp_user.as_ref(),
            config.smtp_password.as_ref(),
        ) {
            (Some(host), Some(port), Some(user), Some(password)) => Some(Arc::new(
                smpt::Client::new(host.to_owned(), port, user.to_owned(), password.to_owned())?,
            )),
            _ => None,
        };

        Ok(Self {
            email_from,
            resend_client,
            smtp_client,
        })
    }

    pub async fn send_email(&self, request: &SendEmailRequest) -> Result<()> {
        if let Some(client) = &self.resend_client {
            return client.send_email(&self.email_from, request).await;
        }

        if let Some(client) = &self.smtp_client {
            return client.send_email(&self.email_from, request).await;
        }

        warn!("no email client configured");

        Ok(())
    }
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
pub enum SendEmailRecipient {
    Single(String),
    Multiple(Vec<String>),
}

pub struct SendEmailRequest {
    pub to: SendEmailRecipient,
    pub subject: String,
    pub html: String,
    pub text: String,
}
