use crate::pkgs::dtos::patent_clam::PatentClam;
use std::collections::HashMap;
use std::fmt;

//////////////////////////////////////////////////////////////////////////////

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

pub struct Patents {
    pub pub_patent_map: HashMap<String, Patent>,
}

impl Patents {
    pub fn new() -> Self {
        Self {
            pub_patent_map: HashMap::new(),
        }
    }

    pub fn add_patent(&mut self, patent: Patent) {
        self.pub_patent_map
            .insert(patent.publication_number.clone(), patent);
    }

    pub fn get_patent(&self, publication_number: &str) -> Option<&Patent> {
        self.pub_patent_map.get(publication_number)
    }
}
