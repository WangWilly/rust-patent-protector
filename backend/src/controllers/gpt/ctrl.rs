use crate::pkgs::common::{internal_error, not_found, ApiResult};
use crate::pkgs::ctx::Ctx;
use crate::pkgs::dtos::infrigement;

use axum::{extract::State, routing::post, Json, Router};

use tracing::info;

use super::dtos::assess_infringement_v1::{AssessInfringementV1Req, AssessInfringementV1Resp};
use super::pkgs::asset_helper::{AssetHelper, AssetHelperConfig};
use super::pkgs::gpt_groq::Groq;
use super::pkgs::gpt_groq::GroqConfig;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct CtrlState {
    asset_helper: AssetHelper,
    groq: Groq,
}

////////////////////////////////////////////////////////////////////////////////

pub fn new(asset_helper_cfg: AssetHelperConfig, groq_cfg: GroqConfig) -> Router {
    // TODO: share asset_helper across controllers
    let asset_helper = AssetHelper::from_config(asset_helper_cfg);
    let groq = Groq::from_config(groq_cfg);

    let s = CtrlState { asset_helper, groq };

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
) -> ApiResult<Json<AssessInfringementV1Resp>> {
    info!("assess_infringement - {:?}", ctx);

    // TODO: ApiError::from(self.ctx)
    // Validation
    let patent = state.asset_helper.get_patents().get(&req.patent_pub_id);
    if patent.is_none() {
        return not_found::<Json<AssessInfringementV1Resp>>(format!(
            "Patent {} not found",
            req.patent_pub_id
        ));
    }
    let patent = patent.unwrap();

    let company = state
        .asset_helper
        .get_company_products()
        .get(req.company_name.clone());
    if company.is_none() {
        return not_found::<Json<AssessInfringementV1Resp>>(format!(
            "Company {} not found",
            req.company_name
        ));
    }
    let company = company.unwrap();

    let infrigements = state.groq.assess_company_v1(&patent, &company).await;

    // filter with infringement_likelihood > 0.3 and sort them by infringement_likelihood
    let mut infrigements = infrigements
        .into_iter()
        .filter(|i| i.infringement_likelihood > 0.3)
        .collect::<Vec<infrigement::ProductInfrigement>>();

    infrigements.sort_by(|a, b| {
        b.infringement_likelihood
            .partial_cmp(&a.infringement_likelihood)
            .unwrap()
    });

    if infrigements.len() == 0 {
        return not_found("No infrigements found".to_string());
    }

    let summary = state
        .groq
        .summarize_v1(&patent, &company, &infrigements)
        .await;
    if summary.is_err() {
        return internal_error(summary.err().unwrap().to_string());
    }

    let summary = summary.unwrap();
    let resp =
        AssessInfringementV1Resp::from_patent_company(&patent, &company, &infrigements, &summary);
    Ok(Json(resp))
}
