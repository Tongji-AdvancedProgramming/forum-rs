use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    config::database::{DatabaseTrait, Db},
    dto::board::{Board, PostLocation},
    entity::{course, homework},
    error::{api_error::ApiError, param_error::ParameterError},
    utils::string_utils::StringUtilsExt,
};

#[derive(Clone)]
pub struct BoardService {
    db_conn: Arc<Db>,
}

impl BoardService {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
}

#[async_trait]
pub trait BoardServiceTrait {
    /// 解析Board Id（但不获取数据）
    fn parse_id(&self, id: &str) -> Result<Board, ParameterError>;

    /// 解析Board Id，并获取数据
    async fn parse_id_and_fetch(&self, id: &str) -> Result<Board, ApiError>;
}

#[async_trait]
impl BoardServiceTrait for BoardService {
    fn parse_id(&self, id: &str) -> Result<Board, ParameterError> {
        // 首先切分ID
        let tokens: Vec<&str> = id.split("_").collect();
        if tokens.len() < 2 || tokens.len() > 4 {
            return Err(ParameterError::InvalidParameter(
                "传入的ID格式不正确：组成部分不为2、3或4.".into(),
            ));
        }

        // 识别学期和课程
        let term = tokens[0];
        let course_code = tokens[1];
        if !term.is_numeric() || !course_code.is_numeric() {
            return Err(ParameterError::InvalidParameter(
                "传入的ID格式不正确：学期或课程代码不是纯数字".into(),
            ));
        }

        // 识别板块位置
        let mut location = PostLocation::Homework;
        if tokens.len() == 2 {
            location = PostLocation::CourseSummary;
        } else if tokens.len() == 3 {
            if tokens[2] == "general" {
                location = PostLocation::Course;
            } else {
                location = PostLocation::Weekly;
            }
        } else if tokens[3] == "p" {
            location = PostLocation::WeekSummary;
        }

        // 识别周次
        let mut week = 0;
        if location != PostLocation::Course && location != PostLocation::CourseSummary {
            let week_str = &tokens[2][1..];
            if !tokens[2].starts_with("w") || !week_str.is_numeric() {
                return Err(ParameterError::InvalidParameter(
                    "传入的ID格式不正确：周次格式不正确".into(),
                ));
            }
            week = week_str.parse().unwrap();
        }

        // 识别所属作业
        let mut hw_id = None;
        if location == PostLocation::Homework {
            hw_id = Some(String::from(tokens[3]));
        }

        // 制作成Board结构体
        let course = Some(course::Model {
            course_term: term.into(),
            course_code: course_code.into(),
            ..Default::default()
        });

        let homework = hw_id.map(|hw_id| homework::Model {
            hw_term: term.into(),
            hw_course_code: course_code.into(),
            hw_id: hw_id.parse().unwrap(),
            ..Default::default()
        });

        Ok(Board {
            id: id.into(),
            course,
            location,
            week,
            homework,
        })
    }

    async fn parse_id_and_fetch(&self, id: &str) -> Result<Board, ApiError> {
        let board = self.parse_id(id)?;

        // 获取课程信息
        let ref cr = board.course.unwrap();
        let course = course::Entity::find()
            .filter(course::Column::CourseTerm.eq(&cr.course_term))
            .filter(course::Column::CourseCode.eq(&cr.course_code))
            .one(self.db_conn.get_db())
            .await?;

        // 获取作业信息
        let homework = if board.homework.is_some() {
            let ref hw = board.homework.unwrap();
            homework::Entity::find()
                .filter(homework::Column::HwTerm.eq(&hw.hw_term))
                .filter(homework::Column::HwCourseCode.eq(&hw.hw_course_code))
                .filter(homework::Column::HwId.eq(hw.hw_id))
                .one(self.db_conn.get_db())
                .await?
        } else {
            None
        };

        Ok(Board {
            course,
            homework,
            ..board
        })
    }
}
