use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

use crate::config::database::{DatabaseTrait, Db};
use crate::entity::student;
use crate::error::api_error::ApiError;
use crate::error::proc_error::ProcessError;
use crate::repository::user_repo::{UserRepository, UserRepositoryTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    db_conn: Arc<Db>,
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: db_conn.clone(),
            user_repo: UserRepository::new(db_conn),
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Option<student::Model> {
        self.user_repo.find_by_id(id).await
    }

    pub async fn guard_user_level(&self, id: &str, level: i32) -> Result<bool, ApiError> {
        let user = student::Entity::find()
            .select_only()
            .column(student::Column::StuUserLevel)
            .filter(student::Column::StuNo.eq(id))
            .into_json()
            .one(self.db_conn.get_db())
            .await
            .map(|v| v.map(|v| serde_json::from_value::<student::Model>(v).unwrap()))?
            .ok_or(ProcessError::GeneralError("未找到指定学生"))?;

        let user_level = user.stu_user_level.parse::<i32>().unwrap();
        Ok(user_level >= level)
    }
}
