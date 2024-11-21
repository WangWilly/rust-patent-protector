use crate::pkgs::dtos::patent_clam::PatentClam;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Patent {
    pub id: i32,
    pub publication_number: String,
    pub title: String,
    pub patent_abstract: String,
    pub description: String,
    pub claims: Vec<PatentClam>,
}

impl Patent {
    pub fn new(
        id: i32,
        publication_number: String,
        title: String,
        patent_abstract: String,
        description: String,
        claims: Vec<PatentClam>,
    ) -> Self {
        Self {
            id,
            publication_number,
            title,
            patent_abstract,
            description,
            claims,
        }
    }

    pub fn from_json(json_data: Value) -> Self {
        let id = json_data["id"].as_i64().unwrap() as i32;
        let publication_number = json_data["publication_number"]
            .as_str()
            .unwrap()
            .to_string();
        let title = json_data["title"].as_str().unwrap().to_string();
        let patent_abstract = json_data["abstract"].as_str().unwrap().to_string();
        let description = json_data["description"].as_str().unwrap().to_string();
        let claims_array_str = json_data["claims"].as_str().unwrap();
        let claims_json = serde_json::from_str(claims_array_str).unwrap();
        let claims = PatentClam::from_json_array(claims_json);
        Self {
            id,
            publication_number,
            title,
            patent_abstract,
            description,
            claims,
        }
    }

    pub fn add_claim(&mut self, claim: PatentClam) {
        self.claims.push(claim);
    }

    pub fn str_for_gpt(&self) -> String {
        let claims_str = self
            .claims
            .iter()
            .map(|c| c.text.clone())
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "The patent '{}' is about {}. Description: {}, Claims: [{}]",
            self.title, self.patent_abstract, self.description, claims_str
        )
    }
}

//////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Patent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} - {}",
            self.publication_number, self.title, self.patent_abstract
        )
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Patents {
    pub pub_patent_map: HashMap<String, Patent>,
}

impl Patents {
    pub fn new() -> Self {
        Self {
            pub_patent_map: HashMap::new(),
        }
    }

    pub fn from_json(json_data: Value) -> Self {
        let mut pub_patent_map = HashMap::new();
        for patent_json in json_data.as_array().unwrap() {
            let patent = Patent::from_json(patent_json.clone());
            pub_patent_map.insert(patent.publication_number.clone(), patent);
        }
        Self { pub_patent_map }
    }

    pub fn add_patent(&mut self, patent: Patent) {
        self.pub_patent_map
            .insert(patent.publication_number.clone(), patent);
    }

    pub fn get(&self, publication_number: &str) -> Option<&Patent> {
        self.pub_patent_map.get(publication_number)
    }
}

//////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Patents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let patents_str = self
            .pub_patent_map
            .iter()
            .map(|(pub_num, patent)| format!("{}: {}", pub_num, patent.to_string()))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", patents_str)
    }
}
