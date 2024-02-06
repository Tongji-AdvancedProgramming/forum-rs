use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 作业
#[derive(Debug, Clone, Default, Deserialize, Serialize, DeriveEntityModel, utoipa::ToSchema)]
#[sea_orm(table_name = "homework")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// 学期(主键+外键)
    #[sea_orm(primary_key)]
    pub hw_term: String,

    /// 作业课程编号
    #[sea_orm(column_name = "hw_ccode")]
    #[serde(rename = "hwCcode")]
    pub hw_course_code: String,

    /// 作业序号(主键)
    #[sea_orm(primary_key)]
    pub hw_id: i16,

    /// 布置周
    pub hw_week: i8,

    /// 章节(0-20: 第0-20章 90:大作业 98:文档作业 99:其它作业)
    pub hw_chapter: i8,

    /// 交作业网站的提交文件名
    pub hw_filename: String,

    /// 作业描述
    pub hw_description: String,

    /// 作业提交开始时间
    #[sea_orm(column_name = "hw_bdate")]
    #[serde(rename = "hwBdate")]
    pub hw_begin_date: NaiveDateTime,

    /// 作业提交结束时间
    #[sea_orm(column_name = "hw_edate")]
    #[serde(rename = "hwEdate")]
    pub hw_end_date: NaiveDateTime,

    /// 本作业加入论坛的时间
    pub hw_add_date: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
