use crate::pkgs::errors::{ApiError, Error};

////////////////////////////////////////////////////////////////////////////////

pub type Result<T> = core::result::Result<T, Error>;

// ApiError has to have the req_id to report to the client and implements IntoResponse.
pub type ApiResult<T> = core::result::Result<T, ApiError>;
// Any error for storing before composing a response.
// For errors that either don't affect the response, or are build before attaching the req_id.
