use std::{sync::Arc, vec};

use async_trait::async_trait;
use sea_orm::{
    ColumnTrait, Condition, DbBackend, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Select, Statement,
};

use crate::{
    config::database::{DatabaseTrait, Db},
    entity::post::{Column as Col, Entity, Model as Post},
};

#[async_trait]
pub trait PostRepositoryTrait {
    type Error: std::error::Error + Send + Sync + 'static;

    /// 获取课程整体的帖子
    async fn get_course_posts(
        &self,
        term: &str,
        course_code: &str,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error>;

    /// 获取某周的整体帖子
    async fn get_week_posts(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error>;

    /// 获取某次作业的帖子
    async fn get_homework_posts(
        &self,
        term: &str,
        course_code: &str,
        homework_id: i16,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error>;

    /// 获取某课程所有帖子（汇总）
    async fn get_course_summary_posts(
        &self,
        term: &str,
        course_code: &str,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error>;

    /// 获取某周所有帖子（汇总）
    async fn get_week_summary_posts(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error>;

    /// 获取课程整体的帖子数量
    async fn get_course_posts_count(
        &self,
        term: &str,
        course_code: &str,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error>;

    /// 获取某周的整体帖子数量
    async fn get_week_posts_count(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error>;

    /// 获取某次作业的帖子数量
    async fn get_homework_posys_count(
        &self,
        term: &str,
        course_code: &str,
        homework_id: i16,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error>;

    /// 获取某课程所有帖子（汇总）数量
    async fn get_course_summary_posts_count(
        &self,
        term: &str,
        course_code: &str,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error>;

    /// 获取某周所有帖子（汇总）数量
    async fn get_week_summary_posts_count(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error>;

    /// 获取帖子但不包含内容
    async fn get_post_without_content(&self, post_id: i32) -> Result<Option<Post>, Self::Error>;

    /// 递归查询某个帖子旗下的子帖子
    async fn get_posts_recursively(&self, post_id: i32) -> Result<Vec<Post>, Self::Error>;

    /// 递归查询某个帖子的父帖子
    async fn get_parent_post_recursively(&self, post_id: i32) -> Result<Option<Post>, Self::Error>;
}

#[derive(Clone)]
pub struct PostRepository {
    db: Arc<Db>,
}

impl PostRepository {
    pub fn new(db: &Arc<Db>) -> Self {
        Self { db: Arc::clone(db) }
    }

    fn select_head(with_content: bool) -> Select<Entity> {
        Entity::find().select_only().columns({
            let mut cols = vec![
                Col::PostId,
                Col::PostTerm,
                Col::PostCourseCode,
                Col::PostHwId,
                Col::PostWeek,
                Col::PostChapter,
                Col::PostAnswerId,
                Col::PostType,
                Col::PostSenderNo,
                Col::PostPriority,
                Col::PostTag01,
                Col::PostTag02,
                Col::PostTag03,
                Col::PostTag04,
                Col::PostTag05,
                Col::PostTag06,
                Col::PostTag07,
                Col::PostTag08,
                Col::PostTag09,
                Col::PostTag10,
                Col::PostTitle,
                Col::PostDate,
                Col::PostIsDel,
                Col::PostComment,
            ];
            if with_content {
                cols.push(Col::PostContent)
            }
            cols
        })
    }

    fn select_filter(tag_names: Vec<&str>, show_hidden: bool, with_repies: bool) -> Condition {
        let mut condition = Condition::all();
        if !tag_names.is_empty() {
            for tag_name in tag_names {
                if let Ok(tag_col) = tag_name.parse::<Col>() {
                    condition = condition.add(tag_col.eq("1"))
                }
            }
        }
        if !show_hidden {
            condition = condition.add(Col::PostIsDel.eq("0"))
        }
        if !with_repies {
            condition = condition.add(Col::PostAnswerId.is_null())
        }

        condition
    }
}

#[async_trait]
impl PostRepositoryTrait for PostRepository {
    type Error = sea_orm::DbErr;

    /// 获取课程整体的帖子
    async fn get_course_posts(
        &self,
        term: &str,
        course_code: &str,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error> {
        Self::select_head(with_content)
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Col::PostHwId.eq(-1))
            .filter(Col::PostChapter.eq(-1))
            .filter(Col::PostWeek.eq(-1))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .order_by(Col::PostPriority, Order::Desc)
            .order_by(Col::PostId, Order::Desc)
            .offset(offset)
            .limit(limit)
            .all(self.db.get_db())
            .await
    }

    /// 获取某周的整体帖子
    async fn get_week_posts(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error> {
        Self::select_head(with_content)
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Col::PostHwId.eq(-1))
            .filter(Col::PostWeek.eq(week))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .order_by(Col::PostPriority, Order::Desc)
            .order_by(Col::PostId, Order::Desc)
            .offset(offset)
            .limit(limit)
            .all(self.db.get_db())
            .await
    }

    /// 获取某次作业的帖子
    async fn get_homework_posts(
        &self,
        term: &str,
        course_code: &str,
        homework_id: i16,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error> {
        Self::select_head(with_content)
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Col::PostHwId.eq(homework_id))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .order_by(Col::PostPriority, Order::Desc)
            .order_by(Col::PostId, Order::Desc)
            .offset(offset)
            .limit(limit)
            .all(self.db.get_db())
            .await
    }

    /// 获取某课程所有帖子（汇总）
    async fn get_course_summary_posts(
        &self,
        term: &str,
        course_code: &str,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error> {
        Self::select_head(with_content)
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .order_by(Col::PostPriority, Order::Desc)
            .order_by(Col::PostId, Order::Desc)
            .offset(offset)
            .limit(limit)
            .all(self.db.get_db())
            .await
    }

    /// 获取某周所有帖子（汇总）
    async fn get_week_summary_posts(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_content: bool,
        with_replies: bool,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Post>, Self::Error> {
        Self::select_head(with_content)
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Col::PostWeek.eq(week))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .order_by(Col::PostPriority, Order::Desc)
            .order_by(Col::PostId, Order::Desc)
            .offset(offset)
            .limit(limit)
            .all(self.db.get_db())
            .await
    }

    /// 获取课程整体的帖子数量
    async fn get_course_posts_count(
        &self,
        term: &str,
        course_code: &str,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error> {
        Entity::find()
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Col::PostHwId.eq(-1))
            .filter(Col::PostChapter.eq(-1))
            .filter(Col::PostWeek.eq(-1))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .count(self.db.get_db())
            .await
    }

    /// 获取某周的整体帖子数量
    async fn get_week_posts_count(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error> {
        Entity::find()
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Col::PostHwId.eq(-1))
            .filter(Col::PostWeek.eq(week))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .count(self.db.get_db())
            .await
    }

    /// 获取某次作业的帖子数量
    async fn get_homework_posys_count(
        &self,
        term: &str,
        course_code: &str,
        homework_id: i16,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error> {
        Entity::find()
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Col::PostHwId.eq(homework_id))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .count(self.db.get_db())
            .await
    }

    /// 获取某课程所有帖子（汇总）数量
    async fn get_course_summary_posts_count(
        &self,
        term: &str,
        course_code: &str,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error> {
        Entity::find()
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .count(self.db.get_db())
            .await
    }

    /// 获取某周所有帖子（汇总）数量
    async fn get_week_summary_posts_count(
        &self,
        term: &str,
        course_code: &str,
        week: i8,
        tag_names: Vec<&str>,
        show_hidden: bool,
        with_replies: bool,
    ) -> Result<u64, Self::Error> {
        Entity::find()
            .filter(Col::PostTerm.eq(term))
            .filter(Col::PostCourseCode.eq(course_code))
            .filter(Col::PostWeek.eq(week))
            .filter(Self::select_filter(tag_names, show_hidden, with_replies))
            .count(self.db.get_db())
            .await
    }

    /// 获取帖子但不包含内容
    async fn get_post_without_content(&self, post_id: i32) -> Result<Option<Post>, Self::Error> {
        Self::select_head(false)
            .filter(Col::PostId.eq(post_id))
            .one(self.db.get_db())
            .await
    }

    /// 递归查询某个帖子旗下的子帖子
    async fn get_posts_recursively(&self, post_id: i32) -> Result<Vec<Post>, Self::Error> {
        let sql = r#"
        with recursive tree_post as (select *
                    from post
                    where post_id = ?
                    union all
                    select post.*
                    from post
                            join tree_post on post.post_answer_id = tree_post.post_id)
        select *
        from tree_post
        order by post_date
        "#;

        Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::MySql,
                sql,
                [post_id.into()],
            ))
            .all(self.db.get_db())
            .await
    }

    /// 递归查询某个帖子的父帖子
    async fn get_parent_post_recursively(&self, post_id: i32) -> Result<Option<Post>, Self::Error> {
        let sql = r#"
        with recursive tree_post as (select post_id, post_answer_id
                    from post
                    where post_id = ?
                    union all
                    select post.post_id, post.post_answer_id
                    from post
                            join tree_post on post.post_id = tree_post.post_answer_id)
        select post_id
        from tree_post
        where post_answer_id is null
        limit 1
        "#;

        Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::MySql,
                sql,
                [post_id.into()],
            ))
            .one(self.db.get_db())
            .await
    }
}
