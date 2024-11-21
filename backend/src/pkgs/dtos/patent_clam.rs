use std::fmt;

////////////////////////////////////////////////////////////////////////////////

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
}

////////////////////////////////////////////////////////////////////////////////

impl fmt::Display for PatentClam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.patent_number, self.text)
    }
}
