use std::sync::Arc;

use crate::config::database::Db;

#[derive(Debug, Clone)]
pub struct HomeworkRepository {
    db_conn: Arc<Db>,
}

impl HomeworkRepository {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
}