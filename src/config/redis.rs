use crate::config::get_config;
use async_trait::async_trait;
use fred::prelude::*;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize, Eq, PartialEq)]
pub struct RedisAppConfig {
    pub url: String,
}

#[derive(Debug)]
pub struct Redis {
    pool: RedisPool,
}

#[async_trait]
pub trait RedisTrait {
    async fn init() -> Result<Self, RedisError>
    where
        Self: Sized;
    fn get_pool(&self) -> &RedisPool;
}

#[async_trait]
impl RedisTrait for Redis {
    async fn init() -> Result<Self, RedisError> {
        let url: String;
        {
            let config = get_config();
            let guard = config.read().unwrap();
            url = guard.redis.url.clone();
        }

        let config = RedisConfig::from_url(&url)?;

        let pool = RedisPool::new(config, None, None, None, 6)?;

        Ok(Self { pool })
    }

    fn get_pool(&self) -> &RedisPool {
        &self.pool
    }
}
