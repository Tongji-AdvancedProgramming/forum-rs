use std::sync::Arc;
use crate::config::database::Db;
use crate::entity::student::Student;
use crate::repository::user_repo::{UserRepository, UserRepositoryTrait};

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository
}

impl UserService {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn)
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Option<Student> {
        self.user_repo.find_by_id(id).await
    }
}
