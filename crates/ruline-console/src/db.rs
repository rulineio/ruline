use error::DatabaseError;

use crate::Result;

pub mod error;
mod model;
mod query;

pub use model::*;

pub struct Database {
    pool: sqlx::MySqlPool,
}

impl Database {
    pub async fn new(url: String) -> Result<Self> {
        Ok(Self {
            pool: sqlx::MySqlPool::connect(&url)
                .await
                .map_err(DatabaseError::Sqlx)?,
        })
    }
}
