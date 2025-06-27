use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// PyO3-compatible wrapper for JSON values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[pyclass(name = "JsonValue")]
pub struct JsonValue {
    #[pyo3(get, set)]
    pub value: String,
}

#[pymethods]
impl JsonValue {
    #[new]
    pub fn new(value: String) -> Self {
        Self { value }
    }
    
    /// Convert to JSON string
    pub fn to_json(&self) -> String {
        self.value.clone()
    }
}

// Import all modules  
mod base;
mod card;
mod deck;
mod foreign_data;
mod game_formats;
mod identifiers;
mod leadership_skills;
mod legalities;
mod meta;
mod prices;
mod purchase_urls;
mod related_cards;
mod rulings;
mod sealed_product;
mod set;
mod translations;
mod utils;

// High-computational modules
mod output_manager;
mod providers;

// Public re-exports
pub use base::*;
pub use card::*;
pub use deck::*;
pub use foreign_data::*;
pub use game_formats::*;
pub use identifiers::*;
pub use leadership_skills::*;
pub use legalities::*;
pub use meta::*;
pub use prices::*;
pub use purchase_urls::*;
pub use related_cards::*;
pub use rulings::*;
pub use sealed_product::*;
pub use set::*;
pub use translations::*;
pub use utils::*;

pub use output_manager::*;
pub use providers::*;

/// Python module initialization
#[pymodule]
fn mtgjson_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add base data structures
    m.add_class::<JsonValue>()?;
    m.add_class::<MtgjsonCard>()?;
    m.add_class::<MtgjsonDeck>()?;
    m.add_class::<MtgjsonForeignData>()?;
    m.add_class::<MtgjsonGameFormats>()?;
    m.add_class::<MtgjsonIdentifiers>()?;
    m.add_class::<MtgjsonLeadershipSkills>()?;
    m.add_class::<MtgjsonLegalities>()?;
    m.add_class::<MtgjsonMeta>()?;
    m.add_class::<MtgjsonPrices>()?;
    m.add_class::<MtgjsonPurchaseUrls>()?;
    m.add_class::<MtgjsonRelatedCards>()?;
    m.add_class::<MtgjsonRulings>()?;
    m.add_class::<MtgjsonSealedProduct>()?;
    m.add_class::<MtgjsonSet>()?;
    m.add_class::<MtgjsonTranslations>()?;

    // Add output manager
    m.add_class::<OutputManager>()?;
    
    // Add properly implemented providers
    m.add_class::<CardHoarderProvider>()?;
    m.add_class::<CardKingdomProvider>()?;
    m.add_class::<CardMarketProvider>()?;
    m.add_class::<EdhrecProviderCardRanks>()?;
    m.add_class::<MtgWikiProviderSecretLair>()?;
    m.add_class::<ScryfallProvider>()?;
    m.add_class::<TCGPlayerProvider>()?;
    m.add_class::<WhatsInStandardProvider>()?;
    m.add_class::<WizardsProvider>()?;

    Ok(())
}