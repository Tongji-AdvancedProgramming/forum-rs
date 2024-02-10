use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{config::database::Db, entity::post, error::api_error::ApiError};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPostsResult {
    pub posts: Vec<post::Model>,
}

#[async_trait]
pub trait PostServiceTrait {
    /// 确认用户可以查询该帖子
    async fn ensure_query_post_permission(
        &self,
        user_id: &str,
        post_id: i32,
    ) -> Result<bool, ApiError>;

    /// 确认用户可以查询该板块
    async fn ensure_query_board_permission(
        &self,
        user_id: &str,
        board_id: &str,
    ) -> Result<bool, ApiError>;

    /// 确认用户可以编辑该帖子
    async fn ensure_edit_post_permission(
        &self,
        user_id: &str,
        post_id: i32,
    ) -> Result<bool, ApiError>;

    /// 获取板块内的帖子
    async fn get_posts(
        &self,
        board_id: &str,
        tags: &str,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        page_size: i32,
        page_index: i32,
    ) -> Result<Vec<post::Model>, ApiError>;

    /// 获取板块内的帖子数量
    async fn get_posts_count(
        &self,
        board_id: &str,
        tags: &str,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<i32, ApiError>;

    /// 添加帖子
    async fn add_post(
        &self,
        user_id: &str,
        ip_addr: &str,
        board_id: &str,
        title: &str,
        content: &str,
    ) -> Result<String, ApiError>;

    /// 添加回复
    async fn add_reply(
        &self,
        user_id: &str,
        ip_addr: &str,
        father_post: i32,
        content: &str,
    ) -> Result<(), ApiError>;

    /// 编辑帖子
    async fn edit_post(
        &self,
        user_id: &str,
        ip_addr: &str,
        post_id: i32,
        new_content: &str,
    ) -> Result<(), ApiError>;

    /// 设置帖子标签
    async fn set_post_tag(
        &self,
        user_id: &str,
        ip_addr: &str,
        post_id: i32,
        tag: Vec<i32>,
    ) -> Result<(), ApiError>;

    /// 设置帖子优先级
    async fn set_post_priority(
        &self,
        user_id: &str,
        ip_addr: &str,
        post_id: i32,
        priority: i32,
    ) -> Result<(), ApiError>;

    /// 删除帖子
    async fn delete_post(&self, user_id: &str, ip_addr: &str, post_id: i32)
        -> Result<(), ApiError>;

    /// 查询帖子，包括所有回帖及回帖的回帖
    async fn get_post(&self, post_id: i32, with_hidden: bool) -> Result<GetPostsResult, ApiError>;

    /// 查询帖子的父帖子
    async fn get_parent_post(&self, post_id: i32) -> Result<i32, ApiError>;
}

pub struct PostService {
    pub db_conn: Arc<Db>,
}
