pub mod error;
mod model;
mod query;

pub use model::*;

use error::CacheError;

use crate::Result;

pub struct Cache {
    client: redis::aio::MultiplexedConnection,
}

impl Cache {
    pub async fn new(url: String) -> Result<Self> {
        let client = redis::Client::open(url).map_err(CacheError::Redis)?;
        Ok(Self {
            client: client
                .get_multiplexed_async_connection()
                .await
                .map_err(CacheError::Redis)?,
        })
    }
}
