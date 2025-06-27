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

// Module declarations
// Classes
mod classes;
// Compiled classes
mod compiled_classes;
// Providers
mod providers;
// High-computational modules (from artifacts directory)
mod artifacts;
mod set_builder;
// Wrapper module for set_builder functions
mod set_builder_functions;
// Wrapper module for utility functions
mod utils_functions;


// Import all the structs
use classes::{
    base::JsonObject,
    card::MtgjsonCardObject,
    deck::{MtgjsonDeckObject, MtgjsonDeckHeaderObject},
    foreign_data::MtgjsonForeignDataObject,
    game_formats::MtgjsonGameFormatsObject,
    identifiers::MtgjsonIdentifiersObject,
    leadership_skills::MtgjsonLeadershipSkillsObject,
    legalities::MtgjsonLegalitiesObject,
    meta::MtgjsonMetaObject,
    prices::MtgjsonPricesObject,
    purchase_urls::MtgjsonPurchaseUrlsObject,
    related_cards::MtgjsonRelatedCardsObject,
    rulings::MtgjsonRulingObject,
    sealed_product::{MtgjsonSealedProductObject, SealedProductCategory, SealedProductSubtype},
    set::MtgjsonSetObject,
    translations::MtgjsonTranslationsObject,
};

// Import compiled classes
use compiled_classes::{
    MtgjsonStructuresObject, MtgjsonCompiledListObject, MtgjsonDeckListObject, 
    MtgjsonKeywordsObject, MtgjsonAllIdentifiersObject, MtgjsonAllPrintingsObject,
    MtgjsonAtomicCardsObject, MtgjsonCardTypesObject, MtgjsonEnumValuesObject,
    MtgjsonSetListObject, MtgjsonTcgplayerSkusObject
};

// Re-export for tests and external usage  
pub use artifacts::output_generator::OutputGenerator;
pub use artifacts::price_builder::PriceBuilder;
pub use artifacts::parallel_call::{ParallelProcessor, ParallelIterator};
pub use set_builder_functions::*;
pub use providers::*;

/// Python module definition
#[pymodule]
fn mtgjson_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the JSON value wrapper
    m.add_class::<JsonValue>()?;
    
    // Add all MTGJSON classes
    m.add_class::<MtgjsonCardObject>()?;
    m.add_class::<MtgjsonDeckObject>()?;
    m.add_class::<MtgjsonDeckHeaderObject>()?;
    m.add_class::<MtgjsonForeignDataObject>()?;
    m.add_class::<MtgjsonGameFormatsObject>()?;
    m.add_class::<MtgjsonIdentifiersObject>()?;
    m.add_class::<MtgjsonLeadershipSkillsObject>()?;
    m.add_class::<MtgjsonLegalitiesObject>()?;
    m.add_class::<MtgjsonMetaObject>()?;
    m.add_class::<MtgjsonPricesObject>()?;
    m.add_class::<MtgjsonPurchaseUrlsObject>()?;
    m.add_class::<MtgjsonRelatedCardsObject>()?;
    m.add_class::<MtgjsonRulingObject>()?;
    m.add_class::<MtgjsonSealedProductObject>()?;
    m.add_class::<MtgjsonSetObject>()?;
    m.add_class::<MtgjsonTranslationsObject>()?;
    
    // Add enums
    m.add_class::<SealedProductCategory>()?;
    m.add_class::<SealedProductSubtype>()?;
    
    // Add compiled classes
    m.add_class::<MtgjsonStructuresObject>()?;
    m.add_class::<MtgjsonCompiledListObject>()?;
    m.add_class::<MtgjsonDeckListObject>()?;
    m.add_class::<MtgjsonKeywordsObject>()?;
    m.add_class::<MtgjsonAllIdentifiersObject>()?;
    m.add_class::<MtgjsonAllPrintingsObject>()?;
    m.add_class::<MtgjsonAtomicCardsObject>()?;
    m.add_class::<MtgjsonCardTypesObject>()?;
    m.add_class::<MtgjsonEnumValuesObject>()?;
    m.add_class::<MtgjsonSetListObject>()?;
    m.add_class::<MtgjsonTcgplayerSkusObject>()?;
    
    // Add high-performance classes
    m.add_class::<artifacts::output_generator::OutputGenerator>()?;
    m.add_class::<artifacts::price_builder::PriceBuilder>()?;
    m.add_class::<artifacts::parallel_call::ParallelProcessor>()?;
    m.add_class::<artifacts::parallel_call::ParallelIterator>()?;
    
    // Add set_builder module functions
    m.add_function(wrap_pyfunction!(set_builder_functions::parse_card_types, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::get_card_colors, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::get_card_cmc, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::is_number, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::parse_legalities, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::build_mtgjson_set, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::parse_foreign, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::parse_printings, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::parse_rulings, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::mark_duel_decks, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::enhance_cards_with_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(set_builder_functions::build_base_mtgjson_cards, m)?)?;
    
    // Add utility functions
    m.add_function(wrap_pyfunction!(utils_functions::to_camel_case, m)?)?;
    m.add_function(wrap_pyfunction!(utils_functions::make_windows_safe_filename, m)?)?;
    m.add_function(wrap_pyfunction!(utils_functions::clean_card_number, m)?)?;
    
    Ok(())
}