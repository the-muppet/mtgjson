use crate::base::{skip_if_empty_optional_string, JsonObject};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// MTGJSON Singular Card.Identifiers Object
/// 
/// This struct represents the identifiers section of a MTGJSON card object,
/// containing various external identifiers used by different card databases,
/// marketplaces, and platforms to uniquely identify Magic: The Gathering cards.
/// 
/// All fields are optional as not every card will have identifiers for every
/// service. Empty strings are treated as None values for serialization.
/// 
/// # Examples
/// 
/// ```rust
/// use mtgjson_rust::MtgjsonIdentifiers;
/// 
/// let mut identifiers = MtgjsonIdentifiers::new();
/// identifiers.scryfall_id = Some("12345678-1234-1234-1234-123456789012".to_string());
/// identifiers.multiverse_id = Some("12345".to_string());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[pyclass(name = "MtgjsonIdentifiers")]
pub struct MtgjsonIdentifiers {
    /// Card Kingdom's etched product identifier
    /// Used for tracking etched versions of cards on Card Kingdom
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub card_kingdom_etched_id: Option<String>,
    
    /// Card Kingdom's foil product identifier  
    /// Used for tracking foil versions of cards on Card Kingdom
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub card_kingdom_foil_id: Option<String>,
    
    /// Card Kingdom's standard product identifier
    /// Used for tracking cards on Card Kingdom marketplace
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub card_kingdom_id: Option<String>,
    
    /// Cardsphere's foil product identifier
    /// Used for tracking foil versions of cards on Cardsphere
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub cardsphere_foil_id: Option<String>,
    
    /// Cardsphere's standard product identifier
    /// Used for tracking cards on Cardsphere marketplace
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub cardsphere_id: Option<String>,
    
    /// Magic Card Market (MKM/Cardmarket) product identifier
    /// Used for tracking cards on the European Cardmarket platform
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub mcm_id: Option<String>,
    
    /// Magic Card Market (MKM/Cardmarket) metadata identifier
    /// Used for additional card metadata on Cardmarket
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub mcm_meta_id: Option<String>,
    
    /// MTG Arena identifier
    /// Used for tracking cards in the MTG Arena digital platform
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub mtg_arena_id: Option<String>,
    
    /// MTGJSON foil version identifier
    /// Internal MTGJSON identifier linking to the foil version of this card
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub mtgjson_foil_version_id: Option<String>,
    
    /// MTGJSON non-foil version identifier
    /// Internal MTGJSON identifier linking to the non-foil version of this card
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub mtgjson_non_foil_version_id: Option<String>,
    
    /// MTGJSON v4 identifier
    /// Legacy identifier from MTGJSON version 4 for backwards compatibility
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub mtgjson_v4_id: Option<String>,
    
    /// MTGO foil identifier
    /// Used for tracking foil cards in Magic: The Gathering Online
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub mtgo_foil_id: Option<String>,
    
    /// MTGO standard identifier
    /// Used for tracking cards in Magic: The Gathering Online
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub mtgo_id: Option<String>,
    
    /// Wizards of the Coast Multiverse identifier
    /// Official Wizards identifier used in Gatherer and other WotC systems
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub multiverse_id: Option<String>,
    
    /// Scryfall card identifier
    /// Unique identifier for this specific card printing on Scryfall
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub scryfall_id: Option<String>,
    
    /// Scryfall illustration identifier
    /// Identifier for the artwork/illustration used on this card
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub scryfall_illustration_id: Option<String>,
    
    /// Scryfall card back identifier
    /// Identifier for the card back design used on this card
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub scryfall_card_back_id: Option<String>,
    
    /// Scryfall oracle identifier
    /// Identifier linking all printings of the same card across different sets
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub scryfall_oracle_id: Option<String>,
    
    /// TCGPlayer etched product identifier
    /// Used for tracking etched versions of cards on TCGPlayer
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub tcgplayer_etched_product_id: Option<String>,
    
    /// TCGPlayer standard product identifier
    /// Used for tracking cards on TCGPlayer marketplace
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub tcgplayer_product_id: Option<String>,
}

impl Default for MtgjsonIdentifiers {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl MtgjsonIdentifiers {
    /// Create a new MtgjsonIdentifiers instance with default values
    /// 
    /// Initializes commonly used identifiers (multiverse_id, card_kingdom_id, tcgplayer_product_id)
    /// as empty strings to match Python behavior, while other identifiers remain None.
    /// 
    /// # Returns
    /// 
    /// A new MtgjsonIdentifiers instance with default values
    /// 
    /// # Examples
    /// 
    /// ```python
    /// identifiers = MtgjsonIdentifiers()
    /// assert identifiers.multiverse_id == ""
    /// assert identifiers.card_kingdom_id == ""
    /// assert identifiers.tcgplayer_product_id == ""
    /// ```
    #[new]
    pub fn new() -> Self {
        Self {
            card_kingdom_etched_id: None,
            card_kingdom_foil_id: None,
            card_kingdom_id: Some(String::new()),
            cardsphere_foil_id: None,
            cardsphere_id: None,
            mcm_id: None,
            mcm_meta_id: None,
            mtg_arena_id: None,
            mtgjson_foil_version_id: None,
            mtgjson_non_foil_version_id: None,
            mtgjson_v4_id: None,
            mtgo_foil_id: None,
            mtgo_id: None,
            multiverse_id: Some(String::new()),
            scryfall_id: None,
            scryfall_illustration_id: None,
            scryfall_card_back_id: None,
            scryfall_oracle_id: None,
            tcgplayer_etched_product_id: None,
            tcgplayer_product_id: Some(String::new()),
        }
    }

    /// Convert the identifiers to a JSON string representation
    /// 
    /// Serializes the identifiers object to JSON, automatically excluding
    /// any empty or None values from the output.
    /// 
    /// # Returns
    /// 
    /// A JSON string representation of the identifiers
    /// 
    /// # Errors
    /// 
    /// Returns a PyValueError if serialization fails
    /// 
    /// # Examples
    /// 
    /// ```python
    /// identifiers = MtgjsonIdentifiers()
    /// identifiers.scryfall_id = "12345678-1234-1234-1234-123456789012"
    /// json_str = identifiers.to_json()
    /// ```
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Serialization error: {}", e))
        })
    }

    /// Convert identifiers to a filtered dictionary for Python compatibility
    /// 
    /// Returns a HashMap containing only the identifiers that have non-empty values,
    /// with keys converted to camelCase to match MTGJSON formatting standards.
    /// This matches the behavior of the Python version's to_json() method.
    /// 
    /// # Returns
    /// 
    /// A HashMap with string keys in camelCase format and string values,
    /// containing only non-empty identifiers
    /// 
    /// # Examples
    /// 
    /// ```python
    /// identifiers = MtgjsonIdentifiers()
    /// identifiers.scryfall_id = "12345678-1234-1234-1234-123456789012"
    /// identifiers.multiverse_id = "12345"
    /// 
    /// dict_result = identifiers.to_dict()
    /// # Returns: {"scryfallId": "12345678-1234-1234-1234-123456789012", "multiverseId": "12345"}
    /// ```
    pub fn to_dict(&self) -> PyResult<HashMap<String, String>> {
        let mut result = HashMap::new();
        
        if let Some(ref val) = self.card_kingdom_etched_id {
            if !val.is_empty() {
                result.insert("cardKingdomEtchedId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.card_kingdom_foil_id {
            if !val.is_empty() {
                result.insert("cardKingdomFoilId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.card_kingdom_id {
            if !val.is_empty() {
                result.insert("cardKingdomId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.cardsphere_foil_id {
            if !val.is_empty() {
                result.insert("cardsphereFoilId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.cardsphere_id {
            if !val.is_empty() {
                result.insert("cardsphereId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.mcm_id {
            if !val.is_empty() {
                result.insert("mcmId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.mcm_meta_id {
            if !val.is_empty() {
                result.insert("mcmMetaId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.mtg_arena_id {
            if !val.is_empty() {
                result.insert("mtgArenaId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.mtgjson_foil_version_id {
            if !val.is_empty() {
                result.insert("mtgjsonFoilVersionId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.mtgjson_non_foil_version_id {
            if !val.is_empty() {
                result.insert("mtgjsonNonFoilVersionId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.mtgjson_v4_id {
            if !val.is_empty() {
                result.insert("mtgjsonV4Id".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.mtgo_foil_id {
            if !val.is_empty() {
                result.insert("mtgoFoilId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.mtgo_id {
            if !val.is_empty() {
                result.insert("mtgoId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.multiverse_id {
            if !val.is_empty() {
                result.insert("multiverseId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.scryfall_id {
            if !val.is_empty() {
                result.insert("scryfallId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.scryfall_illustration_id {
            if !val.is_empty() {
                result.insert("scryfallIllustrationId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.scryfall_card_back_id {
            if !val.is_empty() {
                result.insert("scryfallCardBackId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.scryfall_oracle_id {
            if !val.is_empty() {
                result.insert("scryfallOracleId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.tcgplayer_etched_product_id {
            if !val.is_empty() {
                result.insert("tcgplayerEtchedProductId".to_string(), val.clone());
            }
        }
        if let Some(ref val) = self.tcgplayer_product_id {
            if !val.is_empty() {
                result.insert("tcgplayerProductId".to_string(), val.clone());
            }
        }
        
        Ok(result)
    }

    /// Check if any identifiers have been set
    /// 
    /// Returns true if at least one identifier has a non-empty value
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether any identifiers are present
    /// 
    /// # Examples
    /// 
    /// ```python
    /// identifiers = MtgjsonIdentifiers()
    /// assert not identifiers.has_identifiers()  # All empty initially
    /// 
    /// identifiers.scryfall_id = "12345678-1234-1234-1234-123456789012"
    /// assert identifiers.has_identifiers()  # Now has at least one identifier
    /// ```
    pub fn has_identifiers(&self) -> bool {
        let fields = [
            &self.card_kingdom_etched_id,
            &self.card_kingdom_foil_id,
            &self.card_kingdom_id,
            &self.cardsphere_foil_id,
            &self.cardsphere_id,
            &self.mcm_id,
            &self.mcm_meta_id,
            &self.mtg_arena_id,
            &self.mtgjson_foil_version_id,
            &self.mtgjson_non_foil_version_id,
            &self.mtgjson_v4_id,
            &self.mtgo_foil_id,
            &self.mtgo_id,
            &self.multiverse_id,
            &self.scryfall_id,
            &self.scryfall_illustration_id,
            &self.scryfall_card_back_id,
            &self.scryfall_oracle_id,
            &self.tcgplayer_etched_product_id,
            &self.tcgplayer_product_id,
        ];
        
        fields.iter().any(|field| {
            if let Some(value) = field {
                !value.is_empty()
            } else {
                false
            }
        })
    }

    /// Get count of populated identifiers
    /// 
    /// Returns the number of identifiers that have non-empty values
    /// 
    /// # Returns
    /// 
    /// The count of populated identifiers
    /// 
    /// # Examples
    /// 
    /// ```python
    /// identifiers = MtgjsonIdentifiers()
    /// assert identifiers.count_identifiers() == 0
    /// 
    /// identifiers.scryfall_id = "12345678-1234-1234-1234-123456789012"
    /// identifiers.multiverse_id = "12345"
    /// assert identifiers.count_identifiers() == 2
    /// ```
    pub fn count_identifiers(&self) -> usize {
        let fields = [
            &self.card_kingdom_etched_id,
            &self.card_kingdom_foil_id,
            &self.card_kingdom_id,
            &self.cardsphere_foil_id,
            &self.cardsphere_id,
            &self.mcm_id,
            &self.mcm_meta_id,
            &self.mtg_arena_id,
            &self.mtgjson_foil_version_id,
            &self.mtgjson_non_foil_version_id,
            &self.mtgjson_v4_id,
            &self.mtgo_foil_id,
            &self.mtgo_id,
            &self.multiverse_id,
            &self.scryfall_id,
            &self.scryfall_illustration_id,
            &self.scryfall_card_back_id,
            &self.scryfall_oracle_id,
            &self.tcgplayer_etched_product_id,
            &self.tcgplayer_product_id,
        ];
        
        fields.iter().filter(|field| {
            if let Some(value) = field {
                !value.is_empty()
            } else {
                false
            }
        }).count()
    }
}

impl JsonObject for MtgjsonIdentifiers {
    fn build_keys_to_skip(&self) -> HashSet<String> {
        HashSet::new() // All empty values are handled by serde skip_serializing_if
    }
}