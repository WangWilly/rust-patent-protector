use std::vec;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::controllers::gpt::dtos::assess_infringement_v1::AssessInfringementV1ProductItem;
use crate::pkgs::models::assessments::{Assessment, NewAssessment};
use crate::pkgs::time::{iso8601, parse_iso8601, unix_timestamp};

////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
pub struct CreateAssessmentV1Req {
    pub patent_id: String,
    pub company_name: String,
    pub analysis_date: String,
    pub top_infringing_products: Vec<AssessInfringementV1ProductItem>,
    pub overall_risk_assessment: String,
}

impl CreateAssessmentV1Req {
    pub fn to_model(&self) -> NewAssessment {
        NewAssessment {
            patent_id: self.patent_id.clone(),
            company_name: self.company_name.clone(),
            analysis_date: parse_iso8601(&self.analysis_date),
            top_infringing_products: Some(json!(self.top_infringing_products)),
            overall_risk_assessment: Some(self.overall_risk_assessment.clone()),
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Serialize)]
pub struct CreateAssessmentV1Resp {
    pub id: i32,
    pub patent_id: String,
    pub company_name: String,
    pub analysis_date: String,
    pub top_infringing_products: Vec<AssessInfringementV1ProductItem>,
    pub overall_risk_assessment: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl CreateAssessmentV1Resp {
    pub fn new(
        id: i32,
        patent_id: String,
        company_name: String,
        analysis_date: String,
        top_infringing_products: Vec<AssessInfringementV1ProductItem>,
        overall_risk_assessment: String,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            id,
            patent_id,
            company_name,
            analysis_date,
            top_infringing_products,
            overall_risk_assessment,
            created_at,
            updated_at,
        }
    }

    pub fn from_model(model: &Assessment) -> Self {
        let binding = model.top_infringing_products.clone().unwrap();
        let mut top_infringing_products: Vec<AssessInfringementV1ProductItem> = vec![];
        for item in binding.as_array().unwrap() {
            let product: AssessInfringementV1ProductItem = serde_json::from_value(item.clone()).unwrap();
            top_infringing_products.push(product);
        }

        Self::new(
            model.id,
            model.patent_id.clone(),
            model.company_name.clone(),
            iso8601(&model.analysis_date),
            top_infringing_products,
            model.overall_risk_assessment.clone().unwrap(),
            unix_timestamp(&model.created_at.unwrap()),
            unix_timestamp(&model.updated_at.unwrap()),
        )
    }
}
