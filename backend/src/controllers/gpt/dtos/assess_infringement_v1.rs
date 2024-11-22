use crate::pkgs::{
    dtos::{company::Company, infrigement::ProductInfrigement, patent::Patent},
    time::iso8601,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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

impl AssessInfringementV1Resp {
    pub fn new(
        patent_id: String,
        company_name: String,
        analysis_date: String,
        top_infringing_products: Vec<AssessInfringementV1ProductItem>,
        overall_risk_assessment: String,
    ) -> Self {
        Self {
            patent_id,
            company_name,
            analysis_date,
            top_infringing_products,
            overall_risk_assessment,
        }
    }

    pub fn from_patent_company(
        patent: &Patent,
        company: &Company,
        infrigements: &Vec<ProductInfrigement>,
        summary: &String,
    ) -> Self {
        let top_infringing_products = infrigements
            .iter()
            .map(|infrigement| {
                AssessInfringementV1ProductItem::from_product_infringement_dto(infrigement)
            })
            .collect();

        Self::new(
            patent.publication_number.clone(),
            company.name.clone(),
            iso8601(&SystemTime::now()),
            top_infringing_products,
            summary.clone(),
        )
    }
}

#[derive(Serialize)]
pub struct AssessInfringementV1ProductItem {
    pub product_name: String,
    pub infringement_likelihood: String,
    pub relevant_claims: Vec<String>,
    pub explanation: String,
    pub specific_features: Vec<String>,
}

impl AssessInfringementV1ProductItem {
    pub fn new(
        product_name: String,
        infringement_likelihood: String,
        relevant_claims: Vec<String>,
        explanation: String,
        specific_features: Vec<String>,
    ) -> Self {
        Self {
            product_name,
            infringement_likelihood,
            relevant_claims,
            explanation,
            specific_features,
        }
    }

    pub fn from_product_infringement_dto(product_infringement: &ProductInfrigement) -> Self {
        Self::new(
            product_infringement.product_name.clone(),
            format!(
                "{:.2}%",
                product_infringement.infringement_likelihood * 100.0
            ),
            product_infringement.relevant_claims.clone(),
            product_infringement.explanation.clone(),
            product_infringement.specific_features.clone(),
        )
    }
}
