use std::sync::Arc;

use crate::{
    config::{database::Db, AppConfig},
    service::course_service::CourseService,
};

#[derive(Clone)]
pub struct CourseState {
    pub course_service: CourseService,
}

impl CourseState {
    pub fn new(db_conn: &Arc<Db>, app_config: &Arc<AppConfig>) -> Self {
        Self {
            course_service: CourseService::new(db_conn, app_config),
        }
    }
}
