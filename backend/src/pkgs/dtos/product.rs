use serde_json::Value;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Product {
    pub name: String,
    pub description: String,
}

impl Product {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }

    pub fn from_json(json_data: Value) -> Self {
        Self::new(
            json_data["name"].as_str().unwrap().to_string(),
            json_data["description"].as_str().unwrap().to_string(),
        )
    }

    pub fn str_for_gpt(&self) -> String {
        format!("The product '{}' is about {}", self.name, self.description)
    }
}

////////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}
