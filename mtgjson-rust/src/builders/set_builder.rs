use crate::classes::{
    JsonObject, MtgjsonCardObject, MtgjsonDeckObject, MtgjsonForeignDataObject,
    MtgjsonGameFormatsObject, MtgjsonIdentifiers, MtgjsonLeadershipSkillsObject, 
    MtgjsonLegalitiesObject, MtgjsonMetaObject, MtgjsonRelatedCardsObject, 
    MtgjsonRulingObject, MtgjsonSealedProductObject, MtgjsonSetObject, 
    MtgjsonTranslations
};
use crate::providers::scryfall::ScryfallProvider;

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

/// Parse foreign card data from Scryfall prints URL (async implementation)
pub async fn parse_foreign_async(
    sf_prints_url: &str,
    card_name: &str,
    card_number: &str,
    set_name: &str,
) -> Result<Vec<MtgjsonForeignDataObject>, Box<dyn std::error::Error>> {
    let mut card_foreign_entries = Vec::new();
    
    // Add information to get all languages
    let modified_url = sf_prints_url.replace("&unique=prints", "+lang%3Aany&unique=prints");
    
    // Create Scryfall provider and download all pages
    let provider = ScryfallProvider::new()?;
    let prints_api_json = Python::with_gil(|py| {
        provider.download_all_pages(py, &modified_url, None)
    })?;
    
    if prints_api_json.is_empty() {
        eprintln!("No data found for {}", modified_url);
        return Ok(card_foreign_entries);
    }

    let constants = Constants::new();
    
    // Process each foreign card entry
    for foreign_card_py in prints_api_json.iter() {
        // Convert Python object to JSON Value for processing
        let foreign_card_str = foreign_card_py.to_string();
        let foreign_card: Value = serde_json::from_str(&foreign_card_str)?;
        
        // Skip if wrong set, number, or English
        let card_set = foreign_card.get("set").and_then(|v| v.as_str()).unwrap_or("");
        let card_collector_number = foreign_card.get("collector_number").and_then(|v| v.as_str()).unwrap_or("");
        let card_lang = foreign_card.get("lang").and_then(|v| v.as_str()).unwrap_or("");
        
        if set_name != card_set || card_number != card_collector_number || card_lang == "en" {
            continue;
        }

        let mut card_foreign_entry = MtgjsonForeignDataObject::new();
        
        // Map language using constants
        if let Some(language) = constants.language_map.get(card_lang) {
            card_foreign_entry.language = Some(language.clone());
        } else {
            eprintln!("Warning: Unable to get language for {:?}", foreign_card);
        }

        // Handle multiverse IDs
        if let Some(multiverse_ids) = foreign_card.get("multiverse_ids")
            .and_then(|v| v.as_array()) {
            if !multiverse_ids.is_empty() {
                if let Some(id) = multiverse_ids[0].as_u64() {
                    card_foreign_entry.multiverse_id = Some(id as i32); // Deprecated - Remove in 5.4.0
                    card_foreign_entry.identifiers.multiverse_id = Some(id.to_string());
                }
            }
        }

        // Set Scryfall ID
        if let Some(scryfall_id) = foreign_card.get("id").and_then(|v| v.as_str()) {
            card_foreign_entry.identifiers.scryfall_id = Some(scryfall_id.to_string());
        }

        // Handle card faces for double-faced cards
        let mut actual_card_data = &foreign_card;
        if let Some(card_faces) = foreign_card.get("card_faces").and_then(|v| v.as_array()) {
            // Determine which face to use based on card name
            let face_index = if let Some(card_name_from_data) = foreign_card.get("name").and_then(|v| v.as_str()) {
                let first_face_name = card_name_from_data.split('/').next().unwrap_or("").trim();
                if card_name.to_lowercase() == first_face_name.to_lowercase() {
                    0
                } else {
                    1
                }
            } else {
                0
            };

            println!("Split card found: Using face {} for {}", face_index, card_name);
            
            // Build the full name from all faces
            let face_names: Vec<String> = card_faces.iter()
                .filter_map(|face| {
                    face.get("printed_name").and_then(|v| v.as_str())
                        .or_else(|| face.get("name").and_then(|v| v.as_str()))
                        .map(|s| s.to_string())
                })
                .collect();
            
            if !face_names.is_empty() {
                card_foreign_entry.name = Some(face_names.join(" // "));
            }

            // Use the specific face data
            if let Some(face_data) = card_faces.get(face_index) {
                actual_card_data = face_data;
                
                card_foreign_entry.face_name = face_data.get("printed_name")
                    .and_then(|v| v.as_str())
                    .or_else(|| face_data.get("name").and_then(|v| v.as_str()))
                    .map(|s| s.to_string());
                
                if card_foreign_entry.face_name.is_none() {
                    println!("Unable to resolve face_name for {:?}, using name", face_data);
                    card_foreign_entry.face_name = face_data.get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                }
            }
        }

        // Set the name if not already set
        if card_foreign_entry.name.is_none() {
            card_foreign_entry.name = actual_card_data.get("printed_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // Special case for IKO Japanese cards (https://github.com/mtgjson/mtgjson/issues/611)
            if set_name.to_uppercase() == "IKO" && 
               card_foreign_entry.language.as_deref() == Some("Japanese") {
                if let Some(ref name) = card_foreign_entry.name {
                    card_foreign_entry.name = Some(name.split(" //").next().unwrap_or(name).to_string());
                }
            }
        }

        // Set text fields
        card_foreign_entry.text = actual_card_data.get("printed_text")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        card_foreign_entry.flavor_text = actual_card_data.get("flavor_text")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        card_foreign_entry.type_ = actual_card_data.get("printed_type_line")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Only add if we have a name
        if card_foreign_entry.name.is_some() {
            card_foreign_entries.push(card_foreign_entry);
        }
    }

    Ok(card_foreign_entries)
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

/// Parse printings from Scryfall prints URL (async implementation)
pub async fn parse_printings_async(sf_prints_url: Option<&str>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut card_sets = HashSet::new();

    if let Some(starting_url) = sf_prints_url {
        let provider = ScryfallProvider::new()?;
        let mut current_url = starting_url.to_string();

        loop {
            // Download JSON from Scryfall API using the provider
            let params = None;
            let prints_api_json = provider.download(&current_url, params).await?;
            
            if let Some(object_type) = prints_api_json.get("object").and_then(|v| v.as_str()) {
                if object_type == "error" {
                    eprintln!("Bad download: {}", current_url);
                    break;
                }
            }

            // Extract set codes from the data array
            if let Some(data_array) = prints_api_json.get("data").and_then(|v| v.as_array()) {
                for card in data_array {
                    if let Some(set_code) = card.get("set").and_then(|v| v.as_str()) {
                        card_sets.insert(set_code.to_uppercase());
                    }
                }
            }

            // Check for pagination
            let has_more = prints_api_json.get("has_more")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
                
            if !has_more {
                break;
            }

            if let Some(next_page) = prints_api_json.get("next_page").and_then(|v| v.as_str()) {
                current_url = next_page.to_string();
            } else {
                break;
            }
        }
    }

    let mut result: Vec<String> = card_sets.into_iter().collect();
    result.sort();
    Ok(result)
}

/// Parse legalities from Scryfall format to MTGJSON format
pub fn parse_legalities(sf_card_legalities: &HashMap<String, String>) -> MtgjsonLegalitiesObject {
    let mut card_legalities = MtgjsonLegalitiesObject::new();
    
    for (key, value) in sf_card_legalities {
        if value != "not_legal" {
            let capitalized_value = capitalize_first_letter(value);
            
            match key.to_lowercase().as_str() {
                "standard" => card_legalities.standard = capitalized_value.clone(),
                "pioneer" => card_legalities.pioneer = capitalized_value.clone(),
                "modern" => card_legalities.modern = capitalized_value.clone(),
                "legacy" => card_legalities.legacy = capitalized_value.clone(),
                "vintage" => card_legalities.vintage = capitalized_value.clone(),
                "commander" => card_legalities.commander = capitalized_value.clone(),
                "brawl" => card_legalities.brawl = capitalized_value.clone(),
                "pauper" => card_legalities.pauper = capitalized_value.clone(),
                "penny" => card_legalities.penny = capitalized_value.clone(),
                "duel" => card_legalities.duel = capitalized_value.clone(),
                _ => {} // Unknown format
            }
        }
    }
    
    card_legalities
}

/// Parse rulings from Scryfall URL (async implementation)
pub async fn parse_rulings_async(rulings_url: &str) -> Result<Vec<MtgjsonRulingObject>, Box<dyn std::error::Error>> {
    let mut mtgjson_rules = Vec::new();
    
    // Download JSON from Scryfall API using the provider
    let provider = ScryfallProvider::new()?;
    let rules_api_json = provider.download(rulings_url, None).await?;
    
    if let Some(object_type) = rules_api_json.get("object").and_then(|v| v.as_str()) {
        if object_type == "error" {
            eprintln!("Error downloading URL {}: {:?}", rulings_url, rules_api_json);
            return Ok(mtgjson_rules);
        }
    }

    // Process the rulings data
    if let Some(data_array) = rules_api_json.get("data").and_then(|v| v.as_array()) {
        for sf_rule in data_array {
            let date = sf_rule.get("published_at")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();
                
            let comment = sf_rule.get("comment")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();

            let mtgjson_rule = MtgjsonRulingObject::new(date, comment);
            mtgjson_rules.push(mtgjson_rule);
        }
    }

    // Sort rulings by date and text like the Python version
    mtgjson_rules.sort_by(|a, b| {
        a.date.cmp(&b.date).then_with(|| a.text.cmp(&b.text))
    });

    Ok(mtgjson_rules)
}

/// Get Scryfall set data for a specific set (async implementation)
pub async fn get_scryfall_set_data_async(set_code: &str) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    let provider = ScryfallProvider::new()?;
    let url = format!("https://api.scryfall.com/sets/{}", set_code);
    
    let set_data = provider.download(&url, None).await?;

    if set_data.get("object").and_then(|v| v.as_str()) == Some("error") {
        eprintln!("Failed to download {}", set_code);
        return Ok(None);
    }

    Ok(Some(set_data))
}

/// Parse foreign card data from Scryfall prints URL (main public interface)
pub fn parse_foreign(
    sf_prints_url: &str,
    card_name: &str,
    card_number: &str,
    set_name: &str,
) -> Vec<MtgjsonForeignDataObject> {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(parse_foreign_async(sf_prints_url, card_name, card_number, set_name))
        .unwrap_or_default()
}

/// Parse printings from Scryfall prints URL (main public interface)
pub fn parse_printings(sf_prints_url: Option<&str>) -> Vec<String> {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(parse_printings_async(sf_prints_url))
        .unwrap_or_default()
}

/// Parse rulings from Scryfall URL (main public interface)  
pub fn parse_rulings(rulings_url: &str) -> Vec<MtgjsonRulingObject> {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(parse_rulings_async(rulings_url))
        .unwrap_or_default()
}

/// Get Scryfall set data for a specific set (main public interface)
pub fn get_scryfall_set_data(set_code: &str) -> Option<Value> {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_scryfall_set_data_async(set_code))
        .unwrap_or(None)
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
pub fn add_leadership_skills(mtgjson_card: &mut MtgjsonCardObject) {
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
        mtgjson_card.leadership_skills = Some(MtgjsonLeadershipSkillsObject {
            brawl: is_brawl_legal,
            commander: is_commander_legal,
            oathbreaker: is_oathbreaker_legal,
        });
    }
}

/// Build MTGJSON set from set code
pub fn build_mtgjson_set(set_code: &str) -> Option<MtgjsonSetObject> {
    let mut mtgjson_set = MtgjsonSetObject::new();
    mtgjson_set.code = Some(set_code.to_uppercase());
    
    // Add basic functionality
    add_variations_and_alternative_fields(&mut mtgjson_set);
    add_other_face_ids(&mut mtgjson_set.cards);
    link_same_card_different_details(&mut mtgjson_set);
    add_rebalanced_to_original_linkage(&mut mtgjson_set);
    relocate_miscellaneous_tokens(&mut mtgjson_set);
    add_is_starter_option(&mut mtgjson_set);
    
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
pub fn mark_duel_decks(set_code: &str, mtgjson_cards: &mut [MtgjsonCardObject]) {
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

/// Add variations and alternative fields to cards within a set
pub fn add_variations_and_alternative_fields(mtgjson_set: &mut MtgjsonSetObject) {
    if let Some(ref code) = mtgjson_set.code {
        println!("Adding variations for {}", code);
        
        let mut distinct_card_printings_found: HashSet<String> = HashSet::new();
        let constants = Constants::new();
        
        // We need to work with indices to avoid borrowing issues
        let card_count = mtgjson_set.cards.len();
        
        for i in 0..card_count {
            // Collect variations for this card
            let mut variations = Vec::new();
            let current_card_name = mtgjson_set.cards[i].name.split(" (").next().unwrap_or(&mtgjson_set.cards[i].name).to_string();
            let current_face_name = mtgjson_set.cards[i].face_name.clone();
            let current_uuid = mtgjson_set.cards[i].uuid.clone();
            let current_number = mtgjson_set.cards[i].number.clone();
            
            for j in 0..card_count {
                if i == j {
                    continue;
                }
                
                let other_card_name = mtgjson_set.cards[j].name.split(" (").next().unwrap_or(&mtgjson_set.cards[j].name).to_string();
                let other_face_name = mtgjson_set.cards[j].face_name.clone();
                let other_uuid = mtgjson_set.cards[j].uuid.clone();
                let other_number = mtgjson_set.cards[j].number.clone();
                
                if current_card_name == other_card_name
                    && current_face_name == other_face_name
                    && current_uuid != other_uuid
                    && (other_number != current_number || other_number.is_empty())
                {
                    variations.push(other_uuid);
                }
            }
            
            if !variations.is_empty() {
                mtgjson_set.cards[i].variations = variations;
            }
            
            // Add alternative tag - ignore singleton printings and basics
            let has_variations = !mtgjson_set.cards[i].variations.is_empty();
            if !has_variations || constants.basic_land_names.contains(&mtgjson_set.cards[i].name) {
                continue;
            }
            
            // In each set, a card has to be unique by all of these attributes
            let distinct_card_printing = format!(
                "{}|{}|{}|{}|{}",
                mtgjson_set.cards[i].name,
                mtgjson_set.cards[i].border_color,
                mtgjson_set.cards[i].frame_version,
                mtgjson_set.cards[i].frame_effects.join(","),
                mtgjson_set.cards[i].side.as_deref().unwrap_or("")
            );
            
            // Special handling for certain sets
            if code == "UNH" || code == "10E" {
                let finishes = mtgjson_set.cards[i].finishes.join(",");
                let distinct_card_printing = format!("{}|{}", distinct_card_printing, finishes);
            }
            
            if distinct_card_printings_found.contains(&distinct_card_printing) {
                mtgjson_set.cards[i].is_alternative = Some(true);
            } else {
                distinct_card_printings_found.insert(distinct_card_printing);
            }
        }
        
        println!("Finished adding variations for {}", code);
    }
}

/// Add other face IDs to all cards within a group
pub fn add_other_face_ids(cards_to_act_on: &mut [MtgjsonCardObject]) {
    if cards_to_act_on.is_empty() {
        return;
    }

    println!("Adding otherFaceIds to group");
    
    let card_count = cards_to_act_on.len();
    
    for i in 0..card_count {
        let current_names = cards_to_act_on[i].get_names();
        if current_names.is_empty() {
            continue;
        }
        
        let mut other_face_ids = Vec::new();
        let current_uuid = cards_to_act_on[i].uuid.clone();
        let current_layout = cards_to_act_on[i].layout.clone();
        let current_side = cards_to_act_on[i].side.clone();
        let current_number = cards_to_act_on[i].number.clone();
        
        for j in 0..card_count {
            if i == j {
                continue;
            }
            
            let other_face_name = cards_to_act_on[j].face_name.as_deref().unwrap_or("");
            let other_uuid = cards_to_act_on[j].uuid.clone();
            let other_side = cards_to_act_on[j].side.clone();
            let other_number = cards_to_act_on[j].number.clone();
            
            if !current_names.contains(&other_face_name.to_string()) {
                continue;
            }
            
            if current_uuid == other_uuid {
                continue;
            }
            
            if current_layout == "meld" {
                // Meld cards should account for the other sides
                if current_side != other_side {
                    other_face_ids.push(other_uuid);
                }
            } else if !other_number.is_empty() {
                // Most split cards should have the same number
                if other_number == current_number {
                    other_face_ids.push(other_uuid);
                }
            } else {
                // No number? No problem, just add it!
                other_face_ids.push(other_uuid);
            }
        }
        
        if !other_face_ids.is_empty() {
            cards_to_act_on[i].other_face_ids = other_face_ids;
        }
    }
    
    println!("Finished adding otherFaceIds to group");
}

/// Link same card with different details (foil/non-foil versions)
pub fn link_same_card_different_details(mtgjson_set: &mut MtgjsonSetObject) {
    if let Some(ref code) = mtgjson_set.code {
        println!("Linking multiple printings for {}", code);
        
        let mut cards_seen: HashMap<String, usize> = HashMap::new();
        let card_count = mtgjson_set.cards.len();
        
        for i in 0..card_count {
            let illustration_id = mtgjson_set.cards[i].identifiers.scryfall_illustration_id
                .as_deref()
                .unwrap_or("")
                .to_string();
                
            if let Some(&other_index) = cards_seen.get(&illustration_id) {
                let has_nonfoil = mtgjson_set.cards[i].finishes.contains(&"nonfoil".to_string());
                let other_uuid = mtgjson_set.cards[other_index].uuid.clone();
                let current_uuid = mtgjson_set.cards[i].uuid.clone();
                
                if has_nonfoil {
                    mtgjson_set.cards[other_index].identifiers.mtgjson_non_foil_version_id = Some(current_uuid);
                    mtgjson_set.cards[i].identifiers.mtgjson_foil_version_id = Some(other_uuid);
                } else {
                    mtgjson_set.cards[other_index].identifiers.mtgjson_foil_version_id = Some(current_uuid);
                    mtgjson_set.cards[i].identifiers.mtgjson_non_foil_version_id = Some(other_uuid);
                }
            } else {
                cards_seen.insert(illustration_id, i);
            }
        }
        
        println!("Finished linking multiple printings for {}", code);
    }
}

/// Build base MTGJSON cards from a set
pub fn build_base_mtgjson_cards(
    set_code: &str,
    additional_cards: Option<Vec<HashMap<String, serde_json::Value>>>,
    is_token: bool,
    set_release_date: &str,
) -> Vec<MtgjsonCardObject> {
    println!("Building cards for {}", set_code);
    
    // TODO: Implement actual Scryfall API call
    // let cards = ScryfallProvider::download_cards(set_code);
    
    let mtgjson_cards = Vec::new();
    
    // For now, return empty vector as placeholder
    // In real implementation, this would:
    // 1. Download cards from Scryfall
    // 2. Process each card through build_mtgjson_card
    // 3. Sort cards consistently
    
    println!("Finished building cards for {}", set_code);
    mtgjson_cards
}

/// Add rebalanced to original linkage for Alchemy cards
pub fn add_rebalanced_to_original_linkage(mtgjson_set: &mut MtgjsonSetObject) {
    let mut rebalanced_pairs = Vec::new();
    
    // Check if cards have is_rebalanced field (simplified)
    for i in 0..mtgjson_set.cards.len() {
        // For now, just check if the name starts with "A-" for Alchemy cards
        if mtgjson_set.cards[i].name.starts_with("A-") {
            let original_name = mtgjson_set.cards[i].name.replacen("A-", "", 1);
            for j in 0..mtgjson_set.cards.len() {
                if i != j && mtgjson_set.cards[j].name == original_name {
                    rebalanced_pairs.push((i, j, mtgjson_set.cards[i].uuid.clone()));
                }
            }
        }
    }
    
    // Second pass: update the cards
    for (_rebalanced_idx, original_idx, rebalanced_uuid) in rebalanced_pairs {
        mtgjson_set.cards[original_idx].rebalanced_printings.push(rebalanced_uuid);
    }
}

/// Relocate miscellaneous tokens from cards to tokens array
pub fn relocate_miscellaneous_tokens(mtgjson_set: &mut MtgjsonSetObject) {
    if let Some(ref code) = mtgjson_set.code {
        println!("Relocate tokens for {}", code);
        
        let token_types = vec!["token", "double_faced_token", "emblem", "art_series"];
        
        // Identify unique tokens from cards
        let mut tokens_found = HashSet::new();
        for card in &mtgjson_set.cards {
            if token_types.contains(&card.layout.as_str()) {
                if let Some(ref scryfall_id) = card.identifiers.scryfall_id {
                    tokens_found.insert(scryfall_id.clone());
                }
            }
        }
        
        // Remove tokens from cards array
        mtgjson_set.cards.retain(|card| !token_types.contains(&card.layout.as_str()));
        
        // Store Scryfall IDs for later token processing
        // TODO: Download Scryfall objects for these tokens
        println!("Found {} tokens to relocate", tokens_found.len());
        
        println!("Finished relocating tokens for {}", code);
    }
}

/// Get the base and total set sizes
pub fn get_base_and_total_set_sizes(
    base_set_size: i32,
    total_set_size: i32,
    mtgjson_set: &mut MtgjsonSetObject,
) {
    mtgjson_set.base_set_size = Some(base_set_size);
    mtgjson_set.total_set_size = total_set_size;
}

/// Add starter card designation to cards not available in boosters
pub fn add_is_starter_option(mtgjson_set: &mut MtgjsonSetObject) {
    let release_date = &mtgjson_set.release_date;
    if release_date.as_str() > "2019-10-01" {
        // Implementation here
    }
}

/// Build sealed products for a set
pub fn build_sealed_products(set_code: &str) -> Vec<MtgjsonSealedProductObject> {
    println!("Building sealed products for {}", set_code);
    
    let sealed_products = Vec::new();
    
    // TODO: Implement actual sealed product building
    // This would involve:
    // 1. Getting sealed product data from various providers
    // 2. Creating MtgjsonSealedProduct objects
    // 3. Linking products to sets
    
    println!("Finished building sealed products for {}", set_code);
    sealed_products
}

/// Build decks for a set 
pub fn build_decks(set_code: &str) -> Vec<MtgjsonDeckObject> {
    println!("Building decks for {}", set_code);
    
    let decks = Vec::new();
    
    // TODO: Implement actual deck building
    // This would involve:
    // 1. Getting deck data from GitHub provider
    // 2. Creating MtgjsonDeck objects
    // 3. Linking decks to sets
    
    println!("Finished building decks for {}", set_code);
    decks
}

/// Enhance cards with additional metadata
pub fn enhance_cards_with_metadata(mtgjson_cards: &mut [MtgjsonCardObject]) {
    println!("Enhancing cards with metadata");
    
    for card in mtgjson_cards.iter_mut() {
        // Add color identity for commanders
        if card.type_.contains("Legendary") && card.type_.contains("Creature") {
            card.color_identity = card.colors.clone();
        }
        
        // Mark basic lands
        let constants = Constants::new();
        if constants.basic_land_names.contains(&card.name) {
            card.supertypes.push("Basic".to_string());
        }
        
        // Calculate EDH rec rank (placeholder)
        // TODO: Implement actual EDHREC integration
        
        // Add purchase URLs (placeholder)
        // TODO: Implement actual purchase URL building
    }
    
    println!("Finished enhancing cards");
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