use async_trait::async_trait;
use pyo3::prelude::*;
use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;
use crate::prices::MtgjsonPrices;
use super::{AbstractProvider, BaseProvider, ProviderError, ProviderResult};

#[pyclass(name = "MTGBanProvider")]
pub struct MTGBanProvider {
    base: BaseProvider,
    api_url: String,
    keys_found: bool,
    mtgjson_to_card_kingdom: HashMap<String, HashMap<String, HashMap<String, String>>>,
}

impl MTGBanProvider {
    const API_URL_TEMPLATE: &'static str = "https://www.mtgban.com/api/mtgjson/ck.json?sig={}";
}

#[pymethods]
impl MTGBanProvider {
    #[new]
    pub fn new() -> PyResult<Self> {
        let headers = Self::build_http_header_static()?;
        let base = BaseProvider::new("mtgban".to_string(), headers.clone());
        
        // TODO: In real implementation, read from MtgjsonConfig
        let has_mtgban_section = false; // MtgjsonConfig().has_section("MTGBan")
        let has_api_key = false; // MtgjsonConfig().has_option("MTGBan", "api_key")
        
        let (keys_found, api_url) = if !has_mtgban_section {
            println!("MTGBan section not established. Skipping alerts");
            (false, String::new())
        } else if has_api_key {
            // let api_key = MtgjsonConfig().get("MTGBan", "api_key");
            let api_key = String::new(); // Placeholder
            (true, Self::API_URL_TEMPLATE.replace("{}", &api_key))
        } else {
            println!("MTGBan keys values missing. Skipping imports");
            (false, String::new())
        };
        
        Ok(MTGBanProvider {
            base,
            api_url,
            keys_found,
            mtgjson_to_card_kingdom: HashMap::new(),
        })
    }

    /// Build HTTP header (returns empty dict like Python version)
    pub fn _build_http_header(&self) -> PyResult<HashMap<String, String>> {
        Self::build_http_header_static()
    }

    /// Download a URL
    pub fn download(&mut self, url: String, params: Option<HashMap<String, String>>) -> PyResult<Value> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match self.base.get(&url, params).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let json: Value = response.json().await.map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON parse error: {}", e))
                        })?;
                        Ok(json)
                    } else {
                        println!("MTGBan Download Error ({}): {}", response.status(), response.status());
                        Ok(Value::Object(serde_json::Map::new()))
                    }
                },
                Err(e) => {
                    println!("Unable to download from MTGBan: {}", e);
                    Ok(Value::Object(serde_json::Map::new()))
                }
            }
        })
    }

    /// Get MTGJSON to Card Kingdom translation table
    pub fn get_mtgjson_to_card_kingdom(&mut self) -> PyResult<HashMap<String, HashMap<String, HashMap<String, String>>>> {
        if !self.keys_found {
            return Ok(HashMap::new());
        }

        if self.mtgjson_to_card_kingdom.is_empty() {
            let data = self.download(self.api_url.clone(), None)?;
            
            // Parse the nested JSON structure
            if let Some(data_obj) = data.as_object() {
                for (level1_key, level1_value) in data_obj {
                    if let Some(level1_obj) = level1_value.as_object() {
                        let mut level1_map = HashMap::new();
                        
                        for (level2_key, level2_value) in level1_obj {
                            if let Some(level2_obj) = level2_value.as_object() {
                                let mut level2_map = HashMap::new();
                                
                                for (level3_key, level3_value) in level2_obj {
                                    if let Some(level3_str) = level3_value.as_str() {
                                        level2_map.insert(level3_key.clone(), level3_str.to_string());
                                    }
                                }
                                level1_map.insert(level2_key.clone(), level2_map);
                            }
                        }
                        self.mtgjson_to_card_kingdom.insert(level1_key.clone(), level1_map);
                    }
                }
            }
        }

        Ok(self.mtgjson_to_card_kingdom.clone())
    }

    /// Get the API URL (property access)
    #[getter]
    pub fn get_api_url(&self) -> PyResult<String> {
        Ok(self.api_url.clone())
    }

    /// Get the keys found status (property access)
    #[getter]
    pub fn get_keys_found(&self) -> PyResult<bool> {
        Ok(self.keys_found)
    }

    /// Static method to build HTTP header (for use in constructor)
    fn build_http_header_static() -> PyResult<HashMap<String, String>> {
        // MTGBan doesn't require special headers
        Ok(HashMap::new())
    }
}

#[async_trait]
impl AbstractProvider for MTGBanProvider {
    async fn download_async(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Response> {
        self.base.get(url, params).await
    }

    async fn generate_today_price_dict(&self, _all_printings_path: &str) -> ProviderResult<HashMap<String, MtgjsonPrices>> {
        // MTGBan provides translation data, not direct prices
        Ok(HashMap::new())
    }
}