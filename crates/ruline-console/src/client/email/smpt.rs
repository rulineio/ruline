use std::sync::Arc;

use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::{client::error::ClientError, Result};

use super::*;

pub struct Client {
    mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>,
}

impl Client {
    pub fn new(
        smtp_host: String,
        smtp_port: u16,
        smtp_user: String,
        smtp_password: String,
    ) -> Result<Self> {
        let creds = Credentials::new(smtp_user, smtp_password);

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .map_err(ClientError::LettreSmtp)?
            .port(smtp_port)
            .credentials(creds)
            .build();

        Ok(Self {
            mailer: Arc::new(mailer),
        })
    }

    pub async fn send_email(&self, email_from: &str, request: &SendEmailRequest) -> Result<()> {
        let mut msg_builder = Message::builder()
            .from(
                email_from
                    .parse()
                    .map_err(|_| ClientError::InvalidEmail(email_from.to_owned()))?,
            )
            .reply_to(
                email_from
                    .parse()
                    .map_err(|_| ClientError::InvalidEmail(email_from.to_owned()))?,
            )
            .subject(&request.subject);

        match &request.to {
            SendEmailRecipient::Single(to) => {
                msg_builder = msg_builder.to(to
                    .parse()
                    .map_err(|_| ClientError::InvalidEmail(email_from.to_owned()))?);
            }
            SendEmailRecipient::Multiple(to) => {
                for recipient in to {
                    msg_builder = msg_builder.to(recipient
                        .parse()
                        .map_err(|_| ClientError::InvalidEmail(email_from.to_owned()))?);
                }
            }
        }

        let msg = msg_builder
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(request.text.to_owned()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(request.html.to_owned()),
                    ),
            )
            .map_err(ClientError::LettreEmail)?;

        let _ = self
            .mailer
            .send(msg)
            .await
            .map_err(ClientError::LettreSmtp)?;

        Ok(())
    }
}
