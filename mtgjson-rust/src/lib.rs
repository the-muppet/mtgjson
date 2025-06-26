//! MTGJSON 5.5 - Rust Edition
//!
//! This crate provides Rust structs for all major MTGJSON classes,
//! as well as Python bindings for those structs.
//!
//! The crate is organized into modules, each corresponding to a
//! major MTGJSON class. The `base` module contains the base trait
//! for all MTGJSON objects, and the `utils` module contains utility
//! functions for working with MTGJSON data.
//! To be continued - 

use pyo3::prelude::*;

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

// Re-export all major types
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

/// Python module for MTGJSON Rust implementation
#[pymodule]
fn mtgjson_rust(py: Python, m: &PyModule) -> PyResult<()> {
    // Register classes using PyO3 0.22 API
    m.add("MtgjsonCard", py.get_type::<MtgjsonCard>())?;
    m.add("MtgjsonDeck", py.get_type::<MtgjsonDeck>())?;
    m.add("MtgjsonDeckHeader", py.get_type::<MtgjsonDeckHeader>())?;
    m.add("MtgjsonForeignData", py.get_type::<MtgjsonForeignData>())?;
    m.add("MtgjsonGameFormats", py.get_type::<MtgjsonGameFormats>())?;
    m.add("MtgjsonIdentifiers", py.get_type::<MtgjsonIdentifiers>())?;
    m.add("MtgjsonLeadershipSkills", py.get_type::<MtgjsonLeadershipSkills>())?;
    m.add("MtgjsonLegalities", py.get_type::<MtgjsonLegalities>())?;
    m.add("MtgjsonMeta", py.get_type::<MtgjsonMeta>())?;
    m.add("MtgjsonPrices", py.get_type::<MtgjsonPrices>())?;
    m.add("MtgjsonPurchaseUrls", py.get_type::<MtgjsonPurchaseUrls>())?;
    m.add("MtgjsonRelatedCards", py.get_type::<MtgjsonRelatedCards>())?;
    m.add("MtgjsonRuling", py.get_type::<MtgjsonRuling>())?;
    m.add("MtgjsonSealedProduct", py.get_type::<MtgjsonSealedProduct>())?;
    m.add("MtgjsonSet", py.get_type::<MtgjsonSet>())?;
    m.add("MtgjsonTranslations", py.get_type::<MtgjsonTranslations>())?;
    
    // Register enums
    m.add("SealedProductCategory", py.get_type::<SealedProductCategory>())?;
    m.add("SealedProductSubtype", py.get_type::<SealedProductSubtype>())?;
    
    Ok(())
}