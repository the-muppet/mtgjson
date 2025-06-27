// PyO3 wrapper functions for set_builder module functions
use pyo3::prelude::*;
use std::collections::HashMap;
use crate::set_builder;
use crate::legalities::MtgjsonLegalities;
use crate::set::MtgjsonSet;
use crate::foreign_data::MtgjsonForeignData;
use crate::rulings::MtgjsonRuling;

/// Parse card type line into super types, types, and subtypes
#[pyfunction]
pub fn parse_card_types(card_type: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    set_builder::parse_card_types(card_type)
}

/// Get card colors from mana cost string
#[pyfunction]
pub fn get_card_colors(mana_cost: &str) -> Vec<String> {
    set_builder::get_card_colors(mana_cost)
}

/// Calculate converted mana cost from mana cost string
#[pyfunction]
pub fn get_card_cmc(mana_cost: &str) -> f64 {
    set_builder::get_card_cmc(mana_cost)
}

/// Check if a string represents a number
#[pyfunction]
pub fn is_number(string: &str) -> bool {
    set_builder::is_number(string)
}

/// Parse legalities from Scryfall format to MTGJSON format
#[pyfunction]
pub fn parse_legalities(sf_card_legalities: HashMap<String, String>) -> MtgjsonLegalities {
    set_builder::parse_legalities(&sf_card_legalities)
}

/// Build MTGJSON set from set code
#[pyfunction]
pub fn build_mtgjson_set(set_code: &str) -> Option<MtgjsonSet> {
    set_builder::build_mtgjson_set(set_code)
}

/// Parse foreign card data from Scryfall prints URL
#[pyfunction]
pub fn parse_foreign(
    sf_prints_url: &str,
    card_name: &str,
    card_number: &str,
    set_name: &str,
) -> Vec<MtgjsonForeignData> {
    set_builder::parse_foreign(sf_prints_url, card_name, card_number, set_name)
}

/// Parse printings from Scryfall prints URL
#[pyfunction]
#[pyo3(signature = (sf_prints_url=None))]
pub fn parse_printings(sf_prints_url: Option<&str>) -> Vec<String> {
    set_builder::parse_printings(sf_prints_url)
}

/// Parse rulings from Scryfall URL
#[pyfunction]
pub fn parse_rulings(rulings_url: &str) -> Vec<MtgjsonRuling> {
    set_builder::parse_rulings(rulings_url)
}

/// Add leadership skills to a card
#[pyfunction]
pub fn add_leadership_skills(mtgjson_card: &mut crate::card::MtgjsonCard) {
    set_builder::add_leadership_skills(mtgjson_card)
}

/// Mark duel deck assignments for cards
#[pyfunction]
pub fn mark_duel_decks(set_code: &str, mut mtgjson_cards: Vec<crate::card::MtgjsonCard>) -> Vec<crate::card::MtgjsonCard> {
    set_builder::mark_duel_decks(set_code, &mut mtgjson_cards);
    mtgjson_cards
}

/// Parse keyrune code from URL
#[pyfunction]
pub fn parse_keyrune_code(url: &str) -> String {
    set_builder::parse_keyrune_code(url)
}

/// Get translation data for a set name
#[pyfunction]
pub fn get_translation_data(mtgjson_set_name: &str) -> Option<HashMap<String, String>> {
    set_builder::get_translation_data(mtgjson_set_name)
}

/// Build base MTGJSON cards from a set
#[pyfunction]
pub fn build_base_mtgjson_cards(
    set_code: &str,
    is_token: bool,
    set_release_date: &str,
) -> Vec<crate::card::MtgjsonCard> {
    set_builder::build_base_mtgjson_cards(set_code, None, is_token, set_release_date)
}

/// Build sealed products for a set
#[pyfunction]
pub fn build_sealed_products(set_code: &str) -> Vec<crate::sealed_product::MtgjsonSealedProduct> {
    set_builder::build_sealed_products(set_code)
}

/// Build decks for a set
#[pyfunction]
pub fn build_decks(set_code: &str) -> Vec<crate::deck::MtgjsonDeck> {
    set_builder::build_decks(set_code)
}

/// Enhance cards with additional metadata
#[pyfunction]
pub fn enhance_cards_with_metadata(mut mtgjson_cards: Vec<crate::card::MtgjsonCard>) -> Vec<crate::card::MtgjsonCard> {
    set_builder::enhance_cards_with_metadata(&mut mtgjson_cards);
    mtgjson_cards
}