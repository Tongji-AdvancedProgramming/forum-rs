use serde::{Deserialize, Serialize};

use crate::entity::{course, homework};

/// 帖子所在的位置，分为三种：
/// <ol>
///    <li>第x周的整体问题</li>
///    <li>第x周的某具体作业</li>
///    <li>课程的整体问题</li>
/// </ol>
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PostLocation {
    Weekly,
    Homework,
    Course,
    WeekSummary,
    CourseSummary,
}

/// 板块
#[derive(Debug, Clone, Deserialize, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    /// ID串
    pub id: String,

    /// 板块位置
    pub location: PostLocation,

    /// 所属课程
    pub course: Option<course::Model>,

    /// 周次
    pub week: i8,

    /// 所属作业
    pub homework: Option<homework::Model>,
}
