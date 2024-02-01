use crate::config::get_config;
use serde::Deserialize;
use std::time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::fred::prelude::*;
use tower_sessions_redis_store::fred::types::ConnectHandle;
use tower_sessions_redis_store::RedisStore;

#[derive(Default, Debug, Deserialize, Eq, PartialEq)]
pub struct RedisSessionConfig {
    pub url: String,
}

pub struct RedisSession {
    pub conn_handle: ConnectHandle,
    pub session_layer: SessionManagerLayer<RedisStore<RedisPool>>,
}

impl RedisSession {
    pub async fn init() -> Result<Self, RedisError> {
        let url: String;
        {
            let config = get_config();
            let guard = config.read().unwrap();
            url = guard.redis.url.clone();
        }

        let config = RedisConfig::from_url(&url)?;
        let pool = RedisPool::new(config, None, None, None, 1)?;
        let conn_handle = pool.connect();
        pool.wait_for_connect().await?;

        let session_store = RedisStore::new(pool);
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

        Ok(Self {
            conn_handle,
            session_layer,
        })
    }
}
