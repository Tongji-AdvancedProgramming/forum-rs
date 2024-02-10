pub mod get {
    use axum::extract::{Query, State};
    use axum_login::AuthzBackend;
    use forum_macros::forum_handler;
    use serde::Deserialize;
    use utoipa::ToSchema;

    use crate::{
        config::permission,
        entity::{homework, homework_uploaded},
        error::auth_error::AuthError,
        handler::AuthSession,
        service::homework_service::HomeworkServiceTrait,
        state::homework_state::HomeworkState,
    };

    #[derive(Debug, Clone, Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct HomeworkParam {
        pub term: String,
        pub course_no: String,
        pub hw_id: i32,
    }

    #[forum_handler]
    pub async fn homework(
        State(state): State<HomeworkState>,
        Query(param): Query<HomeworkParam>,
    ) -> Option<homework::Model> {
        state
            .homework_service
            .get_homework(&param.term, param.hw_id, &param.course_no)
            .await
    }

    #[derive(Debug, Clone, Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct HomeworkUploadedParam {
        pub board_id: String,
        pub with_hidden: bool,
    }

    #[forum_handler]
    pub async fn homework_uploaded(
        auth_session: AuthSession,
        State(state): State<HomeworkState>,
        Query(param): Query<HomeworkUploadedParam>,
    ) -> Vec<homework_uploaded::Model> {
        if param.with_hidden {
            let permissions = auth_session
                .backend
                .get_user_permissions(&auth_session.user.unwrap())
                .await
                .unwrap();
            if !permissions.contains(&permission::Permission::ADMIN) {
                return Err(ApiError::AuthError(AuthError::PermissionDenied(
                    "您无权查看隐藏的已上传作业",
                )));
            }
        }

        state
            .homework_service
            .get_homework_uploaded(&param.board_id, param.with_hidden)
            .await
    }
}

pub mod post {
    use axum::{extract::State, Json};
    use forum_macros::forum_handler;

    use crate::{
        entity::homework_uploaded, service::homework_service::HomeworkServiceTrait,
        state::homework_state::HomeworkState,
    };

    #[forum_handler]
    pub async fn homework_uploaded(
        State(state): State<HomeworkState>,
        Json(homework_uploaded): Json<homework_uploaded::Model>,
    ) {
        state
            .homework_service
            .post_homework(homework_uploaded)
            .await
    }
}
