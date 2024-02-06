use crate::config::database::Db;
use crate::service::user_service::UserService;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState {
    pub(crate) user_service: UserService,
}

impl UserState {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            user_service: UserService::new(db_conn),
        }
    }
}
