use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    config::database::{DatabaseTrait, Db},
    entity::{homework, homework_uploaded},
    error::proc_error::ProcessError,
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
    async fn get_homework(
        &self,
        term: &str,
        id: &str,
    ) -> Result<Option<homework::Model>, ProcessError>;
    async fn get_homework_uploaded_by_week(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        with_hidden: bool,
    ) -> Result<Vec<homework_uploaded::Model>, ProcessError>;
}

#[async_trait]
impl HomeworkRepositoryTrait for HomeworkRepository {
    async fn get_homework(
        &self,
        term: &str,
        id: &str,
    ) -> Result<Option<homework::Model>, ProcessError> {
        use homework::Column as Col;

        homework::Entity::find()
            .filter(Col::HwTerm.eq(term).and(Col::HwId.eq(id)))
            .one(self.db_conn.get_db())
            .await
            .map_err(ProcessError::from)
    }

    async fn get_homework_uploaded_by_week(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        with_hidden: bool,
    ) -> Result<Vec<homework_uploaded::Model>, ProcessError> {
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
            .map_err(ProcessError::from)
    }
}
