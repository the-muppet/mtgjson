use crate::base::JsonObject;
use crate::card::MtgjsonCard;
use crate::deck::MtgjsonDeck;
use crate::foreign_data::MtgjsonForeignData;
use crate::game_formats::MtgjsonGameFormats;
use crate::leadership_skills::MtgjsonLeadershipSkills;
use crate::legalities::MtgjsonLegalities;
use crate::meta::MtgjsonMeta;
use crate::related_cards::MtgjsonRelatedCards;
use crate::rulings::MtgjsonRuling;
use crate::sealed_product::MtgjsonSealedProduct;
use crate::set::MtgjsonSet;
use crate::translations::MtgjsonTranslations;

use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Constants for card processing
pub struct Constants {
    pub language_map: HashMap<String, String>,
    pub basic_land_names: Vec<String>,
    pub super_types: Vec<String>,
    pub multi_word_sub_types: Vec<String>,
    pub foreign_sets: Vec<String>,
}

impl Constants {
    pub fn new() -> Self {
        let mut language_map = HashMap::new();
        language_map.insert("en".to_string(), "English".to_string());
        language_map.insert("es".to_string(), "Spanish".to_string());
        language_map.insert("fr".to_string(), "French".to_string());
        language_map.insert("de".to_string(), "German".to_string());
        language_map.insert("it".to_string(), "Italian".to_string());
        language_map.insert("pt".to_string(), "Portuguese".to_string());
        language_map.insert("ja".to_string(), "Japanese".to_string());
        language_map.insert("ko".to_string(), "Korean".to_string());
        language_map.insert("ru".to_string(), "Russian".to_string());
        language_map.insert("zhs".to_string(), "Chinese Simplified".to_string());
        language_map.insert("zht".to_string(), "Chinese Traditional".to_string());

        let basic_land_names = vec![
            "Plains".to_string(),
            "Island".to_string(),
            "Swamp".to_string(),
            "Mountain".to_string(),
            "Forest".to_string(),
            "Wastes".to_string(),
        ];

        let super_types = vec![
            "Basic".to_string(),
            "Legendary".to_string(),
            "Ongoing".to_string(),
            "Snow".to_string(),
            "World".to_string(),
        ];

        let multi_word_sub_types = vec![
            "Aura Curse".to_string(),
            "Equipment Vehicle".to_string(),
        ];

        let foreign_sets = vec![
            "4BB".to_string(),
            "FBB".to_string(),
            "REN".to_string(),
        ];

        Self {
            language_map,
            basic_land_names,
            super_types,
            multi_word_sub_types,
            foreign_sets,
        }
    }
}

/// Parse foreign card data from Scryfall prints URL
pub fn parse_foreign(
    sf_prints_url: &str,
    card_name: &str,
    card_number: &str,
    set_name: &str,
) -> Vec<MtgjsonForeignData> {
    let mut card_foreign_entries = Vec::new();
    
    // Add information to get all languages
    let modified_url = sf_prints_url.replace("&unique=prints", "+lang%3Aany&unique=prints");
    
    // TODO: Implement ScryfallProvider download_all_pages
    // For now, return empty vector as placeholder
    println!("Parsing foreign data for {} #{} in {}", card_name, card_number, set_name);
    
    card_foreign_entries
}

/// Parse card types into super types, types, and subtypes
pub fn parse_card_types(card_type: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut sub_types = Vec::new();
    let mut super_types = Vec::new();
    let mut types = Vec::new();
    
    let constants = Constants::new();
    
    let supertypes_and_types: String;
    
    if !card_type.contains("—") {
        supertypes_and_types = card_type.to_string();
    } else {
        let split_type: Vec<&str> = card_type.split("—").collect();
        supertypes_and_types = split_type[0].to_string();
        let subtypes = split_type[1];
        
        // Planes are an entire sub-type, whereas normal cards are split by spaces
        if card_type.starts_with("Plane") {
            sub_types.push(subtypes.trim().to_string());
        } else {
            let mut modified_subtypes = subtypes.to_string();
            let mut special_case_found = false;
            
            for special_case in &constants.multi_word_sub_types {
                if subtypes.contains(special_case) {
                    modified_subtypes = modified_subtypes.replace(special_case, &special_case.replace(" ", "!"));
                    special_case_found = true;
                }
            }
            
            sub_types = modified_subtypes
                .split_whitespace()
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect();
                
            if special_case_found {
                for sub_type in &mut sub_types {
                    *sub_type = sub_type.replace("!", " ");
                }
            }
        }
    }
    
    for value in supertypes_and_types.split_whitespace() {
        if constants.super_types.contains(&value.to_string()) {
            super_types.push(value.to_string());
        } else if !value.is_empty() {
            types.push(value.to_string());
        }
    }
    
    (super_types, types, sub_types)
}

/// Get card colors from mana cost
pub fn get_card_colors(mana_cost: &str) -> Vec<String> {
    let color_options = vec!["W", "U", "B", "R", "G"];
    let mut ret_val = Vec::new();
    
    for color in color_options {
        if mana_cost.contains(color) {
            ret_val.push(color.to_string());
        }
    }
    
    ret_val
}

/// Check if a string represents a number
pub fn is_number(string: &str) -> bool {
    // Try parsing as float
    if string.parse::<f64>().is_ok() {
        return true;
    }
    
    // Try unicode numeric parsing
    if string.chars().all(|c| c.is_numeric()) {
        return true;
    }
    
    false
}

/// Get card's converted mana cost from mana cost string
pub fn get_card_cmc(mana_cost: &str) -> f64 {
    let mut total = 0.0;
    
    let re = Regex::new(r"\{([^}]*)\}").unwrap();
    let symbols: Vec<String> = re
        .captures_iter(mana_cost.trim())
        .map(|cap| cap[1].to_string())
        .collect();
    
    for element in symbols {
        let mut element = element;
        
        // Address 2/W, G/W, etc as "higher" cost always first
        if element.contains('/') {
            element = element.split('/').next().unwrap().to_string();
        }
        
        if is_number(&element) {
            total += element.parse::<f64>().unwrap_or(0.0);
        } else if element == "X" || element == "Y" || element == "Z" {
            // Placeholder mana - continue without adding
            continue;
        } else if element.starts_with('H') {
            // Half mana
            total += 0.5;
        } else {
            total += 1.0;
        }
    }
    
    total
}

/// Parse printings from Scryfall prints URL
pub fn parse_printings(sf_prints_url: Option<&str>) -> Vec<String> {
    let mut card_sets = HashSet::new();
    
    if let Some(mut url) = sf_prints_url {
        // TODO: Implement actual Scryfall API calls
        // This is a placeholder implementation
        println!("Parsing printings from URL: {}", url);
        
        // For now, return empty vector
    }
    
    let mut result: Vec<String> = card_sets.into_iter().collect();
    result.sort();
    result
}

/// Parse legalities from Scryfall format to MTGJSON format
pub fn parse_legalities(sf_card_legalities: &HashMap<String, String>) -> MtgjsonLegalities {
    let mut card_legalities = MtgjsonLegalities::new();
    
    for (key, value) in sf_card_legalities {
        if value != "not_legal" {
            let capitalized_value = capitalize_first_letter(value);
            
            match key.to_lowercase().as_str() {
                "standard" => card_legalities.standard = Some(capitalized_value),
                "pioneer" => card_legalities.pioneer = Some(capitalized_value),
                "modern" => card_legalities.modern = Some(capitalized_value),
                "legacy" => card_legalities.legacy = Some(capitalized_value),
                "vintage" => card_legalities.vintage = Some(capitalized_value),
                "commander" => card_legalities.commander = Some(capitalized_value),
                "brawl" => card_legalities.brawl = Some(capitalized_value),
                "pauper" => card_legalities.pauper = Some(capitalized_value),
                "penny" => card_legalities.penny = Some(capitalized_value),
                "duel" => card_legalities.duel = Some(capitalized_value),
                _ => {} // Unknown format
            }
        }
    }
    
    card_legalities
}

/// Parse rulings from Scryfall URL
pub fn parse_rulings(rulings_url: &str) -> Vec<MtgjsonRuling> {
    let mut mtgjson_rules = Vec::new();
    
    // TODO: Implement actual Scryfall API call
    println!("Parsing rulings from URL: {}", rulings_url);
    
    // For now, return empty vector as placeholder
    
    // Sort rulings by date and text - TODO: implement after actual data loading
    // mtgjson_rules.sort_by(|a, b| {
    //     a.date.cmp(&b.date).then_with(|| a.text.cmp(&b.text))
    // });
    
    mtgjson_rules
}

/// Add UUID to MTGJSON objects (placeholder implementation)
pub fn add_uuid_placeholder(object_name: &str, is_token: bool, set_code: &str) -> String {
    // This is a simplified version - the actual implementation would need
    // access to all object fields to generate proper UUIDs
    
    // For now, generate a random UUID as placeholder
    // In real implementation, this would use specific object properties
    let uuid_v5 = Uuid::new_v4();
    
    println!("Generated UUID: {} for object {} in set {}", uuid_v5, object_name, set_code);
    uuid_v5.to_string()
}

/// Add leadership skills to a card
pub fn add_leadership_skills(mtgjson_card: &mut MtgjsonCard) {
    let override_cards = vec!["Grist, the Hunger Tide"];
    
    let is_commander_legal = override_cards.contains(&mtgjson_card.name.as_str())
        || (mtgjson_card.type_.contains("Legendary") 
            && mtgjson_card.type_.contains("Creature")
            && mtgjson_card.type_ != "flip"
            && (mtgjson_card.side.as_deref() == Some("a") || mtgjson_card.side.is_none()))
        || mtgjson_card.text.contains("can be your commander");
    
    let is_oathbreaker_legal = mtgjson_card.type_.contains("Planeswalker");
    
    // This would need access to WhatsInStandardProvider to determine brawl legality
    let is_brawl_legal = false; // Placeholder
    
    if is_commander_legal || is_oathbreaker_legal || is_brawl_legal {
        mtgjson_card.leadership_skills = Some(MtgjsonLeadershipSkills {
            brawl: is_brawl_legal,
            commander: is_commander_legal,
            oathbreaker: is_oathbreaker_legal,
        });
    }
}

/// Build MTGJSON set from set code
pub fn build_mtgjson_set(set_code: &str) -> Option<MtgjsonSet> {
    let mut mtgjson_set = MtgjsonSet::new();
    
    // TODO: Implement actual data fetching from Scryfall
    println!("Building MTGJSON set for: {}", set_code);
    
    // Set basic properties (placeholder)
    mtgjson_set.code = set_code.to_uppercase();
    mtgjson_set.name = format!("Set {}", set_code); // Placeholder
    
    // TODO: Implement the full build process:
    // 1. Get set data from Scryfall or local cache
    // 2. Build cards using build_base_mtgjson_cards
    // 3. Add various enhancements (starter cards, variations, etc.)
    // 4. Build tokens
    // 5. Add sealed products
    // 6. Set metadata
    
    Some(mtgjson_set)
}

/// Helper function to capitalize first letter
fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Mark duel deck assignments for cards
pub fn mark_duel_decks(set_code: &str, mtgjson_cards: &mut [MtgjsonCard]) {
    println!("Marking duel deck status for {}", set_code);
    
    if set_code.starts_with("DD") || set_code == "GS1" {
        let mut land_pile_marked = false;
        let mut side_letter_as_number = b'a';
        
        let constants = Constants::new();
        
        // Sort cards for consistent processing - TODO: implement Ord trait
        // mtgjson_cards.sort();
        
        for card in mtgjson_cards.iter_mut() {
            if constants.basic_land_names.contains(&card.name) {
                land_pile_marked = true;
            } else if card.type_.contains("Token") || card.type_.contains("Emblem") {
                continue;
            } else if land_pile_marked {
                side_letter_as_number += 1;
                land_pile_marked = false;
            }
            
            card.duel_deck = Some((side_letter_as_number as char).to_string());
        }
    }
    
    println!("Finished marking duel deck status for {}", set_code);
}

/// Parse keyrune code from URL
pub fn parse_keyrune_code(url: &str) -> String {
    // Extract filename stem from URL
    let path = std::path::Path::new(url);
    let file_stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_uppercase();
    
    // TODO: Load keyrune_code_overrides.json
    // For now, return the file stem as-is
    file_stem
}

/// Get translation data for a set name
pub fn get_translation_data(mtgjson_set_name: &str) -> Option<HashMap<String, String>> {
    // TODO: Load mkm_set_name_translations.json
    // For now, return None as placeholder
    println!("Getting translation data for: {}", mtgjson_set_name);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card_types_basic() {
        let (super_types, types, sub_types) = parse_card_types("Creature — Human Wizard");
        assert_eq!(super_types, Vec::<String>::new());
        assert_eq!(types, vec!["Creature"]);
        assert_eq!(sub_types, vec!["Human", "Wizard"]);
    }

    #[test]
    fn test_parse_card_types_legendary() {
        let (super_types, types, sub_types) = parse_card_types("Legendary Creature — Human Wizard");
        assert_eq!(super_types, vec!["Legendary"]);
        assert_eq!(types, vec!["Creature"]);
        assert_eq!(sub_types, vec!["Human", "Wizard"]);
    }

    #[test]
    fn test_get_card_colors() {
        let colors = get_card_colors("{2}{W}{U}");
        assert_eq!(colors, vec!["W", "U"]);
    }

    #[test]
    fn test_get_card_cmc_simple() {
        assert_eq!(get_card_cmc("{3}"), 3.0);
        assert_eq!(get_card_cmc("{2}{W}{U}"), 4.0);
    }

    #[test]
    fn test_get_card_cmc_hybrid() {
        assert_eq!(get_card_cmc("{2/W}"), 2.0); // Takes higher cost
    }

    #[test]
    fn test_is_number() {
        assert!(is_number("123"));
        assert!(is_number("12.5"));
        assert!(!is_number("abc"));
        assert!(!is_number("X"));
    }
}