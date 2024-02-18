use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::extract::DefaultBodyLimit;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use axum_client_ip::SecureClientIpSource;
use axum_login::{login_required, AuthManagerLayer};
use tower_http::trace::TraceLayer;
use tower_sessions::SessionStore;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::database::Db;
use crate::config::meili::Meili;
use crate::config::redis::Redis;

use crate::config::s3::S3Conn;
use crate::config::APP_CONFIG;
use crate::handler::swagger_handler::ApiDoc;
use crate::routes::{auth_routes, user_routes};
use crate::service::auth_service::AuthBackend;
use crate::state::auth_state::AuthState;
use crate::state::board_state::BoardState;
use crate::state::course_state::CourseState;
use crate::state::homework_state::HomeworkState;
use crate::state::limit_state::LimitState;
use crate::state::metadata_state::MetadataState;
use crate::state::notification_state::NotificationState;
use crate::state::post_state::PostState;
use crate::state::upload_state::UploadState;
use crate::state::user_state::UserState;

use super::{
    board_routes, course_routes, homework_routes, metadata_routes, notification_routes,
    post_routes, upload_routes,
};

pub fn routes(
    db_conn: Arc<Db>,
    redis: Arc<Redis>,
    s3_client: Arc<S3Conn>,
    meili_client: Arc<Meili>,
    auth_layer: AuthManagerLayer<AuthBackend, impl SessionStore + Clone>,
) -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    let app_config = {
        let config = APP_CONFIG.clone();
        let guard = config.read().unwrap();
        Arc::new((*guard).clone())
    };

    let merged_router = {
        let auth_state = AuthState::new(&db_conn);
        let board_state = BoardState::new(&db_conn);
        let course_state = CourseState::new(&db_conn, &app_config);
        let homework_state = HomeworkState::new(&db_conn, &s3_client, &app_config);
        let limit_state = LimitState::new(&redis);
        let metadata_state = MetadataState::new(&db_conn);
        let notification_state = NotificationState::new(&db_conn);
        let post_state = PostState::new(&db_conn, &app_config, &meili_client);
        let user_state = UserState::new(&db_conn);
        let upload_state = UploadState::new(&s3_client, &app_config);

        Router::new()
            .nest("/user", user_routes::routes().with_state(user_state))
            .nest("/board", board_routes::routes().with_state(board_state))
            .nest("/course", course_routes::routes().with_state(course_state))
            .nest(
                "/homework",
                homework_routes::routes().with_state(homework_state),
            )
            .nest(
                "/meta",
                metadata_routes::routes().with_state(metadata_state),
            )
            .nest(
                "/notification",
                notification_routes::routes().with_state(notification_state),
            )
            .nest("/post", post_routes::routes().with_state(post_state))
            .nest("/upload", upload_routes::routes().with_state(upload_state))
            .route_layer(login_required!(AuthBackend))
            .merge(auth_routes::routes(limit_state).with_state(auth_state))
            .route(
                "/",
                get(|| async {
                    "本Rest API应用不提供前端接口。请检查访问地址是否正确。"
                }),
            )
            .merge(Router::new().route("/health", get(|| async { "Healthy" })))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    };

    let app_router = Router::new()
        .merge(merged_router)
        .layer(auth_layer)
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(1024 * 1024 * 10))
        .layer(SecureClientIpSource::ConnectInfo.into_extension()); // 在生产中改成别的

    app_router.into_make_service_with_connect_info()
}
