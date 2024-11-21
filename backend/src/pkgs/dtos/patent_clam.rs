use serde_json::Value;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct PatentClam {
    pub patent_number: String,
    pub text: String,
}

impl PatentClam {
    pub fn new(patent_number: String, text: String) -> Self {
        Self {
            patent_number,
            text,
        }
    }

    pub fn from_json(json_data: Value) -> Self {
        Self::new(
            json_data["num"].as_str().unwrap().to_string(),
            json_data["text"].as_str().unwrap().to_string(),
        )
    }

    pub fn from_json_array(json_data: Value) -> Vec<Self> {
        json_data
            .as_array()
            .unwrap()
            .iter()
            .map(|p| Self::from_json(p.clone()))
            .collect()
    }
}

////////////////////////////////////////////////////////////////////////////////

impl fmt::Display for PatentClam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.patent_number, self.text)
    }
}
