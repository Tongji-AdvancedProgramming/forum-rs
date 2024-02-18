use axum::extract::{Multipart, State};
use axum_login::AuthUser;
use forum_macros::forum_handler;

use crate::{
    error::param_error::ParameterError, service::upload_service::UploadServiceTrait,
    state::upload_state::UploadState,
};

use super::AuthSession;

/// 上传图片，注册用户可调用
#[forum_handler]
pub async fn add_image(
    State(state): State<UploadState>,
    auth_session: AuthSession,
    mut multipart: Multipart,
) -> String {
    let user_id = auth_session.user.unwrap().id();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            let file_name = field.file_name().unwrap().to_string();
            let suffix = file_name.split('.').last().unwrap_or_default();
            let content_type = field.content_type().unwrap().to_string();
            let bytes = field.bytes().await.unwrap();

            return state
                .upload_service
                .upload_image(&user_id, &mut &bytes[..], suffix, &content_type)
                .await
                .map(ApiResponse::ok)
                .map_err(Into::into);
        }
    }

    Err(ParameterError::InvalidParameter("未提供文件"))
}
