use crate::config::database::Db;
use crate::service::log_service::LogService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub log_service: LogService,
}

impl AuthState {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            log_service: LogService::new(db_conn),
        }
    }
}
