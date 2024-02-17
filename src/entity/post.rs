use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 发帖信息表
#[derive(Debug, Clone, Deserialize, Serialize, DeriveEntityModel, utoipa::ToSchema)]
#[sea_orm(table_name = "post")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// ID(主键,自动增长)
    #[sea_orm(primary_key)]
    pub post_id: i32,

    /// 学期(外键)
    pub post_term: String,

    /// 课程序号(对应course中的course_code 注意:不是外键关系,但要检查是否存在)
    #[sea_orm(column_name = "post_ccode")]
    #[serde(rename = "postCcode")]
    pub post_course_code: String,

    /// 对应的具体作业的序号
    //  <p>
    //  如果本项为空但week和/或chapter不为-1，则表示帖子为周总体问题或章节总体问题
    //  <p>
    //  如果本项与week/chapter皆为-1，则表示帖子为学期总体问题
    #[sea_orm(column_name = "post_hwid")]
    #[serde(rename = "postHwid")]
    pub post_hw_id: i16,

    /// 布置周(课程的整体问题则周次为-1)
    pub post_week: i8,

    /// 章节(课程的整体问题则章节为-1)
    pub post_chapter: i8,

    /// 对应帖子的id(与post_id是外键关系)
    //  <p>
    //  如果是发帖,则为NULL
    //  <p>
    //  如果是回帖,则为对应帖子的post_id(以此为依据构建发帖回帖的树形结构)
    pub post_answer_id: Option<i32>,

    /// 帖子类型('Question':首发问题 'QuestionsAdditional':追问 'Answer':回帖 'Other':其它 '/':预留)
    //  <p>
    //  以 post_term + post_ccode + post_hwup_or_hw_id 为基准汇聚,具体排序规则?
    //  <p>
    //  本字段是否多余?
    pub post_type: String,

    /// 发帖人学号
    #[sea_orm(column_name = "post_sno")]
    #[serde(rename = "postSno")]
    pub post_sender_no: String,

    /// 优先级(从'0'~'9' 依次递增,帖子显示是按优先级顺序,相同优先级按发帖时间,可由管理员手工置位进行调整)
    pub post_priority: String,

    /// 约定的tag 1标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_01: String,

    /// 约定的tag 2标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_02: String,

    /// 约定的tag 3标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_03: String,

    /// 约定的tag 4标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_04: String,

    /// 约定的tag 5标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_05: String,

    /// 约定的tag 6标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_06: String,

    /// 约定的tag 7标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_07: String,

    /// 约定的tag 8标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_08: String,

    /// 约定的tag 9标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_09: String,

    /// 约定的tag 10标记(0:此标记未置位 1:此标记已置位)
    pub post_tag_10: String,

    /// 帖子标题
    pub post_title: Option<String>,

    /// 发帖具体内容(允许贴图,Richtext?)
    pub post_content: Option<String>,

    /// 发帖时间
    pub post_date: NaiveDateTime,

    /// 帖子是否已删除('0':正常显示 '1':不显示,包括所有的回帖 注意:enum不要当int处理)
    pub post_is_del: String,

    /// 备注(预留)
    pub post_comment: Option<String>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            post_id: Default::default(),
            post_term: Default::default(),
            post_course_code: Default::default(),
            post_hw_id: Default::default(),
            post_week: Default::default(),
            post_chapter: Default::default(),
            post_answer_id: Default::default(),
            post_type: Default::default(),
            post_sender_no: Default::default(),
            post_priority: "0".into(),
            post_tag_01: "0".into(),
            post_tag_02: "0".into(),
            post_tag_03: "0".into(),
            post_tag_04: "0".into(),
            post_tag_05: "0".into(),
            post_tag_06: "0".into(),
            post_tag_07: "0".into(),
            post_tag_08: "0".into(),
            post_tag_09: "0".into(),
            post_tag_10: "0".into(),
            post_title: Default::default(),
            post_content: Default::default(),
            post_date: Default::default(),
            post_is_del: "0".into(),
            post_comment: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::student::Entity")]
    Student,
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
