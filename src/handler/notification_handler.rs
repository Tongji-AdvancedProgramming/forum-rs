use axum::extract::State;
use axum_login::AuthUser;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use forum_macros::forum_handler;
use utoipa::IntoParams;

use crate::{
    entity::notification::Model as Notification,
    service::notification_service::NotificationServiceTrait,
    state::notification_state::NotificationState,
};

use super::AuthSession;

/// 获取用户的通知
#[utoipa::path(
    get,
    path = "/notification",
    tag = "Notification",
    responses(
        (status = 200, description = "获取通知成功", body = inline(Vec<Notification>))
    ),
)]
#[forum_handler]
pub async fn get_my_notifications(
    State(state): State<NotificationState>,
    auth_session: AuthSession,
) -> Vec<Notification> {
    let user_id = &auth_session.user.as_ref().unwrap().id();
    state.notification_service.get_notifications(user_id).await
}

#[derive(Debug, Clone, TryFromMultipart, IntoParams)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct ReadMyParams {
    pub notification_id: u64,
}

/// 用户已读通知
#[utoipa::path(
    post,
    path = "/notification/read",
    tag = "Notification",
    params(ReadMyParams)
)]
#[forum_handler]
pub async fn read_my_notifications(
    State(state): State<NotificationState>,
    auth_session: AuthSession,
    TypedMultipart(params): TypedMultipart<ReadMyParams>,
) {
    let user_id = &auth_session.user.as_ref().unwrap().id();
    state
        .notification_service
        .user_read_notification(params.notification_id, user_id)
        .await
}

/// 用户已读所有通知
#[utoipa::path(post, path = "/notification/readAll", tag = "Notification")]
#[forum_handler]
pub async fn read_all_my_notifications(
    State(state): State<NotificationState>,
    auth_session: AuthSession,
) {
    let user_id = &auth_session.user.as_ref().unwrap().id();
    state
        .notification_service
        .user_read_all_notification(user_id)
        .await
}

/// 用户清除所有通知
#[utoipa::path(delete, path = "/notification/all", tag = "Notification")]
#[forum_handler]
pub async fn delete_all_my_notifications(
    State(state): State<NotificationState>,
    auth_session: AuthSession,
) {
    let user_id = &auth_session.user.as_ref().unwrap().id();
    state
        .notification_service
        .user_delete_all_notification(user_id)
        .await
}
