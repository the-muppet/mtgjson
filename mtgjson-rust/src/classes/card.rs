use crate::base::{skip_if_empty_optional_string, skip_if_empty_string, skip_if_empty_vec, JsonObject};
use crate::foreign_data::MtgjsonForeignDataObject;
use crate::game_formats::MtgjsonGameFormatsObject;
use crate::identifiers::MtgjsonIdentifiers;
use crate::leadership_skills::MtgjsonLeadershipSkillsObject;
use crate::legalities::MtgjsonLegalitiesObject;
use crate::prices::MtgjsonPricesObject;
use crate::purchase_urls::MtgjsonPurchaseUrls;
use crate::related_cards::MtgjsonRelatedCardsObject;
use crate::rulings::MtgjsonRulingObject;
use crate::utils::MtgjsonUtils;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

/// MTGJSON Singular Card Object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(name = "MtgjsonCardObject")]
pub struct MtgjsonCardObject {
    #[pyo3(get, set)]
    pub artist: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub artist_ids: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub ascii_name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub attraction_lights: Option<Vec<String>>,
    
    #[pyo3(get, set)]
    pub availability: MtgjsonGameFormatsObject,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub booster_types: Vec<String>,
    
    #[pyo3(get, set)]
    pub border_color: String,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub card_parts: Vec<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub color_identity: Vec<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub color_indicator: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub colors: Vec<String>,
    
    #[pyo3(get, set)]
    pub converted_mana_cost: f64,
    
    #[pyo3(get, set)]
    pub count: i32,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub defense: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub duel_deck: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub edhrec_rank: Option<i32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub edhrec_saltiness: Option<f64>,
    
    #[pyo3(get, set)]
    pub face_converted_mana_cost: f64,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub face_flavor_name: Option<String>,
    
    #[pyo3(get, set)]
    pub face_mana_value: f64,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub face_name: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub finishes: Vec<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub first_printing: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub flavor_name: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub flavor_text: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub foreign_data: Vec<MtgjsonForeignDataObject>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub frame_effects: Vec<String>,
    
    #[pyo3(get, set)]
    pub frame_version: String,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub hand: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub has_alternative_deck_limit: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub has_content_warning: Option<bool>,
    
    /// Deprecated - Remove in 5.3.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub has_foil: Option<bool>,
    
    /// Deprecated - Remove in 5.3.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub has_non_foil: Option<bool>,
    
    #[pyo3(get, set)]
    pub identifiers: MtgjsonIdentifiers,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_alternative: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_foil: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_full_art: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_funny: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_game_changer: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_online_only: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_oversized: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_promo: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_rebalanced: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_reprint: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_reserved: Option<bool>,
    
    /// Deprecated - Remove in 5.3.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_starter: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_story_spotlight: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_textless: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub is_timeshifted: Option<bool>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub keywords: Vec<String>,
    
    #[pyo3(get, set)]
    pub language: String,
    
    #[pyo3(get, set)]
    pub layout: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub leadership_skills: Option<MtgjsonLeadershipSkillsObject>,
    
    #[pyo3(get, set)]
    pub legalities: MtgjsonLegalitiesObject,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub life: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub loyalty: Option<String>,
    
    #[pyo3(get, set)]
    pub mana_cost: String,
    
    #[pyo3(get, set)]
    pub mana_value: f64,
    
    #[pyo3(get, set)]
    pub name: String,
    
    #[pyo3(get, set)]
    pub number: String,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub orientation: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub original_printings: Vec<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub original_release_date: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub original_text: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub original_type: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub other_face_ids: Vec<String>,
    
    #[pyo3(get, set)]
    pub power: String,
    
    #[pyo3(get, set)]
    pub prices: MtgjsonPricesObject,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub printings: Vec<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub promo_types: Vec<String>,
    
    #[pyo3(get, set)]
    pub purchase_urls: MtgjsonPurchaseUrls,
    
    #[pyo3(get, set)]
    pub rarity: String,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub rebalanced_printings: Vec<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub related_cards: Option<MtgjsonRelatedCardsObject>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub reverse_related: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub rulings: Option<Vec<MtgjsonRulingObject>>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub security_stamp: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub side: Option<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub signature: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub source_products: Option<HashMap<String, Vec<String>>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub subsets: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub subtypes: Vec<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub supertypes: Vec<String>,
    
    #[pyo3(get, set)]
    pub text: String,
    
    #[pyo3(get, set)]
    pub toughness: String,
    
    #[pyo3(get, set)]
    #[pyo3(name = "type")]
    pub type_: String,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub types: Vec<String>,
    
    #[pyo3(get, set)]
    pub uuid: String,
    
    #[serde(skip_serializing_if = "skip_if_empty_vec")]
    #[pyo3(get, set)]
    pub variations: Vec<String>,
    
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get)]
    pub watermark: Option<String>,

    // Outside entities, not published
    #[serde(skip)]
    #[pyo3(get, set)]
    pub set_code: String,
    
    #[serde(skip)]
    #[pyo3(get, set)]
    pub is_token: bool,
    
    #[serde(skip)]
    #[pyo3(get, set)]
    pub raw_purchase_urls: HashMap<String, String>,
    
    // Internal fields
    #[serde(skip)]
    names: Option<Vec<String>>,
    
    #[serde(skip)]
    illustration_ids: Vec<String>,
    
    #[serde(skip)]
    watermark_resource: HashMap<String, Vec<serde_json::Value>>,
}

#[pymethods]
impl MtgjsonCardObject {
    #[new]
    #[pyo3(signature = (is_token = false))]
    pub fn new(is_token: bool) -> Self {
        Self {
            artist: String::new(),
            artist_ids: None,
            ascii_name: None,
            attraction_lights: None,
            availability: MtgjsonGameFormatsObject::new(),
            booster_types: Vec::new(),
            border_color: String::new(),
            card_parts: Vec::new(),
            color_identity: Vec::new(),
            color_indicator: None,
            colors: Vec::new(),
            converted_mana_cost: 0.0,
            count: 1,
            defense: None,
            duel_deck: None,
            edhrec_rank: None,
            edhrec_saltiness: None,
            face_converted_mana_cost: 0.0,
            face_flavor_name: None,
            face_mana_value: 0.0,
            face_name: None,
            finishes: Vec::new(),
            first_printing: None,
            flavor_name: None,
            flavor_text: None,
            foreign_data: Vec::new(),
            frame_effects: Vec::new(),
            frame_version: String::new(),
            hand: None,
            has_alternative_deck_limit: None,
            has_content_warning: None,
            has_foil: None,
            has_non_foil: None,
            identifiers: MtgjsonIdentifiers::new(),
            is_alternative: None,
            is_foil: None,
            is_full_art: None,
            is_funny: None,
            is_game_changer: None,
            is_online_only: None,
            is_oversized: None,
            is_promo: None,
            is_rebalanced: None,
            is_reprint: None,
            is_reserved: None,
            is_starter: None,
            is_story_spotlight: None,
            is_textless: None,
            is_timeshifted: None,
            keywords: Vec::new(),
            language: String::new(),
            layout: String::new(),
            leadership_skills: None,
            legalities: MtgjsonLegalitiesObject::new(),
            life: None,
            loyalty: None,
            mana_cost: String::new(),
            mana_value: 0.0,
            name: String::new(),
            number: String::new(),
            orientation: None,
            original_printings: Vec::new(),
            original_release_date: None,
            original_text: None,
            original_type: None,
            other_face_ids: Vec::new(),
            power: String::new(),
            prices: MtgjsonPricesObject::new(
                String::new(),
                String::new(), 
                String::new(),
                "USD".to_string(),
                None, None, None, None, None, None
            ),
            printings: Vec::new(),
            promo_types: Vec::new(),
            purchase_urls: MtgjsonPurchaseUrls::new(),
            rarity: String::new(),
            rebalanced_printings: Vec::new(),
            related_cards: None,
            reverse_related: None,
            rulings: None,
            security_stamp: None,
            side: None,
            signature: None,
            source_products: None,
            subsets: None,
            subtypes: Vec::new(),
            supertypes: Vec::new(),
            text: String::new(),
            toughness: String::new(),
            type_: String::new(),
            types: Vec::new(),
            uuid: String::new(),
            variations: Vec::new(),
            watermark: None,
            set_code: String::new(),
            is_token,
            raw_purchase_urls: HashMap::new(),
            names: None,
            illustration_ids: Vec::new(),
            watermark_resource: HashMap::new(),
        }
    }

    /// Convert to JSON Dict (Python-compatible)
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Serialization error: {}", e))
        })
    }

    /// Set internal illustration IDs for this card
    pub fn set_illustration_ids(&mut self, illustration_ids: Vec<String>) {
        self.illustration_ids = illustration_ids;
    }

    /// Get the internal illustration IDs roster for this card
    pub fn get_illustration_ids(&self) -> Vec<String> {
        self.illustration_ids.clone()
    }

    /// Get internal names array for this card
    pub fn get_names(&self) -> Vec<String> {
        self.names.clone().unwrap_or_default()
    }

    /// Set internal names array for this card
    #[pyo3(signature = (names=None))]
    pub fn set_names(&mut self, names: Option<Vec<String>>) {
        self.names = names.map(|n| n.into_iter().map(|s| s.trim().to_string()).collect());
    }

    /// Append to internal names array for this card
    pub fn append_names(&mut self, name: String) {
        if let Some(ref mut names) = self.names {
            names.push(name);
        } else {
            self.set_names(Some(vec![name]));
        }
    }

    /// Set watermark for the card with resource loading
    pub fn set_watermark(&mut self, watermark: Option<String>) {
        if let Some(watermark_value) = watermark {
            if watermark_value == "set" {
                // Load watermark resource if needed
                if self.watermark_resource.is_empty() {
                    self.load_watermark_resource();
                }
                
                // Look up the actual watermark for this card in this set
                if let Some(cards) = self.watermark_resource.get(&self.set_code.to_uppercase()) {
                    for card_entry in cards {
                        if let Some(name) = card_entry.get("name").and_then(|v| v.as_str()) {
                            // Check if this card name matches (handle double-faced cards with " // ")
                            let card_names: Vec<&str> = name.split(" // ").collect();
                            if card_names.contains(&self.name.as_str()) {
                                if let Some(actual_watermark) = card_entry.get("watermark").and_then(|v| v.as_str()) {
                                    self.watermark = Some(actual_watermark.to_string());
                                    return;
                                }
                            }
                        }
                    }
                }
                // If no override found, use the original watermark
                self.watermark = Some(watermark_value);
            } else {
                self.watermark = Some(watermark_value);
            }
        } else {
            self.watermark = None;
        }
    }
    
    /// Load watermark resource from JSON file
    fn load_watermark_resource(&mut self) {
        let resource_path = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("mtgjson5")
            .join("resources")
            .join("set_code_watermarks.json");
        
        match std::fs::read_to_string(&resource_path) {
            Ok(content) => {
                self.watermark_resource = serde_json::from_str(&content).unwrap_or_else(|e| {
                    eprintln!("Warning: Failed to parse set_code_watermarks.json: {}", e);
                    HashMap::new()
                });
            }
            Err(e) => {
                eprintln!("Warning: Failed to read set_code_watermarks.json: {}", e);
                self.watermark_resource = HashMap::new();
            }
        }
    }

    /// Get attributes of a card that don't change from printing to printing
    pub fn get_atomic_keys(&self) -> Vec<String> {
        vec![
            "ascii_name".to_string(),
            "color_identity".to_string(),
            "color_indicator".to_string(),
            "colors".to_string(),
            "converted_mana_cost".to_string(),
            "count".to_string(),
            "defense".to_string(),
            "edhrec_rank".to_string(),
            "edhrec_saltiness".to_string(),
            "face_converted_mana_cost".to_string(),
            "face_mana_value".to_string(),
            "face_name".to_string(),
            "foreign_data".to_string(),
            "hand".to_string(),
            "has_alternative_deck_limit".to_string(),
            "identifiers".to_string(),
            "is_funny".to_string(),
            "is_reserved".to_string(),
            "keywords".to_string(),
            "layout".to_string(),
            "leadership_skills".to_string(),
            "legalities".to_string(),
            "life".to_string(),
            "loyalty".to_string(),
            "mana_cost".to_string(),
            "mana_value".to_string(),
            "name".to_string(),
            "power".to_string(),
            "printings".to_string(),
            "purchase_urls".to_string(),
            "rulings".to_string(),
            "scryfall_oracle_id".to_string(),
            "side".to_string(),
            "subtypes".to_string(),
            "supertypes".to_string(),
            "text".to_string(),
            "toughness".to_string(),
            "type".to_string(),
            "types".to_string(),
        ]
    }

    /// Python equality method
    pub fn __eq__(&self, other: &MtgjsonCardObject) -> bool {
        self.number == other.number && 
        (self.side.as_deref().unwrap_or("") == other.side.as_deref().unwrap_or(""))
    }

    /// Python less-than comparison for sorting
    /// Uses embedded Python logic to ensure 100% compatibility
    pub fn __lt__(&self, other: &MtgjsonCardObject) -> PyResult<bool> {
        Python::with_gil(|py| {
            // Embed the exact Python sorting logic
            let python_code = r#"
def card_lt(self_number, self_side, other_number, other_side):
    if self_number == other_number:
        return (self_side or "") < (other_side or "")

    self_side = self_side or ""
    other_side = other_side or ""

    self_number_clean = "".join(x for x in self_number if x.isdigit()) or "100000"
    self_number_clean_int = int(self_number_clean)

    other_number_clean = "".join(x for x in other_number if x.isdigit()) or "100000"
    other_number_clean_int = int(other_number_clean)

    # Check if both numbers are pure digits
    self_is_digit = self_number == self_number_clean
    other_is_digit = other_number == other_number_clean

    if self_is_digit and other_is_digit:
        if self_number_clean_int == other_number_clean_int:
            if len(self_number_clean) != len(other_number_clean):
                return len(self_number_clean) < len(other_number_clean)
            return self_side < other_side
        return self_number_clean_int < other_number_clean_int

    if self_is_digit:
        if self_number_clean_int == other_number_clean_int:
            return True
        return self_number_clean_int < other_number_clean_int

    if other_is_digit:
        if self_number_clean_int == other_number_clean_int:
            return False
        return self_number_clean_int < other_number_clean_int

    # Case 4: Neither is pure digit
    # First check if digit strings are identical
    if self_number_clean == other_number_clean:
        if not self_side and not other_side:
            return self_number < other_number
        return self_side < other_side

    # Then check if integer values are the same but digit strings differ
    if self_number_clean_int == other_number_clean_int:
        if len(self_number_clean) != len(other_number_clean):
            return len(self_number_clean) < len(other_number_clean)
        return self_side < other_side

    return self_number_clean_int < other_number_clean_int

# Call the function
result = card_lt(self_number, self_side, other_number, other_side)
"#;

                         let locals = PyDict::new_bound(py);
             locals.set_item("self_number", &self.number)?;
             locals.set_item("self_side", &self.side)?;
             locals.set_item("other_number", &other.number)?;
             locals.set_item("other_side", &other.side)?;

             py.run_bound(python_code, None, Some(&locals))?;
            let result: bool = locals.get_item("result")?.unwrap().extract()?;
            Ok(result)
        })
    }

    /// Python string representation
    pub fn __str__(&self) -> String {
        format!("{} ({}) #{}", self.name, self.set_code, self.number)
    }

    /// Python repr representation
    pub fn __repr__(&self) -> String {
        format!("MtgjsonCardObject(name='{}', set_code='{}', uuid='{}')", 
                self.name, self.set_code, self.uuid)
    }

    /// Python hash method
    pub fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.uuid.hash(&mut hasher);
        hasher.finish()
    }

    /// Legacy method for backwards compatibility - use __eq__ instead
    #[deprecated(note = "Use __eq__ instead")]
    pub fn eq(&self, other: &MtgjsonCardObject) -> bool {
        self.__eq__(other)
    }

    /// Legacy method for backwards compatibility - use __lt__ instead
    #[deprecated(note = "Use __lt__ instead")]
    pub fn compare(&self, other: &MtgjsonCardObject) -> PyResult<i32> {
        match self.partial_cmp(other) {
            Some(std::cmp::Ordering::Less) => Ok(-1),
            Some(std::cmp::Ordering::Equal) => Ok(0),
            Some(std::cmp::Ordering::Greater) => Ok(1),
            None => Ok(0),
        }
    }
}

impl Default for MtgjsonCardObject {
    fn default() -> Self {
        Self::new(false)
    }
}

impl PartialEq for MtgjsonCardObject {
    fn eq(&self, other: &Self) -> bool {
        self.__eq__(other)
    }
}

impl PartialOrd for MtgjsonCardObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MtgjsonCardObject {
    fn cmp(&self, other: &Self) -> Ordering {
        // Get side values, defaulting to empty string if None
        let self_side = self.side.as_deref().unwrap_or("");
        let other_side = other.side.as_deref().unwrap_or("");

        // If card numbers are equal, compare by side letter
        if self.number == other.number {
            return self_side.cmp(other_side);
        }

        // Extract numeric parts from card numbers, defaulting to "100000" if no digits
        let self_number_clean: String = self.number.chars().filter(|c| c.is_ascii_digit()).collect();
        let self_number_clean = if self_number_clean.is_empty() { "100000".to_string() } else { self_number_clean };
        let self_number_clean_int: i32 = self_number_clean.parse().unwrap_or(100000);

        let other_number_clean: String = other.number.chars().filter(|c| c.is_ascii_digit()).collect();
        let other_number_clean = if other_number_clean.is_empty() { "100000".to_string() } else { other_number_clean };
        let other_number_clean_int: i32 = other_number_clean.parse().unwrap_or(100000);

        // Case 1: Both numbers are purely numeric
        if self.number == self_number_clean && other.number == other_number_clean {
            if self_number_clean_int == other_number_clean_int {
                // If numeric values are equal, compare by string length first, then by side
                if self_number_clean.len() != other_number_clean.len() {
                    return self_number_clean.len().cmp(&other_number_clean.len());
                }
                return self_side.cmp(other_side);
            }
            return self_number_clean_int.cmp(&other_number_clean_int);
        }

        // Case 2: Self number is purely numeric, other is not
        if self.number == self_number_clean {
            if self_number_clean_int == other_number_clean_int {
                return Ordering::Less; // Numeric comes before non-numeric
            }
            return self_number_clean_int.cmp(&other_number_clean_int);
        }

        // Case 3: Other number is purely numeric, self is not
        if other.number == other_number_clean {
            if self_number_clean_int == other_number_clean_int {
                return Ordering::Greater; // Non-numeric comes after numeric
            }
            return self_number_clean_int.cmp(&other_number_clean_int);
        }

        // Case 4: Both numbers have non-numeric characters
        if self_number_clean == other_number_clean {
            // If no sides exist, fall back to string comparison of full numbers
            if self_side.is_empty() && other_side.is_empty() {
                return self.number.cmp(&other.number);
            }
            return self_side.cmp(other_side);
        }

        // Case 5: Different numeric parts
        if self_number_clean_int == other_number_clean_int {
            // Same numeric value, compare by string length first, then by side
            if self_number_clean.len() != other_number_clean.len() {
                return self_number_clean.len().cmp(&other_number_clean.len());
            }
            return self_side.cmp(other_side);
        }

        // Final fallback: compare by numeric values
        self_number_clean_int.cmp(&other_number_clean_int)
    }
}

impl JsonObject for MtgjsonCardObject {
    fn build_keys_to_skip(&self) -> HashSet<String> {
        let mut excluded_keys = HashSet::new();

        if self.is_token {
            excluded_keys.extend([
                "rulings".to_string(),
                "rarity".to_string(),
                "prices".to_string(),
                "purchase_urls".to_string(),
                "printings".to_string(),
                "converted_mana_cost".to_string(),
                "mana_value".to_string(),
                "foreign_data".to_string(),
                "legalities".to_string(),
                "leadership_skills".to_string(),
            ]);
        } else {
            excluded_keys.insert("reverse_related".to_string());
        }

        excluded_keys.extend([
            "is_token".to_string(),
            "raw_purchase_urls".to_string(),
            "set_code".to_string(),
        ]);

        // Allow certain falsey values
        let allow_if_falsey = [
            "supertypes", "types", "subtypes", "has_foil", "has_non_foil",
            "color_identity", "colors", "converted_mana_cost", "mana_value",
            "face_converted_mana_cost", "face_mana_value", "foreign_data", "reverse_related"
        ];

        // Skip empty values that aren't in the allow list
        if self.artist.is_empty() && !allow_if_falsey.contains(&"artist") {
            excluded_keys.insert("artist".to_string());
        }
        
        // Continue this pattern for other fields as needed...

        excluded_keys
    }
}