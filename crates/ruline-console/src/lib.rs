use std::sync::Arc;

use cache::Cache;
use client::{google, resend};
use db::Database;
use error::Error;
use serde::Deserialize;

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
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let config = envy::from_env::<Self>()?;

        // set public environment variables
        std::env::set_var(
            "RULINE_PUBLIC_GOOGLE_AUTH_ENABLED",
            config.google_auth_enabled().to_string(),
        );

        std::env::set_var(
            "RULINE_PUBLIC_EMAIL_AUTH_ENABLED",
            config.email_auth_enabled().to_string(),
        );

        Ok(config)
    }

    pub fn is_dev(&self) -> bool {
        self.domain.contains("localhost")
    }

    pub fn google_auth_enabled(&self) -> bool {
        self.google_client_id.is_some() && self.google_client_secret.is_some()
    }

    pub fn email_auth_enabled(&self) -> bool {
        self.resend_api_key.is_some()
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct App {
    pub config: Config,
    pub cache: Arc<Cache>,
    pub db: Arc<Database>,
    pub template_client: Arc<template::TemplateClient>,
    pub google_client: Option<Arc<google::Client>>,
    pub resend_client: Option<Arc<resend::Client>>,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        let cache = Cache::new(config.cache_url.to_owned())
            .await
            .map(Arc::new)?;
        let db = Database::new(config.database_url.to_owned())
            .await
            .map(Arc::new)?;
        let email_template = template::TemplateClient::new().map(Arc::new)?;
        let google_client = match (
            config.google_client_id.as_ref(),
            config.google_client_secret.as_ref(),
        ) {
            (Some(client_id), Some(client_secret)) => Some(google::Client::new(
                client_id.to_owned(),
                client_secret.to_owned(),
            )),
            _ => None,
        };

        let resend_client = config
            .resend_api_key
            .as_ref()
            .map(|api_key| resend::Client::new(api_key.to_owned()));

        Ok(Self {
            config,
            cache,
            db,
            template_client: email_template,
            google_client: google_client.map(Arc::new),
            resend_client: resend_client.map(Arc::new),
        })
    }
}
