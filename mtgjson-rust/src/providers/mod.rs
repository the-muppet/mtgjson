use pyo3::prelude::*;
use async_trait::async_trait;
use reqwest::{Client, Response};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

use crate::classes::MtgjsonPricesObject;

// Module declarations
pub mod r#abstract;
pub mod cardmarket;
pub mod edhrec;
pub mod github;
pub mod mtgwiki;
pub mod scryfall;
pub mod third_party;

// Re-exports from modules
pub use r#abstract::AbstractProvider;
pub use edhrec::EdhrecProvider;
pub use mtgwiki::MtgWikiProvider;
pub use scryfall::{ScryfallProvider, ScryfallProviderOrientationDetector};
pub use github::{
    GithubBoostersProvider, 
    GithubCardSealedProvider, 
    GithubDecksProvider, 
    GithubMtgSqliteProvider, 
    GithubSealedProvider
};
pub use third_party::{
    CardHoarderProvider, 
    CardKingdomProvider, 
    GathererProvider, 
    MTGBanProvider, 
    MultiverseBridgeProvider, 
    TCGPlayerProvider, 
    WhatsInStandardProvider, 
    WizardsProvider
};

/// Custom error type for provider operations
#[derive(Debug)]
pub enum ProviderError {
    NetworkError(String),
    ParseError(String),
    AuthenticationError(String),
    RateLimitError(String),
    ConfigurationError(String),
    ProcessingError(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProviderError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ProviderError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProviderError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            ProviderError::RateLimitError(msg) => write!(f, "Rate limit error: {}", msg),
            ProviderError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            ProviderError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
        }
    }
}

impl Error for ProviderError {}

/// Result type for provider operations
pub type ProviderResult<T> = Result<T, ProviderError>;

/// Base provider with common HTTP functionality
pub struct BaseProvider {
    pub provider_name: String,
    pub client: Client,
    pub headers: HashMap<String, String>,
}

impl BaseProvider {
    pub fn new(provider_name: String, headers: HashMap<String, String>) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            provider_name,
            client,
            headers,
        }
    }
    
    pub async fn get(&self, url: &str, params: Option<HashMap<String, String>>) -> ProviderResult<Response> {
        let mut request_builder = self.client.get(url);
        
        // Add headers
        for (key, value) in &self.headers {
            request_builder = request_builder.header(key, value);
        }
        
        // Add query parameters
        if let Some(params) = params {
            request_builder = request_builder.query(&params);
        }
        
        request_builder.send().await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))
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
    // Add all provider classes
    m.add_class::<CardHoarderProvider>()?;
    m.add_class::<CardKingdomProvider>()?;
    m.add_class::<EdhrecProvider>()?;
    m.add_class::<GathererProvider>()?;
    m.add_class::<GithubBoostersProvider>()?;
    m.add_class::<GithubCardSealedProvider>()?;
    m.add_class::<GithubDecksProvider>()?;
    m.add_class::<GithubMtgSqliteProvider>()?;
    m.add_class::<GithubSealedProvider>()?;
    m.add_class::<MTGBanProvider>()?;
    m.add_class::<MtgWikiProvider>()?;
    m.add_class::<MultiverseBridgeProvider>()?;
    m.add_class::<ScryfallProvider>()?;
    m.add_class::<TCGPlayerProvider>()?;
    m.add_class::<WhatsInStandardProvider>()?;
    m.add_class::<WizardsProvider>()?;
    
    Ok(())
}