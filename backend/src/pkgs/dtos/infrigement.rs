use crate::pkgs::dtos::{patent::Patent, product::Product};
use std::fmt;

use super::patent;

//////////////////////////////////////////////////////////////////////////////

pub struct ProductInfrigement {
    pub patent_pub_id: String,
    pub product_name: String,
    pub infringement_likelihood: f32,
    pub relevant_claims: Vec<String>,
    pub explanation: String,
    pub specific_features: Vec<String>,
}

impl ProductInfrigement {
    pub fn new(
        patent_pub_id: String,
        product_name: String,
        infringement_likelihood: f32,
        relevant_claims: Vec<String>,
        explanation: String,
        specific_features: Vec<String>,
    ) -> Self {
        Self {
            patent_pub_id,
            product_name,
            infringement_likelihood,
            relevant_claims,
            explanation,
            specific_features,
        }
    }

    pub fn str_for_gpt(&self) -> String {
        let specific_features_str = self.specific_features.join(", ");
        format!(
            "The evidence suggests that the product '{}' infringes on the patent '{}' with a likelihood of {}. The explanation is: {}. The specific features are {}",
            self.product_name, self.patent_pub_id, self.infringement_likelihood, self.explanation, specific_features_str
        )
    }

    pub fn from_gpt_response(patent: &Patent, product: &Product, response: String) -> Self {
        let lines = response.split("\n").collect::<Vec<&str>>();
        let mut relevant_claims: Vec<String> = Vec::new();
        let mut explanation = String::new();
        let mut specific_features: Vec<String> = Vec::new();

        for line in lines {
            if line.starts_with("relevant_claims:") {
                let claims_str = line.split(":").collect::<Vec<&str>>()[1].trim();
                relevant_claims = claims_str
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect();
            } else if line.starts_with("explanation:") {
                explanation = line.split(":").collect::<Vec<&str>>()[1].trim().to_string();
            } else if line.starts_with("specific_features:") {
                let features_str = line.split(":").collect::<Vec<&str>>()[1].trim();
                specific_features = features_str
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        }

        let likelihood = relevant_claims.len() as f32 / patent.claims.len() as f32;

        Self::new(
            patent.publication_number.clone(),
            product.name.clone(),
            likelihood,
            relevant_claims,
            explanation,
            specific_features,
        )
    }
}

//////////////////////////////////////////////////////////////////////////////

impl fmt::Display for ProductInfrigement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let relevant_claims_str = self.relevant_claims.join(", ");
        let specific_features_str = self.specific_features.join(", ");
        write!(
            f,
            "Product '{}' infringes patent '{}'. Likelihood: {}. Relevant claims: [{}], Explanation: {}, Specific features: [{}]",
            self.product_name, self.patent_pub_id, self.infringement_likelihood, relevant_claims_str, self.explanation, specific_features_str
        )
    }
}
