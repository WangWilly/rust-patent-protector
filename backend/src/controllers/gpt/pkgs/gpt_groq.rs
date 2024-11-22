use crate::pkgs::dtos::{
    company::Company, infrigement::ProductInfrigement, patent::Patent, product::Product,
};
use crate::pkgs::errors::Error;
use reqwest::header::HeaderMap;
use reqwest::{Client, RequestBuilder};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::error;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct GroqConfig {
    pub api_key: String,
}

impl GroqConfig {
    pub fn from_env() -> Self {
        Self {
            api_key: std::env::var("GPT_GROQ_API_KEY").unwrap(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Groq {
    api_key: String,
    client: Client,
}

impl Groq {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub fn from_config(config: GroqConfig) -> Self {
        Self::new(config.api_key)
    }

    ////////////////////////////////////////////////////////////////////////////

    fn base_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers
    }

    fn base_body() -> HashMap<String, Value> {
        let mut body: HashMap<String, Value> = HashMap::new();
        body.insert("model".to_string(), json!("llama3-8b-8192"));
        body.insert("temperature".to_string(), json!(0));
        body
    }

    fn build_post_request(&self, body: HashMap<String, Value>) -> RequestBuilder {
        let headers = self.base_headers();
        self.client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .headers(headers)
            .json(&body)
    }

    ////////////////////////////////////////////////////////////////////////////
    /// v1

    pub async fn assess_product_v1(
        &self,
        patent: &Patent,
        product: &Product,
    ) -> Result<ProductInfrigement, Error> {
        let msg_json = json!([
            {
                "role": "system",
                "content": format!("You are a professional patent attorney. You have been asked to assess whether the product '{}' infringes on the patent '{}'. Please provide a detailed explanation of your assessment.", product.name, patent.publication_number)
            },
            {
                "role": "user",
                "content": format!("These are sources for you to assess.\n\n{}\n\n{}. I want you to reply strictly formatted like this:\n\nrelevant_claims: claim1_number, claim2_number, claim3_number\nexplanation: explanation\nspecific_features: feature1, feature2, feature3", patent.str_for_gpt(), product.str_for_gpt())
            }
        ]);

        let mut body = Self::base_body();
        body.insert("messages".to_string(), msg_json);

        let request = self.build_post_request(body);
        let response_raw = request.send().await;
        if response_raw.is_err() {
            let err = Error::ClientReqError {
                source: format!("{}", response_raw.err().unwrap()),
            };
            return Err(err);
        }

        let json_response = response_raw.unwrap().json().await;
        if json_response.is_err() {
            let err = Error::ClientReqError {
                source: format!("{}", json_response.err().unwrap()),
            };
            return Err(err);
        }

        let response: Value = json_response.unwrap();
        let response_str = response["choices"][0]["message"]["content"].as_str();
        if response_str.is_none() {
            let err = Error::ClientReqError {
                source: "response_str is None".to_string(),
            };
            return Err(err);
        }

        Ok(ProductInfrigement::from_gpt_response(
            patent,
            product,
            response_str.unwrap().to_string(),
        ))
    }

    pub async fn assess_company_v1(
        &self,
        patent: &Patent,
        company: &Company,
    ) -> Vec<ProductInfrigement> {
        let mut infringements = Vec::new();
        for product in &company.products {
            let infringement = self.assess_product_v1(patent, product).await;
            if infringement.is_err() {
                error!("Error assessing product: {}", infringement.err().unwrap());
                continue;
            }
            infringements.push(infringement.unwrap());
        }
        infringements
    }

    pub async fn summarize_v1(
        &self,
        patent: &Patent,
        company: &Company,
        infringements: &Vec<ProductInfrigement>,
    ) -> Result<String, reqwest::Error> {
        let infrigements_str = infringements
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("; ");
        let msg_json = json!([
            {
                "role": "system",
                "content": format!("You are a professional patent attorney. You have been asked to summarize the assessment of whether the company '{}' infringes on the patent '{}'. Please provide a detailed explanation of your assessment.", company.name, patent.publication_number)
            },
            {
                "role": "user",
                "content": format!("These are sources for you to summarize.\n\n{}\n\nGive a summary of the assessment.", infrigements_str)
            }
        ]);

        let mut body = Self::base_body();
        body.insert("messages".to_string(), msg_json);

        let request = self.build_post_request(body);
        let response_raw = request.send().await;
        if response_raw.is_err() {
            return Err(response_raw.err().unwrap());
        }
        let response_json = response_raw.unwrap().json().await;
        if response_json.is_err() {
            return Err(response_json.err().unwrap());
        }

        let response: Value = response_json.unwrap();
        let response_str = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap()
            .to_string();

        Ok(response_str)
    }
}
