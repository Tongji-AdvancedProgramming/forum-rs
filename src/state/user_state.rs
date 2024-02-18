use crate::config::database::Db;
use crate::config::s3::S3Conn;
use crate::config::AppConfig;
use crate::service::student_info_service::StudentInfoService;
use crate::service::user_service::UserService;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState {
    pub(crate) user_service: UserService,
    pub(crate) student_info_service: StudentInfoService,
}

impl UserState {
    pub fn new(db_conn: &Arc<Db>, s3: &Arc<S3Conn>, app_config: &Arc<AppConfig>) -> Self {
        Self {
            user_service: UserService::new(db_conn),
            student_info_service: StudentInfoService::new(db_conn, app_config, s3),
        }
    }
}
