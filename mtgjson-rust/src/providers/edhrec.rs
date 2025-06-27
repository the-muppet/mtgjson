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
        let headers = HashMap::new();
        let base = BaseProvider::new("edhrec".to_string(), headers);
        
        // TODO: Read from MtgjsonConfig in a real implementation
        let keys_found = false; // MtgjsonConfig().has_option("EDHRec", "api_url")
        let api_url = String::new(); // MtgjsonConfig().get("EDHRec", "api_url")
        
        if !keys_found {
            println!("EDHRec keys values missing. Skipping imports");
        }
        
        Ok(Self {
            base,
            keys_found,
            api_url,
            data_table: HashMap::new(),
        })
    }
    
    /// Get salt rating for a card
    pub fn get_salt_rating(&mut self, card_name: &str) -> PyResult<Option<f64>> {
        if self.data_table.is_empty() {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(async {
                self.generate_data_table().await
            })?;
        }
        
        let salt_rating = self.data_table.get(card_name)
            .and_then(|entry| entry.get("salt"))
            .and_then(|v| v.as_f64());
        
        Ok(salt_rating.map(|rating| (rating * 100.0).round() / 100.0)) // Round to 2 decimal places
    }
}

impl EdhrecProviderCardRanks {
    async fn generate_data_table(&mut self) -> ProviderResult<()> {
        if !self.keys_found {
            return Ok(());
        }
        
        let raw_json_data = self.download(&self.api_url, None).await?;
        
        if let Some(array) = raw_json_data.as_array() {
            for entry in array {
                if let Some(obj) = entry.as_object() {
                    if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                        let mut entry_data = HashMap::new();
                        for (key, value) in obj {
                            if key != "name" {
                                entry_data.insert(key.clone(), value.clone());
                            }
                        }
                        self.data_table.insert(name.to_string(), entry_data);
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
    
    async fn download(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Value> {
        self.base.download_json(url, params).await
    }
    
    async fn download_raw(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<String> {
        self.base.download_text(url, params).await
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