use async_trait::async_trait;

use crate::{dto::course_tree, entity::course};

#[async_trait]
pub trait CourseService {
    /// 获取用户有权查看的课程代码列表
    async fn get_user_course_codes(user_id: &str) -> Vec<Vec<String>>;

    /// 获取用户有权查看的课程列表
    async fn get_user_courses(user_id: &str) -> Vec<Vec<String>>;

    /// 获取用户有权查看的课程详情
    async fn get_user_course_detail(user_id: &str) -> Vec<course::Model>;

    /// 获取用户有权查看的课程及其以树形式的详细信息
    async fn get_user_courses_tree(user_id: &str) -> course_tree::CourseTree;

    /// 获取某门课程的数据
    async fn get_courses(keys: &Vec<Vec<String>>) -> Vec<course::Model>;

    /// 获取某门课程的周次数据
    async fn get_weeks(term: &str, courses_code: &str) -> Vec<course_tree::Week>;
}
