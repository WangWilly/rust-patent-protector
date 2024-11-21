use crate::pkgs::dtos::product::Product;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Company {
    pub name: String,
    pub products: Vec<Product>,
}

impl Company {
    pub fn new(name: String, products: Vec<Product>) -> Self {
        Self { name, products }
    }

    pub fn from_json(json_data: Value) -> Self {
        let name = json_data["name"].as_str().unwrap().to_string();
        let products = json_data["products"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| Product::from_json(p.clone()))
            .collect();
        Self { name, products }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }
}

//////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Company {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let products_str = self
            .products
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}: {}", self.name, products_str)
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Companies {
    pub companies: HashMap<String, Company>,
}

impl Companies {
    pub fn new() -> Self {
        Self {
            companies: HashMap::new(),
        }
    }

    pub fn from_json(json_data: Value) -> Self {
        let mut companies = HashMap::new();

        let companies_json = json_data.get("companies").unwrap().as_array().unwrap();
        for company_json in companies_json {
            let company = Company::from_json(company_json.clone());
            companies.insert(company.name.clone(), company);
        }
        Self { companies }
    }

    pub fn add_company(&mut self, company: Company) {
        self.companies.insert(company.name.clone(), company);
    }

    pub fn get(&self, name: String) -> Option<&Company> {
        self.companies.get(&name)
    }
}

//////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Companies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let companies_str = self
            .companies
            .iter()
            .map(|(name, company)| format!("{}: {}", name, company.to_string()))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", companies_str)
    }
}
