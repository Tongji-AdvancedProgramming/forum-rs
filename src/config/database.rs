use crate::config::get_config;
use crate::panic;
use async_trait::async_trait;
use log::error;
use serde::Deserialize;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{Database, Error, MySql, Pool};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

#[derive(Default, Debug, Deserialize, Eq, PartialEq)]
pub struct DatabaseConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

pub struct Db {
    pool: Pool<MySql>,
}

impl Debug for Db {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Db").finish()
    }
}

#[async_trait]
pub trait DatabaseTrait<T>
where
    T: Database,
{
    async fn init() -> Result<Self, Error>
    where
        Self: Sized;
    fn get_pool(&self) -> &Pool<T>;
}

#[async_trait]
impl DatabaseTrait<MySql> for Db {
    async fn init() -> Result<Self, Error> {
        let url: String;
        let username: String;
        let password: String;
        {
            let config = get_config();
            let guard = config.read().unwrap();
            url = guard.database.url.clone();
            username = guard.database.username.clone();
            password = guard.database.password.clone();
        }

        let options = MySqlConnectOptions::from_str(&url)
            .unwrap_or_else(|e| {
                error!(
                    "\n[MySQL Url Parsing Failed]\n解析MySQL连接URL失败，请检查格式是否正确\n\n{}",
                    e
                );
                panic()
            })
            .username(&username)
            .password(&password);

        let pool = MySqlPoolOptions::new()
            .max_connections(16)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    fn get_pool(&self) -> &Pool<MySql> {
        &self.pool
    }
}
