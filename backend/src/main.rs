use axum::{routing::get, Router};

mod controllers;
use controllers::root::ctrl::new as new_root_router;
use controllers::root_v2::ctrl::new as new_root_v2_router;

mod pkgs;
use pkgs::db_helper::get_connection_pool;
use pkgs::errors::handler_404;

////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .fallback(handler_404);

    ////////////////////////////////////////////////////////////////////////////

    let db = get_connection_pool();

    ////////////////////////////////////////////////////////////////////////////

    let root_router = new_root_router(db.clone());
    let app = app.merge(root_router);

    let root_v2_router = new_root_v2_router(db.clone());
    let app = app.merge(root_v2_router);

    ////////////////////////////////////////////////////////////////////////////

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
