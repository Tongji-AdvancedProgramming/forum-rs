use std::sync::Arc;

use crate::{
    config::{database::Db, s3::S3Conn},
    service::homework_service::HomeworkService,
};

#[derive(Clone)]
pub struct HomeworkState {
    pub homework_service: HomeworkService,
}

impl HomeworkState {
    pub fn new(db_conn: &Arc<Db>, s3: &Arc<S3Conn>) -> Self {
        Self {
            homework_service: HomeworkService::new(s3, db_conn),
        }
    }
}
