mod app_config;
pub mod database;
pub mod redis;
pub mod session;

use crate::config::app_config::{AppConfig, APP_CONFIG};
pub use app_config::init;
use std::sync::{Arc, RwLock};

pub fn get_config() -> Arc<RwLock<AppConfig>> {
    APP_CONFIG.clone()
}
