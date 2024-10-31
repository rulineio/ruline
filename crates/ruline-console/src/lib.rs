use std::sync::Arc;

use anyhow::anyhow;
use cache::Cache;
use client::{email, google};
use db::Database;
use error::Error;
use serde::Deserialize;
use util::ResultExt;

pub mod api;
pub mod cache;
pub mod client;
pub mod db;
pub mod domain;
pub mod error;
pub mod template;
pub mod util;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub domain: String,
    pub cache_url: String,
    pub database_url: String,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub resend_api_key: Option<String>,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_user: Option<String>,
    pub smtp_password: Option<String>,
    pub email_from: Option<String>,
    pub otel_service_name: Option<String>,
    pub otel_agent_endpoint: Option<String>,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let config = envy::from_env::<Self>()?;

        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.google_client_id.is_some() != self.google_client_secret.is_some() {
            return Err(
                anyhow!("google_client_id and google_client_secret must both be set").into(),
            );
        }

        if self.smtp_host.is_some() {
            if self.smtp_port.is_none() {
                return Err(anyhow!("smtp_port must be set if smtp_host is set").into());
            }

            if self.smtp_user.is_none() {
                return Err(anyhow!("smtp_user must be set if smtp_host is set").into());
            }

            if self.smtp_password.is_none() {
                return Err(anyhow!("smtp_password must be set if smtp_host is set").into());
            }

            if self.email_from.is_none() {
                return Err(anyhow!("email_from must be set if smtp_host is set").into());
            }
        }

        if self.resend_api_key.is_some() && self.email_from.is_none() {
            return Err(anyhow!("email_from must be set if resend_api_key is set").into());
        }

        if self.google_client_id.is_none() && self.email_from.is_none() {
            return Err(anyhow!("Either google oauth or magic link must be enabled").into());
        }

        Ok(())
    }

    pub fn is_dev(&self) -> bool {
        self.domain.contains("localhost")
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct App {
    pub config: Config,
    pub cache: Arc<Cache>,
    pub db: Arc<Database>,
    pub template_client: Arc<template::TemplateClient>,
    pub google_client: Option<Arc<google::Client>>,
    pub email_client: Option<Arc<email::Client>>,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        let cache = Cache::new(config.cache_url.to_owned())
            .await
            .map(Arc::new)
            .log_error("Error initializing cache")?;
        let db = Database::new(config.database_url.to_owned())
            .await
            .map(Arc::new)
            .log_error("Error initializing db")?;
        let template_client = template::TemplateClient::new()
            .map(Arc::new)
            .log_error("Error initializing template client")?;
        let google_client = match (
            config.google_client_id.as_ref(),
            config.google_client_secret.as_ref(),
        ) {
            (Some(client_id), Some(client_secret)) => Some(google::Client::new(
                client_id.to_owned(),
                client_secret.to_owned(),
            )?),
            _ => None,
        };
        let email_client = match config.email_from.as_ref() {
            Some(email_from) => Some(email::Client::new(email_from.to_owned(), &config)?),
            None => None,
        };

        Ok(Self {
            config,
            cache,
            db,
            template_client,
            google_client: google_client.map(Arc::new),
            email_client: email_client.map(Arc::new),
        })
    }
}
