use async_trait::async_trait;

use crate::{
    entity::{homework, homework_uploaded},
    error::db_error::DbError,
};

#[async_trait]
pub trait HomeworkServiceTrait {
    async fn get_homework(
        &self,
        term: &str,
        id: &str,
        course_no: &str,
    ) -> Result<Option<homework::Model>, DbError>;

    async fn get_homework_uploaded(
        &self,
        board_id: &str,
        with_hidden: bool,
    ) -> Result<Vec<homework_uploaded::Model>, DbError>;

    async fn post_homework(
        &self,
        homework_uploaded: &homework_uploaded::Model,
    ) -> Result<String, DbError>;
}
