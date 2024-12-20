use super::*;

use redis::{AsyncCommands, RedisError};
use tracing::instrument;

const SESSION_KEY: &str = "session";

impl Cache {
    pub async fn set_session(&self, id: &str, sess: &session::Session) -> Result<()> {
        self.set_session_exp(id, sess, 7 * 24 * 60 * 60).await
    }

    #[instrument(
        skip_all,
        fields(
            otel.name = "SETEX session",
            otel.kind = "CLIENT",
            db.system = "redis",
            db.operation.name = "SETEX"
        )
    )]
    pub async fn set_session_exp(&self, id: &str, sess: &session::Session, exp: u64) -> Result<()> {
        let mut con = self.client.to_owned();
        let str_sess = serde_json::to_string(&sess).map_err(CacheError::Serde)?;
        let res: Result<(), RedisError> = con.set_ex(session_key(id), str_sess, exp).await;
        Ok(res.map_err(CacheError::Redis)?)
    }

    #[instrument(
        skip_all,
        fields(
            otel.name = "GET session",
            otel.kind = "CLIENT",
            db.system = "redis",
            db.operation.name = "GET"
        )
    )]
    pub async fn get_session(&self, id: &str) -> Result<Option<session::Session>> {
        let mut con = self.client.to_owned();
        let result: Option<String> = con.get(session_key(id)).await.map_err(CacheError::Redis)?;
        Ok(result
            .map(|s| serde_json::from_str(&s).map_err(CacheError::Serde))
            .transpose()?)
    }

    #[instrument(
        skip_all,
        fields(
            otel.name = "DEL session",
            otel.kind = "CLIENT",
            db.system = "redis",
            db.operation.name = "DEL"
        )
    )]
    pub async fn delete_session(&self, id: &str) -> Result<()> {
        let mut con = self.client.to_owned();
        let res: Result<(), RedisError> = con.del(session_key(id)).await;
        Ok(res.map_err(CacheError::Redis)?)
    }
}

fn session_key(id: &str) -> String {
    format!("{}:{}", SESSION_KEY, id)
}
