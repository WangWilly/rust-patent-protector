use axum::{routing::get, Router};

mod controllers;
use controllers::root::controller::Controller as RootController;

mod pkgs;
use pkgs::db_helper;
use pkgs::errors::handler_404;

////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .fallback(handler_404);

    ////////////////////////////////////////////////////////////////////////////

    let db = db_helper::get_connection_pool();

    ////////////////////////////////////////////////////////////////////////////

    let root_ctrl = RootController::new(db);
    let app = root_ctrl.register(app);

    ////////////////////////////////////////////////////////////////////////////

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
