use std::sync::Arc;

use async_trait::async_trait;

use crate::config::database::{DatabaseTrait, Db};
use crate::entity::student::Student;

#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Db>,
}

#[async_trait]
pub trait UserRepositoryTrait {
    fn new(db_conn: &Arc<Db>) -> Self;
    async fn find_by_id(&self, id: &str) -> Option<Student>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn)
        }
    }

    async fn find_by_id(&self, id: &str) -> Option<Student> {
        let user = sqlx::query_as::<_, Student>("SELECT * FROM student WHERE stu_no = ?")
            .bind(id)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None);
        user
    }
}
