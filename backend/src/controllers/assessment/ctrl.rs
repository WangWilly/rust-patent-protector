use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use crate::pkgs::common::ApiResult;
use crate::pkgs::ctx::Ctx;
use crate::pkgs::db_helper::DbPool;
use crate::pkgs::errors::Error;
use crate::pkgs::repos::assessment::{create::create, get::get_by_id};
use crate::{info, pkgs::errors::ApiError};

use super::dtos::create_assessment_v1::{CreateAssessmentV1Req, CreateAssessmentV1Resp};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct CtrlState {
    db: DbPool,
}

////////////////////////////////////////////////////////////////////////////////

pub fn new(db: DbPool) -> Router {
    let s = CtrlState { db };

    Router::new()
        .route("/api/infringement/v1", post(create_assessment_v1))
        .route("/api/infringement/v1/:aid", get(get_assessment_v1))
        .with_state(s)
}

////////////////////////////////////////////////////////////////////////////////

async fn create_assessment_v1(
    ctx: Ctx,
    State(state): State<CtrlState>,
    Json(req): Json<CreateAssessmentV1Req>,
) -> ApiResult<Json<CreateAssessmentV1Resp>> {
    info!("create_assessment_v1 - {:?}", ctx);

    match create(&state.db, &req.to_model()) {
        Ok(assessment) => Ok(Json(CreateAssessmentV1Resp::from_model(&assessment))),
        Err(e) => Err(ApiError {
            error: Error::Generic {
                description: e.to_string(),
            },
            req_id: ctx.req_id(),
        }),
    }
}

async fn get_assessment_v1(
    ctx: Ctx,
    State(state): State<CtrlState>,
    Path(aid): Path<i32>,
) -> ApiResult<Json<CreateAssessmentV1Resp>> {
    info!("get_assessment_v1 - {:?}", ctx);

    match get_by_id(&state.db, aid) {
        Ok(assessment) => Ok(Json(CreateAssessmentV1Resp::from_model(&assessment))),
        Err(e) => Err(ApiError {
            error: Error::Generic {
                description: e.to_string(),
            },
            req_id: ctx.req_id(),
        }),
    }
}
