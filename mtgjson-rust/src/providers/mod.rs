use pyo3::prelude::*;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use crate::prices::MtgjsonPrices;

// Module declarations
pub mod abstract_provider;
pub mod cardhoarder;
pub mod cardkingdom;
pub mod cardmarket;
pub mod edhrec;
pub mod gatherer;
pub mod github_boosters;
pub mod github_card_sealed_products;
pub mod github_decks;
pub mod github_mtgsqlite;
pub mod github_sealed;
pub mod mtgban;
pub mod mtgwiki;
pub mod multiversebridge;
pub mod scryfall;
pub mod tcgplayer;
pub mod whats_in_standard;
pub mod wizards;

// Re-exports
pub use abstract_provider::AbstractProvider;
pub use cardhoarder::CardHoarderProvider;
pub use cardkingdom::CardKingdomProvider;
pub use cardmarket::CardMarketProvider;
pub use edhrec::EdhrecProvider;
pub use gatherer::GathererProvider;
pub use github_boosters::GitHubBoostersProvider;
pub use github_card_sealed_products::GitHubCardSealedProductsProvider;
pub use github_decks::GitHubDecksProvider;
pub use github_mtgsqlite::GitHubMTGSqliteProvider;
pub use github_sealed::GitHubSealedProvider;
pub use mtgban::MTGBanProvider;
pub use mtgwiki::MtgWikiProvider;
pub use multiversebridge::MultiverseBridgeProvider;
pub use scryfall::ScryfallProvider;
pub use tcgplayer::TCGPlayerProvider;
pub use whats_in_standard::WhatsInStandardProvider;
pub use wizards::WizardsProvider;

/// Custom error type for provider operations
#[derive(Debug)]
pub enum ProviderError {
    NetworkError(String),
    ParseError(String),
    AuthError(String),
    RateLimitError(String),
    ConfigError(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProviderError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ProviderError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProviderError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            ProviderError::RateLimitError(msg) => write!(f, "Rate limit error: {}", msg),
            ProviderError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl Error for ProviderError {}

/// Result type for provider operations
pub type ProviderResult<T> = Result<T, ProviderError>;

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
    m.add_class::<CardMarketProvider>()?;
    m.add_class::<EdhrecProvider>()?;
    m.add_class::<GathererProvider>()?;
    m.add_class::<GitHubBoostersProvider>()?;
    m.add_class::<GitHubCardSealedProductsProvider>()?;
    m.add_class::<GitHubDecksProvider>()?;
    m.add_class::<GitHubMTGSqliteProvider>()?;
    m.add_class::<GitHubSealedProvider>()?;
    m.add_class::<MTGBanProvider>()?;
    m.add_class::<MtgWikiProvider>()?;
    m.add_class::<MultiverseBridgeProvider>()?;
    m.add_class::<ScryfallProvider>()?;
    m.add_class::<TCGPlayerProvider>()?;
    m.add_class::<WhatsInStandardProvider>()?;
    m.add_class::<WizardsProvider>()?;
    
    Ok(())
}