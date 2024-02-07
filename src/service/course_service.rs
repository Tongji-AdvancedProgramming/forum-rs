use async_trait::async_trait;
use moka::future::{Cache, CacheBuilder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use std::sync::Arc;
use std::time::Duration;

use crate::config::database::{DatabaseTrait, Db};
use crate::config::AppConfig;
use crate::dto::course_tree::{CourseTree, Week};
use crate::entity::course::Model;
use crate::entity::student;
use crate::error::db_error::DbError;
use crate::repository::user_repo::{UserRepository, UserRepositoryTrait};
use crate::{dto::course_tree, entity::course};

#[async_trait]
pub trait CourseServiceTrait {
    /// 获取用户有权查看的课程代码列表
    async fn get_user_course_codes(&self, user_id: &str) -> Result<Vec<(String, String)>, DbError>;

    /// 获取用户有权查看的课程列表
    async fn get_user_courses(&self, user_id: &str) -> Result<Vec<(String, String)>, DbError>;

    /// 获取用户有权查看的课程详情
    async fn get_user_course_detail(&self, user_id: &str) -> Result<Vec<course::Model>, DbError>;

    /// 获取用户有权查看的课程及其以树形式的详细信息
    async fn get_user_courses_tree(&self, user_id: &str) -> Result<CourseTree, DbError>;

    /// 获取某门课程的数据
    async fn get_courses(
        &self,
        keys: &Vec<(String, String)>,
    ) -> Result<Vec<course::Model>, DbError>;

    /// 获取某门课程的周次数据
    async fn get_weeks(
        &self,
        term: &str,
        courses_code: &str,
    ) -> Result<Vec<course_tree::Week>, DbError>;
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct CourseService {
    course_cache: Cache<(String, String), course::Model>,
    week_cache: Cache<(String, String), course_tree::Week>,
    user_course_cache: Cache<String, Vec<(String, String)>>,
    db_conn: Arc<Db>,
    user_repo: UserRepository,
    app_config: Arc<AppConfig>,
}

#[allow(dead_code)]
impl CourseService {
    pub fn new(db_conn: &Arc<Db>, app_config: &Arc<AppConfig>) -> Self {
        let course_cache = CacheBuilder::new(20)
            .time_to_live(Duration::from_secs(60 * 60))
            .build();
        let week_cache = CacheBuilder::new(50)
            .time_to_live(Duration::from_secs(60 * 3))
            .build();
        let user_course_cache = CacheBuilder::new(100)
            .time_to_live(Duration::from_secs(60 * 60 * 24))
            .build();
        let user_repo = UserRepository::new(db_conn);
        let app_config = Arc::clone(app_config);
        let db_conn = Arc::clone(db_conn);

        Self {
            course_cache,
            week_cache,
            user_course_cache,
            db_conn,
            user_repo,
            app_config,
        }
    }

    fn get_student_courses_from_entity(stu: &student::Model) -> Vec<String> {
        let mut result = vec![];

        if stu.stu_cno_1.is_some() && stu.stu_cno_1_is_del == "0" {
            result.push(stu.stu_cno_1.as_ref().unwrap().clone());
        }
        if stu.stu_cno_2.is_some() && stu.stu_cno_2_is_del == "0" {
            result.push(stu.stu_cno_2.as_ref().unwrap().clone());
        }
        if stu.stu_cno_3.is_some() && stu.stu_cno_3_is_del == "0" {
            result.push(stu.stu_cno_3.as_ref().unwrap().clone());
        }

        result
    }
}

#[async_trait]
impl CourseServiceTrait for CourseService {
    async fn get_user_course_codes(&self, user_id: &str) -> Result<Vec<(String, String)>, DbError> {
        let stu: Option<student::Model> = self.user_repo.select_courses(user_id).await;
        if stu.is_none() {
            return Ok(vec![]);
        }
        let stu = stu.unwrap();

        if stu.stu_user_level.parse::<i32>().unwrap() >= self.app_config.permission.ta {
            // 用户是管理员或更高，允许访问所有课程
            let courses = course::Entity::find()
                .group_by(course::Column::CourseCode)
                .columns([course::Column::CourseTerm, course::Column::CourseCode])
                .all(self.db_conn.get_db())
                .await?;
            Ok(courses
                .iter()
                .map(|c| (c.course_term.clone(), c.course_code.clone()))
                .collect())
        } else {
            let result = Self::get_student_courses_from_entity(&stu);

            let courses = course::Entity::find()
                .filter(course::Column::CourseTerm.eq(&stu.stu_term))
                .filter(course::Column::CourseNo.is_in(&result))
                .columns([course::Column::CourseTerm, course::Column::CourseCode])
                .all(self.db_conn.get_db())
                .await?;

            Ok(courses
                .iter()
                .map(|c| (c.course_term.clone(), c.course_code.clone()))
                .collect())
        }
    }

    async fn get_user_courses(&self, user_id: &str) -> Result<Vec<(String, String)>, DbError> {
        if self.user_course_cache.contains_key(user_id) {
            Ok(self.user_course_cache.get(user_id).await.unwrap().clone())
        } else {
            let result: Result<Vec<(String, String)>, DbError> = {
                let stu: Option<student::Model> = self.user_repo.select_courses(user_id).await;
                if stu.is_none() {
                    return Ok(vec![]);
                }
                let stu = stu.unwrap();

                if stu.stu_user_level.parse::<i32>().unwrap() >= self.app_config.permission.ta {
                    // 用户是管理员或更高，允许访问所有课程
                    let courses = course::Entity::find()
                        .columns([course::Column::CourseTerm, course::Column::CourseNo])
                        .all(self.db_conn.get_db())
                        .await?;
                    Ok(courses
                        .iter()
                        .map(|c| (c.course_term.clone(), c.course_no.clone()))
                        .collect())
                } else {
                    let result = Self::get_student_courses_from_entity(&stu);

                    Ok(result
                        .iter()
                        .map(|c| (stu.stu_term.clone(), c.clone()))
                        .collect())
                }
            };

            if result.is_ok() {
                self.user_course_cache
                    .insert(user_id.to_string(), result.as_ref().unwrap().clone())
                    .await;
            }
            result
        }
    }

    async fn get_user_course_detail(&self, _user_id: &str) -> Result<Vec<Model>, DbError> {
        todo!()
    }

    async fn get_user_courses_tree(&self, _user_id: &str) -> Result<CourseTree, DbError> {
        todo!()
    }

    async fn get_courses(&self, _keys: &Vec<(String, String)>) -> Result<Vec<Model>, DbError> {
        todo!()
    }

    async fn get_weeks(&self, _term: &str, _courses_code: &str) -> Result<Vec<Week>, DbError> {
        todo!()
    }
}
