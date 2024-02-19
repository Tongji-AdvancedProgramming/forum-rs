use crate::config::permission::Permission;
use crate::entity::post;
use crate::error::param_error::ParameterError::InvalidParameter;
use crate::error::proc_error::ProcessError;
use crate::service::post_service::GetPostsResult;
use crate::{error::auth_error::AuthError, service::post_service::PostServiceTrait};
use axum::extract::Query;
use axum::extract::State;
use axum_client_ip::SecureClientIp;
use axum_extra::extract::Query as ExQuery;
use axum_login::{AuthUser, AuthzBackend};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use forum_macros::forum_handler;
use forum_utils::encoding_helper::EncodingHelper;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};

use crate::state::post_state::PostState;

use super::AuthSession;

#[derive(Debug, Clone, TryFromMultipart, IntoParams)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct AddPostParams {
    /// 板块Id
    pub board_id: String,

    /// 帖子标题
    pub title: String,

    /// 帖子内容
    pub content: String,
}

/// 发布帖子
#[utoipa::path(
    post,
    path = "/post",
    tag = "Post",
    responses(
        (status = 200, body = inline(i32))
    ),
    params(AddPostParams),
)]
#[forum_handler]
pub async fn add_post(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    TypedMultipart(params): TypedMultipart<AddPostParams>,
) -> i32 {
    if !EncodingHelper::gbk_guard(&params.content) {
        return Err(InvalidParameter("帖子内容包含非GBK字符").into());
    }

    let user_id = auth_session.user.unwrap().id();
    state
        .post_service
        .add_post(
            &user_id,
            &ip_addr,
            &params.board_id,
            &params.title,
            &params.content,
        )
        .await
}

#[derive(Debug, Clone, TryFromMultipart, IntoParams)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct AddReplyParams {
    /// 帖子Id
    pub post_id: i32,

    /// 回复内容
    pub reply_content: String,
}

/// 发送回帖
#[utoipa::path(post, path = "/post/reply", tag = "Post", params(AddReplyParams))]
#[forum_handler]
pub async fn add_reply(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    TypedMultipart(params): TypedMultipart<AddReplyParams>,
) {
    if !EncodingHelper::gbk_guard(&params.reply_content) {
        return Err(InvalidParameter("帖子内容包含非GBK字符").into());
    }

    let user_id = auth_session.user.unwrap().id();
    state
        .post_service
        .add_reply(&user_id, &ip_addr, params.post_id, &params.reply_content)
        .await
}

#[derive(Debug, Clone, TryFromMultipart, IntoParams)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct EditPostParams {
    /// 帖子Id
    pub post_id: i32,

    /// 编辑内容
    pub content: String,
}

/// 编辑帖子或回复
#[utoipa::path(put, path = "/post", tag = "Post", params(AddReplyParams))]
#[forum_handler]
pub async fn edit_post(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    TypedMultipart(params): TypedMultipart<EditPostParams>,
) {
    if !EncodingHelper::gbk_guard(&params.content) {
        return Err(InvalidParameter("帖子内容包含非GBK字符").into());
    }

    let user_id = auth_session.user.unwrap().id();
    if state
        .post_service
        .ensure_edit_post_permission(&user_id, params.post_id)
        .await?
    {
        state
            .post_service
            .edit_post(&user_id, &ip_addr, params.post_id, &params.content)
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

/// 设置帖子标签
#[utoipa::path(put, path = "/post/tag", tag = "Post", params(SetPostTagsParams))]
#[forum_handler]
pub async fn set_post_tags(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    ExQuery(params): ExQuery<SetPostTagsParams>,
) {
    let user_id = auth_session.user.unwrap().id();

    if state
        .post_service
        .ensure_edit_posts_permission(&user_id, &params.post_id)
        .await?
    {
        for id in params.post_id {
            state
                .post_service
                .set_post_tag(&user_id, &ip_addr, id, &params.tag)
                .await?
        }
        Ok::<(), ApiError>(())
    } else {
        Err(AuthError::PermissionDenied("请求中存在无权设置标签的帖子").into())
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SetPostPriorityParams {
    /// 帖子Id
    pub post_id: Vec<i32>,

    /// 优先级
    #[serde(default)]
    pub priority: i32,
}

/// 设置帖子优先级
#[utoipa::path(
    put,
    path = "/post/priority",
    tag = "Post",
    params(SetPostPriorityParams)
)]
#[forum_handler]
pub async fn set_post_priority(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    ExQuery(params): ExQuery<SetPostPriorityParams>,
) {
    let user_id = auth_session.user.unwrap().id();

    if params.priority < 0 || params.priority > 9 {
        return Err(InvalidParameter("优先级只能为0~9").into());
    }

    if state
        .post_service
        .ensure_edit_posts_permission(&user_id, &params.post_id)
        .await?
    {
        for id in params.post_id {
            state
                .post_service
                .set_post_priority(&user_id, &ip_addr, id, params.priority)
                .await?
        }
        Ok::<(), ApiError>(())
    } else {
        Err(AuthError::PermissionDenied("请求中存在无权设置优先级的帖子").into())
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct DeletePostsParams {
    /// 帖子Id
    pub post_id: Vec<i32>,
}

/// 删除帖子或回复
#[utoipa::path(delete, path = "/post", tag = "Post", params(DeletePostsParams))]
#[forum_handler]
pub fn delete_posts(
    State(state): State<PostState>,
    auth_session: AuthSession,
    SecureClientIp(ip_addr): SecureClientIp,
    ExQuery(params): ExQuery<DeletePostsParams>,
) {
    let user_id = &auth_session.user.as_ref().unwrap().id();

    if state
        .post_service
        .ensure_edit_posts_permission(&user_id, &params.post_id)
        .await?
    {
        for id in params.post_id {
            state
                .post_service
                .delete_post(user_id, &ip_addr, id)
                .await?
        }
        Ok::<(), ApiError>(())
    } else {
        Err(AuthError::PermissionDenied("请求中存在无权删除的帖子").into())
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListPostsParams {
    /// 板块id
    pub board_id: String,

    /// 标签序号
    pub tags: String,

    /// 是否显示隐藏帖子
    pub show_hidden: bool,

    /// 分页: 页面大小
    pub page_size: u64,

    /// 分页: 页面编号
    pub page_index: u64,
}

#[derive(Debug, Clone, Serialize, ToResponse, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListPostsResult {
    pub total_count: u64,
    pub posts: Vec<post::Model>,
}

/// 列出帖子
#[utoipa::path(
    get,
    path = "/post/list",
    tag = "Post",
    responses(
        (status = 200, body = inline(ListPostsResult))
    ),
    params(ListPostsParams)
)]
#[forum_handler]
pub async fn list_posts(
    State(state): State<PostState>,
    auth_session: AuthSession,
    Query(params): Query<ListPostsParams>,
) -> ListPostsResult {
    let user_id = &auth_session.user.as_ref().unwrap().id();
    let tags = urlencoding::decode(&params.tags).map_err(|_| InvalidParameter("传入的tag无效"))?;

    if params.show_hidden
        && !auth_session
            .backend
            .has_perm(auth_session.user.as_ref().unwrap(), Permission::TA)
            .await
            .map_err(|_| ProcessError::GeneralError("验证权限失败"))?
    {
        return Err(AuthError::PermissionDenied("您无权查看隐藏帖子").into());
    }

    if state
        .post_service
        .ensure_query_board_permission(&user_id, &params.board_id)
        .await?
    {
        Ok::<_, ApiError>(ListPostsResult {
            total_count: state
                .post_service
                .get_posts_count(&params.board_id, &tags, params.show_hidden, false)
                .await?,
            posts: state
                .post_service
                .get_posts(
                    &params.board_id,
                    &tags,
                    params.show_hidden,
                    false,
                    false,
                    params.page_size,
                    params.page_index,
                )
                .await?,
        })
    } else {
        Err(AuthError::PermissionDenied("您无权查看本板块").into())
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetPostsParams {
    pub post_id: i32,
    pub show_hidden: bool,
}

/// 显示帖子（含回帖等）
///
/// 用于在帖子页中使用，包含回帖等内容
#[utoipa::path(get, path = "/post", tag = "Post", params(GetPostsParams))]
#[forum_handler]
pub async fn get_posts(
    State(state): State<PostState>,
    auth_session: AuthSession,
    Query(params): Query<GetPostsParams>,
) -> GetPostsResult {
    if params.show_hidden
        && !auth_session
            .backend
            .has_perm(auth_session.user.as_ref().unwrap(), Permission::TA)
            .await
            .map_err(|_| ProcessError::GeneralError("验证权限失败"))?
    {
        return Err(AuthError::PermissionDenied("您无权查看隐藏帖子").into());
    }

    let user_id = &auth_session.user.as_ref().unwrap().id();
    if state
        .post_service
        .ensure_query_post_permission(user_id, params.post_id)
        .await?
    {
        state
            .post_service
            .get_post(params.post_id, params.show_hidden)
            .await
    } else {
        Err(AuthError::PermissionDenied("您无权查看此帖子").into())
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetPostParentParams {
    pub post_id: i32,
}

/// 查询帖子的父亲帖子
///
/// 用于跳转的时候准确跳转到父亲帖子
#[utoipa::path(get, path = "/post/parent", tag = "Post", params(GetPostParentParams))]
#[forum_handler]
pub async fn get_post_parent(
    State(state): State<PostState>,
    Query(params): Query<GetPostParentParams>,
) -> Option<i32> {
    state.post_service.get_parent_post(params.post_id).await
}
