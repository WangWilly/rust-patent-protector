use crate::pkgs::errors::{ApiError, Error};
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////

pub type Result<T> = core::result::Result<T, Error>;

// ApiError has to have the req_id to report to the client and implements IntoResponse.
pub type ApiResult<T> = core::result::Result<T, ApiError>;
// Any error for storing before composing a response.
// For errors that either don't affect the response, or are build before attaching the req_id.

////////////////////////////////////////////////////////////////////////////////

pub fn not_found<T>(msg: String) -> ApiResult<T> {
    Err(ApiError {
        error: Error::Generic { description: msg },
        req_id: Uuid::new_v4(),
    })
}

pub fn internal_error<T>(msg: String) -> ApiResult<T> {
    Err(ApiError {
        error: Error::Generic { description: msg },
        req_id: Uuid::new_v4(),
    })
}
