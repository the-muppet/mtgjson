use async_trait::async_trait;
use pyo3::prelude::*;
use reqwest::Response;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use regex::Regex;
use chrono::{DateTime, Utc};
use crate::prices::MtgjsonPrices;
use super::{AbstractProvider, BaseProvider, ProviderError, ProviderResult};

#[pyclass(name = "WizardsProvider")]
pub struct WizardsProvider {
    base: BaseProvider,
    magic_rules_url: String,
    magic_rules: String,
    one_week_ago: i64,
}

impl WizardsProvider {
    const TRANSLATION_URL: &'static str = "https://magic.wizards.com/{}/products/card-set-archive";
    const INITIAL_MAGIC_RULES_URL: &'static str = "https://magic.wizards.com/en/rules";
}

#[pymethods]
impl WizardsProvider {
    #[new]
    pub fn new() -> PyResult<Self> {
        let headers = HashMap::new();
        let base = BaseProvider::new("wizards".to_string(), headers);
        
        // Calculate one week ago timestamp
        let now = Utc::now().timestamp();
        let one_week_ago = now - (7 * 86400); // 7 days * 24 hours * 60 minutes * 60 seconds
        
        Ok(WizardsProvider {
            base,
            magic_rules_url: Self::INITIAL_MAGIC_RULES_URL.to_string(),
            magic_rules: String::new(),
            one_week_ago,
        })
    }

    /// Build HTTP header (returns empty dict)
    pub fn _build_http_header(&self) -> PyResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    /// Download from Wizard's website
    pub fn download(&mut self, url: String, params: Option<HashMap<String, String>>) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match self.base.get(&url, params).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let text = response.text().await.map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Text parse error: {}", e))
                        })?;
                        Ok(text)
                    } else {
                        println!("Error downloading Wizards data: {}", response.status());
                        Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                            format!("HTTP error: {}", response.status())
                        ))
                    }
                },
                Err(e) => {
                    println!("Error downloading Wizards data: {}", e);
                    Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                        format!("Request error: {}", e)
                    ))
                }
            }
        })
    }

    /// Download the comp rules from Wizards site
    pub fn get_magic_rules(&mut self) -> PyResult<String> {
        if !self.magic_rules.is_empty() {
            return Ok(self.magic_rules.clone());
        }

        // First, get the rules page to find the actual rules URL
        let response = self.download(self.magic_rules_url.clone(), None)?;

        // Extract the .txt file URL using regex
        let re = Regex::new(r#"href="([^"]*\.txt)""#).unwrap();
        if let Some(captures) = re.captures(&response) {
            if let Some(txt_url) = captures.get(1) {
                self.magic_rules_url = txt_url.as_str().to_string();
            }
        }

        // Now download the actual rules file
        let rules_response = self.download(self.magic_rules_url.clone(), None)?;
        
        // Clean up the text similar to Python version
        let cleaned_rules = rules_response
            .replace("â€™", "'") // Replace weird apostrophe encoding
            .lines()
            .collect::<Vec<&str>>()
            .join("\n");

        self.magic_rules = cleaned_rules.clone();
        Ok(cleaned_rules)
    }

    /// Get the translation URL template
    #[getter]
    pub fn get_translation_url(&self) -> PyResult<String> {
        Ok(Self::TRANSLATION_URL.to_string())
    }

    /// Get the magic rules URL
    #[getter]
    pub fn get_magic_rules_url(&self) -> PyResult<String> {
        Ok(self.magic_rules_url.clone())
    }

    /// Get the one week ago timestamp
    #[getter]
    pub fn get_one_week_ago(&self) -> PyResult<i64> {
        Ok(self.one_week_ago)
    }

    /// Get the cached magic rules
    #[getter]
    pub fn get_magic_rules_cached(&self) -> PyResult<String> {
        Ok(self.magic_rules.clone())
    }

    /// Set the magic rules URL (for testing or updates)
    #[setter]
    pub fn set_magic_rules_url(&mut self, url: String) -> PyResult<()> {
        self.magic_rules_url = url;
        Ok(())
    }
}

#[async_trait]
impl AbstractProvider for WizardsProvider {
    fn get_class_id(&self) -> &str {
        &self.base.class_id
    }
    
    fn get_class_name(&self) -> &str {
        "WizardsProvider"
    }
    
    fn build_http_header(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    
    async fn download(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Value> {
        // This provider primarily returns HTML/text, not JSON
        let text = self.download_raw(url, params).await?;
        Ok(Value::String(text))
    }
    
    async fn download_raw(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<String> {
        let response = self.base.get_request(url, params).await?;
        self.log_download(&response);
        
        response.text().await.map_err(|e| {
            ProviderError::NetworkError(format!("Text download error: {}", e))
        })
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

    async fn download_async(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Response> {
        self.base.get(url, params).await
    }

    async fn generate_today_price_dict(&self, _all_printings_path: &str) -> ProviderResult<HashMap<String, MtgjsonPrices>> {
        // Wizards doesn't provide price data
        Ok(HashMap::new())
    }
}