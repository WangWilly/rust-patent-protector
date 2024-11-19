use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

// https://github.com/tokio-rs/axum/blob/main/examples/global-404-handler/src/main.rs
pub async fn handler_404() -> impl IntoResponse {
    let data = json!({
        "error": "Not Found",
        "message": "The requested resource could not be found.",
    });

    (StatusCode::NOT_FOUND, Json(data))
}
