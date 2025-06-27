use async_trait::async_trait;
use pyo3::prelude::*;
use reqwest::Response;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use crate::prices::MtgjsonPrices;
use super::{AbstractProvider, BaseProvider, ProviderError, ProviderResult};

#[pyclass(name = "EdhrecProviderCardRanks")]
pub struct EdhrecProviderCardRanks {
    base: BaseProvider,
    keys_found: bool,
    api_url: String,
    data_table: HashMap<String, HashMap<String, Value>>,
}

#[pymethods]
impl EdhrecProviderCardRanks {
    #[new]
    pub fn new() -> PyResult<Self> {
        // TODO: In a real implementation, read from MtgjsonConfig
        // For now, simulate the config check
        let keys_found = false; // MtgjsonConfig().has_option("EDHRec", "api_url")
        let api_url = String::new(); // MtgjsonConfig().get("EDHRec", "api_url")
        
        let headers = HashMap::new();
        let base = BaseProvider::new("edhrec".to_string(), headers);
        
        if !keys_found {
            println!("EDHRec keys values missing. Skipping imports");
        }
        
        Ok(EdhrecProviderCardRanks {
            base,
            keys_found,
            api_url,
            data_table: HashMap::new(),
        })
    }

    /// Download JSON data from EDHRec API
    pub fn download(&mut self, url: String, params: Option<HashMap<String, String>>) -> PyResult<Value> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match self.base.get(&url, params).await {
                Ok(response) => {
                    let json: Value = response.json().await.map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON parse error: {}", e))
                    })?;
                    Ok(json)
                },
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Request error: {}", e)))
            }
        })
    }

    /// Get salt rating for a card name
    pub fn get_salt_rating(&mut self, card_name: String) -> PyResult<Option<f64>> {
        if self.data_table.is_empty() {
            self.generate_data_table()?;
        }

        if let Some(card_data) = self.data_table.get(&card_name) {
            if let Some(salt_value) = card_data.get("salt") {
                if let Some(salt_float) = salt_value.as_f64() {
                    return Ok(Some((salt_float * 100.0).round() / 100.0)); // Round to 2 decimal places
                }
            }
        }
        Ok(None)
    }

    /// Build HTTP header (returns empty dict like Python version)
    pub fn _build_http_header(&self) -> PyResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    /// Generate data table from EDHRec API
    fn generate_data_table(&mut self) -> PyResult<()> {
        if !self.keys_found {
            return Ok(());
        }

        let raw_json_data = self.download(self.api_url.clone(), None)?;
        
        if let Some(entries) = raw_json_data.as_array() {
            for entry in entries {
                if let Some(entry_obj) = entry.as_object() {
                    if let Some(name_value) = entry_obj.get("name") {
                        if let Some(entry_name) = name_value.as_str() {
                            let mut entry_data = HashMap::new();
                            for (key, value) in entry_obj {
                                if key != "name" {
                                    entry_data.insert(key.clone(), value.clone());
                                }
                            }
                            self.data_table.insert(entry_name.to_string(), entry_data);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl AbstractProvider for EdhrecProviderCardRanks {
    fn get_class_id(&self) -> &str {
        &self.base.class_id
    }
    
    fn get_class_name(&self) -> &str {
        "EdhrecProviderCardRanks"
    }
    
    fn build_http_header(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    
    async fn download_async(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Response> {
        self.base.get(url, params).await
    }
    
    async fn generate_today_price_dict(&self, _all_printings_path: &str) -> ProviderResult<HashMap<String, MtgjsonPrices>> {
        // EDHRec doesn't provide price data
        Ok(HashMap::new())
    }
    
    fn log_download(&self, response: &Response) {
        println!("Downloaded {} (Status: {})", response.url(), response.status());
    }
    
    fn generic_generate_today_price_dict(
        &self,
        _third_party_to_mtgjson: &HashMap<String, HashSet<String>>,
        _price_data_rows: &[Value],
        _card_platform_id_key: &str,
        _default_prices_object: &MtgjsonPrices,
        _foil_key: &str,
        _retail_key: Option<&str>,
        _retail_quantity_key: Option<&str>,
        _buy_key: Option<&str>,
        _buy_quantity_key: Option<&str>,
        _etched_key: Option<&str>,
        _etched_value: Option<&str>,
    ) -> HashMap<String, MtgjsonPrices> {
        HashMap::new()
    }
}