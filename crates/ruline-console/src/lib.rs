use std::sync::Arc;

use cache::Cache;
use client::google;
use db::Database;
use error::Error;
use serde::Deserialize;

pub mod api;
pub mod cache;
pub mod client;
pub mod db;
pub mod error;
pub mod util;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub domain: String,
    pub cache_url: String,
    pub database_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
}

impl Config {
    pub fn is_dev(&self) -> bool {
        self.domain.contains("localhost")
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct App {
    pub config: Config,
    pub cache: Arc<Cache>,
    pub db: Arc<Database>,
    pub google_client: Arc<google::Client>,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        let cache = Cache::new(config.cache_url.to_owned())
            .await
            .map(Arc::new)?;
        let db = Database::new(config.database_url.to_owned())
            .await
            .map(Arc::new)?;
        let google_client = google::Client::new(
            config.google_client_id.to_owned(),
            config.google_client_secret.to_owned(),
        );

        Ok(Self {
            config,
            cache,
            db,
            google_client: Arc::new(google_client),
        })
    }
}
