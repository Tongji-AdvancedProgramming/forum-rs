use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityOrSelect, EntityTrait, QueryFilter, QuerySelect};

use crate::config::database::{DatabaseTrait, Db};
use crate::entity::student;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Db>,
}

#[async_trait]
pub trait UserRepositoryTrait {
    fn new(db_conn: &Arc<Db>) -> Self;
    async fn find_by_id(&self, id: &str) -> Option<student::Model>;
    async fn select_courses(&self, id: &str) -> Option<student::Model>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn find_by_id(&self, id: &str) -> Option<student::Model> {
        use student::Entity as Student;

        let user = Student::find()
            .filter(student::Column::StuNo.eq(id))
            .one(self.db_conn.get_db())
            .await
            .ok()?;
        user
    }

    async fn select_courses(&self, id: &str) -> Option<student::Model> {
        use student::Column as Col;
        use student::Entity as Student;

        let user = Student::find()
            .select()
            .columns([
                Col::StuTerm,
                Col::StuUserLevel,
                Col::StuCno1,
                Col::StuCno1IsDel,
                Col::StuCno2,
                Col::StuCno3,
                Col::StuCno3IsDel,
            ])
            .filter(student::Column::StuNo.eq(id))
            .one(self.db_conn.get_db())
            .await
            .ok()?;
        user
    }
}
