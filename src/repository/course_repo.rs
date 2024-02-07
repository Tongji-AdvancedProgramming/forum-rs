use std::sync::Arc;

use crate::config::database::Db;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CourseRepository {
    db_conn: Arc<Db>,
}

#[allow(dead_code)]
impl CourseRepository {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
}
