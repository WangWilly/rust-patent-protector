use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////
/// Handlers
////////////////////////////////////////////////////////////////////////////////

// https://github.com/tokio-rs/axum/blob/main/examples/global-404-handler/src/main.rs
pub async fn handler_404() -> impl IntoResponse {
    let data = json!({
        "error": "Not Found",
        "message": "The requested resource could not be found.",
    });

    (StatusCode::NOT_FOUND, Json(data))
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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic { description } => write!(f, "{description}"),
            Self::LoginFail => write!(f, "Login fail"),
            Self::AuthFailCtxNotInRequestExt => write!(f, "Auth fail - Ctx not in request extensions"),
            Self::DbRecordNoResult { id, .. } => write!(f, "No record for id {id}"),
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
            Error::Generic { .. }
            | Error::LoginFail => StatusCode::FORBIDDEN,
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
