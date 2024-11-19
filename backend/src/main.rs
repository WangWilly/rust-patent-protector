use axum::{
    routing::get,
    Router,
};

mod controllers;
use controllers::root::controller::Controller as RootController;

#[tokio::main]
async fn main() {
    let app = Router::new().
        route("/", get(|| async { "Hello, World!" }));

    let root_ctrl = RootController::new();
    let app = root_ctrl.register(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
