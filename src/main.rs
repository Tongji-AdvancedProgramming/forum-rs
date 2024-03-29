use axum_login::AuthManagerLayerBuilder;
use fred::interfaces::ClientLike;
use std::process::exit;
use std::sync::Arc;

use log::{error, info};
use time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};

use crate::config::database::DatabaseTrait;
use crate::config::redis::RedisTrait;
use crate::config::{database, meili, redis, s3, session};
use crate::service::auth_service::AuthBackend;

pub mod config;
mod dto;
mod entity;
mod error;
mod handler;
pub mod middleware;
mod repository;
mod response;
mod routes;
mod service;
mod state;
pub mod utils;

fn panic() -> ! {
    exit(1);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    config::init();

    let db_conn = database::Db::init()
        .await
        .unwrap_or_else(|e| {
            error!("\n[Database Connection Failed]\n数据库连接失败，请检查配置是否正确、网络连接情况和数据库端配置\n\n{}",e);
            panic()
        });
    let db_conn = Arc::new(db_conn);

    let redis_conn = Arc::new(redis::Redis::init().await.unwrap_or_else(|e| {
        error!("\n[Redis Connection Failed]\nRedis连接失败，请检查配置是否正确、网络连接情况和服务端配置\n\n{}",e);
        panic()
    }));
    let redis_conn_handle = redis_conn.get_pool().connect();
    redis_conn.get_pool().wait_for_connect().await.unwrap_or_else(|e| {
        error!("\n[Redis Connection Failed]\nRedis连接失败，请检查配置是否正确、网络连接情况和服务端配置\n\n{}",e);
        panic()
    });

    let s3_client = Arc::new(s3::S3Conn::init().unwrap_or_else(|e| {
        error!("\n[S3 Client Configurate Failed]\nS3客户端初始化失败，请检查配置是否正确、网络连接情况和服务端配置\n\n{}",e);
        panic()
    }));

    let meili_client = Arc::new(meili::Meili::init());

    // Session层
    let session_store = session::RedisSession::new(&redis_conn);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    // 验证层
    let auth_backend = AuthBackend::new(&db_conn);
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    let host: String;
    {
        let config = config::get_config();
        let guard = config.read().unwrap();
        host = format!("0.0.0.0:{}", guard.port);
    }

    let listener = tokio::net::TcpListener::bind(host)
        .await
        .unwrap_or_else(|e| {
            error!(
                "\n[Tcp bind failed]\n绑定端口失败，请检查端口是否正确、是否被占用。\n\n{}",
                e
            );
            panic()
        });
    info!("应用即将启动");
    axum::serve(
        listener,
        routes::root::routes(
            Arc::clone(&db_conn),
            Arc::clone(&redis_conn),
            Arc::clone(&s3_client),
            Arc::clone(&meili_client),
            auth_layer,
        ),
    )
    .await
    .unwrap_or_else(|e| {
        error!("\n[Http server inner error]\nHTTP服务器内部错误\n\n{}", e);
        panic()
    });

    redis_conn_handle.await.unwrap().unwrap();
}
