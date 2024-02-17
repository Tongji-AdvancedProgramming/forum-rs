use crate::{error::auth_error::AuthError, service::post_service::PostServiceTrait};
use axum::{extract::State, http::StatusCode, Form};
use axum_client_ip::SecureClientIp;
use axum_extra::extract::Form as ExForm;
use axum_login::AuthUser;
use forum_macros::forum_handler;
use forum_utils::encoding_helper::EncodingHelper;
use log::info;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::state::post_state::PostState;

use super::AuthSession;

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AddPostParams {
    /// 板块Id
    pub board_id: String,

    /// 帖子标题
    pub title: String,

    /// 帖子内容
    pub content: String,
}

/// 发布帖子
#[forum_handler]
pub async fn add_post(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    Form(params): Form<AddPostParams>,
) -> i32 {
    let content = EncodingHelper::utf2gbk(&params.content);

    let user_id = auth_session.user.unwrap().id();
    state
        .post_service
        .add_post(
            &user_id,
            &ip_addr,
            &params.board_id,
            &params.title,
            &content,
        )
        .await
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AddReplyParams {
    /// 帖子Id
    pub post_id: i32,

    /// 回复内容
    pub reply_content: String,
}

/// 发送回帖
#[forum_handler]
pub async fn add_reply(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    Form(params): Form<AddReplyParams>,
) {
    let content = EncodingHelper::utf2gbk(&params.reply_content);

    let user_id = auth_session.user.unwrap().id();
    state
        .post_service
        .add_reply(&user_id, &ip_addr, params.post_id, &content)
        .await
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct EditPostParams {
    /// 帖子Id
    pub post_id: i32,

    /// 编辑内容
    pub content: String,
}

#[forum_handler]
pub async fn edit_post(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    Form(params): Form<EditPostParams>,
) {
    let content = EncodingHelper::utf2gbk(&params.content);

    let user_id = auth_session.user.unwrap().id();
    if state
        .post_service
        .ensure_edit_post_permission(&user_id, params.post_id)
        .await?
    {
        state
            .post_service
            .edit_post(&user_id, &ip_addr, params.post_id, &content)
            .await
    } else {
        Err(AuthError::PermissionDenied("您无权编辑此帖子").into())
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SetPostTagsParams {
    /// 帖子Id
    pub post_id: Vec<i32>,

    /// 标签
    #[serde(default)]
    pub tag: Vec<i32>,
}

// #[forum_handler]
pub async fn set_post_tags(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    ExForm(params): ExForm<SetPostTagsParams>,
) {
    let user_id = auth_session.user.unwrap().id();
}
