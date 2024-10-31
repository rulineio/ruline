use error::DatabaseError;
use sqlx::{MySql, MySqlPool, Transaction};
use tracing::instrument;

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

    #[instrument(
        skip_all,
        fields(
            otel.name = "START TRANSACTION",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.namespace = "ruline",
            db.operation.name = "STRAT",
            db.query.text = "START TRANSACTION"
        )
    )]
    pub async fn begin(&self) -> Result<Transaction<'_, MySql>> {
        let trx = self.pool.begin().await.map_err(DatabaseError::Sqlx)?;
        Ok(trx)
    }

    #[instrument(
        skip_all,
        fields(
            otel.name = "COMMIT",
            otel.kind = "CLIENT",
            db.system = "mariadb",
            db.namespace = "ruline",
            db.operation.name = "COMMIT",
            db.query.text = "COMMIT"
        )
    )]
    pub async fn commit(&self, trx: Transaction<'_, MySql>) -> Result<()> {
        trx.commit().await.map_err(DatabaseError::Sqlx)?;
        Ok(())
    }
}
