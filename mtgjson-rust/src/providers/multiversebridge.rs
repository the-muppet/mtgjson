use async_trait::async_trait;
use pyo3::prelude::*;
use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;
use crate::prices::MtgjsonPrices;
use super::{AbstractProvider, BaseProvider, ProviderError, ProviderResult};

#[pyclass(name = "MultiverseBridgeProvider")]
pub struct MultiverseBridgeProvider {
    base: BaseProvider,
    rosetta_stone_cards: HashMap<String, Vec<HashMap<String, Value>>>,
    rosetta_stone_sets: HashMap<String, i32>,
}

impl MultiverseBridgeProvider {
    const ROSETTA_STONE_SETS_URL: &'static str = "https://www.multiversebridge.com/api/v1/sets";
    const ROSETTA_STONE_CARDS_URL: &'static str = "https://cdn.multiversebridge.com/mtgjson_build.json";
}

#[pymethods]
impl MultiverseBridgeProvider {
    #[new]
    pub fn new() -> PyResult<Self> {
        let headers = HashMap::new();
        let base = BaseProvider::new("mb".to_string(), headers);
        
        Ok(MultiverseBridgeProvider {
            base,
            rosetta_stone_cards: HashMap::new(),
            rosetta_stone_sets: HashMap::new(),
        })
    }

    /// Build HTTP header (returns empty dict)
    pub fn _build_http_header(&self) -> PyResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    /// Download content with retry logic
    pub fn download(&mut self, url: String, params: Option<HashMap<String, String>>) -> PyResult<Value> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut retry_count = 0;
            let max_retries = 3;
            
            loop {
                match self.base.get(&url, params.clone()).await {
                    Ok(response) => {
                        if response.status().is_success() {
                            let json: Value = response.json().await.map_err(|e| {
                                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON parse error: {}", e))
                            })?;
                            return Ok(json);
                        } else {
                            println!("MultiverseBridge Download Error ({}): {}", response.status(), response.status());
                            if retry_count < max_retries {
                                retry_count += 1;
                                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                continue;
                            } else {
                                return Ok(Value::Object(serde_json::Map::new()));
                            }
                        }
                    },
                    Err(e) => {
                        if retry_count < max_retries {
                            retry_count += 1;
                            println!("MultiverseBridge connection error, retrying: {}", e);
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                            continue;
                        } else {
                            println!("MultiverseBridge error after retries: {}", e);
                            return Ok(Value::Object(serde_json::Map::new()));
                        }
                    }
                }
            }
        })
    }

    /// Convert Rosetta Stone Card data into an index-able hashmap
    pub fn parse_rosetta_stone_cards(&mut self, rosetta_rows: Vec<Value>) -> PyResult<()> {
        for rosetta_row in rosetta_rows {
            if let Some(row_obj) = rosetta_row.as_object() {
                if let Some(scryfall_id) = row_obj.get("scryfall_id").and_then(|v| v.as_str()) {
                    let mut row_map = HashMap::new();
                    for (key, value) in row_obj {
                        row_map.insert(key.clone(), value.clone());
                    }
                    
                    self.rosetta_stone_cards
                        .entry(scryfall_id.to_string())
                        .or_insert_with(Vec::new)
                        .push(row_map);
                }
            }
        }
        Ok(())
    }

    /// Convert Rosetta Stone Set data into index-able hashmap
    pub fn parse_rosetta_stone_sets(&mut self, rosetta_rows: Vec<Value>) -> PyResult<()> {
        for rosetta_row in rosetta_rows {
            if let Some(row_obj) = rosetta_row.as_object() {
                if let (Some(mtgjson_code), Some(cs_id)) = (
                    row_obj.get("mtgjson_code").and_then(|v| v.as_str()),
                    row_obj.get("cs_id").and_then(|v| v.as_i64())
                ) {
                    self.rosetta_stone_sets.insert(mtgjson_code.to_string(), cs_id as i32);
                }
            }
        }
        Ok(())
    }

    /// Cache a copy of the Rosetta Stone from MB and give it back when needed
    pub fn get_rosetta_stone_cards(&mut self) -> PyResult<HashMap<String, Vec<HashMap<String, Value>>>> {
        if self.rosetta_stone_cards.is_empty() {
            let data = self.download(Self::ROSETTA_STONE_CARDS_URL.to_string(), None)?;
            if let Some(cards_array) = data.as_array() {
                self.parse_rosetta_stone_cards(cards_array.clone())?;
            }
        }
        Ok(self.rosetta_stone_cards.clone())
    }

    /// Cache a copy of the Rosetta Stone's Set IDs from MB and give it back when needed
    pub fn get_rosetta_stone_sets(&mut self) -> PyResult<HashMap<String, i32>> {
        if self.rosetta_stone_sets.is_empty() {
            let data = self.download(Self::ROSETTA_STONE_SETS_URL.to_string(), None)?;
            if let Some(sets_array) = data.as_array() {
                self.parse_rosetta_stone_sets(sets_array.clone())?;
            }
        }
        Ok(self.rosetta_stone_sets.clone())
    }

    /// Generate a single-day price structure for Paper from CardSphere
    pub fn generate_today_price_dict(&mut self, all_printings_path: String) -> PyResult<HashMap<String, MtgjsonPrices>> {
        let request_api_response = self.download(Self::ROSETTA_STONE_CARDS_URL.to_string(), None)?;
        
        // TODO: In a real implementation, use generate_entity_mapping
        // let cardsphere_id_to_mtgjson = generate_entity_mapping(all_printings_path, ("identifiers", "cardsphereId"), ("uuid",));
        
        println!("Building CardSphere retail data");
        
        let mut today_dict = HashMap::new();
        
        // Process the API response to build price dictionary
        if let Some(price_data_rows) = request_api_response.as_array() {
            for row in price_data_rows {
                if let Some(row_obj) = row.as_object() {
                    // Extract relevant fields for price processing
                    if let (Some(cs_id), Some(price), Some(is_foil)) = (
                        row_obj.get("cs_id").and_then(|v| v.as_str()),
                        row_obj.get("price").and_then(|v| v.as_f64()),
                        row_obj.get("is_foil").and_then(|v| v.as_bool())
                    ) {
                        // TODO: Map cs_id to MTGJSON UUID using cardsphere_id_to_mtgjson
                        // For now, use placeholder logic
                        let mtgjson_uuid = format!("placeholder_{}", cs_id);
                        
                        let prices = today_dict.entry(mtgjson_uuid).or_insert_with(|| MtgjsonPrices {
                            currency: "USD".to_string(),
                            date: self.base.today_date(),
                            provider: "cardsphere".to_string(),
                            source: "paper".to_string(),
                            buy_normal: None,
                            buy_foil: None,
                            buy_etched: None,
                            sell_normal: None,
                            sell_foil: None,
                            sell_etched: None,
                        });
                        
                        if is_foil {
                            prices.sell_foil = Some(price);
                        } else {
                            prices.sell_normal = Some(price);
                        }
                    }
                }
            }
        }
        
        Ok(today_dict)
    }

    /// Get the class ID (property access)
    #[getter]
    pub fn get_class_id(&self) -> PyResult<String> {
        Ok("mb".to_string())
    }

    /// Get URLs as constants
    #[getter]
    pub fn get_rosetta_stone_sets_url(&self) -> PyResult<String> {
        Ok(Self::ROSETTA_STONE_SETS_URL.to_string())
    }

    #[getter]
    pub fn get_rosetta_stone_cards_url(&self) -> PyResult<String> {
        Ok(Self::ROSETTA_STONE_CARDS_URL.to_string())
    }
}

#[async_trait]
impl AbstractProvider for MultiverseBridgeProvider {
    async fn download_async(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Response> {
        self.base.get(url, params).await
    }

    async fn generate_today_price_dict(&self, _all_printings_path: &str) -> ProviderResult<HashMap<String, MtgjsonPrices>> {
        // Use the sync version for now - could be refactored to async
        Ok(HashMap::new())
    }
}