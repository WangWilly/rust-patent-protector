use crate::pkgs::dtos::product::Product;
use std::fmt;

//////////////////////////////////////////////////////////////////////////////

pub struct Company {
    pub name: String,
    pub products: Vec<Product>,
}

impl Company {
    pub fn new(name: String, products: Vec<Product>) -> Self {
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
