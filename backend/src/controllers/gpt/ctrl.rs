use crate::pkgs::ctx::Ctx;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{post},
    Router,
};

use tracing::info;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct CtrlState {
    db: Pool<ConnectionManager<PgConnection>>,
    name: String,
}

////////////////////////////////////////////////////////////////////////////////

pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Router {
    let s = CtrlState {
        db: db.clone(),
        name: "root".to_string(),
    };

    Router::new()
        .route("/api/gpt/v1/assess_infringement", post(assess_infringement_v1))
        .with_state(s)
}

////////////////////////////////////////////////////////////////////////////////

// dummy
async fn assess_infringement_v1(ctx: Ctx, State(state): State<CtrlState>) -> impl IntoResponse {
    info!("assess_infringement - {:?}", ctx);

    // match create(&state.db) {
    //     Ok(test_log) => {
    //         format!("Created test log: {}", test_log.id)
    //     }
    //     Err(e) => {
    //         format!("Failed to create test log: {}", e)
    //     }
    // }

    format!("assess_infringement_v1")
}