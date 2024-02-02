use std::sync::Arc;

use crate::config::redis::{Redis, RedisTrait};
use async_trait::async_trait;
use fred::prelude::*;
use fred::serde_json;
use tower_sessions::session::{Id, Record};
use tower_sessions::SessionStore;

#[derive(Debug, Clone)]
pub struct RedisSession {
    pool: Arc<Redis>,
}

impl RedisSession {
    pub fn new(redis_pool: &Arc<Redis>) -> Self {
        Self {
            pool: Arc::clone(redis_pool),
        }
    }
}

#[async_trait]
impl SessionStore for RedisSession {
    async fn save(&self, session_record: &Record) -> tower_sessions::session_store::Result<()> {
        let json = serde_json::to_string(session_record);
        if json.is_err() {
            return Err(tower_sessions::session_store::Error::Backend(
                json.err().unwrap().to_string(),
            ));
        }
        match self
            .pool
            .get_pool()
            .set::<String, _, _>(
                session_record.id.0,
                json.unwrap(),
                Some(Expiration::EXAT(
                    session_record.expiry_date.unix_timestamp(),
                )),
                None,
                false,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(tower_sessions::session_store::Error::Backend(e.to_string())),
        }
    }

    async fn load(&self, session_id: &Id) -> tower_sessions::session_store::Result<Option<Record>> {
        match self.pool.get_pool().get::<String, _>(session_id.0).await {
            Ok(json) => match serde_json::from_str(&json) {
                Ok(record) => Ok(Some(record)),
                Err(e) => Err(tower_sessions::session_store::Error::Backend(e.to_string())),
            },
            Err(_) => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> tower_sessions::session_store::Result<()> {
        match self.pool.get_pool().del::<String, _>(session_id.0).await {
            Ok(_) => Ok(()),
            Err(e) => Err(tower_sessions::session_store::Error::Backend(e.to_string())),
        }
    }
}
