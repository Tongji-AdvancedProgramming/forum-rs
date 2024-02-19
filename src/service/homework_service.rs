use std::sync::Arc;

use async_trait::async_trait;
use minio::s3::args::StatObjectArgs;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter,
    TransactionTrait,
};

use crate::{
    config::{
        database::{DatabaseTrait, Db},
        s3::S3Conn,
    },
    dto::board::PostLocation,
    entity::{homework, homework_uploaded},
    error::{api_error::ApiError, proc_error::ProcessError},
    repository::homework_repo::{HomeworkRepository, HomeworkRepositoryTrait},
    service::board_service::BoardServiceTrait,
};

use super::board_service::BoardService;

#[async_trait]
pub trait HomeworkServiceTrait {
    async fn get_homework(
        &self,
        term: &str,
        id: i32,
        course_no: &str,
    ) -> Result<Option<homework::Model>, ProcessError>;

    async fn get_homework_uploaded(
        &self,
        board_id: &str,
        with_hidden: bool,
    ) -> Result<Vec<homework_uploaded::Model>, ApiError>;

    async fn post_homework(
        &self,
        homework_uploaded: homework_uploaded::Model,
    ) -> Result<(), ProcessError>;
}

#[derive(Clone)]
pub struct HomeworkService {
    pub s3_client: Arc<S3Conn>,
    pub db_conn: Arc<Db>,
    pub board_service: BoardService,
    pub homework_repository: HomeworkRepository,
}

impl HomeworkService {
    pub fn new(s3_client: &Arc<S3Conn>, db_conn: &Arc<Db>) -> Self {
        Self {
            s3_client: Arc::clone(s3_client),
            db_conn: Arc::clone(db_conn),
            board_service: BoardService::new(db_conn),
            homework_repository: HomeworkRepository::new(db_conn),
        }
    }
}

#[async_trait]
impl HomeworkServiceTrait for HomeworkService {
    async fn get_homework(
        &self,
        term: &str,
        id: i32,
        course_no: &str,
    ) -> Result<Option<homework::Model>, ProcessError> {
        use homework::Column as Col;

        homework::Entity::find()
            .filter(Col::HwTerm.eq(term))
            .filter(Col::HwId.eq(id))
            .filter(Col::HwCourseCode.eq(course_no))
            .one(self.db_conn.get_db())
            .await
            .map_err(ProcessError::from)
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
                    board.course.as_ref().unwrap().course_code.as_ref().unwrap(),
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
        mut homework_uploaded: homework_uploaded::Model,
    ) -> Result<(), ProcessError> {
        // 出现文件名时，检查正确性，并下载内容并计算MD5
        if !homework_uploaded.hwup_filename.as_ref().unwrap().is_empty() {
            let object_name: String = if homework_uploaded
                .hwup_filename
                .as_ref()
                .unwrap()
                .starts_with("forum/")
            {
                String::from(&homework_uploaded.hwup_filename.as_ref().unwrap()[5..])
            } else {
                String::from(homework_uploaded.hwup_filename.clone().unwrap())
            };

            let stat = self
                .s3_client
                .client
                .stat_object(&StatObjectArgs {
                    bucket: &self.s3_client.config.bucket,
                    object: &object_name,
                    ..Default::default()
                })
                .await
                .map_err(|_| {
                    ProcessError::GeneralError(
                        "无法从存储系统中寻找到文件，请检查文件名填写是否正确".into(),
                    )
                })?;
            homework_uploaded.hwup_file_md5 = Some(stat.etag);
        }

        use homework_uploaded::Column as Col;
        let filter = Condition::all()
            .add(Col::HwupTerm.eq(&homework_uploaded.hwup_term))
            .add(Col::HwupCourseCode.eq(&homework_uploaded.hwup_course_code))
            .add(Col::HwupId.eq(&homework_uploaded.hwup_id));

        let db = self.db_conn.get_db();

        let txn = db.begin().await?;
        let mut hw: homework_uploaded::ActiveModel = homework_uploaded.into();
        match homework_uploaded::Entity::find()
            .filter(filter)
            .count(&txn)
            .await?
        {
            0 => hw.insert(&txn).await?,
            _ => {
                hw = hw.reset_all();
                hw.update(&txn).await?
            }
        };

        txn.commit().await?;

        Ok(())
    }
}
