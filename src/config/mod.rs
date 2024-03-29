mod app_config;
pub mod database;
pub mod meili;
pub mod permission;
pub mod redis;
pub mod s3;
pub mod session;

pub use crate::config::app_config::{AppConfig, APP_CONFIG};
pub use app_config::init;
use std::sync::{Arc, RwLock};

pub fn get_config() -> Arc<RwLock<AppConfig>> {
    APP_CONFIG.clone()
}
