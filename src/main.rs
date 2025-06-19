use axum::{
    extract::{MatchedPath, Request},
    routing::get,
    Router,
};
use core_utils::AppState;
use dotenvy::dotenv;
use permissions::PermissionController;
use redis_db::init_redis;
use std::{env, net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
mod auth;
mod permissions;
mod role;
mod user;
use auth::AuthController;
use role::RoleController;
use user::UserController;
mod core_utils;
mod db;
mod redis_db;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let pool = db::init_pool().await;
    let redis_pool = init_redis().await;
    let state = AppState {
        pool: Arc::new(pool),
        redis_pool: Arc::new(redis_pool),
    };
    let port: u16 = env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("EXPECT u16 form");
    let app = Router::new()
        .route("/", get(|| async { "pong".to_string() }))
        .nest("/users", UserController::new(state.clone()))
        .nest("/roles", RoleController::new(state.clone()))
        .nest("/auth", AuthController::new(state.clone()))
        .nest("/permissions", PermissionController::new(state.clone()))
        .layer(
            TraceLayer::new_for_http()
                // Create our own span for the request and include the matched path. The matched
                // path is useful for figuring out which handler the request was routed to.
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();
                    // axum automatically adds this extension.
                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());
                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                .on_failure(()),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap()
}
