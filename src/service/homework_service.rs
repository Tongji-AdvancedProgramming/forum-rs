use std::sync::Arc;

use async_trait::async_trait;
use minio::s3::types::S3;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    config::database::{DatabaseTrait, Db},
    dto::board::PostLocation,
    entity::{homework, homework_uploaded},
    error::{api_error::ApiError, db_error::DbError},
    repository::homework_repo::{HomeworkRepository, HomeworkRepositoryTrait},
    service::board_service::BoardServiceTrait,
};

use super::board_service::BoardService;

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
    ) -> Result<Vec<homework_uploaded::Model>, ApiError>;

    async fn post_homework(
        &self,
        homework_uploaded: &homework_uploaded::Model,
    ) -> Result<String, DbError>;
}

#[derive(Clone)]
pub struct HomeWorkService {
    pub s3_client: Arc<S3>,
    pub db_conn: Arc<Db>,
    pub board_service: BoardService,
    pub homework_repository: HomeworkRepository,
}

impl HomeWorkService {
    pub fn new(s3_client: &Arc<S3>, db_conn: &Arc<Db>) -> Self {
        Self {
            s3_client: Arc::clone(s3_client),
            db_conn: Arc::clone(db_conn),
            board_service: BoardService::new(db_conn),
            homework_repository: HomeworkRepository::new(db_conn),
        }
    }
}

#[async_trait]
impl HomeworkServiceTrait for HomeWorkService {
    async fn get_homework(
        &self,
        term: &str,
        id: &str,
        course_no: &str,
    ) -> Result<Option<homework::Model>, DbError> {
        use homework::Column as Col;

        homework::Entity::find()
            .filter(Col::HwTerm.eq(term))
            .filter(Col::HwId.eq(id))
            .filter(Col::HwCourseCode.eq(course_no))
            .one(self.db_conn.get_db())
            .await
            .map_err(DbError::from)
    }

    async fn get_homework_uploaded(
        &self,
        board_id: &str,
        with_hidden: bool,
    ) -> Result<Vec<homework_uploaded::Model>, ApiError> {
        let board = self.board_service.parse_id(board_id)?;

        if board.location == PostLocation::Weekly {
            self.homework_repository
                .get_homework_uploaded_by_week(
                    &board.course.as_ref().unwrap().course_term,
                    &board.course.as_ref().unwrap().course_code,
                    board.week,
                    with_hidden,
                )
                .await
                .map_err(ApiError::from)
        } else {
            Ok(vec![])
        }
    }

    async fn post_homework(
        &self,
        _homework_uploaded: &homework_uploaded::Model,
    ) -> Result<String, DbError> {
        todo!()
    }
}
