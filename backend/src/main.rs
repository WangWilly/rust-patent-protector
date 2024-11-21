use axum::middleware::from_fn;
use axum::{routing::get, Router};

mod controllers;
use controllers::gpt::ctrl::new as new_gpt_router;
use controllers::root::ctrl::new as new_root_router;
use controllers::root_v2::ctrl::new as new_root_v2_router;
use controllers::gpt::pkgs::asset_helper::AssetHelperConfig;
use controllers::gpt::pkgs::gpt_groq::GroqConfig;

mod pkgs;
use pkgs::db_helper::get_connection_pool;
use pkgs::errors::handler_404;
use pkgs::logging::Level;

use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

mod middlewares;
use middlewares::ctx::ctx_constructor;

////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        // .json()
        .pretty()
        .init();

    info!("Starting server...");

    ////////////////////////////////////////////////////////////////////////////

    info!("Connecting to database...");
    let db = get_connection_pool();
    info!("Connected to database.");

    ////////////////////////////////////////////////////////////////////////////

    let asset_helper_cfg = AssetHelperConfig::from_env();
    let groq_cfg = GroqConfig::from_env();

    ////////////////////////////////////////////////////////////////////////////

    info!("Creating routers...");
    let root_router = new_root_router(db.clone());
    let root_v2_router = new_root_v2_router(db.clone());
    let gpt_router = new_gpt_router(asset_helper_cfg, groq_cfg);
    info!("Routers created.");

    ////////////////////////////////////////////////////////////////////////////

    info!("Creating app...");
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(root_router)
        .merge(root_v2_router)
        .merge(gpt_router)
        .fallback(handler_404)
        .layer(from_fn(ctx_constructor))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    ////////////////////////////////////////////////////////////////////////////

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    info!("Server stopped.");
}
