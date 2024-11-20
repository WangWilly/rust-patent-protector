use crate::pkgs::ctx::Ctx;
use axum::{extract::Request, middleware::Next, response::Response};
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////

// State(CtxState { _db, key_dec, .. }): State<CtxState>, // TODO: auth
// cookies: Cookies,
pub async fn ctx_constructor(
    mut req: Request,
    next: Next,
) -> Response {
    println!("->> {:<12} - mw_ctx_constructor", "MIDDLEWARE");

    let uuid = Uuid::new_v4();
    // let result_user_id: Result<String> = extract_token(key_dec, &cookies).map_err(|err| {
    //     // Remove an invalid cookie
    //     if let Error::AuthFailJwtInvalid { .. } = err {
    //         cookies.remove(Cookie::named(JWT_KEY))
    //     }
    //     err
    // });
    // // NOTE: DB should be checked here

    let result_user_id = Ok("dummy_user_id".to_string());

    // Store Ctx in the request extension, for extracting in rest handlers
    let ctx = Ctx::new(result_user_id, uuid);
    req.extensions_mut().insert(ctx);

    next.run(req).await
}
