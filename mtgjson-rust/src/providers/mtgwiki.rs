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
        
        Ok(Self { base })
    }
    
    /// Download MTG.Wiki Secret Lair page and parse it
    pub fn download(&self, url: Option<&str>, params: Option<HashMap<String, String>>) -> PyResult<HashMap<String, String>> {
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(async {
            self.download_async(url, params).await
        }).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Download error: {}", e)))
    }
}

impl MtgWikiProviderSecretLair {
    async fn download_async(&self, url: Option<&str>, params: Option<HashMap<String, String>>) -> ProviderResult<HashMap<String, String>> {
        let url_to_use = url.unwrap_or(Self::PAGE_URL);
        let response_text = self.download_raw(url_to_use, params).await?;
        self.parse_secret_lair_table(&response_text)
    }
    
    fn parse_secret_lair_table(&self, page_text: &str) -> ProviderResult<HashMap<String, String>> {
        let mut results = HashMap::new();
        
        let document = Html::parse_document(page_text);
        let table_selector = Selector::parse("table.wikitable.sortable").map_err(|_| {
            ProviderError::ParseError("Invalid table selector".to_string())
        })?;
        
        let row_selector = Selector::parse("tr").map_err(|_| {
            ProviderError::ParseError("Invalid row selector".to_string())
        })?;
        
        let col_selector = Selector::parse("td").map_err(|_| {
            ProviderError::ParseError("Invalid column selector".to_string())
        })?;
        
        if let Some(table) = document.select(&table_selector).next() {
            let table_rows: Vec<_> = table.select(&row_selector).collect();
            
            for (index, table_row) in table_rows.iter().enumerate().skip(1) { // Skip header row
                let table_cols: Vec<_> = table_row.select(&col_selector).collect();
                
                let mut extra_card_numbers = String::new();
                
                if !table_cols.is_empty() {
                    let first_col_html = table_cols[0].html();
                    if first_col_html.contains("rowspan") {
                        // We have multiple segments split up
                        if index + 1 < table_rows.len() {
                            let next_tr_cols: Vec<_> = table_rows[index + 1].select(&col_selector).collect();
                            if !next_tr_cols.is_empty() {
                                extra_card_numbers = format!(",{}", next_tr_cols[0].inner_html().trim());
                            }
                        }
                    } else if table_cols.len() < 3 {
                        continue;
                    }
                }
                
                if table_cols.len() >= 3 {
                    let secret_lair_name = table_cols[1].inner_html().trim().to_string();
                    let card_numbers_text = format!("{}{}", table_cols[2].inner_html().trim(), extra_card_numbers);
                    let card_numbers = self.convert_range_to_page_style(&card_numbers_text);
                    
                    if !secret_lair_name.is_empty() && !card_numbers.is_empty() {
                        for card_num in card_numbers {
                            results.insert(card_num.to_string(), secret_lair_name.clone());
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }
    
    fn convert_range_to_page_style(&self, range_string: &str) -> Vec<i32> {
        let re = Regex::new(r"[0123456789\-,]").unwrap();
        let filtered: String = range_string.chars()
            .filter(|c| re.is_match(&c.to_string()))
            .collect();
        
        if filtered.is_empty() {
            return vec![];
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
            } else if let Ok(num) = part.parse::<i32>() {
                result.push(num);
            }
        }
        
        result
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
}