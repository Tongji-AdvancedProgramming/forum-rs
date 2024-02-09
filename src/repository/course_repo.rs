use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    config::database::{DatabaseTrait, Db},
    entity::course,
    error::proc_error::ProcessError,
};

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

#[async_trait]
pub trait CourseRepositoryTrait {
    async fn get_all_course_detail<T, U>(&self, keys: T) -> Result<U, ProcessError>
    where
        T: IntoIterator<Item = (String, String)> + Send + Sync,
        U: FromIterator<course::Model> + Send + Sync;

    async fn get_course_detail(
        &self,
        key: &(String, String),
    ) -> Result<Option<course::Model>, ProcessError>;
}

#[async_trait]
impl CourseRepositoryTrait for CourseRepository {
    async fn get_all_course_detail<T, U>(&self, keys: T) -> Result<U, ProcessError>
    where
        T: IntoIterator<Item = (String, String)> + Send + Sync,
        U: FromIterator<course::Model> + Send + Sync,
    {
        let mut condition = Condition::any();

        for (term, course_no) in keys {
            let sub_condition = Condition::all()
                .add(course::Column::CourseTerm.eq(&term))
                .add(course::Column::CourseNo.eq(&course_no));
            condition = condition.add(sub_condition);
        }

        Ok(course::Entity::find()
            .filter(condition)
            .all(self.db_conn.get_db())
            .await?
            .into_iter()
            .collect())
    }

    async fn get_course_detail(
        &self,
        key: &(String, String),
    ) -> Result<Option<course::Model>, ProcessError> {
        let (term, course_no) = key;
        let course = course::Entity::find()
            .filter(course::Column::CourseTerm.eq(term))
            .filter(course::Column::CourseNo.eq(course_no))
            .one(self.db_conn.get_db())
            .await?;

        Ok(course)
    }
}
