use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::config::database::DatabaseTrait;
use crate::config::s3::S3Conn;
use crate::config::AppConfig;
use crate::dto::student_short_info::StudentShortInfo;
use crate::error::api_error::ApiError;
use crate::error::param_error::ParameterError;
use crate::repository::student_info_repo::StudentInfoRepositoryTrait;
use crate::{config::database::Db, repository::student_info_repo::StudentInfoRepository};

use crate::entity::student_info::{Column as Cols, Entity, Model as StudentInfo};

use super::upload_service::{UploadService, UploadServiceTrait};

pub trait StudentInfoServiceTrait {
    async fn get_by_stu_no(&self, stu_no: &str) -> Result<Option<StudentInfo>, ApiError>;

    async fn upload_student_avatar(
        &self,
        stu_no: &str,
        input_file: &[u8],
        content_type: &str,
    ) -> Result<(), ApiError>;

    async fn upload_student_card_background(
        &self,
        stu_no: &str,
        input_file: &[u8],
        content_type: &str,
    ) -> Result<(), ApiError>;

    async fn set_nickname(&self, stu_no: &str, nickname: &str) -> Result<(), ApiError>;

    async fn set_signature(&self, stu_no: &str, signature: &str) -> Result<(), ApiError>;

    async fn get_student_short_info(
        &self,
        stu_no: &str,
    ) -> Result<Option<StudentShortInfo>, ApiError>;
}

#[derive(Clone)]
pub struct StudentInfoService {
    upload_service: UploadService,
    student_info_repository: StudentInfoRepository,
    db_conn: Arc<Db>,
    app_config: Arc<AppConfig>,
}

impl StudentInfoService {
    pub fn new(db_conn: &Arc<Db>, app_config: &Arc<AppConfig>, s3: &Arc<S3Conn>) -> Self {
        Self {
            upload_service: UploadService::new(s3, app_config),
            student_info_repository: StudentInfoRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
            app_config: Arc::clone(app_config),
        }
    }
}

impl StudentInfoServiceTrait for StudentInfoService {
    async fn get_by_stu_no(&self, stu_no: &str) -> Result<Option<StudentInfo>, ApiError> {
        let result = Entity::find()
            .filter(Cols::StuNo.eq(stu_no))
            .one(self.db_conn.get_db())
            .await?;

        if result.is_some() {
            Ok(result)
        } else {
            // 如果不存在，就生成一个默认的补上。
            let student = self
                .student_info_repository
                .get_student_default_info(stu_no)
                .await?;
            if student.is_none() {
                Ok(None)
            } else {
                let student_info = StudentInfo {
                    stu_no: stu_no.to_string(),
                    description: "".to_string(),
                    nickname: student.unwrap().stu_name,
                }
                .into_active_model();

                Ok(Some(student_info.insert(self.db_conn.get_db()).await?))
            }
        }
    }

    async fn upload_student_avatar(
        &self,
        stu_no: &str,
        input_file: &[u8],
        content_type: &str,
    ) -> Result<(), ApiError> {
        let ref prefix = self.app_config.s3.prefix.avatar;

        self.upload_service
            .upload_student_assets(stu_no, input_file, prefix, content_type)
            .await
            .map_err(Into::into)
    }

    async fn upload_student_card_background(
        &self,
        stu_no: &str,
        input_file: &[u8],
        content_type: &str,
    ) -> Result<(), ApiError> {
        let ref prefix = self.app_config.s3.prefix.card_bg;

        self.upload_service
            .upload_student_assets(stu_no, input_file, prefix, content_type)
            .await
            .map_err(Into::into)
    }

    async fn set_nickname(&self, stu_no: &str, nickname: &str) -> Result<(), ApiError> {
        let mut si = Entity::find_by_id(stu_no)
            .one(self.db_conn.get_db())
            .await?
            .ok_or(ParameterError::InvalidParameter("指定学生不存在"))?
            .into_active_model();
        si.nickname = Set(nickname.to_string());
        si.save(self.db_conn.get_db()).await?;
        Ok(())
    }

    async fn set_signature(&self, stu_no: &str, signature: &str) -> Result<(), ApiError> {
        let mut si = Entity::find_by_id(stu_no)
            .one(self.db_conn.get_db())
            .await?
            .ok_or(ParameterError::InvalidParameter("指定学生不存在"))?
            .into_active_model();
        si.description = Set(signature.to_string());
        si.save(self.db_conn.get_db()).await?;
        Ok(())
    }

    async fn get_student_short_info(
        &self,
        stu_no: &str,
    ) -> Result<Option<StudentShortInfo>, ApiError> {
        self.student_info_repository
            .get_student_short_info(stu_no)
            .await
            .map_err(Into::into)
    }
}
