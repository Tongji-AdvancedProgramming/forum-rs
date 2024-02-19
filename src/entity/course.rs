use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 课程信息表
#[derive(Debug, Clone, Default, Deserialize, Serialize, DeriveEntityModel, utoipa::ToSchema)]
#[sea_orm(table_name = "course")]
#[serde(default, rename_all = "camelCase")]
pub struct Model {
    /// 学期(主键+外键)
    #[sea_orm(primary_key)]
    pub course_term: String,

    /// 课程序号(主键,目前同济的规则是代码+两位序号)
    #[sea_orm(primary_key)]
    pub course_no: String,

    /// 课程代码
    pub course_code: Option<String>,

    /// 教务系统中的全名
    #[sea_orm(column_name = "course_fname")]
    #[serde(rename = "courseFname")]
    pub course_full_name: Option<String>,

    /// 课程简称
    #[sea_orm(column_name = "course_sname")]
    #[serde(rename = "courseSname")]
    pub course_short_name: Option<String>,

    /// 课程类别(1-基础 2-专业，暂时无用，未来和学校的课程编码匹配)
    pub course_type: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
