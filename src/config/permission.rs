use serde::Deserialize;

#[derive(Default, Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct PermissionConfig {
    pub ta: i32,
    pub admin: i32,
    #[serde(rename = "super")]
    pub _super: i32,
}

#[derive(Hash, Debug, Eq, PartialEq)]
pub enum Permission {
    TA,
    ADMIN,
    SUPER,
}
