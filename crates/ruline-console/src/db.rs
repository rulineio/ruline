use error::DatabaseError;
use sqlx::{MySql, MySqlPool, Transaction};

use crate::Result;

pub mod error;
mod model;
mod query;

use model::*;

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new(url: String) -> Result<Self> {
        Ok(Self {
            pool: MySqlPool::connect(&url)
                .await
                .map_err(DatabaseError::Sqlx)?,
        })
    }

    pub async fn begin(&self) -> Result<Transaction<'_, MySql>> {
        let trx = self.pool.begin().await.map_err(DatabaseError::Sqlx)?;
        Ok(trx)
    }

    pub async fn commit(&self, trx: Transaction<'_, MySql>) -> Result<()> {
        trx.commit().await.map_err(DatabaseError::Sqlx)?;
        Ok(())
    }
}
