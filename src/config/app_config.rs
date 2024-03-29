//! 应用的配置

use std::sync::{Arc, RwLock};

use crate::config::database::DatabaseConfig;
use crate::config::permission::PermissionConfig;
use crate::config::redis::RedisAppConfig;
use config::Config;
use lazy_static::lazy_static;
use log::{error, info};
use serde::Deserialize;

use crate::panic;

use super::meili::MeiliSearchConfig;
use super::s3::S3Config;

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct AppConfig {
    pub port: u16,
    pub database: DatabaseConfig,
    pub redis: RedisAppConfig,
    pub permission: PermissionConfig,
    pub s3: S3Config,
    pub meili: MeiliSearchConfig,
}

pub type AppConf = Arc<RwLock<AppConfig>>;

lazy_static! {
    pub static ref APP_CONFIG: AppConf = Arc::new(RwLock::new(Default::default()));
}

pub fn init() {
    let production = std::env::var("PROD").map(|_| true).unwrap_or(false);
    info!("Production mode: {}", production);

    let settings = Config::builder()
        .add_source(config::File::with_name("app_config.toml"))
        .add_source(if production {
            config::File::with_name("app_config.prod.toml")
        } else {
            config::File::with_name("app_config.dev.toml")
        })
        .build()
        .unwrap_or_else(|e| {
            error!(
                "\n[Config Not Found]\n配置文件不存在，请检查app_config.toml文件是否存在\n\n{}",
                e
            );
            panic();
        });

    let app_config = settings.try_deserialize::<AppConfig>();
    match app_config {
        Ok(app_config) => {
            *APP_CONFIG.write().unwrap() = app_config;
        }
        Err(err) => {
            error!("\n[Config Invalid]\n配置文件字段不完整\n\n{}", err);
            panic();
        }
    }
}
