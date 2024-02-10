use std::sync::Arc;

use crate::{
    config::{database::Db, s3::S3Conn, AppConfig},
    service::homework_service::HomeworkService,
};

#[derive(Clone)]
pub struct HomeworkState {
    pub homework_service: HomeworkService,
    pub config: Arc<AppConfig>,
}

impl HomeworkState {
    pub fn new(db_conn: &Arc<Db>, s3: &Arc<S3Conn>, config: &Arc<AppConfig>) -> Self {
        Self {
            homework_service: HomeworkService::new(s3, db_conn),
            config: Arc::clone(config),
        }
    }
}
