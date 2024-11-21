use crate::pkgs::common::ApiResult;
use crate::pkgs::ctx::Ctx;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use tracing::info;

use super::dtos::assess_infringement_v1::{AssessInfringementV1Req, AssessInfringementV1Resp};
use super::pkgs::asset_helper::{AssetHelper, AssetHelperConfig};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct CtrlState {
    asset_helper: AssetHelper,
}

////////////////////////////////////////////////////////////////////////////////

pub fn new(
    asset_helper_cfg: AssetHelperConfig,
) -> Router {
    // TODO: share asset_helper across controllers
    let asset_helper = AssetHelper::from_config(asset_helper_cfg);

    let s = CtrlState {
        asset_helper,
    };

    Router::new()
        .route(
            "/api/gpt/v1/assess_infringement",
            post(assess_infringement_v1),
        )
        .with_state(s)
}

////////////////////////////////////////////////////////////////////////////////

async fn assess_infringement_v1(
    ctx: Ctx,
    State(state): State<CtrlState>,
    Json(req): Json<AssessInfringementV1Req>,
    // ) -> ApiResult<Json<AssessInfringementV1Resp>> {
) -> impl IntoResponse {
    info!("assess_infringement - {:?}", ctx);

    // Validation
    let patent = state.asset_helper.get_patents().get(&req.patent_pub_id);
    if patent.is_none() {
        return format!("Patent {} not found", req.patent_pub_id);
    }
    let patent = patent.unwrap();
    info!("patent: {}", patent);

    let product = state
        .asset_helper
        .get_company_products()
        .get(req.company_name.clone());
    if product.is_none() {
        return format!("Company {} not found", req.company_name);
    }
    let product = product.unwrap();
    info!("product: {}", product);

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
