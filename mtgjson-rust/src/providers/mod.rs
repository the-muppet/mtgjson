use pyo3::prelude::*;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use crate::prices::MtgjsonPrices;

// Module declarations - only for properly implemented providers
pub mod abstract_provider;
pub mod cardhoarder;
pub mod cardkingdom;
pub mod cardmarket;
pub mod edhrec;
pub mod gatherer;
pub mod mtgban;
pub mod mtgwiki;
pub mod multiversebridge;
pub mod scryfall;
pub mod tcgplayer;
pub mod whats_in_standard;
pub mod wizards;

// Re-exports with correct names
pub use abstract_provider::AbstractProvider;
pub use cardhoarder::CardHoarderProvider;
pub use cardkingdom::CardKingdomProvider;
pub use cardmarket::CardMarketProvider;
pub use edhrec::EdhrecProviderCardRanks;
pub use gatherer::GathererProvider;
pub use mtgban::MTGBanProvider;
pub use mtgwiki::MtgWikiProviderSecretLair;
pub use multiversebridge::MultiverseBridgeProvider;
pub use scryfall::ScryfallProvider;
pub use tcgplayer::TCGPlayerProvider;
pub use whats_in_standard::WhatsInStandardProvider;
pub use wizards::WizardsProvider;

// Error types
#[derive(Debug)]
pub enum ProviderError {
    HttpError(reqwest::Error),
    ParseError(String),
    ConfigError(String),
    RateLimitError(String),
    AuthenticationError(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderError::HttpError(e) => write!(f, "HTTP error: {}", e),
            ProviderError::ParseError(e) => write!(f, "Parse error: {}", e),
            ProviderError::ConfigError(e) => write!(f, "Config error: {}", e),
            ProviderError::RateLimitError(e) => write!(f, "Rate limit error: {}", e),
            ProviderError::AuthenticationError(e) => write!(f, "Authentication error: {}", e),
        }
    }
}

impl Error for ProviderError {}

impl From<reqwest::Error> for ProviderError {
    fn from(error: reqwest::Error) -> Self {
        ProviderError::HttpError(error)
    }
}

pub type ProviderResult<T> = Result<T, ProviderError>;

// Base provider implementation
#[derive(Clone)]
pub struct BaseProvider {
    pub class_id: String,
    pub client: Client,
    pub headers: HashMap<String, String>,
}

impl BaseProvider {
    pub fn new(class_id: String, headers: HashMap<String, String>) -> Self {
        let mut default_headers = reqwest::header::HeaderMap::new();
        
        // Add custom headers
        for (key, value) in &headers {
            if let (Ok(name), Ok(val)) = (
                reqwest::header::HeaderName::from_bytes(key.as_bytes()),
                reqwest::header::HeaderValue::from_str(value)
            ) {
                default_headers.insert(name, val);
            }
        }

        let client = Client::builder()
            .default_headers(default_headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();

        BaseProvider {
            class_id,
            client,
            headers,
        }
    }

    pub async fn get(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<reqwest::Response> {
        let mut request = self.client.get(url);
        
        if let Some(query_params) = params {
            request = request.query(&query_params);
        }
        
        let response = request.send().await?;
        Ok(response)
    }

    pub async fn post(&self, url: &str, body: Option<&str>) -> ProviderResult<reqwest::Response> {
        let mut request = self.client.post(url);
        
        if let Some(body_content) = body {
            request = request.body(body_content.to_string());
        }
        
        let response = request.send().await?;
        Ok(response)
    }

    pub fn today_date(&self) -> String {
        chrono::Utc::now().format("%Y-%m-%d").to_string()
    }
}

/// Price field name helper function
pub fn get_price_field_name(is_foil: bool, is_etched: bool, is_sell: bool) -> &'static str {
    match (is_etched, is_foil, is_sell) {
        (true, _, true) => "sell_etched",
        (true, _, false) => "buy_etched",
        (false, true, true) => "sell_foil",
        (false, true, false) => "buy_foil",
        (false, false, true) => "sell_normal",
        (false, false, false) => "buy_normal",
    }
}

/// Python module for providers
#[pymodule]
pub fn providers(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add all provider classes with correct names
    m.add_class::<CardHoarderProvider>()?;
    m.add_class::<CardKingdomProvider>()?;
    m.add_class::<CardMarketProvider>()?;
    m.add_class::<EdhrecProviderCardRanks>()?;
    m.add_class::<GathererProvider>()?;
    m.add_class::<MTGBanProvider>()?;
    m.add_class::<MtgWikiProviderSecretLair>()?;
    m.add_class::<MultiverseBridgeProvider>()?;
    m.add_class::<ScryfallProvider>()?;
    m.add_class::<TCGPlayerProvider>()?;
    m.add_class::<WhatsInStandardProvider>()?;
    m.add_class::<WizardsProvider>()?;
    
    Ok(())
}