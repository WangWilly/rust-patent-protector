use axum::{
    error_handling::HandleErrorLayer, http, response::IntoResponse, BoxError, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;
use std::time::Duration;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////
/// Handlers
////////////////////////////////////////////////////////////////////////////////

type StatusCode = http::StatusCode;

// https://github.com/tokio-rs/axum/blob/main/examples/global-404-handler/src/main.rs
pub async fn handler_404() -> impl IntoResponse {
    let data = json!({
        "error": "Not Found",
        "message": "The requested resource could not be found.",
    });

    (StatusCode::NOT_FOUND, Json(data))
}

pub fn add_timeout_layer(router: Router, timeout_secs: u64) -> Router {
    router.layer(
        ServiceBuilder::new()
            // this middleware goes above `TimeoutLayer` because it will receive
            // errors returned by `TimeoutLayer`
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::REQUEST_TIMEOUT
            }))
            .layer(TimeoutLayer::new(Duration::from_secs(timeout_secs))),
    )
}

////////////////////////////////////////////////////////////////////////////////
/// Errors
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    Generic { description: String },
    LoginFail,
    AuthFailCtxNotInRequestExt,
    DbRecordNoResult { source: String, id: String },

    ClientReqError { source: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic { description } => write!(f, "{description}"),
            Self::LoginFail => write!(f, "Login fail"),
            Self::AuthFailCtxNotInRequestExt => {
                write!(f, "Auth fail - Ctx not in request extensions")
            }
            Self::DbRecordNoResult { id, .. } => write!(f, "No record for id {id}"),
            Self::ClientReqError { source } => write!(f, "Client request error: {source}"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub struct ApiError {
    pub error: Error,
    pub req_id: Uuid,
}

// REST error response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - into_response - {self:?}", "ERROR");

        let status_code = match self.error {
            Error::DbRecordNoResult { .. } => StatusCode::NOT_FOUND,
            Error::AuthFailCtxNotInRequestExt => StatusCode::UNAUTHORIZED,
            Error::Generic { .. } | Error::LoginFail => StatusCode::FORBIDDEN,
            Error::ClientReqError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(json!({
            "error": {
                "error": self.error.to_string(),
                "req_id": self.req_id.to_string()
            }
        }));

        let mut response = (status_code, body).into_response();
        // Insert the real Error into the response - for the logger
        response.extensions_mut().insert(self.error);
        response
    }
}
