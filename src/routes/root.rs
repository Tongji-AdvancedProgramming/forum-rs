use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::handler::HandlerWithoutStateExt;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::{get, IntoMakeService};
use axum::Router;
use axum_client_ip::SecureClientIpSource;
use axum_login::{login_required, AuthManagerLayer};
use tower_http::trace::TraceLayer;
use tower_sessions::SessionStore;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::database::Db;
use crate::config::redis::Redis;

use crate::handler::swagger_handler::ApiDoc;
use crate::routes::{auth_routes, user_routes};
use crate::service::auth_service::AuthBackend;
use crate::state::auth_state::AuthState;
use crate::state::user_state::UserState;

pub fn routes(
    db_conn: Arc<Db>,
    redis: Arc<Redis>,
    auth_layer: AuthManagerLayer<AuthBackend, impl SessionStore + Clone>,
) -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    let merged_router = {
        let user_state = UserState::new(&db_conn);
        let auth_state = AuthState::new(&db_conn);

        Router::new()
            .nest("/user", user_routes::routes().with_state(user_state))
            .route_layer(login_required!(AuthBackend))
            .merge(auth_routes::routes().with_state(auth_state))
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
        .layer(SecureClientIpSource::ConnectInfo.into_extension()); // 在生产中改成别的

    app_router.into_make_service_with_connect_info()
}
