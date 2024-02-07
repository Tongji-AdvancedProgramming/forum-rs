pub mod post {
    use axum::extract::State;
    use axum::http::HeaderMap;
    use axum::response::{IntoResponse, Response};
    use axum::Form;
    use axum_client_ip::SecureClientIp;
    use axum_login::AuthSession;
    use easy_captcha::extension::axum_tower_sessions::CaptchaAxumTowerSessionStaticExt;
    use easy_captcha::extension::CaptchaUtil;
    use tower_sessions::Session;

    use crate::error::auth_error::AuthError;
    use crate::response::api_response::ApiResponse;
    use crate::service::auth_service::{AuthBackend, Credentials};
    use crate::service::log_service::LogServiceTrait;
    use crate::state::auth_state::AuthState;

    /// 登录
    #[utoipa::path(
        post,
        path = "/login",
        tag = "Auth",
        responses(
            (status = 200, description = "登录成功", body = inline(ApiResponse<i32>)),
            (status = 401, description = "密码错误"),
            (status = 403, description = "验证码错误"),
            (status = 500, description = "登录异常"),
        ),
        request_body(
            content = Credentials,
            content_type = "application/x-www-form-urlencoded"
        ),
    )]
    pub async fn login(
        State(state): State<AuthState>,
        mut auth_session: AuthSession<AuthBackend>,
        session: Session,
        headers: HeaderMap,
        SecureClientIp(ip_addr): SecureClientIp,
        Form(creds): Form<Credentials>,
    ) -> Result<Response, AuthError> {
        let result = {
            if creds.code.is_empty() {
                return Err(AuthError::CaptchaMissing);
            }

            if !CaptchaUtil::ver(&creds.code, &session).await {
                return Err(AuthError::CaptchaWrong);
            }

            let user = match auth_session.authenticate(creds.clone()).await {
                Ok(Some(user)) => user,
                Ok(None) => {
                    return Err(AuthError::WrongUsernameOrPassword);
                }
                Err(_) => return Err(AuthError::AuthFailed),
            };

            if auth_session.login(&user).await.is_err() {
                return Err(AuthError::AuthFailed);
            }

            Ok(ApiResponse::send(Ok(None::<i32>)).into_response())
        };

        let agent = headers
            .get("User-Agent")
            .map(|v| v.to_str().unwrap_or("<error>"))
            .unwrap_or("<null>");

        let comment = {
            match &result {
                Ok(_) => "登录成功",
                Err(err) => match err {
                    AuthError::WrongUsernameOrPassword => "用户名或密码错误",
                    AuthError::AuthFailed => "系统内部异常",
                    AuthError::CaptchaWrong => "验证码错误",
                    AuthError::CaptchaMissing => "无验证码",
                    AuthError::CaptchaGenerateFailed => "验证码生成失败",
                },
            }
        };

        state
            .log_service
            .log_login(&creds.username, &ip_addr, agent, comment)
            .await;

        result
    }
}

pub mod get {
    use crate::error::api_error::ApiError;
    use crate::error::auth_error::AuthError;
    use crate::response::api_response::ApiResponse;
    use crate::service::auth_service::AuthBackend;
    use askama::Template;
    use axum::response::{IntoResponse, Response};
    use axum_login::AuthSession;
    use easy_captcha::captcha::gif::GifCaptcha;
    use easy_captcha::extension::axum_tower_sessions::CaptchaAxumTowerSessionExt;
    use easy_captcha::extension::CaptchaUtil;
    use easy_captcha::NewCaptcha;
    use tower_sessions::Session;

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
            (status = 500, description = "登出异常"),
        ),
    )]
    pub async fn logout(mut auth_session: AuthSession<AuthBackend>) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => ApiResponse::send(Ok(None::<i32>)).into_response(),
            Err(_) => AuthError::AuthFailed.into_response(),
        }
    }

    pub async fn captcha(session: Session) -> Result<Response, ApiError> {
        CaptchaUtil::<GifCaptcha>::new()
            .out(&session)
            .await
            .map_err(|_| ApiError::AuthError(AuthError::CaptchaGenerateFailed))
    }
}
