use diesel::prelude::*;
use std::time::SystemTime;

use crate::pkgs::schema::assessments;

////////////////////////////////////////////////////////////////////////////////

#[derive(Queryable)]
pub struct Assessment {
    pub id: i32,
    pub patent_id: String,
    pub company_name: String,
    pub analysis_date: SystemTime,
    pub top_infringing_products: Option<serde_json::Value>,
    pub overall_risk_assessment: Option<String>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = assessments)]
pub struct NewAssessment {
    pub patent_id: String,
    pub company_name: String,
    pub analysis_date: SystemTime,
    pub top_infringing_products: Option<serde_json::Value>,
    pub overall_risk_assessment: Option<String>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}
