use async_trait::async_trait;
use pyo3::prelude::*;
use reqwest::Response;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use scraper::{Html, Selector};
use regex::Regex;
use crate::prices::MtgjsonPrices;
use super::{AbstractProvider, BaseProvider, ProviderError, ProviderResult};

#[pyclass(name = "MtgWikiProviderSecretLair")]
pub struct MtgWikiProviderSecretLair {
    base: BaseProvider,
}

impl MtgWikiProviderSecretLair {
    const PAGE_URL: &'static str = "https://mtg.wiki/page/Secret_Lair/Drop_Series";
}

#[pymethods]
impl MtgWikiProviderSecretLair {
    #[new]
    pub fn new(headers: Option<HashMap<String, String>>) -> PyResult<Self> {
        let headers = headers.unwrap_or_default();
        let base = BaseProvider::new("mtgwiki".to_string(), headers);
        
        Ok(MtgWikiProviderSecretLair {
            base,
        })
    }

    /// Build HTTP header (returns empty dict)
    pub fn _build_http_header(&self) -> PyResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    /// Download MTG.Wiki Secret Lair page and parse it out for user consumption
    /// Returns mapping of Card ID to Secret Lair Drop Name
    pub fn download(&mut self, url: Option<String>, params: Option<HashMap<String, String>>) -> PyResult<HashMap<String, String>> {
        let target_url = url.unwrap_or_else(|| Self::PAGE_URL.to_string());
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match self.base.get(&target_url, params).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let page_text = response.text().await.map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Text parse error: {}", e))
                        })?;
                        
                        Ok(self.parse_secret_lair_table(page_text)?)
                    } else {
                        println!("Error downloading MTG Wiki data: {}", response.status());
                        Ok(HashMap::new())
                    }
                },
                Err(e) => {
                    println!("Error downloading MTG Wiki data: {}", e);
                    Ok(HashMap::new())
                }
            }
        })
    }

    /// Parse the Secret Lair table from the MTG Wiki page
    fn parse_secret_lair_table(&self, page_text: String) -> PyResult<HashMap<String, String>> {
        let mut results = HashMap::new();
        
        let document = Html::parse_document(&page_text);
        let table_selector = Selector::parse("table.wikitable.sortable").unwrap();
        
        if let Some(table) = document.select(&table_selector).next() {
            let row_selector = Selector::parse("tr").unwrap();
            let rows: Vec<_> = table.select(&row_selector).collect();
            
            for (index, table_row) in rows.iter().enumerate().skip(1) { // Skip header row
                let col_selector = Selector::parse("td").unwrap();
                let table_cols: Vec<_> = table_row.select(&col_selector).collect();
                
                if table_cols.is_empty() {
                    continue;
                }
                
                let mut extra_card_numbers = String::new();
                
                // Check for rowspan (multiple segments)
                if let Some(first_col) = table_cols.get(0) {
                    if first_col.value().attr("rowspan").is_some() && index + 1 < rows.len() {
                        // Get the next row's first column for extra card numbers
                        if let Some(next_row) = rows.get(index + 1) {
                            let next_cols: Vec<_> = next_row.select(&col_selector).collect();
                            if let Some(next_first_col) = next_cols.get(0) {
                                extra_card_numbers = format!(",{}", next_first_col.inner_html().trim());
                            }
                        }
                    }
                }
                
                if table_cols.len() < 3 {
                    continue;
                }
                
                let secret_lair_name = table_cols[1].inner_html().trim().to_string();
                let card_numbers_text = format!("{}{}", table_cols[2].inner_html().trim(), extra_card_numbers);
                
                let card_numbers = Self::convert_range_to_page_style(&card_numbers_text)?;
                
                if secret_lair_name.is_empty() || card_numbers.is_empty() {
                    continue;
                }
                
                for card_num in card_numbers {
                    results.insert(card_num.to_string(), secret_lair_name.clone());
                }
            }
        }
        
        Ok(results)
    }

    /// Convert range string to list of numbers
    fn convert_range_to_page_style(range_string: &str) -> PyResult<Vec<i32>> {
        // Filter to keep only digits, hyphens, and commas
        let filtered: String = range_string.chars()
            .filter(|c| "0123456789-,".contains(*c))
            .collect();
        
        if filtered.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut result = Vec::new();
        
        for part in filtered.split(',') {
            if part.contains('-') {
                let range_parts: Vec<&str> = part.split('-').collect();
                if range_parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (range_parts[0].parse::<i32>(), range_parts[1].parse::<i32>()) {
                        for num in start..=end {
                            result.push(num);
                        }
                    }
                }
            } else if !part.is_empty() {
                if let Ok(num) = part.parse::<i32>() {
                    result.push(num);
                }
            }
        }
        
        Ok(result)
    }
}

#[async_trait]
impl AbstractProvider for MtgWikiProviderSecretLair {
    fn get_class_id(&self) -> &str {
        &self.base.class_id
    }
    
    fn get_class_name(&self) -> &str {
        "MtgWikiProviderSecretLair"
    }
    
    fn build_http_header(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    
    async fn download(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Value> {
        // This provider returns HTML, not JSON
        let text = self.download_raw(url, params).await?;
        Ok(Value::String(text))
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

    async fn download_async(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Response> {
        self.base.get(url, params).await
    }

    async fn generate_today_price_dict(&self, _all_printings_path: &str) -> ProviderResult<HashMap<String, MtgjsonPrices>> {
        // MTG Wiki doesn't provide price data
        Ok(HashMap::new())
    }
}