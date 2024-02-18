use axum::extract::{Multipart, State};
use axum::Form;
use axum_login::AuthUser;
use forum_macros::forum_handler;
use serde::Deserialize;

use crate::entity::{student, student_info};
use crate::error::param_error::ParameterError;
use crate::service::student_info_service::StudentInfoServiceTrait;
use crate::state::user_state::UserState;

use super::AuthSession;

/// 当前登录的用户信息
#[forum_handler]
pub async fn get_me(State(state): State<UserState>, auth_session: AuthSession) -> student::Model {
    let user_id = &auth_session.user.as_ref().unwrap().id();
    Ok::<_, ApiError>(state.user_service.get_by_id(user_id).await.unwrap())
}

/// 当前登录的用户论坛信息
///
/// 返回的是一些帮助论坛更加友好运行的信息
#[forum_handler]
pub async fn get_my_info(
    State(state): State<UserState>,
    auth_session: AuthSession,
) -> Option<student_info::Model> {
    let user_id = &auth_session.user.as_ref().unwrap().id();
    state.student_info_service.get_by_stu_no(user_id).await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetNicknameParams {
    pub nick_name: String,
}

/// 设置昵称
#[forum_handler]
pub async fn set_nickname(
    State(state): State<UserState>,
    auth_session: AuthSession,
    Form(params): Form<SetNicknameParams>,
) {
    let user_id = &auth_session.user.as_ref().unwrap().id();
    state
        .student_info_service
        .set_nickname(user_id, &params.nick_name)
        .await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSignatureParams {
    pub signature: String,
}

/// 设置签名档
#[forum_handler]
pub async fn set_signature(
    State(state): State<UserState>,
    auth_session: AuthSession,
    Form(params): Form<SetSignatureParams>,
) {
    let user_id = &auth_session.user.as_ref().unwrap().id();
    state
        .student_info_service
        .set_signature(user_id, &params.signature)
        .await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetShortInfoParams {
    pub id: String,
}

/// 指定用户的简短信息
///
/// 返回的是一些帮助论坛更加友好运行的信息
#[forum_handler]
pub async fn get_student_short_info(
    State(state): State<UserState>,
    Form(params): Form<GetShortInfoParams>,
) -> Option<student_info::Model> {
    state.student_info_service.get_by_stu_no(&params.id).await
}

/// 上传头像
#[forum_handler]
pub async fn put_avatar(
    State(state): State<UserState>,
    auth_session: AuthSession,
    mut multipart: Multipart,
) {
    let user_id = auth_session.user.unwrap().id();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            let content_type = field.content_type().unwrap().to_string();
            let bytes = field.bytes().await.unwrap();

            return state
                .student_info_service
                .upload_student_avatar(&user_id, &mut &bytes[..], &content_type)
                .await
                .map(ApiResponse::ok)
                .map_err(Into::into);
        }
    }

    Err(ParameterError::InvalidParameter("未提供文件"))
}

/// 上传签名档背景
#[forum_handler]
pub async fn put_card_background(
    State(state): State<UserState>,
    auth_session: AuthSession,
    mut multipart: Multipart,
) {
    let user_id = auth_session.user.unwrap().id();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            let content_type = field.content_type().unwrap().to_string();
            let bytes = field.bytes().await.unwrap();

            return state
                .student_info_service
                .upload_student_card_background(&user_id, &mut &bytes[..], &content_type)
                .await
                .map(ApiResponse::ok)
                .map_err(Into::into);
        }
    }

    Err(ParameterError::InvalidParameter("未提供文件"))
}
