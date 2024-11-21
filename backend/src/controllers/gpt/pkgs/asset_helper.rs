use std::{env, fs};

use crate::info;

use crate::pkgs::dtos::{company::Companies, patent::Patents};

////////////////////////////////////////////////////////////////////////////////

pub struct AssetHelperConfig {
    pub company_products_path: String,
    pub patents_path: String,
}

impl AssetHelperConfig {
    pub fn from_env() -> Self {
        Self {
            company_products_path: env::var("ASSET_HELPER_COMPANY_PRODUCTS_PATH")
                .expect("ASSET_HELPER_COMPANY_PRODUCTS_PATH is required"),
            patents_path: env::var("ASSET_HELPER_PATENTS_PATH")
                .expect("ASSET_HELPER_PATENTS_PATH is required"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AssetHelper {
    company_products: Companies,
    patents: Patents,
}

impl AssetHelper {
    pub fn new(company_products: Companies, patents: Patents) -> Self {
        Self {
            company_products,
            patents,
        }
    }

    pub fn from_config(config: AssetHelperConfig) -> Self {
        info!(
            "Loading company products from: {}",
            config.company_products_path
        );
        let company_products_json =
            serde_json::from_str(&fs::read_to_string(config.company_products_path).unwrap())
                .unwrap();
        let company_products = Companies::from_json(company_products_json);

        info!("Loading patents from: {}", config.patents_path);
        let patents_json =
            serde_json::from_str(&fs::read_to_string(config.patents_path).unwrap()).unwrap();
        let patents = Patents::from_json(patents_json);

        Self::new(company_products, patents)
    }

    pub fn get_company_products(&self) -> &Companies {
        &self.company_products
    }

    pub fn get_patents(&self) -> &Patents {
        &self.patents
    }
}
