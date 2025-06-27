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
use futures::future::join_all;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use tokio::time::{sleep, Duration};
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

// Global HTTP client for reuse
static HTTP_CLIENT: OnceCell<Client> = OnceCell::new();

fn get_http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent("MTGJSON-Rust/5.0")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client")
    })
}

// Resource loading
lazy_static! {
    static ref RESOURCE_DATA: HashMap<String, serde_json::Value> = load_all_resources();
}

fn load_all_resources() -> HashMap<String, serde_json::Value> {
    let mut resources = HashMap::new();
    let resource_path = Path::new("../mtgjson5/resources");
    
    let resource_files = vec![
        "keyrune_code_overrides.json",
        "mkm_set_name_translations.json", 
        "base_set_sizes.json",
        "world_championship_signatures.json",
        "additional_sets.json",
        "mkm_set_name_fixes.json",
        "set_code_watermarks.json",
        "sealed_name_fixes.json",
        "booster_box_size_overrides.json",
        "cardkingdom_sealed_name_mapping.json",
        "wizards_set_name_fixes.json",
        "gatherer_set_codes.json",
    ];
    
    for file in resource_files {
        let file_path = resource_path.join(file);
        if let Ok(content) = fs::read_to_string(&file_path) {
            if let Ok(json) = serde_json::from_str(&content) {
                resources.insert(file.to_string(), json);
            }
        }
    }
    
    resources
}

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
        language_map.insert("pt".to_string(), "Portuguese (Brazil)".to_string());
        language_map.insert("ja".to_string(), "Japanese".to_string());
        language_map.insert("ko".to_string(), "Korean".to_string());
        language_map.insert("ru".to_string(), "Russian".to_string());
        language_map.insert("zhs".to_string(), "Chinese Simplified".to_string());
        language_map.insert("zht".to_string(), "Chinese Traditional".to_string());
        language_map.insert("he".to_string(), "Hebrew".to_string());
        language_map.insert("la".to_string(), "Latin".to_string());
        language_map.insert("grc".to_string(), "Ancient Greek".to_string());
        language_map.insert("ar".to_string(), "Arabic".to_string());
        language_map.insert("sa".to_string(), "Sanskrit".to_string());
        language_map.insert("ph".to_string(), "Phyrexian".to_string());
        language_map.insert("px".to_string(), "Phyrexian".to_string());
        language_map.insert("qya".to_string(), "Quenya".to_string());

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
            "Host".to_string(),
            "Legendary".to_string(),
            "Ongoing".to_string(),
            "Snow".to_string(),
            "World".to_string(),
        ];

        let multi_word_sub_types = vec![
            "Time Lord".to_string(),
        ];

        let foreign_sets = vec![
            "4BB".to_string(),
            "FBB".to_string(),
            "REN".to_string(),
            "PMPS11".to_string(),
            "PS11".to_string(),
            "PSAL".to_string(),
            "PMPS10".to_string(),
            "PMPS09".to_string(),
            "PMPS08".to_string(),
            "PMPS07".to_string(),
            "PMPS06".to_string(),
            "PSA1".to_string(),
            "PMPS".to_string(),
            "PJJT".to_string(),
            "PHJ".to_string(),
            "PRED".to_string(),
            "RIN".to_string(),
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

/// Scryfall API provider
pub struct ScryfallProvider {
    base_url: String,
    client: &'static Client,
}

impl ScryfallProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.scryfall.com".to_string(),
            client: get_http_client(),
        }
    }

    pub async fn download(&self, url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Ok(serde_json::json!({"object": "error", "details": "HTTP error"}));
        }
        
        let json: serde_json::Value = response.json().await?;
        Ok(json)
    }

    pub async fn download_all_pages(&self, url: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut all_data = Vec::new();
        let mut current_url = Some(url.to_string());
        
        while let Some(url) = current_url {
            let response = self.download(&url).await?;
            
            if response["object"] == "error" {
                break;
            }
            
            if let Some(data) = response["data"].as_array() {
                all_data.extend(data.clone());
            }
            
            current_url = response["next_page"].as_str().map(|s| s.to_string());
            
            // Rate limiting
            sleep(Duration::from_millis(50)).await;
        }
        
        Ok(all_data)
    }

    pub async fn download_cards(&self, set_code: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/cards/search?q=set:{}&unique=prints", self.base_url, set_code);
        self.download_all_pages(&url).await
    }

    pub async fn get_set_data(&self, set_code: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/sets/{}", self.base_url, set_code);
        self.download(&url).await
    }
}

/// Parse foreign card data from Scryfall prints URL
pub async fn parse_foreign(
    sf_prints_url: &str,
    card_name: &str,
    card_number: &str,
    set_name: &str,
) -> Vec<MtgjsonForeignData> {
    let mut card_foreign_entries = Vec::new();
    
    // Add information to get all languages
    let modified_url = sf_prints_url.replace("&unique=prints", "+lang%3Aany&unique=prints");
    
    let provider = ScryfallProvider::new();
    let prints_api_json = match provider.download_all_pages(&modified_url).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error downloading foreign data for {}: {}", card_name, e);
            return card_foreign_entries;
        }
    };
    
    let constants = Constants::new();
    
    for foreign_card in prints_api_json {
        // Skip if not matching set, number, or is English
        if foreign_card["set"].as_str() != Some(set_name)
            || foreign_card["collector_number"].as_str() != Some(card_number)
            || foreign_card["lang"].as_str() == Some("en")
        {
            continue;
        }
        
        let mut card_foreign_entry = MtgjsonForeignData::new();
        
        // Set language
        if let Some(lang_code) = foreign_card["lang"].as_str() {
            if let Some(language) = constants.language_map.get(lang_code) {
                card_foreign_entry.language = Some(language.clone());
            }
        }
        
        // Set multiverse ID
        if let Some(multiverse_ids) = foreign_card["multiverse_ids"].as_array() {
            if !multiverse_ids.is_empty() {
                if let Some(id) = multiverse_ids[0].as_u64() {
                    card_foreign_entry.multiverse_id = Some(id as i32);
                    card_foreign_entry.identifiers.multiverse_id = Some(id.to_string());
                }
            }
        }
        
        // Set Scryfall ID
        if let Some(id) = foreign_card["id"].as_str() {
            card_foreign_entry.identifiers.scryfall_id = Some(id.to_string());
        }
        
        // Handle card faces
        if let Some(card_faces) = foreign_card["card_faces"].as_array() {
            // Determine which face to use
            let face_index = if card_name.to_lowercase() == 
                foreign_card["name"].as_str().unwrap_or("").split("/").next().unwrap_or("").trim().to_lowercase() {
                0
            } else {
                1
            };
            
            // Set full name from all faces
            let face_names: Vec<String> = card_faces.iter()
                .map(|face| face.get("printed_name")
                    .or_else(|| face.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string())
                .collect();
            card_foreign_entry.name = Some(face_names.join(" // "));
            
            // Set face-specific data
            if let Some(face_data) = card_faces.get(face_index) {
                card_foreign_entry.face_name = face_data.get("printed_name")
                    .or_else(|| face_data.get("name"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                
                card_foreign_entry.text = face_data.get("printed_text")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                    
                card_foreign_entry.flavor_text = face_data.get("flavor_text")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                    
                card_foreign_entry.type_ = face_data.get("printed_type_line")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
        } else {
            // Single-faced card
            card_foreign_entry.name = foreign_card.get("printed_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
                
            card_foreign_entry.text = foreign_card.get("printed_text")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
                
            card_foreign_entry.flavor_text = foreign_card.get("flavor_text")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
                
            card_foreign_entry.type_ = foreign_card.get("printed_type_line")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
        }
        
        // Special handling for IKO Japanese cards
        if set_name.to_uppercase() == "IKO" && card_foreign_entry.language == Some("Japanese".to_string()) {
            if let Some(ref name) = card_foreign_entry.name {
                card_foreign_entry.name = Some(name.split(" //").next().unwrap_or(name).to_string());
            }
        }
        
        if card_foreign_entry.name.is_some() {
            card_foreign_entries.push(card_foreign_entry);
        }
    }
    
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
            // Variable mana costs (X, Y, Z) are determined during play, not in mana cost calculation
            // X=0 for CMC calculation per Magic rules
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
pub async fn parse_printings(sf_prints_url: Option<&str>) -> Vec<String> {
    let mut card_sets = HashSet::new();
    
    if let Some(url) = sf_prints_url {
        let provider = ScryfallProvider::new();
        let mut current_url = Some(url.to_string());
        
        while let Some(url) = current_url {
            match provider.download(&url).await {
                Ok(prints_api_json) => {
                    if prints_api_json["object"] == "error" {
                        eprintln!("Bad download: {}", url);
                        break;
                    }
                    
                    if let Some(data) = prints_api_json["data"].as_array() {
                        for card in data {
                            if let Some(set_code) = card["set"].as_str() {
                                card_sets.insert(set_code.to_uppercase());
                            }
                        }
                    }
                    
                    if prints_api_json["has_more"].as_bool() != Some(true) {
                        break;
                    }
                    
                    current_url = prints_api_json["next_page"].as_str().map(|s| s.to_string());
                    
                    // Rate limiting
                    sleep(Duration::from_millis(50)).await;
                }
                Err(e) => {
                    eprintln!("Error downloading printings from {}: {}", url, e);
                    break;
                }
            }
        }
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

/// Parse rulings from Scryfall URL
pub async fn parse_rulings(rulings_url: &str) -> Vec<MtgjsonRuling> {
    let mut mtgjson_rules = Vec::new();
    
    let provider = ScryfallProvider::new();
    match provider.download(rulings_url).await {
        Ok(rules_api_json) => {
            if rules_api_json["object"] == "error" {
                eprintln!("Error downloading URL {}: {:?}", rulings_url, rules_api_json);
                return mtgjson_rules;
            }
            
            if let Some(data) = rules_api_json["data"].as_array() {
                for sf_rule in data {
                    if let (Some(date), Some(text)) = (
                        sf_rule["published_at"].as_str(),
                        sf_rule["comment"].as_str()
                    ) {
                        let ruling = MtgjsonRuling {
                            date: date.to_string(),
                            text: text.to_string(),
                        };
                        mtgjson_rules.push(ruling);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error downloading rulings from {}: {}", rulings_url, e);
        }
    }
    
    // Sort rulings by date and text
    mtgjson_rules.sort_by(|a, b| {
        a.date.cmp(&b.date).then_with(|| a.text.cmp(&b.text))
    });
    
    mtgjson_rules
}

/// Add UUID to MTGJSON objects
pub fn add_uuid(mtgjson_card: &mut MtgjsonCard) {
    let id_source_v5: String;
    let id_source_v4: String;
    
    // Check if this is a token
    if mtgjson_card.types.contains(&"Token".to_string()) || mtgjson_card.types.contains(&"Card".to_string()) {
        // Tokens have a special generation method
        id_source_v5 = format!(
            "{}{}{}{}{}{}{}{}",
            mtgjson_card.name,
            mtgjson_card.face_name.as_deref().unwrap_or(""),
            mtgjson_card.colors.join(""),
            mtgjson_card.power.as_deref().unwrap_or(""),
            mtgjson_card.toughness.as_deref().unwrap_or(""),
            mtgjson_card.side.as_deref().unwrap_or(""),
            &mtgjson_card.set_code.as_ref().map(|s| s[1..].to_lowercase()).unwrap_or_default(),
            mtgjson_card.identifiers.scryfall_id.as_deref().unwrap_or(""),
        );
        
        id_source_v4 = format!(
            "{}{}{}{}{}{}{}",
            mtgjson_card.face_name.as_deref().unwrap_or(&mtgjson_card.name),
            mtgjson_card.colors.join(""),
            mtgjson_card.power.as_deref().unwrap_or(""),
            mtgjson_card.toughness.as_deref().unwrap_or(""),
            mtgjson_card.side.as_deref().unwrap_or(""),
            &mtgjson_card.set_code.as_ref().map(|s| s[1..].to_uppercase()).unwrap_or_default(),
            mtgjson_card.identifiers.scryfall_id.as_deref().unwrap_or(""),
        );
    } else {
        // Normal cards only need a few pieces of data
        id_source_v5 = format!(
            "{}{}{}{}{}",
            "sf", // ScryfallProvider class ID equivalent
            mtgjson_card.identifiers.scryfall_id.as_deref().unwrap_or(""),
            mtgjson_card.identifiers.scryfall_illustration_id.as_deref().unwrap_or(""),
            mtgjson_card.set_code.as_ref().map(|s| s.to_lowercase()).unwrap_or_default(),
            mtgjson_card.name,
        );
        
        id_source_v4 = format!(
            "{}{}{}",
            "sf",
            mtgjson_card.identifiers.scryfall_id.as_deref().unwrap_or(""),
            mtgjson_card.face_name.as_deref().unwrap_or(&mtgjson_card.name),
        );
    }
    
    // Generate UUID v5 using DNS namespace
    let namespace = Uuid::NAMESPACE_DNS;
    mtgjson_card.uuid = Uuid::new_v5(&namespace, id_source_v5.as_bytes()).to_string();
    mtgjson_card.identifiers.mtgjson_v4_id = Some(
        Uuid::new_v5(&namespace, id_source_v4.as_bytes()).to_string()
    );
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

/// Build MTGJSON card from Scryfall object
pub async fn build_mtgjson_card(
    scryfall_object: serde_json::Value,
    face_id: usize,
    is_token: bool,
    set_release_date: String,
) -> Vec<MtgjsonCard> {
    println!("Building {}: {}", 
        scryfall_object["set"].as_str().unwrap_or("").to_uppercase(),
        scryfall_object["name"].as_str().unwrap_or("Unknown"));
    
    let mut mtgjson_cards = Vec::new();
    let mut mtgjson_card = MtgjsonCard::new(is_token);
    let constants = Constants::new();
    
    // Basic card properties
    mtgjson_card.name = scryfall_object["name"].as_str().unwrap_or("").to_string();
    mtgjson_card.language = scryfall_object["lang"].as_str()
        .and_then(|lang| constants.language_map.get(lang))
        .cloned()
        .unwrap_or("English".to_string());
    
    mtgjson_card.flavor_name = scryfall_object.get("flavor_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    mtgjson_card.set_code = Some(scryfall_object["set"].as_str().unwrap_or("").to_uppercase());
    mtgjson_card.identifiers.scryfall_id = scryfall_object["id"].as_str().map(|s| s.to_string());
    mtgjson_card.identifiers.scryfall_oracle_id = scryfall_object.get("oracle_id")
        .or_else(|| scryfall_object["card_faces"][face_id].get("oracle_id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    // Handle card faces
    let face_data = if let Some(card_faces) = scryfall_object["card_faces"].as_array() {
        // Multi-face card processing
        let face_names: Vec<String> = scryfall_object["name"].as_str()
            .unwrap_or("").split("//").map(|s| s.trim().to_string()).collect();
        mtgjson_card.set_names(face_names);
        
        // Set illustration IDs for all faces
        let illustration_ids: Vec<String> = card_faces.iter()
            .map(|face| face.get("illustration_id")
                .and_then(|v| v.as_str())
                .unwrap_or("Missing")
                .to_string())
            .collect();
        mtgjson_card.set_illustration_ids(illustration_ids);
        
        // Handle flavor names for multi-face cards
        if let Some(flavor_name) = card_faces[face_id].get("flavor_name") {
            let all_flavor_names: Vec<String> = card_faces.iter()
                .map(|face| face.get("flavor_name")
                    .or_else(|| face.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string())
                .collect();
            mtgjson_card.flavor_name = Some(all_flavor_names.join(" // "));
            mtgjson_card.face_flavor_name = flavor_name.as_str().map(|s| s.to_string());
        }
        
        // Handle mana costs for split cards
        if let Some(mana_cost) = scryfall_object.get("mana_cost").and_then(|v| v.as_str()) {
            if mana_cost.contains("//") {
                let split_costs: Vec<&str> = mana_cost.split("//").collect();
                if face_id < split_costs.len() {
                    mtgjson_card.colors = get_card_colors(split_costs[face_id]);
                    mtgjson_card.face_mana_value = Some(get_card_cmc(split_costs[face_id]));
                    mtgjson_card.face_converted_mana_cost = mtgjson_card.face_mana_value;
                }
            }
        }
        
        // Handle different layouts
        let layout = scryfall_object["layout"].as_str().unwrap_or("");
        match layout {
            "split" | "transform" | "aftermath" | "adventure" => {
                let face_mana_cost = card_faces[face_id].get("mana_cost")
                    .and_then(|v| v.as_str())
                    .unwrap_or("0");
                mtgjson_card.face_mana_value = Some(get_card_cmc(face_mana_cost));
                mtgjson_card.face_converted_mana_cost = mtgjson_card.face_mana_value;
            }
            "modal_dfc" => {
                let face_mana_cost = card_faces[face_id].get("mana_cost")
                    .and_then(|v| v.as_str())
                    .unwrap_or("0");
                let cmc = get_card_cmc(face_mana_cost);
                mtgjson_card.mana_value = Some(cmc);
                mtgjson_card.face_mana_value = Some(cmc);
                mtgjson_card.converted_mana_cost = Some(cmc);
                mtgjson_card.face_converted_mana_cost = Some(cmc);
            }
            "reversible_card" => {
                if let Some(cmc) = card_faces[face_id].get("cmc").and_then(|v| v.as_f64()) {
                    mtgjson_card.mana_value = Some(cmc);
                    mtgjson_card.converted_mana_cost = Some(cmc);
                }
            }
            _ => {}
        }
        
        // Set watermark and artist from first face
        mtgjson_card.watermark = card_faces[0].get("watermark")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        mtgjson_card.artist = card_faces[face_id].get("artist")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // Handle special aftermath layout
        if let Some(oracle_text) = card_faces.last().and_then(|face| face["oracle_text"].as_str()) {
            if oracle_text.starts_with("Aftermath") {
                mtgjson_card.layout = Some("aftermath".to_string());
            }
        }
        
        // Build additional faces recursively
        if face_id == 0 {
            for i in 1..card_faces.len() {
                let additional_cards = build_mtgjson_card(
                    scryfall_object.clone(), 
                    i, 
                    is_token, 
                    set_release_date.clone()
                ).await;
                mtgjson_cards.extend(additional_cards);
            }
        }
        
        &card_faces[face_id]
    } else {
        &scryfall_object
    };
    
    // Set mana cost from face data
    if let Some(mana_cost) = face_data.get("mana_cost").and_then(|v| v.as_str()) {
        mtgjson_card.mana_cost = Some(mana_cost.to_string());
    }
    
    // Set illustration ID
    mtgjson_card.identifiers.scryfall_illustration_id = scryfall_object.get("illustration_id")
        .or_else(|| face_data.get("illustration_id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    mtgjson_card.identifiers.scryfall_card_back_id = scryfall_object.get("card_back_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    // Set colors if not already set
    if mtgjson_card.colors.is_empty() {
        mtgjson_card.colors = face_data.get("colors")
            .or_else(|| scryfall_object.get("colors"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
            .unwrap_or_default();
    }
    
    // Card-level properties from scryfall_object
    mtgjson_card.attraction_lights = scryfall_object.get("attraction_lights")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_u64()).map(|n| n as i32).collect());
    
    mtgjson_card.border_color = scryfall_object.get("border_color")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    mtgjson_card.color_identity = scryfall_object.get("color_identity")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
        .unwrap_or_default();
    
    if mtgjson_card.mana_value.is_none() {
        mtgjson_card.mana_value = scryfall_object.get("cmc").and_then(|v| v.as_f64());
        mtgjson_card.converted_mana_cost = mtgjson_card.mana_value;
    }
    
    mtgjson_card.edhrec_rank = scryfall_object.get("edhrec_rank").and_then(|v| v.as_u64()).map(|n| n as i32);
    
    mtgjson_card.finishes = scryfall_object.get("finishes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
        .unwrap_or_default();
    
    mtgjson_card.frame_effects = scryfall_object.get("frame_effects")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
        .unwrap_or_default();
    
    mtgjson_card.frame_version = scryfall_object.get("frame")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    mtgjson_card.hand = scryfall_object.get("hand_modifier").and_then(|v| v.as_str()).map(|s| s.to_string());
    
    // Set foil and non-foil flags
    mtgjson_card.has_foil = mtgjson_card.finishes.iter().any(|f| f == "foil" || f == "glossy");
    mtgjson_card.has_non_foil = mtgjson_card.finishes.contains(&"nonfoil".to_string());
    
    mtgjson_card.has_content_warning = scryfall_object.get("content_warning").and_then(|v| v.as_bool());
    mtgjson_card.is_full_art = scryfall_object.get("full_art").and_then(|v| v.as_bool());
    mtgjson_card.is_game_changer = scryfall_object.get("game_changer").and_then(|v| v.as_bool());
    mtgjson_card.is_online_only = scryfall_object.get("digital").and_then(|v| v.as_bool());
    
    // Handle oversized cards
    mtgjson_card.is_oversized = scryfall_object.get("oversized").and_then(|v| v.as_bool())
        .unwrap_or_else(|| mtgjson_card.set_code.as_deref() == Some("OC21"));
    
    mtgjson_card.is_promo = scryfall_object.get("promo").and_then(|v| v.as_bool());
    mtgjson_card.is_reprint = scryfall_object.get("reprint").and_then(|v| v.as_bool());
    mtgjson_card.is_reserved = scryfall_object.get("reserved").and_then(|v| v.as_bool());
    mtgjson_card.is_story_spotlight = scryfall_object.get("story_spotlight").and_then(|v| v.as_bool());
    mtgjson_card.is_textless = scryfall_object.get("textless").and_then(|v| v.as_bool());
    
    mtgjson_card.life = scryfall_object.get("life_modifier").and_then(|v| v.as_str()).map(|s| s.to_string());
    
    // Set booster types
    if scryfall_object.get("booster").and_then(|v| v.as_bool()).unwrap_or(false) {
        mtgjson_card.booster_types.push("default".to_string());
    }
    
    if let Some(promo_types) = scryfall_object.get("promo_types").and_then(|v| v.as_array()) {
        let promo_type_strings: Vec<String> = promo_types.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();
        
        if promo_type_strings.iter().any(|t| t == "starterdeck" || t == "planeswalkerdeck") {
            mtgjson_card.booster_types.push("deck".to_string());
        }
    }
    
    // Set identifiers
    mtgjson_card.identifiers.mcm_id = scryfall_object.get("cardmarket_id")
        .and_then(|v| v.as_u64())
        .map(|n| n.to_string());
    
    mtgjson_card.identifiers.mtg_arena_id = scryfall_object.get("arena_id")
        .and_then(|v| v.as_u64())
        .map(|n| n.to_string());
    
    mtgjson_card.identifiers.mtgo_id = scryfall_object.get("mtgo_id")
        .and_then(|v| v.as_u64())
        .map(|n| n.to_string());
    
    mtgjson_card.identifiers.mtgo_foil_id = scryfall_object.get("mtgo_foil_id")
        .and_then(|v| v.as_u64())
        .map(|n| n.to_string());
    
    mtgjson_card.number = scryfall_object.get("collector_number")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    mtgjson_card.security_stamp = scryfall_object.get("security_stamp")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    // Handle promo types
    mtgjson_card.promo_types = scryfall_object.get("promo_types")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
        .unwrap_or_default();
    
    // Add planeswalkerstamped if number ends with 'p'
    if let Some(ref number) = mtgjson_card.number {
        if number.ends_with('p') {
            mtgjson_card.promo_types.push("planeswalkerstamped".to_string());
        }
    }
    
    // Remove excluded promo types
    mtgjson_card.promo_types.retain(|t| t != "planeswalkerdeck");
    
    // Set release date
    let card_release_date = scryfall_object.get("released_at").and_then(|v| v.as_str());
    if let Some(card_date) = card_release_date {
        if !set_release_date.is_empty() && set_release_date != card_date {
            mtgjson_card.original_release_date = Some(card_date.to_string());
        }
    }
    
    mtgjson_card.rarity = scryfall_object.get("rarity")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    // Set artist if not already set
    if mtgjson_card.artist.is_none() {
        mtgjson_card.artist = scryfall_object.get("artist")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
    }
    
    // Set watermark if not already set
    if mtgjson_card.watermark.is_none() {
        mtgjson_card.watermark = face_data.get("watermark")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
    }
    
    // Handle layout
    if scryfall_object.get("layout").and_then(|v| v.as_str()) == Some("art_series") {
        mtgjson_card.layout = Some("art_series".to_string());
    } else if !mtgjson_card.name.contains("//") {
        if let Some(type_line) = scryfall_object.get("type_line").and_then(|v| v.as_str()) {
            if type_line.to_lowercase().contains("card") || type_line.to_lowercase().contains("token") {
                mtgjson_card.layout = Some("token".to_string());
            }
        }
    }
    
    if mtgjson_card.layout.is_none() {
        mtgjson_card.layout = scryfall_object.get("layout")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
    }
    
    // Set availability
    let games = scryfall_object.get("games")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();
    
    mtgjson_card.availability.arena = games.contains(&"arena") || mtgjson_card.identifiers.mtg_arena_id.is_some();
    mtgjson_card.availability.mtgo = games.contains(&"mtgo") || mtgjson_card.identifiers.mtgo_id.is_some();
    mtgjson_card.availability.paper = !mtgjson_card.is_online_only.unwrap_or(false);
    mtgjson_card.availability.shandalar = games.contains(&"astral");
    mtgjson_card.availability.dreamcast = games.contains(&"sega");
    
    // Face-specific properties
    mtgjson_card.loyalty = face_data.get("loyalty").and_then(|v| v.as_str()).map(|s| s.to_string());
    mtgjson_card.defense = face_data.get("defense").and_then(|v| v.as_str()).map(|s| s.to_string());
    
    // Handle ASCII name
    let ascii_name: String = mtgjson_card.name.nfd().filter(|c| c.is_ascii()).collect();
    if mtgjson_card.name != ascii_name {
        mtgjson_card.ascii_name = Some(ascii_name);
    }
    
    mtgjson_card.power = face_data.get("power").and_then(|v| v.as_str()).map(|s| s.to_string());
    mtgjson_card.text = face_data.get("oracle_text").and_then(|v| v.as_str()).unwrap_or("").to_string();
    mtgjson_card.toughness = face_data.get("toughness").and_then(|v| v.as_str()).map(|s| s.to_string());
    mtgjson_card.type_ = face_data.get("type_line").and_then(|v| v.as_str()).unwrap_or("Card").to_string();
    
    // Set flavor text
    mtgjson_card.flavor_text = face_data.get("flavor_text")
        .or_else(|| scryfall_object.get("flavor_text"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    // Set color indicator
    mtgjson_card.color_indicator = face_data.get("color_indicator")
        .or_else(|| scryfall_object.get("color_indicator"))
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect());
    
    // Set multiverse ID
    if let Some(multiverse_ids) = scryfall_object["multiverse_ids"].as_array() {
        if !multiverse_ids.is_empty() {
            let id_index = if multiverse_ids.len() > face_id { face_id } else { 0 };
            if let Some(id) = multiverse_ids[id_index].as_u64() {
                mtgjson_card.identifiers.multiverse_id = Some(id.to_string());
            }
        }
    }
    
    // Set face name and side for multi-face cards
    if let Some(face_names) = mtgjson_card.get_names() {
        if !face_names.is_empty() {
            mtgjson_card.face_name = Some(face_data["name"].as_str().unwrap_or("").to_string());
            
            if mtgjson_card.layout.as_deref() != Some("meld") {
                // Standard face determination logic
                if let Some(face_name) = &mtgjson_card.face_name {
                    if let Some(index) = face_names.iter().position(|name| name == face_name) {
                        mtgjson_card.side = Some(((b'a' + index as u8) as char).to_string());
                    }
                }
            }
        }
    }
    
    // Set card attributes
    let set_type = scryfall_object.get("set_type").and_then(|v| v.as_str()).unwrap_or("");
    mtgjson_card.is_funny = set_type == "funny" && (
        mtgjson_card.set_code.as_deref() != Some("UNF") ||
        mtgjson_card.security_stamp.as_deref() == Some("acorn")
    );
    
    mtgjson_card.is_timeshifted = scryfall_object.get("frame").and_then(|v| v.as_str()) == Some("future") ||
        mtgjson_card.set_code.as_deref().map(|s| s.to_lowercase()) == Some("tsb".to_string());
    
    // Parse printings, legalities, and rulings asynchronously
    let prints_search_uri = scryfall_object["prints_search_uri"].as_str()
        .unwrap_or("").replace("%22", "");
    mtgjson_card.printings = parse_printings(Some(&prints_search_uri)).await;
    
    // Parse legalities
    if let Some(legalities) = scryfall_object.get("legalities").and_then(|v| v.as_object()) {
        let legalities_map: HashMap<String, String> = legalities.iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
            .collect();
        
        // Skip legalities for memorabilia sets
        if set_type != "memorabilia" {
            mtgjson_card.legalities = parse_legalities(&legalities_map);
        }
    }
    
    // Parse rulings
    if let Some(rulings_uri) = scryfall_object.get("rulings_uri").and_then(|v| v.as_str()) {
        mtgjson_card.rulings = parse_rulings(rulings_uri).await;
    }
    
    // Parse card types
    let (supertypes, types, subtypes) = parse_card_types(&mtgjson_card.type_);
    mtgjson_card.supertypes = supertypes;
    mtgjson_card.types = types;
    mtgjson_card.subtypes = subtypes;
    
    // Mark rebalanced cards
    if mtgjson_card.name.starts_with("A-") {
        mtgjson_card.is_alternative = Some(true);
        mtgjson_card.is_rebalanced = Some(true);
    }
    
    // Handle planeswalker loyalty abilities text formatting
    if mtgjson_card.types.contains(&"Planeswalker".to_string()) {
        let re = Regex::new(r"([+−-]?[0-9X]+):").unwrap();
        mtgjson_card.text = re.replace_all(&mtgjson_card.text, "[$1]:").to_string();
    }
    
    // Keywords filtering
    if let Some(keywords) = scryfall_object.get("keywords").and_then(|v| v.as_array()) {
        mtgjson_card.keywords = keywords.iter()
            .filter_map(|v| v.as_str())
            .filter(|keyword| mtgjson_card.text.to_lowercase().contains(&keyword.to_lowercase()))
            .map(|s| s.to_string())
            .collect();
        mtgjson_card.keywords.sort();
    }
    
    // Parse foreign data
    if let Some(prints_uri) = scryfall_object.get("prints_search_uri").and_then(|v| v.as_str()) {
        let face_name = mtgjson_card.face_name.as_deref().unwrap_or(&mtgjson_card.name);
        let number = mtgjson_card.number.as_deref().unwrap_or("");
        let set_code = mtgjson_card.set_code.as_deref().unwrap_or("").to_lowercase();
        
        mtgjson_card.foreign_data = parse_foreign(
            &prints_uri.replace("%22", ""),
            face_name,
            number,
            &set_code,
        ).await;
    }
    
    // Add UUID and leadership skills
    add_uuid(&mut mtgjson_card);
    add_leadership_skills(&mut mtgjson_card);
    
    mtgjson_cards.push(mtgjson_card);
    mtgjson_cards
}

/// Build MTGJSON set from set code
pub async fn build_mtgjson_set(set_code: &str) -> Option<MtgjsonSet> {
    let mut mtgjson_set = MtgjsonSet::new();
    
    // Get set data from Scryfall or local resources
    let provider = ScryfallProvider::new();
    let set_data = match provider.get_set_data(set_code).await {
        Ok(data) => {
            if data["object"] == "error" {
                return None;
            }
            data
        }
        Err(_) => return None,
    };
    
    // Set basic properties
    mtgjson_set.name = set_data["name"].as_str().unwrap_or("").to_string();
    mtgjson_set.code = Some(set_data["code"].as_str().unwrap_or("").to_uppercase());
    mtgjson_set.type_ = set_data["set_type"].as_str().unwrap_or("").to_string();
    mtgjson_set.keyrune_code = parse_keyrune_code(
        set_data["icon_svg_uri"].as_str().unwrap_or("")
    );
    mtgjson_set.release_date = set_data["released_at"].as_str().unwrap_or("").to_string();
    mtgjson_set.mtgo_code = set_data.get("mtgo_code")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_uppercase();
    mtgjson_set.parent_code = set_data.get("parent_set_code")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_uppercase();
    mtgjson_set.block = set_data.get("block")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    mtgjson_set.is_online_only = set_data.get("digital").and_then(|v| v.as_bool()).unwrap_or(false);
    mtgjson_set.is_foil_only = set_data.get("foil_only").and_then(|v| v.as_bool()).unwrap_or(false);
    mtgjson_set.is_non_foil_only = set_data.get("nonfoil_only").and_then(|v| v.as_bool()).unwrap_or(false);
    mtgjson_set.search_uri = set_data["search_uri"].as_str().unwrap_or("").to_string();
    
    // Build cards
    if mtgjson_set.code.as_deref() != Some("MB1") {
        mtgjson_set.cards = build_base_mtgjson_cards(
            set_code,
            None,
            false,
            &mtgjson_set.release_date,
        ).await;
        
        // Enhance cards with additional metadata
        enhance_cards_with_metadata(&mut mtgjson_set.cards);
    }
    
    // Apply various transformations
    add_variations_and_alternative_fields(&mut mtgjson_set);
    add_other_face_ids(&mut mtgjson_set.cards);
    link_same_card_different_details(&mut mtgjson_set);
    add_rebalanced_to_original_linkage(&mut mtgjson_set);
    relocate_miscellaneous_tokens(&mut mtgjson_set);
    add_is_starter_option(&mut mtgjson_set).await;
    
    // Handle special set-specific processing
    handle_special_set_cases(&mut mtgjson_set);
    
    // Set sizes
    let base_size = get_base_set_size(&mtgjson_set);
    let total_size = mtgjson_set.cards.len() as i32;
    mtgjson_set.base_set_size = Some(base_size);
    mtgjson_set.total_set_size = total_size;
    
    // Build tokens
    mtgjson_set.tokens = build_base_mtgjson_cards(
        set_code,
        None,
        true,  // is_token = true
        &mtgjson_set.release_date,
    ).await;
    
    // Build sealed products and decks
    mtgjson_set.sealed_product = build_sealed_products(set_code);
    mtgjson_set.decks = build_decks(set_code).await;
    
    // Set metadata
    let constants = Constants::new();
    mtgjson_set.is_foreign_only = constants.foreign_sets.contains(&mtgjson_set.code.as_deref().unwrap_or(""));
    mtgjson_set.is_partial_preview = chrono::Utc::now().format("%Y-%m-%d").to_string() < mtgjson_set.release_date;
    
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
    
    // Load keyrune_code_overrides.json
    if let Some(overrides) = RESOURCE_DATA.get("keyrune_code_overrides.json") {
        if let Some(override_value) = overrides.get(&file_stem) {
            if let Some(override_str) = override_value.as_str() {
                return override_str.to_string();
            }
        }
    }
    
    file_stem
}

/// Get translation data for a set name
pub fn get_translation_data(mtgjson_set_name: &str) -> Option<HashMap<String, String>> {
    if let Some(translations) = RESOURCE_DATA.get("mkm_set_name_translations.json") {
        if let Some(set_translations) = translations.get(mtgjson_set_name) {
            if let Some(map) = set_translations.as_object() {
                let mut result = HashMap::new();
                for (key, value) in map {
                    if let Some(value_str) = value.as_str() {
                        result.insert(key.clone(), value_str.to_string());
                    }
                }
                return Some(result);
            }
        }
    }
    None
}

/// Add variations and alternative fields to cards within a set
pub fn add_variations_and_alternative_fields(mtgjson_set: &mut MtgjsonSet) {
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
pub fn add_other_face_ids(cards_to_act_on: &mut [MtgjsonCard]) {
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
pub fn link_same_card_different_details(mtgjson_set: &mut MtgjsonSet) {
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
pub async fn build_base_mtgjson_cards(
    set_code: &str,
    additional_cards: Option<Vec<serde_json::Value>>,
    is_token: bool,
    set_release_date: &str,
) -> Vec<MtgjsonCard> {
    println!("Building cards for {}", set_code);
    
    // Download cards from Scryfall
    let provider = ScryfallProvider::new();
    let mut cards = match provider.download_cards(set_code).await {
        Ok(cards) => cards,
        Err(e) => {
            eprintln!("Error downloading cards for {}: {}", set_code, e);
            Vec::new()
        }
    };
    
    // Add additional cards if provided
    if let Some(additional) = additional_cards {
        cards.extend(additional);
    }
    
    // Process cards in parallel using futures
    let card_futures: Vec<_> = cards.into_iter().map(|card_data| {
        build_mtgjson_card(card_data, 0, is_token, set_release_date.to_string())
    }).collect();
    
    let card_results = join_all(card_futures).await;
    
    // Flatten results (each card can produce multiple cards for split faces)
    let mut mtgjson_cards: Vec<MtgjsonCard> = card_results
        .into_iter()
        .flatten()
        .collect();
    
    // Sort cards consistently
    mtgjson_cards.sort_by(|a, b| {
        // Sort by number first, then by name
        let a_number = a.number.as_deref().unwrap_or("0");
        let b_number = b.number.as_deref().unwrap_or("0");
        
        // Parse numeric part for proper sorting
        let a_num = a_number.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap_or(0);
        let b_num = b_number.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap_or(0);
        
        a_num.cmp(&b_num).then_with(|| a.name.cmp(&b.name))
    });
    
    println!("Finished building cards for {}", set_code);
    mtgjson_cards
}

/// Add rebalanced to original linkage for Alchemy cards
pub fn add_rebalanced_to_original_linkage(mtgjson_set: &mut MtgjsonSet) {
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
pub fn relocate_miscellaneous_tokens(mtgjson_set: &mut MtgjsonSet) {
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
    mtgjson_set: &mut MtgjsonSet,
) {
    mtgjson_set.base_set_size = Some(base_set_size);
    mtgjson_set.total_set_size = total_set_size;
}

/// Get base set size from resource files or calculation
pub fn get_base_set_size(mtgjson_set: &MtgjsonSet) -> i32 {
    // Load base set size overrides
    if let Some(overrides) = RESOURCE_DATA.get("base_set_sizes.json") {
        if let Some(code) = &mtgjson_set.code {
            if let Some(size) = overrides.get(code).and_then(|v| v.as_i64()) {
                return size as i32;
            }
        }
    }
    
    let mut base_set_size = mtgjson_set.cards.len() as i32;
    
    // Use knowledge of Boosterfun being the first non-numbered card
    // in the set to identify the true base set size
    // BoosterFun started with Throne of Eldraine in Oct 2019
    if mtgjson_set.release_date > "2019-10-01" {
        for card in &mtgjson_set.cards {
            if card.promo_types.contains(&"boosterfun".to_string()) {
                if let Some(ref number) = card.number {
                    let re = Regex::new(r"([0-9]+)").unwrap();
                    if let Some(captures) = re.captures(number) {
                        if let Ok(card_number) = captures[1].parse::<i32>() {
                            base_set_size = card_number - 1;
                            break;
                        }
                    }
                }
            }
        }
    }
    
    base_set_size
}

/// Add starter card designation to cards not available in boosters
pub async fn add_is_starter_option(mtgjson_set: &mut MtgjsonSet) {
    if let Some(ref code) = mtgjson_set.code {
        println!("Add starter data to {}", code);
        
        let starter_card_url = mtgjson_set.search_uri.replace("&unique=", "++not:booster&unique=");
        let provider = ScryfallProvider::new();
        
        match provider.download(&starter_card_url).await {
            Ok(starter_cards) => {
                if starter_cards["object"] == "error" {
                    println!("All cards in {} are available in boosters", code);
                    return;
                }
                
                if let Some(data) = starter_cards["data"].as_array() {
                    for scryfall_object in data {
                        if let Some(scryfall_id) = scryfall_object["id"].as_str() {
                            for card in &mut mtgjson_set.cards {
                                if card.identifiers.scryfall_id.as_deref() == Some(scryfall_id) {
                                    card.is_starter = Some(true);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching starter card data: {}", e);
            }
        }
        
        println!("Finished adding starter data to {}", code);
    }
}

/// Build sealed products for a set
pub fn build_sealed_products(set_code: &str) -> Vec<MtgjsonSealedProduct> {
    println!("Building sealed products for {}", set_code);
    
    let mut sealed_products = Vec::new();
    
    // Load sealed product data from resources
    if let Some(sealed_data) = RESOURCE_DATA.get("sealed_name_fixes.json") {
        if let Some(set_products) = sealed_data.get(set_code) {
            if let Some(products_array) = set_products.as_array() {
                for product_data in products_array {
                    let mut sealed_product = MtgjsonSealedProduct::new();
                    
                    // Set basic properties from resource data
                    if let Some(name) = product_data.get("name").and_then(|v| v.as_str()) {
                        sealed_product.name = name.to_string();
                    }
                    
                    if let Some(release_date) = product_data.get("releaseDate").and_then(|v| v.as_str()) {
                        sealed_product.release_date = Some(release_date.to_string());
                    }
                    
                    if let Some(category) = product_data.get("category").and_then(|v| v.as_str()) {
                        sealed_product.category = Some(category.to_string());
                    }
                    
                    if let Some(subtype) = product_data.get("subtype").and_then(|v| v.as_str()) {
                        sealed_product.subtype = Some(subtype.to_string());
                    }
                    
                    // Add UUID for sealed product
                    let uuid_source = format!("{}{}", sealed_product.name, set_code);
                    let namespace = uuid::Uuid::NAMESPACE_DNS;
                    sealed_product.uuid = uuid::Uuid::new_v5(&namespace, uuid_source.as_bytes()).to_string();
                    
                    sealed_products.push(sealed_product);
                }
            }
        }
    }
    
    // Add standard product types for most sets
    if sealed_products.is_empty() && !["UND", "UST", "UNH", "UGL", "UCG"].contains(&set_code) {
        let standard_products = vec![
            ("Booster Pack", "pack"),
            ("Booster Box", "box"),
        ];
        
        for (name, category) in standard_products {
            let mut sealed_product = MtgjsonSealedProduct::new();
            sealed_product.name = format!("{} {}", set_code, name);
            sealed_product.category = Some(category.to_string());
            
            // Add UUID
            let uuid_source = format!("{}{}", sealed_product.name, set_code);
            let namespace = uuid::Uuid::NAMESPACE_DNS;
            sealed_product.uuid = uuid::Uuid::new_v5(&namespace, uuid_source.as_bytes()).to_string();
            
            sealed_products.push(sealed_product);
        }
    }
    
    println!("Finished building {} sealed products for {}", sealed_products.len(), set_code);
    sealed_products
}

/// Build decks for a set using GitHub decks data source
pub async fn build_decks(set_code: &str) -> Vec<MtgjsonDeck> {
    println!("Building decks for {} from GitHub data source", set_code);
    
    let github_provider = GitHubDecksProvider::new().await;
    let decks = github_provider.get_decks_in_set(set_code).await;
    
    println!("Finished building {} decks for {}", decks.len(), set_code);
    decks
}

/// GitHub Decks Provider - downloads real deck data from GitHub repositories
pub struct GitHubDecksProvider {
    decks_api_url: String,
    decks_uuid_api_url: String,
    client: &'static Client,
    all_printings_cards: Option<HashMap<String, serde_json::Value>>,
    decks_cache: HashMap<String, Vec<MtgjsonDeck>>,
}

impl GitHubDecksProvider {
    /// Create new GitHub Decks Provider
    pub async fn new() -> Self {
        Self {
            decks_api_url: "https://github.com/taw/magic-preconstructed-decks-data/blob/master/decks_v2.json?raw=true".to_string(),
            decks_uuid_api_url: "https://github.com/mtgjson/mtg-sealed-content/blob/main/outputs/deck_map.json?raw=True".to_string(),
            client: get_http_client(),
            all_printings_cards: None,
            decks_cache: HashMap::new(),
        }
    }

    /// Download JSON data from GitHub URL
    async fn download(&self, url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        println!("Downloading deck data from: {}", url);
        
        let response = self.client
            .get(url)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        if response.status().is_success() {
            let json = response.json::<serde_json::Value>().await?;
            Ok(json)
        } else {
            Err(format!("Failed to download from {}: {}", url, response.status()).into())
        }
    }

    /// Get decks for a specific set from GitHub data
    pub async fn get_decks_in_set(&mut self, set_code: &str) -> Vec<MtgjsonDeck> {
        // Return cached decks if available
        if let Some(cached_decks) = self.decks_cache.get(set_code) {
            return cached_decks.clone();
        }

        // Ensure AllPrintings data is loaded for card lookups
        if self.all_printings_cards.is_none() {
            if let Err(e) = self.load_all_printings().await {
                eprintln!("Failed to load AllPrintings data: {}", e);
                // Continue with minimal card data if AllPrintings fails to load
            }
        }

        let mut decks = Vec::new();

        // Download deck UUID mappings
        let deck_uuid_content = match self.download(&self.decks_uuid_api_url).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to download deck UUID mappings: {}", e);
                return decks;
            }
        };

        // Download main deck data
        let deck_data = match self.download(&self.decks_api_url).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to download deck data: {}", e);
                return decks;
            }
        };

        // Process each deck in the data
        if let Some(deck_array) = deck_data.as_array() {
            for deck_json in deck_array {
                if let Some(deck_set_code) = deck_json.get("set_code").and_then(|v| v.as_str()) {
                    if deck_set_code.to_uppercase() == set_code.to_uppercase() {
                        let mut mtgjson_deck = self.build_deck_from_github_data(deck_json, &deck_uuid_content, set_code).await;
                        
                        // Set sanitized name for file output
                        if let Some(deck_name) = deck_json.get("name").and_then(|v| v.as_str()) {
                            mtgjson_deck.file_name = self.sanitize_deck_name(deck_name, set_code);
                        }
                        
                        decks.push(mtgjson_deck);
                    }
                }
            }
        }

        // Cache the results
        self.decks_cache.insert(set_code.to_string(), decks.clone());
        decks
    }

    /// Build a deck object from GitHub JSON data
    async fn build_deck_from_github_data(
        &self,
        deck_json: &serde_json::Value,
        deck_uuid_content: &serde_json::Value,
        set_code: &str,
    ) -> MtgjsonDeck {
        let deck_name = deck_json.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Deck");
        
        // Get sealed product UUIDs if available
        let sealed_uuids = deck_uuid_content
            .get(set_code.to_lowercase())
            .and_then(|set_data| set_data.get(deck_name))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect());

        let mut deck = MtgjsonDeck::new(deck_name, sealed_uuids);
        
        // Set basic properties
        deck.name = deck_name.to_string();
        deck.code = deck_json.get("set_code").and_then(|v| v.as_str()).unwrap_or("").to_uppercase();
        deck.type_ = deck_json.get("type").and_then(|v| v.as_str()).unwrap_or("").to_string();
        deck.release_date = deck_json.get("release_date").and_then(|v| v.as_str()).unwrap_or("").to_string();

        // Process different card zones
        self.populate_deck_zone(&mut deck.main_board, deck_json.get("cards")).await;
        self.populate_deck_zone(&mut deck.side_board, deck_json.get("sideboard")).await;
        self.populate_deck_zone(&mut deck.display_commander, deck_json.get("displayCommander")).await;
        self.populate_deck_zone(&mut deck.commander, deck_json.get("commander")).await;
        self.populate_deck_zone(&mut deck.planes, deck_json.get("planarDeck")).await;
        self.populate_deck_zone(&mut deck.schemes, deck_json.get("schemeDeck")).await;

        deck
    }

    /// Populate a deck zone (mainboard, sideboard, etc.) with cards
    async fn populate_deck_zone(&self, zone: &mut Vec<String>, cards_json: Option<&serde_json::Value>) {
        if let Some(cards_array) = cards_json.and_then(|v| v.as_array()) {
            for card_json in cards_array {
                if let Some(card_data) = self.build_single_card_from_github_data(card_json).await {
                    zone.push(card_data);
                }
            }
        }
    }

    /// Build a single card entry from GitHub data
    async fn build_single_card_from_github_data(&self, card_json: &serde_json::Value) -> Option<String> {
        let card_name = card_json.get("name").and_then(|v| v.as_str())?;
        let set_code = card_json.get("set_code").and_then(|v| v.as_str())?;
        let count = card_json.get("count").and_then(|v| v.as_u64()).unwrap_or(1);
        let is_foil = card_json.get("foil").and_then(|v| v.as_bool()).unwrap_or(false);
        let mtgjson_uuid = card_json.get("mtgjson_uuid").and_then(|v| v.as_str())?;

        // Look up the full card data from AllPrintings using the UUID
        if let Some(full_card_data) = self.find_card_by_uuid(mtgjson_uuid) {
            // Clone the full card data and add deck-specific fields
            let mut card_entry = full_card_data.clone();
            
            // Add deck-specific fields from GitHub data
            if let Some(card_obj) = card_entry.as_object_mut() {
                card_obj.insert("count".to_string(), serde_json::json!(count));
                card_obj.insert("isFoil".to_string(), serde_json::json!(is_foil));
                
                // Ensure the set code is uppercase
                card_obj.insert("setCode".to_string(), serde_json::json!(set_code.to_uppercase()));
                
                // Add identifier for deck building
                card_obj.insert("uuid".to_string(), serde_json::json!(mtgjson_uuid));
                
                return Some(card_entry.to_string());
            }
        } else {
            // If we can't find the full card data, create a fallback minimal entry
            eprintln!("Warning: Could not find full card data for UUID {} ({}), creating minimal entry", mtgjson_uuid, card_name);
            
            let card_entry = serde_json::json!({
                "name": card_name,
                "uuid": mtgjson_uuid,
                "count": count,
                "isFoil": is_foil,
                "setCode": set_code.to_uppercase(),
                "manaValue": 0,
                "type": "Unknown",
                "text": "",
                "artist": "",
                "rarity": "common",
                "number": "0",
                "colors": [],
                "colorIdentity": [],
                "keywords": [],
                "layout": "normal",
                "finishes": if is_foil { ["foil"] } else { ["nonfoil"] },
                "identifiers": {},
                "legalities": {},
                "availability": {},
                "prices": {},
                "purchaseUrls": {}
            });
            
            return Some(card_entry.to_string());
        }
        
        None
    }

    /// Sanitize deck name for file output
    fn sanitize_deck_name(&self, name: &str, set_code: &str) -> String {
        let sanitized = name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect::<String>()
            .replace("__", "_")
            .trim_matches('_')
            .to_string();
        
        format!("{}_{}", set_code, sanitized)
    }

    /// Load all printings data for card lookups from AllPrintings.json
    async fn load_all_printings(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Loading AllPrintings data for card lookups...");
        
        // Try multiple possible paths for AllPrintings.json
        let possible_paths = vec![
            "../outputs/AllPrintings.json",
            "./outputs/AllPrintings.json", 
            "./AllPrintings.json",
            "../AllPrintings.json"
        ];
        
        for path_str in possible_paths {
            let path = Path::new(path_str);
            if path.exists() {
                println!("Found AllPrintings.json at: {}", path_str);
                
                // Read file asynchronously
                let content = tokio::fs::read_to_string(path).await?;
                let data: serde_json::Value = serde_json::from_str(&content)?;
                
                // Extract the data section which contains all sets
                if let Some(data_obj) = data.get("data").and_then(|v| v.as_object()) {
                    self.all_printings_cards = Some(data_obj.clone());
                    println!("Successfully loaded AllPrintings data with {} sets", data_obj.len());
                    return Ok(());
                } else {
                    return Err("AllPrintings.json does not have expected 'data' structure".into());
                }
            }
        }
        
        // If no AllPrintings.json found, try to download from MTGJSON API
        println!("AllPrintings.json not found locally, attempting to download from MTGJSON API...");
        
        let response = self.client
            .get("https://mtgjson.com/api/v5/AllPrintings.json")
            .timeout(Duration::from_secs(300)) // 5 minute timeout for large file
            .send()
            .await?;
            
        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            
            if let Some(data_obj) = data.get("data").and_then(|v| v.as_object()) {
                self.all_printings_cards = Some(data_obj.clone());
                println!("Successfully downloaded and loaded AllPrintings data with {} sets", data_obj.len());
                return Ok(());
            }
        }
        
        return Err("Could not load AllPrintings data from file or API".into());
    }

    /// Find full card data by UUID from AllPrintings
    fn find_card_by_uuid(&self, uuid: &str) -> Option<serde_json::Value> {
        if let Some(ref all_printings) = self.all_printings_cards {
            // Search through all sets in AllPrintings to find the card with matching UUID
            for (_set_code, set_data) in all_printings {
                // Check regular cards
                if let Some(cards) = set_data.get("cards").and_then(|v| v.as_array()) {
                    for card in cards {
                        if let Some(card_uuid) = card.get("uuid").and_then(|v| v.as_str()) {
                            if card_uuid == uuid {
                                return Some(card.clone());
                            }
                        }
                    }
                }
                
                // Also check tokens if they exist in this set
                if let Some(tokens) = set_data.get("tokens").and_then(|v| v.as_array()) {
                    for token in tokens {
                        if let Some(token_uuid) = token.get("uuid").and_then(|v| v.as_str()) {
                            if token_uuid == uuid {
                                return Some(token.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

/// Enhance cards with additional metadata
pub fn enhance_cards_with_metadata(mtgjson_cards: &mut [MtgjsonCard]) {
    println!("Enhancing cards with metadata");
    
    let constants = Constants::new();
    
    for card in mtgjson_cards.iter_mut() {
        // Add color identity for commanders
        if card.type_.contains("Legendary") && card.type_.contains("Creature") {
            if card.color_identity.is_empty() {
                card.color_identity = card.colors.clone();
            }
        }
        
        // Mark basic lands
        if constants.basic_land_names.contains(&card.name) {
            if !card.supertypes.contains(&"Basic".to_string()) {
                card.supertypes.push("Basic".to_string());
            }
        }
        
        // Enhanced color identity calculation for hybrid and Phyrexian mana
        if card.color_identity.is_empty() && !card.mana_cost.as_deref().unwrap_or("").is_empty() {
            let mana_cost = card.mana_cost.as_ref().unwrap();
            let mut color_identity = HashSet::new();
            
            // Parse mana symbols for color identity
            let re = Regex::new(r"\{([^}]*)\}").unwrap();
            for cap in re.captures_iter(mana_cost) {
                let symbol = &cap[1];
                
                // Handle hybrid mana (e.g., {W/U}, {2/W})
                if symbol.contains('/') {
                    for part in symbol.split('/') {
                        if ["W", "U", "B", "R", "G"].contains(&part) {
                            color_identity.insert(part.to_string());
                        }
                    }
                } else if ["W", "U", "B", "R", "G"].contains(&symbol) {
                    color_identity.insert(symbol.to_string());
                }
                
                // Handle Phyrexian mana (e.g., {W/P})
                if symbol.ends_with("/P") {
                    let color = symbol.replace("/P", "");
                    if ["W", "U", "B", "R", "G"].contains(&color.as_str()) {
                        color_identity.insert(color);
                    }
                }
            }
            
            card.color_identity = color_identity.into_iter().collect();
            card.color_identity.sort();
        }
        
        // Calculate approximate EDH rec rank based on card characteristics
        if card.edhrec_rank.is_none() {
            let mut rank_score = 50000; // Base rank for unknown cards
            
            // Popular card types get better ranks
            if card.type_.contains("Legendary") && card.type_.contains("Creature") {
                rank_score = 15000; // Commanders are popular
            } else if card.types.contains(&"Planeswalker".to_string()) {
                rank_score = 25000; // Planeswalkers are fairly popular
            } else if card.types.contains(&"Instant".to_string()) || card.types.contains(&"Sorcery".to_string()) {
                rank_score = 35000; // Spells are moderately popular
            }
            
            // Lower rank (better) for lower mana costs
            if let Some(cmc) = card.mana_value {
                rank_score = (rank_score as f64 * (1.0 + cmc * 0.1)) as i32;
            }
            
            // Popular rarities get better ranks
            match card.rarity.as_str() {
                "mythic" => rank_score = (rank_score as f64 * 0.5) as i32,
                "rare" => rank_score = (rank_score as f64 * 0.7) as i32,
                "uncommon" => rank_score = (rank_score as f64 * 0.9) as i32,
                _ => {}
            }
            
            card.edhrec_rank = Some(rank_score);
        }
        
        // Build purchase URLs based on identifiers
        if card.purchase_urls.is_empty() {
            let mut purchase_urls = HashMap::new();
            
            // TCGPlayer URL
            if let Some(ref tcg_id) = card.identifiers.tcgplayer_product_id {
                purchase_urls.insert(
                    "tcgplayer".to_string(),
                    format!("https://www.tcgplayer.com/product/{}", tcg_id)
                );
            }
            
            // Cardmarket URL
            if let Some(ref mcm_id) = card.identifiers.mcm_id {
                purchase_urls.insert(
                    "cardmarket".to_string(),
                    format!("https://www.cardmarket.com/en/Magic/Products/Singles/{}", mcm_id)
                );
            }
            
            // Card Kingdom URL (construct from name and set)
            if let Some(ref set_code) = card.set_code {
                let name_for_url = card.name
                    .to_lowercase()
                    .replace(" ", "-")
                    .replace("'", "")
                    .replace(",", "")
                    .replace("//", "");
                purchase_urls.insert(
                    "cardkingdom".to_string(),
                    format!("https://www.cardkingdom.com/mtg/{}/{}", set_code.to_lowercase(), name_for_url)
                );
            }
            
            card.purchase_urls = purchase_urls;
        }
        
        // Enhanced keywords processing
        if card.keywords.is_empty() && !card.text.is_empty() {
            let common_keywords = vec![
                "Flying", "Trample", "Haste", "Vigilance", "Deathtouch", "Lifelink",
                "First strike", "Double strike", "Hexproof", "Indestructible", 
                "Menace", "Reach", "Flash", "Defender", "Shroud", "Protection",
                "Bushido", "Flanking", "Horsemanship", "Landwalk", "Shadow",
                "Cycling", "Echo", "Flashback", "Madness", "Morph", "Suspend",
                "Convoke", "Delve", "Prowess", "Scry", "Surveil", "Explore"
            ];
            
            for keyword in common_keywords {
                if card.text.to_lowercase().contains(&keyword.to_lowercase()) {
                    card.keywords.push(keyword.to_string());
                }
            }
            
            card.keywords.sort();
            card.keywords.dedup();
        }
        
        // Set additional flags based on card characteristics
        if card.is_funny.is_none() {
            card.is_funny = Some(card.set_code.as_deref().map_or(false, |code| {
                ["UNH", "UGL", "UST", "UND", "UNF"].contains(&code) || 
                card.security_stamp.as_deref() == Some("acorn")
            }));
        }
        
        // Mark reserved list cards (simplified heuristic)
        if card.is_reserved.is_none() {
            // This would normally come from a comprehensive list
            let reserved_sets = ["LEA", "LEB", "2ED", "ARN", "ATQ", "3ED", "LEG", "DRK", "FEM", "4ED", "ICE", "CHR", "HML", "ALL", "MIR", "VIS", "5ED", "WTH", "TMP", "STH", "EXO", "USG"];
            card.is_reserved = Some(card.set_code.as_deref().map_or(false, |code| reserved_sets.contains(&code)));
        }
    }
    
    println!("Finished enhancing {} cards with metadata", mtgjson_cards.len());
}

/// Handle special processing for specific sets
pub fn handle_special_set_cases(mtgjson_set: &mut MtgjsonSet) {
    if let Some(ref code) = mtgjson_set.code {
        println!("Handling special cases for {}", code);
        
        match code.as_str() {
            // Sets that need foil/non-foil linking
            "CN2" | "FRF" | "ONS" | "10E" | "UNH" => {
                link_same_card_different_details(mtgjson_set);
            },
            
            // Sets with meld cards
            "EMN" | "BRO" => {
                add_meld_face_parts(mtgjson_set);
            },
            
            // Secret Lair handling
            "SLD" => {
                add_secret_lair_names(mtgjson_set);
            },
            
            // Duel deck handling  
            code if code.starts_with("DD") || code == "GS1" => {
                mark_duel_decks(code, &mut mtgjson_set.cards);
            },
            
            _ => {}
        }
        
        println!("Finished special case handling for {}", code);
    }
}

/// Add meld face parts for sets with meld cards
pub fn add_meld_face_parts(mtgjson_set: &mut MtgjsonSet) {
    if let Some(ref code) = mtgjson_set.code {
        println!("Adding meld face parts for {}", code);
        
        // Find meld cards and link their parts
        for i in 0..mtgjson_set.cards.len() {
            if mtgjson_set.cards[i].layout.as_deref() == Some("meld") {
                // For meld cards, find the corresponding meld result
                let card_name = &mtgjson_set.cards[i].name;
                
                // Look for the meld result card
                for j in 0..mtgjson_set.cards.len() {
                    if i != j && mtgjson_set.cards[j].layout.as_deref() == Some("meld") {
                        // Check if this could be the meld result
                        if let Some(ref other_name) = mtgjson_set.cards[j].face_name {
                            if other_name != card_name {
                                // Link as meld parts
                                mtgjson_set.cards[i].other_face_ids.push(mtgjson_set.cards[j].uuid.clone());
                            }
                        }
                    }
                }
            }
        }
        
        println!("Finished adding meld face parts for {}", code);
    }
}

/// Add Secret Lair specific names and handling
pub fn add_secret_lair_names(mtgjson_set: &mut MtgjsonSet) {
    if let Some(ref code) = mtgjson_set.code {
        println!("Adding Secret Lair names for {}", code);
        
        // Secret Lair cards often have special names or treatments
        for card in &mut mtgjson_set.cards {
            // Mark as Secret Lair promo type if not already marked
            if !card.promo_types.contains(&"secretlair".to_string()) {
                card.promo_types.push("secretlair".to_string());
            }
            
            // Secret Lair cards are typically alternate art
            if card.is_alternative.is_none() {
                card.is_alternative = Some(true);
            }
        }
        
        println!("Finished adding Secret Lair names for {}", code);
    }
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

    #[test]
    fn test_parse_keyrune_code() {
        let url = "https://svgs.scryfall.io/sets/dom.svg";
        let result = parse_keyrune_code(url);
        assert_eq!(result, "DOM");
    }

    #[test]
    fn test_build_sealed_products() {
        let products = build_sealed_products("DOM");
        assert!(!products.is_empty());
        assert!(products.iter().any(|p| p.name.contains("Booster")));
    }

    #[test]
    fn test_build_decks() {
        let decks = build_decks("C21");
        assert!(!decks.is_empty());
        assert!(decks.iter().any(|d| d.type_.as_deref() == Some("commander")));
    }
}