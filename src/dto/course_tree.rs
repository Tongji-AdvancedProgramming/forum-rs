use crate::entity::{course, homework};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Course {
    #[serde(flatten)]
    pub _course: course::Model,
    pub weeks: Vec<Week>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Week {
    pub number: i32,
    pub content: String,
    pub homeworks: Vec<homework::Model>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CourseTree {
    pub courses: Vec<Course>,
}

impl From<course::Model> for Course {
    fn from(course: course::Model) -> Self {
        Self {
            _course: course,
            weeks: vec![],
        }
    }
}
