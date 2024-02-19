use crate::config::get_config;
use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::time::Duration;

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct DatabaseConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

pub struct Db {
    db: DatabaseConnection,
}

impl Debug for Db {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Db").finish()
    }
}

#[async_trait]
pub trait DatabaseTrait {
    type Error;
    async fn init() -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn get_db(&self) -> &DatabaseConnection;
}

#[async_trait]
impl DatabaseTrait for Db {
    type Error = Box<dyn Error + Send + Sync + 'static>;

    async fn init() -> Result<Self, Self::Error> {
        let url: String;
        // let username: String;
        // let password: String;
        {
            let config = get_config();
            let guard = config.read().unwrap();
            url = guard.database.url.clone();
            // username = guard.database.username.clone();
            // password = guard.database.password.clone();
        }

        let options = ConnectOptions::new(&url)
            .max_connections(100)
            .min_connections(2)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Trace)
            .to_owned();
        // .unwrap_or_else(|e| {
        //     error!(
        //         "\n[MySQL Url Parsing Failed]\n解析MySQL连接URL失败，请检查格式是否正确\n\n{}",
        //         e
        //     );
        //     panic()
        // })
        // .username(&username)
        // .password(&password);

        // let pool = MySqlPoolOptions::new()
        //     .max_connections(16)
        //     .connect_with(options)
        //     .await?;

        let db = Database::connect(options).await?;

        Ok(Self { db })
    }

    fn get_db(&self) -> &DatabaseConnection {
        &self.db
    }
}
