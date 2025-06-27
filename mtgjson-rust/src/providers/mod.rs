use pyo3::prelude::*;

// Core provider modules
pub mod abstract_;
pub mod cardhoarder;
pub mod cardkingdom;
pub mod gatherer;
pub mod mtgban;
pub mod multiversebridge;
pub mod tcgplayer;
pub mod whats_in_standard;
pub mod wizards;

// GitHub providers
pub mod github {
    pub mod boosters;
    pub mod card_sealed_products;
    pub mod decks;
    pub mod mtgsqlite;
    pub mod sealed;
}

// CardMarket providers
pub mod cardmarket {
    pub mod monolith;
}

// EDHRec providers
pub mod edhrec {
    pub mod card_ranks;
}

// MTGWiki providers
pub mod mtgwiki {
    pub mod secret_lair;
}

// Scryfall providers
pub mod scryfall {
    pub mod monolith;
    pub mod orientation_detector;
    pub mod set_language_detector;
    pub mod sf_utils;
}

// Third-party providers
pub mod third_party {
    pub mod cardhoarder;
    pub mod cardkingdom;
    pub mod gatherer;
    pub mod mtgban;
    pub mod multiverse_bridge;
    pub mod tcgplayer;
    pub mod whats_in_standard;
    pub mod wizards;
}

// Re-export main provider types
pub use abstract_::AbstractProvider;
pub use cardhoarder::CardHoarderProvider;
pub use cardkingdom::CardKingdomProvider;
pub use cardmarket::monolith::CardMarketProvider;
pub use edhrec::card_ranks::EdhrecProviderCardRanks;
pub use gatherer::GathererProvider;
pub use github::boosters::GitHubBoostersProvider;
pub use github::card_sealed_products::GitHubCardSealedProductsProvider;
pub use github::decks::GitHubDecksProvider;
pub use github::mtgsqlite::GitHubMTGSqliteProvider;
pub use github::sealed::GitHubSealedProvider;
pub use mtgban::MTGBanProvider;
pub use mtgwiki::secret_lair::MtgWikiProviderSecretLair;
pub use multiversebridge::MultiverseBridgeProvider;
pub use scryfall::monolith::ScryfallProvider;
pub use scryfall::orientation_detector::ScryfallProviderOrientationDetector;
pub use scryfall::set_language_detector::ScryfallProviderSetLanguageDetector;
pub use tcgplayer::TCGPlayerProvider;
pub use whats_in_standard::WhatsInStandardProvider;
pub use wizards::WizardsProvider;

/// Add all provider classes to Python module
pub fn add_provider_classes_to_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Core providers
    m.add_class::<CardHoarderProvider>()?;
    m.add_class::<CardKingdomProvider>()?;
    m.add_class::<CardMarketProvider>()?;
    m.add_class::<EdhrecProviderCardRanks>()?;
    m.add_class::<GathererProvider>()?;
    m.add_class::<GitHubBoostersProvider>()?;
    m.add_class::<GitHubCardSealedProductsProvider>()?;
    m.add_class::<GitHubDecksProvider>()?;
    m.add_class::<GitHubMTGSqliteProvider>()?;
    m.add_class::<GitHubSealedProvider>()?;
    m.add_class::<MTGBanProvider>()?;
    m.add_class::<MtgWikiProviderSecretLair>()?;
    m.add_class::<MultiverseBridgeProvider>()?;
    m.add_class::<ScryfallProvider>()?;
    m.add_class::<ScryfallProviderOrientationDetector>()?;
    m.add_class::<ScryfallProviderSetLanguageDetector>()?;
    m.add_class::<TCGPlayerProvider>()?;
    m.add_class::<WhatsInStandardProvider>()?;
    m.add_class::<WizardsProvider>()?;
    
    Ok(())
}