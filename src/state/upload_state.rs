use std::sync::Arc;

use crate::{
    config::{s3::S3Conn, AppConfig},
    service::upload_service::UploadService,
};

#[derive(Clone)]
pub struct UploadState {
    pub upload_service: UploadService,
}

impl UploadState {
    pub fn new(s3: &Arc<S3Conn>, app_config: &Arc<AppConfig>) -> Self {
        Self {
            upload_service: UploadService::new(s3, app_config),
        }
    }
}
