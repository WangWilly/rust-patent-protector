use crate::pkgs::common::{ApiResult, Result};
use crate::pkgs::errors::{ApiError, Error};
use axum::extract::FromRequestParts;
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////
/// Ctx
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Ctx {
    result_user_id: Result<String>,
    req_id: Uuid,
}

impl Ctx {
    pub fn new(result_user_id: Result<String>, uuid: Uuid) -> Self {
        Self {
            result_user_id,
            req_id: uuid,
        }
    }

    pub fn user_id(&self) -> ApiResult<String> {
        self.result_user_id.clone().map_err(|error| ApiError {
            error,
            req_id: self.req_id,
        })
    }

    pub fn req_id(&self) -> Uuid {
        self.req_id
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Extractor
/// ugly but direct implementation from axum, until "async trait fn" are in stable rust, instead of importing some 3rd party macro
/// Extractor: makes it possible to specify Ctx as a param - fetches the result from the header parts extension

// The trait is generic over a type S which must implement the Send and Sync traits.
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    // This defines an associated type Rejection for the trait, which is set to ApiError.
    // This means that if the request parts cannot be converted into a Ctx, an ApiError will be returned.
    type Rejection = ApiError;

    // Return is a pinned, boxed future that will eventually produce an ApiResult<Self> (where Self is Ctx).
    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut axum::http::request::Parts,
        _state: &'life1 S,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ApiResult<Self>> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        // TODO: The function body is an async block that is boxed and pinned. (?)
        Box::pin(async {
            println!(
                "->> {:<12} - Ctx::from_request_parts - extract Ctx from extension",
                "EXTRACTOR"
            );

            // It tries to get a Ctx from the request parts' extensions.
            // If successful, it clones the Ctx and returns it. If not, it returns an ApiError with a new UUID and an AuthFailCtxNotInRequestExt error.
            parts.extensions.get::<Ctx>().cloned().ok_or(ApiError {
                req_id: Uuid::new_v4(),
                error: Error::AuthFailCtxNotInRequestExt,
            })
        })
    }
}
