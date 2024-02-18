use std::{io::Cursor, sync::Arc};

use async_trait::async_trait;
use chrono::Local;
use minio::s3::{args::PutObjectArgs, sse::SseS3};

use crate::{
    config::{s3::S3Conn, AppConfig},
    error::api_error::ApiError,
};

#[async_trait]
pub trait UploadServiceTrait {
    type Error: Into<ApiError>;

    async fn upload_image(
        &self,
        uploader_id: &str,
        input_file: &[u8],
        suffix: &str,
        content_type: &str,
    ) -> Result<String, Self::Error>;

    async fn upload_student_assets(
        &self,
        stu_no: &str,
        input_file: &[u8],
        prefix: &str,
        content_type: &str,
    ) -> Result<(), Self::Error>;
}

#[derive(Clone)]
pub struct UploadService {
    pub s3: Arc<S3Conn>,
    pub config: Arc<AppConfig>,
}

impl UploadService {
    pub fn new(s3: &Arc<S3Conn>, config: &Arc<AppConfig>) -> Self {
        Self {
            s3: Arc::clone(s3),
            config: Arc::clone(config),
        }
    }
}

#[async_trait]
impl UploadServiceTrait for UploadService {
    type Error = minio::s3::error::Error;

    async fn upload_image(
        &self,
        uploader_id: &str,
        input_file: &[u8],
        suffix: &str,
        content_type: &str,
    ) -> Result<String, Self::Error> {
        let ref conf = self.config.s3;

        let timestamp = Local::now().timestamp_millis();
        let key = format!(
            "{}/{}/{}.{}",
            conf.prefix.upload, uploader_id, timestamp, suffix
        );

        let mut cursor = Cursor::new(input_file);

        let mut put_object_args: PutObjectArgs<'_, Cursor<&[u8]>, SseS3> =
            PutObjectArgs::new(&conf.bucket, &key, &mut cursor, None, None)?;
        put_object_args.content_type = content_type;

        self.s3.client.put_object(&mut put_object_args).await?;

        Ok(conf.base_url.clone() + &key)
    }

    async fn upload_student_assets(
        &self,
        stu_no: &str,
        input_file: &[u8],
        prefix: &str,
        content_type: &str,
    ) -> Result<(), Self::Error> {
        let ref conf = self.config.s3;

        let mut cursor = Cursor::new(input_file);

        let key = format!("{}/{}", prefix, stu_no);
        let mut put_obj_args: PutObjectArgs<'_, Cursor<&[u8]>, SseS3> =
            PutObjectArgs::new(&conf.bucket, &key, &mut cursor, None, None)?;
        put_obj_args.content_type = content_type;

        self.s3.client.put_object(&mut put_obj_args).await?;

        Ok(())
    }
}
