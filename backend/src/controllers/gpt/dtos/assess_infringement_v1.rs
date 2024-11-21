use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
pub struct AssessInfringementV1Req {
    #[serde(rename = "patent_id")]
    pub patent_pub_id: String,
    pub company_name: String,
}

#[derive(Serialize)]
pub struct AssessInfringementV1Resp {
    pub patent_id: String,
    pub company_name: String,
    pub analysis_date: String,
    pub top_infringing_products: Vec<AssessInfringementV1ProductItem>,
    pub overall_risk_assessment: String,
}

#[derive(Serialize)]
pub struct AssessInfringementV1ProductItem {
    pub product_name: String,
    pub infringement_likelihood: String,
    pub relevant_claims: Vec<String>,
    pub explanation: String,
    pub specific_features: Vec<String>,
}
