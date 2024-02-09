use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 已上传的作业
#[derive(Debug, Clone, Default, Deserialize, Serialize, DeriveEntityModel, utoipa::ToSchema)]
#[sea_orm(table_name = "homework_uploaded")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// 学期(主键+外键)
    #[sea_orm(primary_key)]
    pub hwup_term: String,

    /// 课程序号(主键,对应course中的course_code 注意:不是外键关系,但要检查是否存在)
    #[sea_orm(primary_key, column_name = "hwup_ccode")]
    #[serde(rename = "hwupCcode")]
    pub hwup_course_code: String,

    /// 文件编号(例: 22232-030101-W0102)
    #[sea_orm(primary_key)]
    pub hwup_id: String,

    /// 布置周
    pub hwup_week: i32,

    /// 章节(0-20: 第0-20章 90:大作业 98:文档作业 99:其它作业)
    pub hwup_chapter: i32,

    /// 上传的文件名
    pub hwup_filename: String,

    /// 上传的文件的MD5
    #[sea_orm(primary_key, column_name = "hwup_filemd5")]
    #[serde(rename = "hwupFilemd5")]
    pub hwup_file_md5: String,

    /// 文件导入本论坛的时间
    pub hwup_date_add: NaiveDateTime,

    /// 文件是否已删除('0':可正常显示/下载 '1':不显示/不提供下载 注意:enum不要当int处理)
    pub hwup_is_del: String,

    /// 备注
    pub hwup_comment: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
