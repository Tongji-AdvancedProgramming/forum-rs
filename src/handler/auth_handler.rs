pub mod post {
    use axum::response::IntoResponse;
    use axum::Form;
    use axum_login::AuthSession;
    use either::Left;

    use crate::error::auth_error::AuthError;
    use crate::response::api_response::ApiResponse;
    use crate::service::auth::{AuthBackend, Credentials};

    /// 登录
    #[utoipa::path(
        post,
        path = "/login",
        tag = "Auth",
        responses(
            (status = 200, description = "登录成功", body = inline(ApiResponse<i32>)),
            (status = 401, description = "密码错误"),
            (status = 500, description = "登录异常")
        ),
        request_body(
            content = Credentials,
            content_type = "application/x-www-form-urlencoded"
        ),
    )]
    pub async fn login(
        mut auth_session: AuthSession<AuthBackend>,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return AuthError::WrongUsernameOrPassword.into_response();
            }
            Err(_) => return AuthError::AuthFailed.into_response(),
        };

        if auth_session.login(&user).await.is_err() {
            return AuthError::AuthFailed.into_response();
        }

        ApiResponse::send(Left(None::<i32>)).into_response()
    }
}

pub mod get {
    use crate::error::auth_error::AuthError;
    use crate::response::api_response::ApiResponse;
    use crate::service::auth::AuthBackend;
    use askama::Template;
    use axum::response::IntoResponse;
    use axum_login::AuthSession;
    use either::Left;

    #[derive(Template)]
    #[template(path = "login.html")]
    pub struct LoginTemplate;

    pub async fn login() -> LoginTemplate {
        LoginTemplate {}
    }

    /// 登出
    #[utoipa::path(
        get,
        path = "/logout",
        tag = "Auth",
        responses(
            (status = 200, description = "登出成功", body = inline(ApiResponse<i32>)),
            (status = 500, description = "登出异常")
        ),
    )]
    pub async fn logout(mut auth_session: AuthSession<AuthBackend>) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => ApiResponse::send(Left(None::<i32>)).into_response(),
            Err(_) => AuthError::AuthFailed.into_response(),
        }
    }
}
