use async_trait::async_trait;
use pyo3::prelude::*;
use reqwest::Response;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use crate::prices::MtgjsonPrices;
use super::{AbstractProvider, BaseProvider, ProviderError, ProviderResult};

#[pyclass(name = "WhatsInStandardProvider")]
pub struct WhatsInStandardProvider {
    base: BaseProvider,
    set_codes: HashSet<String>,
    standard_legal_sets: HashSet<String>,
}

impl WhatsInStandardProvider {
    const API_ENDPOINT: &'static str = "https://whatsinstandard.com/api/v6/standard.json";
}

#[pymethods]
impl WhatsInStandardProvider {
    #[new]
    pub fn new() -> PyResult<Self> {
        let headers = HashMap::new();
        let base = BaseProvider::new("standard".to_string(), headers);
        
        let mut provider = WhatsInStandardProvider {
            base,
            set_codes: HashSet::new(),
            standard_legal_sets: HashSet::new(),
        };
        
        // Initialize set codes
        provider.set_codes = provider.standard_legal_set_codes()?;
        
        Ok(provider)
    }

    /// Build HTTP header (returns empty dict)
    pub fn _build_http_header(&self) -> PyResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    /// Download content from Whats in Standard
    /// API calls always return JSON from them
    pub fn download(&mut self, url: String, params: Option<HashMap<String, String>>) -> PyResult<Value> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut retry_count = 0;
            let max_retries = 5;
            
            loop {
                match self.base.get(&url, params.clone()).await {
                    Ok(response) => {
                        if response.status().is_success() {
                            let json: Value = response.json().await.map_err(|e| {
                                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON parse error: {}", e))
                            })?;
                            return Ok(json);
                        } else {
                            println!("WhatsInStandard Download Error ({}): {}", response.status(), response.status());
                            if retry_count < max_retries {
                                retry_count += 1;
                                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                continue;
                            } else {
                                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                    format!("Max retries exceeded for URL: {}", url)
                                ));
                            }
                        }
                    },
                    Err(e) => {
                        if retry_count < max_retries {
                            retry_count += 1;
                            println!("WhatsInStandard connection error, retrying: {}", e);
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                            continue;
                        } else {
                            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                format!("Request error after retries: {}", e)
                            ));
                        }
                    }
                }
            }
        })
    }

    /// Get all set codes from sets that are currently legal in Standard
    pub fn standard_legal_set_codes(&mut self) -> PyResult<HashSet<String>> {
        if !self.standard_legal_sets.is_empty() {
            return Ok(self.standard_legal_sets.clone());
        }

        let api_response = self.download(Self::API_ENDPOINT.to_string(), None)?;
        let mut standard_set_codes = HashSet::new();
        
        if let Some(sets) = api_response.get("sets") {
            if let Some(sets_array) = sets.as_array() {
                let now = Utc::now();
                
                for set_object in sets_array {
                    if let Some(set_obj) = set_object.as_object() {
                        // Get set code
                        if let Some(code) = set_obj.get("code").and_then(|v| v.as_str()) {
                            // Parse enter date
                            let enter_date = set_obj.get("enterDate")
                                .and_then(|ed| ed.get("exact"))
                                .and_then(|exact| exact.as_str())
                                .and_then(|date_str| {
                                    if date_str.is_empty() {
                                        Some(DateTime::<Utc>::from_timestamp(9999999999, 0).unwrap())
                                    } else {
                                        date_str.parse::<DateTime<Utc>>().ok()
                                    }
                                });
                            
                            // Parse exit date  
                            let exit_date = set_obj.get("exitDate")
                                .and_then(|ed| ed.get("exact"))
                                .and_then(|exact| exact.as_str())
                                .and_then(|date_str| {
                                    if date_str.is_empty() {
                                        Some(DateTime::<Utc>::from_timestamp(9999999999, 0).unwrap())
                                    } else {
                                        date_str.parse::<DateTime<Utc>>().ok()
                                    }
                                });
                            
                            // Check if set is currently legal in standard
                            if let (Some(enter), Some(exit)) = (enter_date, exit_date) {
                                if enter <= now && now <= exit {
                                    standard_set_codes.insert(code.to_uppercase());
                                }
                            }
                        }
                    }
                }
            }
        }

        self.standard_legal_sets = standard_set_codes.clone();
        Ok(standard_set_codes)
    }

    /// Get the set codes (property access)
    #[getter]
    pub fn get_set_codes(&self) -> PyResult<HashSet<String>> {
        Ok(self.set_codes.clone())
    }

    /// Get the standard legal sets (property access)
    #[getter]
    pub fn get_standard_legal_sets(&self) -> PyResult<HashSet<String>> {
        Ok(self.standard_legal_sets.clone())
    }
}

#[async_trait]
impl AbstractProvider for WhatsInStandardProvider {
    async fn download_async(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Response> {
        self.base.get(url, params).await
    }

    async fn generate_today_price_dict(&self, _all_printings_path: &str) -> ProviderResult<HashMap<String, MtgjsonPrices>> {
        // WhatsInStandard doesn't provide price data
        Ok(HashMap::new())
    }
}