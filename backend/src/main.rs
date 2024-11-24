use axum::middleware::from_fn;
use axum::{routing::get, Router};

use serde::Deserialize;

use tokio::signal;

mod controllers;
use controllers::assessment::ctrl::new as new_assessment_router;
use controllers::gpt::ctrl::new as new_gpt_router;
use controllers::gpt::pkgs::asset_helper::AssetHelperConfig;
use controllers::gpt::pkgs::gpt_groq::GroqConfig;
use controllers::root::ctrl::new as new_root_router;
use controllers::root_v2::ctrl::new as new_root_v2_router;

mod pkgs;
use pkgs::db_helper::get_connection_pool;
use pkgs::env::get_env;
use pkgs::errors::handler_404;
use pkgs::logging::Level;

use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

mod middlewares;
use middlewares::ctx::ctx_constructor;

////////////////////////////////////////////////////////////////////////////////

fn default_debug() -> bool {
    false
}

fn default_app_host() -> String {
    "localhost".to_string()
}

fn default_app_port() -> String {
    "3000".to_string()
}

#[derive(Deserialize)]
struct MainConfig {
    #[serde(default = "default_debug")]
    debug: bool,
    #[serde(default = "default_app_host")]
    app_host: String,
    #[serde(default = "default_app_port")]
    app_port: String,
}

////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    let main_cfg = get_env::<MainConfig>("").unwrap();

    if main_cfg.debug {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .json()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_target(false)
            .pretty()
            .init();
    }

    info!("Starting server...");

    ////////////////////////////////////////////////////////////////////////////

    info!("Connecting to database...");
    let db = get_connection_pool();
    info!("Connected to database.");

    ////////////////////////////////////////////////////////////////////////////

    let asset_helper_cfg = get_env::<AssetHelperConfig>("ASSET_HELPER_").unwrap();
    let groq_cfg = get_env::<GroqConfig>("GPT_GROQ_").unwrap();

    ////////////////////////////////////////////////////////////////////////////

    info!("Creating routers...");
    let root_router = new_root_router(db.clone());
    let root_v2_router = new_root_v2_router(db.clone());
    let gpt_router = new_gpt_router(asset_helper_cfg, groq_cfg);
    let assessment_router = new_assessment_router(db.clone());
    info!("Routers created.");

    ////////////////////////////////////////////////////////////////////////////

    info!("Creating app...");
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(root_router)
        .merge(root_v2_router)
        .merge(gpt_router)
        .merge(assessment_router)
        .fallback(handler_404)
        .layer(from_fn(ctx_constructor))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    ////////////////////////////////////////////////////////////////////////////

    let bind_url = format!("{}:{}", main_cfg.app_host, main_cfg.app_port);
    info!("Starting server at: {}", bind_url);
    let listener = tokio::net::TcpListener::bind(bind_url).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

////////////////////////////////////////////////////////////////////////////////

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Ctrl-C received, shutting down...");
        },
        _ = terminate => {
            info!("Terminate signal received, shutting down...");
        },
    }
}
