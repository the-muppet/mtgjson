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

// Import all modules from the classes subdirectory
mod classes {
    pub mod base;
    pub mod card;
    pub mod deck;
    pub mod foreign_data;
    pub mod game_formats;
    pub mod identifiers;
    pub mod leadership_skills;
    pub mod legalities;
    pub mod meta;
    pub mod prices;
    pub mod purchase_urls;
    pub mod related_cards;
    pub mod rulings;
    pub mod sealed_product;
    pub mod set;
    pub mod translations;
    pub mod utils;
}

// Re-export classes at the root level for easier access
pub use classes::base;
pub use classes::card;
pub use classes::deck;
pub use classes::foreign_data;
pub use classes::game_formats;
pub use classes::identifiers;
pub use classes::leadership_skills;
pub use classes::legalities;
pub use classes::meta;
pub use classes::prices;
pub use classes::purchase_urls;
pub use classes::related_cards;
pub use classes::rulings;
pub use classes::sealed_product;
pub use classes::set;
pub use classes::translations;
pub use classes::utils;

// Builders module containing high-computational and set builder modules
mod builders {
    pub mod output_generator;
    pub mod parallel_call;
    pub mod price_builder;
    pub mod set_builder;
    pub mod set_builder_functions;
}

// Wrapper modules for PyO3 functions
mod utils_functions;

// Compiled classes
mod compiled_classes;

// Providers module - for 100% API coverage
mod providers;



// Re-export for tests and external usage  
pub use builders::output_generator::OutputGenerator;
pub use builders::parallel_call::{ParallelProcessor, ParallelIterator};
pub use builders::price_builder::PriceBuilder;
pub use builders::set_builder_functions::*;
pub use utils_functions::*;

/// Python module definition
#[pymodule]
fn mtgjson_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the JSON value wrapper
    m.add_class::<JsonValue>()?;
    
    // Add all MTGJSON classes
    m.add_class::<classes::card::MtgjsonCardObject>()?;
    m.add_class::<classes::deck::MtgjsonDeckObject>()?;
    m.add_class::<classes::deck::MtgjsonDeckHeaderObject>()?;
    m.add_class::<classes::foreign_data::MtgjsonForeignDataObject>()?;
    m.add_class::<classes::game_formats::MtgjsonGameFormatsObject>()?;
    m.add_class::<classes::identifiers::MtgjsonIdentifiers>()?;
    m.add_class::<classes::leadership_skills::MtgjsonLeadershipSkillsObject>()?;
    m.add_class::<classes::legalities::MtgjsonLegalitiesObject>()?;
    m.add_class::<classes::meta::MtgjsonMetaObject>()?;
    m.add_class::<classes::prices::MtgjsonPricesObject>()?;
    m.add_class::<classes::purchase_urls::MtgjsonPurchaseUrls>()?;
    m.add_class::<classes::related_cards::MtgjsonRelatedCardsObject>()?;
    m.add_class::<classes::rulings::MtgjsonRulingObject>()?;
    m.add_class::<classes::sealed_product::MtgjsonSealedProductObject>()?;
    m.add_class::<classes::set::MtgjsonSetObject>()?;
    m.add_class::<classes::translations::MtgjsonTranslations>()?;
    
    // Add enums
    m.add_class::<classes::sealed_product::SealedProductCategory>()?;
    m.add_class::<classes::sealed_product::SealedProductSubtype>()?;
    
    // Add compiled classes
    m.add_class::<compiled_classes::structures::MtgjsonStructures>()?;
    m.add_class::<compiled_classes::compiled_list::MtgjsonCompiledList>()?;
    m.add_class::<compiled_classes::deck_list::MtgjsonDeckObjectList>()?;
    m.add_class::<compiled_classes::keywords::MtgjsonKeywords>()?;
    m.add_class::<compiled_classes::all_identifiers::MtgjsonAllIdentifiers>()?;
    m.add_class::<compiled_classes::all_printings::MtgjsonAllPrintings>()?;
    m.add_class::<compiled_classes::atomic_cards::MtgjsonAtomicCards>()?;
    m.add_class::<compiled_classes::card_types::MtgjsonCardObjectTypes>()?;
    m.add_class::<compiled_classes::enum_values::MtgjsonEnumValues>()?;
    m.add_class::<compiled_classes::set_list::MtgjsonSetObjectList>()?;
    m.add_class::<compiled_classes::tcgplayer_skus::MtgjsonTcgplayerSkus>()?;
    
    // Add high-performance classes
    m.add_class::<builders::output_generator::OutputGenerator>()?;
    m.add_class::<builders::price_builder::PriceBuilder>()?;
    m.add_class::<builders::parallel_call::ParallelProcessor>()?;
    m.add_class::<builders::parallel_call::ParallelIterator>()?;
    
    // Add set_builder module functions
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::parse_card_types, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::get_card_colors, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::get_card_cmc, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::is_number, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::parse_legalities, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::build_mtgjson_set, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::parse_foreign, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::parse_printings, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::parse_rulings, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::mark_duel_decks, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::enhance_cards_with_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(builders::set_builder_functions::build_base_mtgjson_cards, m)?)?;
    
    // Add utility functions
    m.add_function(wrap_pyfunction!(utils_functions::to_camel_case, m)?)?;
    m.add_function(wrap_pyfunction!(utils_functions::make_windows_safe_filename, m)?)?;
    m.add_function(wrap_pyfunction!(utils_functions::clean_card_number, m)?)?;
    
    // Add all provider classes for 100% Python API coverage
    providers::add_provider_classes_to_module(m)?;
    
    Ok(())
}