use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Default, Serialize, utoipa::ToSchema, DeriveEntityModel)]
#[sea_orm(table_name = "student")]
#[serde(default, rename_all = "camelCase")]
pub struct Model {
    /// 学期(与term表有外键关系)
    #[sea_orm(primary_key)]
    pub stu_term: String,

    /// 年级
    pub stu_grade: Option<String>,

    /// 学号(主键)
    #[sea_orm(primary_key)]
    pub stu_no: String,

    /// 姓名
    pub stu_name: Option<String>,

    /// 性别
    pub stu_sex: Option<String>,

    /// 密码(md5)
    pub stu_password: Option<String>,

    /// 专业/班级全称
    #[sea_orm(column_name = "stu_class_fname")]
    #[serde(rename = "stuClassFname")]
    pub stu_class_full_name: Option<String>,

    /// 专业/班级简称
    #[sea_orm(column_name = "stu_class_sname")]
    #[serde(rename = "stuClassSname")]
    pub stu_class_short_name: Option<String>,

    /// 学生用户等级(0:普通用户 1:助教 5:管理员 9:超级用户)
    #[sea_orm(column_name = "stu_userlevel")]
    #[serde(rename = "stuUserlevel")]
    pub stu_user_level: Option<String>,

    /// 账号是否启用('0':禁止登录 '1':允许登录 注意:enum不要当int处理)
    pub stu_enable: Option<String>,

    /// 系统注册时间
    pub stu_add_date: Option<NaiveDateTime>,

    /// 学生选修的课程1的课号(与course表有外键关系)
    pub stu_cno_1: Option<String>,

    /// 学生选修的课程1是否退课('0':正常 '1':已退课 注意:enum不要当int处理,退课学生对应课程的作业信息不要显示出来)
    pub stu_cno_1_is_del: Option<String>,

    /// 学生选修的课程2的课号(与course表有外键关系)
    pub stu_cno_2: Option<String>,

    /// 学生选修的课程2是否退课('0':正常 '1':已退课 注意:enum不要当int处理,退课学生对应课程的作业信息不要显示出来)
    pub stu_cno_2_is_del: Option<String>,

    /// 学生选修的课程3的课号(与course表有外键关系)
    pub stu_cno_3: Option<String>,

    /// 学生选修的课程3是否退课('0':正常 '1':已退课 注意:enum不要当int处理,退课学生对应课程的作业信息不要显示出来)
    pub stu_cno_3_is_del: Option<String>,

    /// 该学生是否被删除('0':正常 '1':已删除 注意:被删除则无论stu_enbale置何值均不允许登录)
    pub stu_is_del: Option<String>,

    /// 备注信息
    pub stu_comment: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
