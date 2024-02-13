use std::sync::Arc;

use async_trait::async_trait;
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::{
    config::{
        database::{DatabaseTrait, Db},
        AppConfig,
    },
    dto::board::PostLocation,
    entity::{
        post::{self, Column as Cols, Entity},
        student,
    },
    error::{api_error::ApiError, auth_error::AuthError, param_error::ParameterError},
    repository::post_repo::{PostRepository, PostRepositoryTrait},
    service::board_service::BoardServiceTrait,
};

use super::{
    board_service::BoardService,
    course_service::{CourseService, CourseServiceTrait},
    metadata_service::{MetadataService, MetadataServiceTrait},
    user_service::UserService,
};

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
        page_size: u64,
        page_index: u64,
    ) -> Result<Vec<post::Model>, ApiError>;

    /// 获取板块内的帖子数量
    async fn get_posts_count(
        &self,
        board_id: &str,
        tags: &str,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, ApiError>;

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
    pub app_config: Arc<AppConfig>,
    pub metadata_service: MetadataService,
    pub user_service: UserService,
    pub course_service: CourseService,
    pub board_service: BoardService,
    pub post_repository: PostRepository,
}

impl PostService {
    pub fn new(db_conn: &Arc<Db>, app_config: &Arc<AppConfig>) -> Self {
        PostService {
            db_conn: Arc::clone(db_conn),
            app_config: Arc::clone(app_config),
            metadata_service: MetadataService::new(db_conn),
            user_service: UserService::new(db_conn),
            course_service: CourseService::new(db_conn, app_config),
            board_service: BoardService::new(db_conn),
            post_repository: PostRepository::new(db_conn),
        }
    }

    pub async fn resolve_tags(&self, tags: &str) -> Result<Vec<String>, ApiError> {
        let error = ApiError::ParameterError(ParameterError::InvalidParameter("无效的tag传入"));
        if tags == "[]" {
            return Ok(vec![]);
        }

        let tag_list = self.metadata_service.get_tags().await?;
        let tag_indexes: Vec<usize> = serde_json::from_str(tags).map_err(|_| error)?;
        let tag_indexes: Vec<_> = tag_indexes
            .into_iter()
            .filter(|i| *i >= 0 && *i < tag_list.len())
            .map(|index| tag_list[index].tag_field_name.clone())
            .collect();

        Ok(tag_indexes)
    }
}

#[async_trait]
impl PostServiceTrait for PostService {
    /// 确认用户可以查询该帖子
    async fn ensure_query_post_permission(
        &self,
        user_id: &str,
        post_id: i32,
    ) -> Result<bool, ApiError> {
        let stu_course_codes = self.course_service.get_user_course_codes(user_id).await?;

        let post = Entity::find()
            .select_only()
            .columns([Cols::PostId, Cols::PostTerm, Cols::PostCourseCode])
            .filter(Cols::PostId.eq(post_id))
            .one(self.db_conn.get_db())
            .await?;

        Ok(post.is_some()
            && stu_course_codes.into_iter().any(|codes| {
                codes.0 == post.as_ref().unwrap().post_term
                    && codes.1 == post.as_ref().unwrap().post_course_code
            }))
    }

    /// 确认用户可以查询该板块
    async fn ensure_query_board_permission(
        &self,
        user_id: &str,
        board_id: &str,
    ) -> Result<bool, ApiError> {
        let stu_course_codes = self.course_service.get_user_course_codes(user_id).await?;

        let board = self.board_service.parse_id(board_id)?;

        Ok(stu_course_codes.into_iter().any(|codes| {
            codes.0 == board.course.as_ref().unwrap().course_term
                && codes.1 == board.course.as_ref().unwrap().course_code
        }))
    }

    /// 确认用户可以编辑该帖子
    async fn ensure_edit_post_permission(
        &self,
        user_id: &str,
        post_id: i32,
    ) -> Result<bool, ApiError> {
        let post = Entity::find()
            .select_only()
            .column(Cols::PostSenderNo)
            .filter(Cols::PostId.eq(post_id))
            .one(self.db_conn.get_db())
            .await?
            .ok_or(ParameterError::InvalidParameter("无效的帖子id"))?;

        if user_id == post.post_sender_no {
            return Ok(true);
        }

        let post_user = student::Entity::find()
            .select_only()
            .column(student::Column::StuUserLevel)
            .filter(student::Column::StuNo.eq(post.post_sender_no))
            .one(self.db_conn.get_db())
            .await?
            .ok_or(ParameterError::InvalidParameter("无效的用户id"))?;

        let user = student::Entity::find()
            .select_only()
            .column(student::Column::StuUserLevel)
            .filter(student::Column::StuNo.eq(user_id))
            .one(self.db_conn.get_db())
            .await?
            .ok_or(ParameterError::InvalidParameter("无效的用户id"))?;

        Ok(user.stu_user_level.parse::<i32>().unwrap_or(0)
            < post_user.stu_user_level.parse::<i32>().unwrap_or(0))
    }

    /// 获取板块内的帖子
    async fn get_posts(
        &self,
        board_id: &str,
        tags: &str,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        page_size: u64,
        page_index: u64,
    ) -> Result<Vec<post::Model>, ApiError> {
        // 解析Tags
        let tag_names = self.resolve_tags(tags).await?;
        let tag_names_ref: Vec<_> = tag_names.iter().map(AsRef::as_ref).collect();

        // 计算Offset
        let offset = page_size * (page_index - 1);

        let board = self.board_service.parse_id(board_id)?;
        let course = board.course.as_ref().unwrap();
        match board.location {
            PostLocation::Weekly => self
                .post_repository
                .get_week_posts(
                    &course.course_term,
                    &course.course_code,
                    board.week,
                    tag_names_ref,
                    show_hidden,
                    with_content,
                    with_replies,
                    page_size,
                    offset,
                )
                .await
                .map_err(Into::into),
            PostLocation::Homework => self
                .post_repository
                .get_homework_posts(
                    &course.course_term,
                    &course.course_code,
                    board.homework.as_ref().unwrap().hw_id,
                    tag_names_ref,
                    show_hidden,
                    with_content,
                    with_replies,
                    page_size,
                    offset,
                )
                .await
                .map_err(Into::into),
            PostLocation::Course => self
                .post_repository
                .get_course_posts(
                    &course.course_term,
                    &course.course_code,
                    tag_names_ref,
                    show_hidden,
                    with_content,
                    with_replies,
                    page_size,
                    offset,
                )
                .await
                .map_err(Into::into),
            PostLocation::WeekSummary => self
                .post_repository
                .get_week_summary_posts(
                    &course.course_term,
                    &course.course_code,
                    board.week,
                    tag_names_ref,
                    show_hidden,
                    with_content,
                    with_replies,
                    page_size,
                    offset,
                )
                .await
                .map_err(Into::into),
            PostLocation::CourseSummary => self
                .post_repository
                .get_course_summary_posts(
                    &course.course_term,
                    &course.course_code,
                    tag_names_ref,
                    show_hidden,
                    with_content,
                    with_replies,
                    page_size,
                    offset,
                )
                .await
                .map_err(Into::into),
        }
    }

    /// 获取板块内的帖子数量
    async fn get_posts_count(
        &self,
        board_id: &str,
        tags: &str,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, ApiError> {
        // 解析Tags
        let tag_names = self.resolve_tags(tags).await?;
        let tag_names_ref: Vec<_> = tag_names.iter().map(AsRef::as_ref).collect();

        let board = self.board_service.parse_id(board_id)?;
        let course = board.course.as_ref().unwrap();

        match board.location {
            PostLocation::Weekly => self
                .post_repository
                .get_week_posts_count(
                    &course.course_term,
                    &course.course_code,
                    board.week,
                    tag_names_ref,
                    show_hidden,
                    with_replies,
                )
                .await
                .map_err(Into::into),
            PostLocation::Homework => self
                .post_repository
                .get_homework_posys_count(
                    &course.course_term,
                    &course.course_code,
                    board.homework.as_ref().unwrap().hw_id,
                    tag_names_ref,
                    show_hidden,
                    with_replies,
                )
                .await
                .map_err(Into::into),
            PostLocation::Course => self
                .post_repository
                .get_course_posts_count(
                    &course.course_term,
                    &course.course_code,
                    tag_names_ref,
                    show_hidden,
                    with_replies,
                )
                .await
                .map_err(Into::into),
            PostLocation::WeekSummary => self
                .post_repository
                .get_week_summary_posts_count(
                    &course.course_term,
                    &course.course_code,
                    board.week,
                    tag_names_ref,
                    show_hidden,
                    with_replies,
                )
                .await
                .map_err(Into::into),
            PostLocation::CourseSummary => self
                .post_repository
                .get_course_posts_count(
                    &course.course_term,
                    &course.course_code,
                    tag_names_ref,
                    show_hidden,
                    with_replies,
                )
                .await
                .map_err(Into::into),
        }
    }

    /// 添加帖子
    async fn add_post(
        &self,
        user_id: &str,
        ip_addr: &str,
        board_id: &str,
        title: &str,
        content: &str,
    ) -> Result<String, ApiError> {
        let board = self.board_service.parse_id_and_fetch(board_id).await?;

        let post_term = board.course.as_ref().unwrap().course_term.clone();
        let post_course_code = board.course.as_ref().unwrap().course_code.clone();

        let post_hw_id: i16;
        let post_week: i8;
        let post_chapter: i8;

        match board.location {
            PostLocation::Course => {
                if !self
                    .user_service
                    .guard_user_level(user_id, self.app_config.permission.admin)
                    .await?
                {
                    return Err(AuthError::PermissionDenied("权限不足，无法发帖。").into());
                }
                post_hw_id = -1;
                post_week = -1;
                post_chapter = -1;
            }
            PostLocation::Weekly => {
                post_hw_id = -1;
                post_week = board.week;
                post_chapter = -1;
            }
            PostLocation::Homework => {
                post_hw_id = board.homework.as_ref().unwrap().hw_id;
                post_week = board.week;
                post_chapter = board.homework.as_ref().unwrap().hw_chapter;
            }
            _ => {
                return Err(
                    AuthError::PermissionDenied("错误的传入参数，为保护系统不允许发帖。").into(),
                )
            }
        }

        let post_answer_id = None;
        let post_type = "Question".into();
        let post_sender_no = user_id.into();
        let post_priority = "0".into();

        let post_title = Some(title.into());
        let post_content = Some(content.into());
        let post_date = Local::now().naive_local();

        let post = post::Model {
            post_term,
            post_course_code,
            post_hw_id,
            post_week,
            post_chapter,
            post_answer_id,
            post_type,
            post_sender_no,
            post_priority,
            post_title,
            post_content,
            post_date,
            ..Default::default()
        };

        let post: post::ActiveModel = post.into();
        post.insert(self.db_conn.get_db()).await?;

        todo!()
    }

    /// 添加回复
    async fn add_reply(
        &self,
        user_id: &str,
        ip_addr: &str,
        father_post: i32,
        content: &str,
    ) -> Result<(), ApiError> {
        todo!()
    }

    /// 编辑帖子
    async fn edit_post(
        &self,
        user_id: &str,
        ip_addr: &str,
        post_id: i32,
        new_content: &str,
    ) -> Result<(), ApiError> {
        todo!()
    }

    /// 设置帖子标签
    async fn set_post_tag(
        &self,
        user_id: &str,
        ip_addr: &str,
        post_id: i32,
        tag: Vec<i32>,
    ) -> Result<(), ApiError> {
        todo!()
    }

    /// 设置帖子优先级
    async fn set_post_priority(
        &self,
        user_id: &str,
        ip_addr: &str,
        post_id: i32,
        priority: i32,
    ) -> Result<(), ApiError> {
        todo!()
    }

    /// 删除帖子
    async fn delete_post(
        &self,
        user_id: &str,
        ip_addr: &str,
        post_id: i32,
    ) -> Result<(), ApiError> {
        todo!()
    }

    /// 查询帖子，包括所有回帖及回帖的回帖
    async fn get_post(&self, post_id: i32, with_hidden: bool) -> Result<GetPostsResult, ApiError> {
        todo!()
    }

    /// 查询帖子的父帖子
    async fn get_parent_post(&self, post_id: i32) -> Result<i32, ApiError> {
        todo!()
    }
}
