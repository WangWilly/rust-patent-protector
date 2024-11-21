use crate::pkgs::dtos::{
    company::Company, infrigement::ProductInfrigement, patent::Patent, product::Product,
};
use reqwest::header::HeaderMap;
use reqwest::{Client, RequestBuilder};
use serde_json::json;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////

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

    fn base_body() -> HashMap<String, String> {
        let mut body: HashMap<String, String> = HashMap::new();
        body.insert("model".to_string(), "llama3-8b-8192".to_string());
        body.insert("temperature".to_string(), "0".to_string());
        body
    }

    fn build_post_request(&self, body: HashMap<String, String>) -> RequestBuilder {
        let headers = self.base_headers();
        self.client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .headers(headers)
            .json(&body)
    }

    ////////////////////////////////////////////////////////////////////////////
    /// v1

    async fn assess_product_v1(
        &self,
        patent: &Patent,
        product: &Product,
    ) -> Result<ProductInfrigement, reqwest::Error> {
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
        body.insert("messages".to_string(), msg_json.to_string());

        let request = self.build_post_request(body);
        let response = request.send().await;
        if response.is_err() {
            return Err(response.err().unwrap());
        }

        let json_response = response.unwrap().json().await;
        if json_response.is_err() {
            return Err(json_response.err().unwrap());
        }

        let response_json: serde_json::Value = json_response.unwrap();
        let response_str = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap();
        Ok(ProductInfrigement::from_gpt_response(
            patent,
            product,
            response_str.to_string(),
        ))
    }

    async fn assess_company_v1(
        &self,
        patent: &Patent,
        company: &Company,
    ) -> Vec<ProductInfrigement> {
        let mut infringements = Vec::new();
        for product in &company.products {
            let infringement = self.assess_product_v1(patent, product).await;
            if infringement.is_err() {
                continue;
            }
            infringements.push(infringement.unwrap());
        }
        infringements
    }

    async fn summarize_v1(
        &self,
        patent: &Patent,
        company: &Company,
        infringements: &Vec<ProductInfrigement>,
    ) -> String {
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
        body.insert("messages".to_string(), msg_json.to_string());

        let request = self.build_post_request(body);
        let response = request.send().await.unwrap();
        response.text().await.unwrap()
    }
}
