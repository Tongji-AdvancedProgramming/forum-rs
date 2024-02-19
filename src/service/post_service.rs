use std::{net::IpAddr, sync::Arc};

use async_trait::async_trait;
use chrono::Local;
use forum_utils::html_cleaner::HtmlCleaner;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, NotSet, QueryFilter, QuerySelect,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::entity::notification;
use crate::error::param_error::ParameterError::InvalidParameter;
use crate::service::notification_service::{NotificationService, NotificationServiceTrait};
use crate::utils::string_utils::StringUtilsExt;
use crate::{
    config::{
        database::{DatabaseTrait, Db},
        meili::Meili,
        AppConfig,
    },
    dto::board::PostLocation,
    entity::{
        post::{self, Column as Cols, Entity},
        student,
    },
    error::{api_error::ApiError, auth_error::AuthError, param_error::ParameterError},
    repository::post_repo::{PostRepository, PostRepositoryTrait},
    service::{
        board_service::BoardServiceTrait, log_service::LogServiceTrait,
        search_engine_service::SearchEngineServiceTrait,
    },
};

use super::{
    board_service::BoardService,
    course_service::{CourseService, CourseServiceTrait},
    log_service::LogService,
    metadata_service::{MetadataService, MetadataServiceTrait},
    search_engine_service::SearchEngineService,
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

    /// 确认用户可以编辑这些帖子
    async fn ensure_edit_posts_permission(
        &self,
        user_id: &str,
        post_ids: &Vec<i32>,
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
        ip_addr: &IpAddr,
        board_id: &str,
        title: &str,
        content: &str,
    ) -> Result<i32, ApiError>;

    /// 添加回复
    async fn add_reply(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        father_post: i32,
        content: &str,
    ) -> Result<(), ApiError>;

    /// 编辑帖子
    async fn edit_post(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        post_id: i32,
        new_content: &str,
    ) -> Result<(), ApiError>;

    /// 设置帖子标签
    async fn set_post_tag(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        post_id: i32,
        tag: &Vec<i32>,
    ) -> Result<(), ApiError>;

    /// 设置帖子优先级
    async fn set_post_priority(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        post_id: i32,
        priority: i32,
    ) -> Result<(), ApiError>;

    /// 删除帖子
    async fn delete_post(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        post_id: i32,
    ) -> Result<(), ApiError>;

    /// 查询帖子，包括所有回帖及回帖的回帖
    async fn get_post(&self, post_id: i32, with_hidden: bool) -> Result<GetPostsResult, ApiError>;

    /// 查询帖子的父帖子
    async fn get_parent_post(&self, post_id: i32) -> Result<Option<i32>, ApiError>;
}

#[derive(Clone)]
pub struct PostService {
    pub db_conn: Arc<Db>,
    pub app_config: Arc<AppConfig>,
    pub metadata_service: MetadataService,
    pub user_service: UserService,
    pub course_service: CourseService,
    pub board_service: BoardService,
    pub search_engine_service: SearchEngineService,
    pub notification_service: NotificationService,
    pub log_service: LogService,
    pub post_repository: PostRepository,
}

impl PostService {
    pub fn new(db_conn: &Arc<Db>, app_config: &Arc<AppConfig>, meili_client: &Arc<Meili>) -> Self {
        PostService {
            db_conn: Arc::clone(db_conn),
            app_config: Arc::clone(app_config),
            metadata_service: MetadataService::new(db_conn),
            user_service: UserService::new(db_conn),
            course_service: CourseService::new(db_conn, app_config),
            board_service: BoardService::new(db_conn),
            search_engine_service: SearchEngineService::new(meili_client, db_conn, app_config),
            notification_service: NotificationService::new(db_conn),
            log_service: LogService::new(db_conn),
            post_repository: PostRepository::new(db_conn),
        }
    }

    pub async fn resolve_tags(&self, tags: &str) -> Result<Vec<String>, ApiError> {
        let error = ApiError::ParameterError(InvalidParameter("无效的tag传入"));
        if tags == "[]" {
            return Ok(vec![]);
        }

        let tag_list = self.metadata_service.get_tags().await?;
        let tag_indexes: Vec<usize> = serde_json::from_str(tags).map_err(|_| error)?;
        let tag_indexes: Vec<_> = tag_indexes
            .into_iter()
            .filter(|i| *i < tag_list.len())
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
        let post_level = self
            .post_repository
            .get_post_sender_user_level(post_id)
            .await?
            .ok_or(InvalidParameter("无效的帖子Id"))?;

        let user = student::Entity::find()
            .select_only()
            .column(student::Column::StuUserLevel)
            .filter(student::Column::StuNo.eq(user_id))
            .into_json()
            .one(self.db_conn.get_db())
            .await?
            .map(|v| serde_json::from_value::<student::Model>(v).unwrap())
            .ok_or(ParameterError::InvalidParameter("无效的用户id"))?;

        Ok(user.stu_user_level.parse::<i64>().unwrap_or(0) < post_level)
    }

    async fn ensure_edit_posts_permission(
        &self,
        user_id: &str,
        post_ids: &Vec<i32>,
    ) -> Result<bool, ApiError> {
        let post_level = self
            .post_repository
            .get_posts_sender_max_user_level(post_ids)
            .await?
            .ok_or(InvalidParameter("无效的帖子Id"))?;

        let user = student::Entity::find()
            .select_only()
            .column(student::Column::StuUserLevel)
            .filter(student::Column::StuNo.eq(user_id))
            .into_json()
            .one(self.db_conn.get_db())
            .await
            .map(|v| v.map(|v| serde_json::from_value::<student::Model>(v).unwrap()))?
            .ok_or(ParameterError::InvalidParameter("无效的用户id"))?;

        Ok(user.stu_user_level.parse::<i64>().unwrap_or(0) < post_level)
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
        ip_addr: &IpAddr,
        board_id: &str,
        title: &str,
        content: &str,
    ) -> Result<i32, ApiError> {
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

        let post = post::ActiveModel {
            post_id: NotSet,
            ..post.into_active_model()
        };
        let post = post.insert(self.db_conn.get_db()).await?;

        // 添加到搜索引擎
        self.search_engine_service.add_post(post.post_id).await?;

        // 记录日志
        let comment = "POST 发表帖子";
        self.log_service
            .log_post(post.post_id, user_id, ip_addr, comment)
            .await;

        Ok(post.post_id)
    }

    /// 添加回复
    async fn add_reply(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        father_post_id: i32,
        content: &str,
    ) -> Result<(), ApiError> {
        let father_post = self
            .post_repository
            .get_post_without_content(father_post_id)
            .await?
            .ok_or(InvalidParameter("不存在的回帖对象"))?;

        let post_term = father_post.post_term;
        let post_course_code = father_post.post_course_code;
        let post_hw_id = father_post.post_hw_id;
        let post_week = father_post.post_week;
        let post_chapter = father_post.post_chapter;

        let post_title = None;
        let post_sender_no = user_id.into();
        let post_answer_id = Some(father_post_id);
        let post_content = Some(content.into());
        let post_date = Local::now().naive_local();

        let post_type: String;
        if user_id == father_post.post_sender_no {
            post_type = "Answer".into();
        } else {
            post_type = "QuestionsAdditional".into();
        }

        let new_post = post::Model {
            post_term,
            post_course_code,
            post_hw_id,
            post_week,
            post_chapter,
            post_answer_id,
            post_type,
            post_sender_no,
            post_title,
            post_content,
            post_date,
            ..Default::default()
        };
        let new_post = post::ActiveModel {
            post_id: NotSet,
            ..new_post.into_active_model()
        };

        let new_post = new_post.insert(self.db_conn.get_db()).await?;
        self.search_engine_service
            .add_post(new_post.post_id)
            .await?;

        // 记录日志
        let comment = format!("REPLY 回复{}", father_post_id);
        self.log_service
            .log_post(new_post.post_id, user_id, ip_addr, &comment)
            .await;

        // 发送通知
        if father_post.post_sender_no != user_id {
            let ntf_title = "收到新回复".to_string();
            let ntf_content = format!(
                "{}",
                HtmlCleaner::html_to_text(new_post.post_content.as_ref().unwrap()).abbreviate(35)
            );
            let ntf_type = "REPLY".to_string();
            let ntf_receiver = father_post.post_sender_no.clone();

            let notification = notification::Model {
                ntf_id: 0,
                ntf_type,
                ntf_title,
                ntf_content,
                ntf_receiver,
                ntf_datetime: Default::default(),
                ntf_read: false,
            };

            self.notification_service
                .send_notification(notification)
                .await?;
        }

        Ok(())
    }

    /// 编辑帖子
    async fn edit_post(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        post_id: i32,
        new_content: &str,
    ) -> Result<(), ApiError> {
        let mut post = Entity::find_by_id(post_id)
            .one(self.db_conn.get_db())
            .await?
            .ok_or(InvalidParameter("帖子不存在"))?
            .into_active_model();

        post.post_content = Set(Some(new_content.to_string()));
        post.save(self.db_conn.get_db()).await?;

        // 记录日志
        let comment = "EDIT 进行了编辑";
        self.log_service
            .log_post(post_id, user_id, ip_addr, comment)
            .await;

        self.search_engine_service.add_post(post_id).await?;

        Ok(())
    }

    /// 设置帖子标签
    async fn set_post_tag(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        post_id: i32,
        tag: &Vec<i32>,
    ) -> Result<(), ApiError> {
        let mut post = Entity::find_by_id(post_id)
            .one(self.db_conn.get_db())
            .await?
            .ok_or(InvalidParameter("帖子不存在"))?
            .into_active_model();

        let mut tags_ref = [
            &mut post.post_tag_01,
            &mut post.post_tag_02,
            &mut post.post_tag_03,
            &mut post.post_tag_04,
            &mut post.post_tag_05,
            &mut post.post_tag_06,
            &mut post.post_tag_07,
            &mut post.post_tag_08,
            &mut post.post_tag_09,
            &mut post.post_tag_10,
        ];
        let tags_len = tags_ref.len();

        tags_ref
            .iter_mut()
            .for_each(|tag_ref| **tag_ref = Set("0".to_string()));
        tag.iter()
            .filter(|&&t| t >= 0 && t < (tags_len as i32).clone())
            .for_each(|&i| *tags_ref[i as usize] = Set("1".to_string()));

        post.save(self.db_conn.get_db()).await?;

        // 记录日志
        let comment = format!("TAG 设置了新标签: {:?}", tag);
        self.log_service
            .log_post(post_id, user_id, ip_addr, &comment)
            .await;

        Ok(())
    }

    /// 设置帖子优先级
    async fn set_post_priority(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        post_id: i32,
        priority: i32,
    ) -> Result<(), ApiError> {
        let mut post = Entity::find_by_id(post_id)
            .one(self.db_conn.get_db())
            .await?
            .ok_or(InvalidParameter("帖子不存在"))?
            .into_active_model();

        post.post_priority = Set(priority.to_string());
        post.save(self.db_conn.get_db()).await?;

        // 记录日志
        let comment = format!("PRIORITY 新优先级为：{}", priority);
        self.log_service
            .log_post(post_id, user_id, ip_addr, &comment)
            .await;

        Ok(())
    }

    /// 删除帖子
    async fn delete_post(
        &self,
        user_id: &str,
        ip_addr: &IpAddr,
        post_id: i32,
    ) -> Result<(), ApiError> {
        let res = Entity::delete_by_id(post_id)
            .exec(self.db_conn.get_db())
            .await?;

        if res.rows_affected > 0 {
            // 记录日志
            let comment = "DELETE";
            self.log_service
                .log_post(post_id, user_id, ip_addr, comment)
                .await;
        }

        Ok(())
    }

    /// 查询帖子，包括所有回帖及回帖的回帖
    async fn get_post(&self, post_id: i32, with_hidden: bool) -> Result<GetPostsResult, ApiError> {
        let mut posts = self.post_repository.get_posts_recursively(post_id).await?;

        if !with_hidden {
            posts = posts.into_iter().filter(|p| p.post_is_del == "0").collect();
        }

        Ok(GetPostsResult { posts })
    }

    /// 查询帖子的父帖子
    async fn get_parent_post(&self, post_id: i32) -> Result<Option<i32>, ApiError> {
        self.post_repository
            .get_parent_post_recursively(post_id)
            .await
            .map(|p| p.map(|p| p.post_id))
            .map_err(Into::into)
    }
}
