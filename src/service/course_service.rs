use async_trait::async_trait;
use futures::future::ready;
use futures::{stream, StreamExt};
use moka::future::{Cache, CacheBuilder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::vec;

use crate::config::database::{DatabaseTrait, Db};
use crate::config::AppConfig;
use crate::dto::course_tree::{CourseTree, Week};
use crate::entity::course::Model;
use crate::entity::{homework, student};
use crate::error::db_error::DbError;
use crate::repository::course_repo::{CourseRepository, CourseRepositoryTrait};
use crate::repository::user_repo::{UserRepository, UserRepositoryTrait};
use crate::{dto::course_tree, entity::course};

#[async_trait]
pub trait CourseServiceTrait {
    /// 获取用户有权查看的课程代码列表
    async fn get_user_course_codes(&self, user_id: &str) -> Result<Vec<(String, String)>, DbError>;

    /// 获取用户有权查看的课程列表
    async fn get_user_courses(&self, user_id: &str) -> Result<Vec<(String, String)>, DbError>;

    /// 获取用户有权查看的课程详情
    async fn get_user_courses_detail(&self, user_id: &str) -> Result<Vec<course::Model>, DbError>;

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
    week_cache: Cache<(String, String), Vec<course_tree::Week>>,
    user_course_cache: Cache<String, Vec<(String, String)>>,
    db_conn: Arc<Db>,
    course_repo: CourseRepository,
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
        let course_repo = CourseRepository::new(db_conn);
        let user_repo = UserRepository::new(db_conn);
        let app_config = Arc::clone(app_config);
        let db_conn = Arc::clone(db_conn);

        Self {
            course_cache,
            week_cache,
            user_course_cache,
            db_conn,
            course_repo,
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

    async fn get_weeks_loads(&self, term: &str, course_code: &str) -> Result<Vec<Week>, DbError> {
        // 获取原始数据
        let data = homework::Entity::find()
            .filter(homework::Column::HwTerm.eq(term))
            .filter(homework::Column::HwCourseCode.eq(course_code))
            .all(self.db_conn.get_db())
            .await?;

        // 加工数据
        let week_homework_map: HashMap<_, Vec<_>> =
            data.into_iter().fold(HashMap::new(), |mut acc, hw| {
                acc.entry(hw.hw_week).or_insert_with(Vec::new).push(hw);
                acc
            });

        // 构造返回数据
        let result = week_homework_map
            .into_iter()
            .map(|(week_index, homework)| {
                let chapters: Vec<_> = homework
                    .iter()
                    .map(|hw| hw.hw_chapter)
                    .filter(|c| c < &20)
                    .map(|c| c.to_string())
                    .collect();

                let mut week_title = format!("第{}周", week_index);
                if !chapters.is_empty() {
                    week_title = format!("{} - 第{}章", week_title, chapters.join(","))
                }

                Week {
                    number: 1,
                    homeworks: homework,
                    content: week_title,
                }
            })
            .collect();

        Ok(result)
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

    async fn get_user_courses_detail(&self, user_id: &str) -> Result<Vec<Model>, DbError> {
        let keys = self.get_user_courses(user_id).await?;

        self.course_repo.get_all_course_detail(keys).await
    }

    async fn get_courses(&self, keys: &Vec<(String, String)>) -> Result<Vec<Model>, DbError> {
        let mut result = vec![];

        for key in keys {
            if !self.course_cache.contains_key(key) {
                let course = self.course_repo.get_course_detail(key).await?;
                if course.is_some() {
                    self.course_cache
                        .insert(key.clone(), course.as_ref().unwrap().clone())
                        .await;
                    result.push(course.unwrap());
                }
            } else {
                let course = self.course_cache.get(key).await.unwrap();
                result.push(course);
            }
        }

        Ok(result)
    }

    async fn get_user_courses_tree(&self, user_id: &str) -> Result<CourseTree, DbError> {
        // 获取所有可以访问的课程
        let course_keys = self.get_user_courses(user_id).await?;
        // 获取完整的可访问的课程列表
        let courses = self.get_courses(&course_keys).await?;

        // 将课程按照课程代码分类
        let course_code_map: HashMap<_, Vec<_>> =
            courses.into_iter().fold(HashMap::new(), |mut acc, course| {
                acc.entry((course.course_no.clone(), course.course_code.clone()))
                    .or_insert_with(Vec::new)
                    .push(course);
                acc
            });

        // 构造树形结构
        let result = CourseTree {
            courses: {
                let courses_stream = stream::iter(course_code_map)
                    .filter(|(_, v)| ready(!v.is_empty()))
                    .then(|(_, v)| async move {
                        let course = course::Model {
                            course_short_name: "".into(),
                            course_no: "".into(),
                            ..v.first().cloned().unwrap()
                        };

                        let mut course: course_tree::Course = course.into();
                        course.weeks = self
                            .get_weeks(&course._course.course_term, &course._course.course_code)
                            .await?;
                        Ok(course)
                    })
                    .collect::<Vec<Result<course_tree::Course, DbError>>>()
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;

                courses_stream
            },
        };

        Ok(result)
    }

    async fn get_weeks(&self, term: &str, courses_code: &str) -> Result<Vec<Week>, DbError> {
        if self
            .week_cache
            .contains_key(&(term.to_string(), courses_code.to_string()))
        {
            Ok(self
                .week_cache
                .get(&(term.into(), courses_code.into()))
                .await
                .unwrap()
                .clone())
        } else {
            let result = self.get_weeks_loads(term, courses_code).await;
            if result.is_ok() {
                self.week_cache
                    .insert(
                        (term.into(), courses_code.into()),
                        result.as_ref().unwrap().clone(),
                    )
                    .await;
            }
            result
        }
    }
}
