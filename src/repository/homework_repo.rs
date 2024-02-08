use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    config::database::{DatabaseTrait, Db},
    entity::{homework, homework_uploaded},
    error::db_error::DbError,
};

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

#[async_trait]
pub trait HomeworkRepositoryTrait {
    async fn get_homework(&self, term: &str, id: &str) -> Result<Option<homework::Model>, DbError>;
    async fn get_homework_uploaded_by_week(
        &self,
        term: &str,
        course_code: &str,
        week: i32,
        with_hidden: bool,
    ) -> Result<Vec<homework_uploaded::Model>, DbError>;
}

#[async_trait]
impl HomeworkRepositoryTrait for HomeworkRepository {
    async fn get_homework(&self, term: &str, id: &str) -> Result<Option<homework::Model>, DbError> {
        use homework::Column as Col;

        Ok(homework::Entity::find()
            .filter(Col::HwTerm.eq(term).and(Col::HwId.eq(id)))
            .one(self.db_conn.get_db())
            .await?)
    }

    async fn get_homework_uploaded_by_week(
        &self,
        term: &str,
        course_code: &str,
        week: i32,
        with_hidden: bool,
    ) -> Result<Vec<homework_uploaded::Model>, DbError> {
        use homework_uploaded::Column as Col;

        let mut filter = Col::HwupTerm
            .eq(term)
            .and(Col::HwupCourseCode.eq(course_code))
            .and(Col::HwupWeek.eq(week));

        if !with_hidden {
            filter = filter.and(Col::HwupIsDel.eq("0"));
        }

        homework_uploaded::Entity::find()
            .filter(filter)
            .all(self.db_conn.get_db())
            .await
            .map_err(DbError::from)
    }
}
