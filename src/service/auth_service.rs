use crate::config::database::Db;
use crate::config::permission::PermissionConfig;
use crate::config::{get_config, permission};
use crate::entity::student::Model as Student;
use crate::repository::user_repo::{UserRepository, UserRepositoryTrait};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use easy_hex::Hex;
use md5::{Digest, Md5};
use serde::Deserialize;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use utoipa::ToSchema;

impl Debug for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Student")
            .field("stu_term", &self.stu_term)
            .field("stu_no", &self.stu_no)
            .finish()
    }
}

impl AuthUser for Student {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.stu_no.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.stu_password.as_bytes()
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct AuthBackend {
    user_repo: UserRepository,
    permission_config: PermissionConfig,
}

impl AuthBackend {
    pub fn new(db: &Arc<Db>) -> Self {
        let config = get_config();
        let guard = config.read().unwrap();
        Self {
            user_repo: UserRepository::new(db),
            permission_config: guard.permission.clone(),
        }
    }

    pub fn verify_password(hash: &str, input: &str) -> bool {
        let input_hash = {
            let mut hasher = Md5::new();
            md5::Digest::update(&mut hasher, input.as_bytes());
            hasher.finalize()
        };
        let input_hex = Hex(input_hash).to_string();
        hash == input_hex
    }
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = Student;
    type Credentials = Credentials;
    type Error = sea_orm::error::RuntimeErr;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self.user_repo.find_by_id(&creds.username).await;

        Ok(user.filter(|user| Self::verify_password(&user.stu_password, &creds.password)))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.user_repo.find_by_id(user_id).await)
    }
}

#[async_trait]
impl AuthzBackend for AuthBackend {
    type Permission = permission::Permission;

    async fn get_user_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let mut permission_set: HashSet<Self::Permission> = Default::default();

        let level: i32 = user.stu_user_level.parse().unwrap_or(0);
        if level >= self.permission_config.admin {
            permission_set.insert(Self::Permission::ADMIN);
        }
        if level >= self.permission_config._super {
            permission_set.insert(Self::Permission::SUPER);
        }
        if level >= self.permission_config.ta {
            permission_set.insert(Self::Permission::TA);
        }

        Ok(permission_set)
    }
}
