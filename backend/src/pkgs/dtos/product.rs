use std::fmt;

////////////////////////////////////////////////////////////////////////////////

pub struct Product {
    pub name: String,
    pub description: String,
}

impl Product {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
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
