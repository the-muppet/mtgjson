// MTGJSON output generator - High performance file writing and JSON processing
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::classes::{MtgjsonMetaObject, MtgjsonSetObject, MtgjsonDeckHeaderObject};
use crate::compiled_classes::{
    MtgjsonAllIdentifiers, MtgjsonAllPrintings, MtgjsonAtomicCards, MtgjsonCardTypesObject,
    MtgjsonCompiledList, MtgjsonDeckObjectList, MtgjsonEnumValues, MtgjsonKeywords,
    MtgjsonSetObjectList, MtgjsonStructures, MtgjsonTcgplayerSkus,
};
use crate::config::get_config;
use crate::constants::SUPPORTED_FORMAT_OUTPUTS;
use crate::providers::GitHubDecksProvider;
use crate::utils_functions::get_file_hash;

/// MTGJSON Output Generator - Equivalent to Python's output_generator.py
#[derive(Debug, Clone)]
#[pyclass(name = "OutputGenerator")]
pub struct OutputGenerator {
    #[pyo3(get, set)]
    pub output_path: PathBuf,
    
    #[pyo3(get, set)]
    pub pretty_print: bool,
}

#[pymethods]
impl OutputGenerator {
    #[new]
    pub fn new() -> Self {
        let config = get_config();
        Self {
            output_path: config.get_output_path(),
            pretty_print: false,
        }
    }

    /// Generate compiled prices output
    pub fn generate_compiled_prices_output(
        &self,
        all_price_data: HashMap<String, Value>,
        today_price_data: HashMap<String, Value>,
        pretty_print: bool,
    ) -> PyResult<()> {
        println!("Building Prices");
        
        let structures = MtgjsonStructures::new();
        
        // AllPrices.json
        self.create_compiled_output(
            &structures.all_prices,
            &all_price_data,
            pretty_print,
            false, // don't sort keys for large price files
        )?;

        // AllPricesToday.json
        self.create_compiled_output(
            &structures.all_prices_today,
            &today_price_data,
            pretty_print,
            false, // don't sort keys for large price files
        )?;

        Ok(())
    }

    /// Build format-specific files based on AllPrintings
    pub fn build_format_specific_files(
        &self,
        all_printings: &MtgjsonAllPrintings,
        pretty_print: bool,
    ) -> PyResult<()> {
        let format_map = self.construct_format_map(None, true)?;
        let structures = MtgjsonStructures::new();

        // Standard.json
        self.create_compiled_output(
            &structures.all_printings_standard,
            &all_printings.get_set_contents(&format_map.get("standard").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        // Pioneer.json
        self.create_compiled_output(
            &structures.all_printings_pioneer,
            &all_printings.get_set_contents(&format_map.get("pioneer").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        // Modern.json
        self.create_compiled_output(
            &structures.all_printings_modern,
            &all_printings.get_set_contents(&format_map.get("modern").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        // Legacy.json
        self.create_compiled_output(
            &structures.all_printings_legacy,
            &all_printings.get_set_contents(&format_map.get("legacy").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        // Vintage.json
        self.create_compiled_output(
            &structures.all_printings_vintage,
            &all_printings.get_set_contents(&format_map.get("vintage").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        Ok(())
    }

    /// Build atomic-specific files based on AtomicCards
    pub fn build_atomic_specific_files(&self, pretty_print: bool) -> PyResult<()> {
        let card_format_map = self.construct_atomic_cards_format_map(None)?;
        let structures = MtgjsonStructures::new();

        // StandardCards.json
        self.create_compiled_output(
            &structures.atomic_cards_standard,
            &MtgjsonAtomicCards::new_with_cards(&card_format_map.get("standard").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        // PioneerCards.json
        self.create_compiled_output(
            &structures.atomic_cards_pioneer,
            &MtgjsonAtomicCards::new_with_cards(&card_format_map.get("pioneer").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        // ModernCards.json
        self.create_compiled_output(
            &structures.atomic_cards_modern,
            &MtgjsonAtomicCards::new_with_cards(&card_format_map.get("modern").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        // LegacyCards.json
        self.create_compiled_output(
            &structures.atomic_cards_legacy,
            &MtgjsonAtomicCards::new_with_cards(&card_format_map.get("legacy").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        // VintageCards.json
        self.create_compiled_output(
            &structures.atomic_cards_vintage,
            &MtgjsonAtomicCards::new_with_cards(&card_format_map.get("vintage").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        // PauperCards.json
        self.create_compiled_output(
            &structures.atomic_cards_pauper,
            &MtgjsonAtomicCards::new_with_cards(&card_format_map.get("pauper").unwrap_or(&Vec::new())),
            pretty_print,
            true,
        )?;

        Ok(())
    }

    /// Build all printings files
    pub fn build_all_printings_files(&self, pretty_print: bool) -> PyResult<()> {
        let all_printings = MtgjsonAllPrintings::new();
        let structures = MtgjsonStructures::new();

        // AllPrintings.json
        self.create_compiled_output(
            &structures.all_printings,
            &all_printings.get_set_contents(&Vec::new()),
            pretty_print,
            true,
        )?;

        // Format-specific files
        self.build_format_specific_files(&all_printings, pretty_print)?;

        // AllIdentifiers.json
        self.create_compiled_output(
            &structures.all_identifiers,
            &MtgjsonAllIdentifiers::new(&all_printings.to_json()?),
            pretty_print,
            true,
        )?;

        Ok(())
    }

    /// Generate all compiled output files
    pub fn generate_compiled_output_files(&self, pretty_print: bool) -> PyResult<()> {
        println!("Building Compiled Outputs");
        let structures = MtgjsonStructures::new();

        // AllPrintings, <FORMAT>, & AllIdentifiers
        self.build_all_printings_files(pretty_print)?;

        // AllTcgplayerSkus.json
        let all_printings_path = self.output_path.join("AllPrintings.json");
        self.create_compiled_output(
            &structures.all_tcgplayer_skus,
            &MtgjsonTcgplayerSkus::new(&all_printings_path),
            pretty_print,
            true,
        )?;

        // CompiledList.json
        self.create_compiled_output(
            &structures.compiled_list,
            &MtgjsonCompiledList::new(),
            pretty_print,
            true,
        )?;

        // Keywords.json
        self.create_compiled_output(
            &structures.key_words,
            &MtgjsonKeywords::new(),
            pretty_print,
            true,
        )?;

        // CardTypes.json
        self.create_compiled_output(
            &structures.card_types,
            &MtgjsonCardTypesObject::new(),
            pretty_print,
            true,
        )?;

        // Meta.json (Formerly version.json)
        self.create_compiled_output(
            &structures.version,
            &MtgjsonMetaObject::new(),
            pretty_print,
            true,
        )?;

        // SetList.json
        self.create_compiled_output(
            &structures.set_list,
            &MtgjsonSetObjectList::new(),
            pretty_print,
            true,
        )?;

        // AtomicCards.json
        self.create_compiled_output(
            &structures.atomic_cards,
            &MtgjsonAtomicCards::new(),
            pretty_print,
            true,
        )?;

        // Format-specific atomic files
        self.build_atomic_specific_files(pretty_print)?;

        // All Pre-constructed Decks
        let mut deck_names = Vec::new();
        let deck_provider = GitHubDecksProvider::new();
        
        for mtgjson_deck_obj in deck_provider.iterate_precon_decks() {
            let mtgjson_deck_header_obj = MtgjsonDeckHeaderObject::new(&mtgjson_deck_obj);
            let deck_filename = format!("decks/{}", mtgjson_deck_header_obj.file_name);
            
            self.create_compiled_output(
                &deck_filename,
                &mtgjson_deck_obj,
                pretty_print,
                true,
            )?;
            
            deck_names.push(mtgjson_deck_header_obj);
        }

        // DeckList.json
        self.create_compiled_output(
            &structures.deck_list,
            &MtgjsonDeckObjectList::new(&deck_names),
            pretty_print,
            true,
        )?;

        // EnumValues.json - Depends on Keywords & Decks
        self.create_compiled_output(
            &structures.enum_values,
            &MtgjsonEnumValues::new(),
            pretty_print,
            true,
        )?;

        Ok(())
    }

    /// Create compiled output with logging
    pub fn create_compiled_output(
        &self,
        compiled_name: &str,
        compiled_object: &dyn ToJson,
        pretty_print: bool,
        sort_keys: bool,
    ) -> PyResult<()> {
        println!("Generating {}", compiled_name);
        self.write_to_file(compiled_name, compiled_object, pretty_print)?;
        println!("Finished Generating {}", compiled_name);
        Ok(())
    }

    /// Construct format map for sets
    pub fn construct_format_map(
        &self,
        all_printings_path: Option<&Path>,
        normal_sets_only: bool,
    ) -> PyResult<HashMap<String, Vec<String>>> {
        let path = all_printings_path.unwrap_or(&self.output_path.join("AllPrintings.json"));
        let mut format_map: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize format map
        for format in SUPPORTED_FORMAT_OUTPUTS {
            format_map.insert(format.to_string(), Vec::new());
        }

        if !path.exists() {
            eprintln!("Warning: {:?} was not found, skipping format map", path);
            return Ok(format_map);
        }

        let content = fs::read_to_string(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let json_data: Value = serde_json::from_str(&content)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        if let Some(data) = json_data.get("data").and_then(|d| d.as_object()) {
            for (set_code_key, set_code_content) in data {
                if let Some(set_obj) = set_code_content.as_object() {
                    // Check if this is a normal set type if filtering is enabled
                    if normal_sets_only {
                        if let Some(set_type) = set_obj.get("type").and_then(|t| t.as_str()) {
                            if !self.is_supported_set_type(set_type) {
                                continue;
                            }
                        }
                    }

                    // Get formats this set is legal in
                    let mut formats_set_legal_in: Vec<String> = SUPPORTED_FORMAT_OUTPUTS.iter()
                        .map(|s| s.to_string())
                        .collect();

                    if let Some(cards) = set_obj.get("cards").and_then(|c| c.as_array()) {
                        for card in cards {
                            if let Some(card_obj) = card.as_object() {
                                // Don't include Alchemy cards in determining legality
                                if let Some(name) = card_obj.get("name").and_then(|n| n.as_str()) {
                                    if name.starts_with("A-") {
                                        continue;
                                    }
                                }

                                if let Some(legalities) = card_obj.get("legalities").and_then(|l| l.as_object()) {
                                    let card_legalities: Vec<String> = legalities.keys()
                                        .map(|k| k.to_string())
                                        .collect();
                                    
                                    // Keep only formats that are legal for this card too
                                    formats_set_legal_in.retain(|format| card_legalities.contains(format));
                                }
                            }
                        }
                    }

                    // Add this set to each format it's legal in
                    for magic_format in formats_set_legal_in {
                        if let Some(format_sets) = format_map.get_mut(&magic_format) {
                            format_sets.push(set_code_key.clone());
                        }
                    }
                }
            }
        }

        Ok(format_map)
    }

    /// Construct atomic cards format map
    pub fn construct_atomic_cards_format_map(
        &self,
        all_printings_path: Option<&Path>,
    ) -> PyResult<HashMap<String, Vec<Value>>> {
        let path = all_printings_path.unwrap_or(&self.output_path.join("AllPrintings.json"));
        let mut format_card_map: HashMap<String, Vec<Value>> = HashMap::new();

        // Initialize format map
        for format in SUPPORTED_FORMAT_OUTPUTS {
            format_card_map.insert(format.to_string(), Vec::new());
        }

        if !path.exists() {
            eprintln!("Warning: {:?} was not found, skipping format map", path);
            return Ok(format_card_map);
        }

        let content = fs::read_to_string(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let json_data: Value = serde_json::from_str(&content)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        if let Some(data) = json_data.get("data").and_then(|d| d.as_object()) {
            for set_contents in data.values() {
                if let Some(set_obj) = set_contents.as_object() {
                    let mut set_cards = Vec::new();

                    // Get regular cards
                    if let Some(cards) = set_obj.get("cards").and_then(|c| c.as_array()) {
                        set_cards.extend(cards.iter().cloned());
                    }

                    // Workaround for Dungeons so they can be included
                    if let Some(tokens) = set_obj.get("tokens").and_then(|t| t.as_array()) {
                        for token in tokens {
                            if let Some(token_obj) = token.as_object() {
                                if let Some(card_type) = token_obj.get("type").and_then(|t| t.as_str()) {
                                    if card_type == "Dungeon" {
                                        // Add legal status for dungeons
                                        let mut modified_token = token.clone();
                                        if let Some(modified_obj) = modified_token.as_object_mut() {
                                            let mut legalities = serde_json::Map::new();
                                            for format in SUPPORTED_FORMAT_OUTPUTS {
                                                legalities.insert(format.to_string(), json!("Legal"));
                                            }
                                            modified_obj.insert("legalities".to_string(), json!(legalities));
                                        }
                                        set_cards.push(modified_token);
                                    }
                                }
                            }
                        }
                    }

                    // Process each card
                    for card in set_cards {
                        if let Some(card_obj) = card.as_object() {
                            if let Some(legalities) = card_obj.get("legalities").and_then(|l| l.as_object()) {
                                for format in SUPPORTED_FORMAT_OUTPUTS {
                                    if let Some(legality) = legalities.get(format).and_then(|l| l.as_str()) {
                                        if legality == "Legal" || legality == "Restricted" {
                                            if let Some(format_cards) = format_card_map.get_mut(format) {
                                                format_cards.push(card.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(format_card_map)
    }

    /// Generate output file hashes
    pub fn generate_output_file_hashes(&self, directory: &Path) -> PyResult<()> {
        for entry in fs::read_dir(directory)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?
        {
            let entry = entry.map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively process subdirectories
                self.generate_output_file_hashes(&path)?;
                continue;
            }

            // Don't hash the hash file itself
            if path.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with(".sha256"))
                .unwrap_or(false)
            {
                continue;
            }

            if let Some(generated_hash) = get_file_hash(&path) {
                let hash_file_name = format!("{}.sha256", path.file_name().unwrap().to_str().unwrap());
                let hash_file_path = path.parent().unwrap().join(hash_file_name);
                
                fs::write(&hash_file_path, generated_hash)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
            }
        }

        Ok(())
    }

    /// Write content to a file in the outputs directory
    pub fn write_to_file(
        &self,
        file_name: &str,
        file_contents: &dyn ToJson,
        pretty_print: bool,
    ) -> PyResult<()> {
        let write_file = self.output_path.join(format!("{}.json", file_name));
        
        // Create parent directories if they don't exist
        if let Some(parent) = write_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        }

        // Convert to JSON
        let json_content = file_contents.to_json()?;
        let mut content_value: Value = serde_json::from_str(&json_content)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        // Sort keys if needed (for consistency)
        if let Some(content_obj) = content_value.as_object_mut() {
            // Sort the keys for consistent output
            let sorted_keys: Vec<_> = content_obj.keys().cloned().collect();
            let mut sorted_content = serde_json::Map::new();
            for key in sorted_keys {
                if let Some(value) = content_obj.remove(&key) {
                    sorted_content.insert(key, value);
                }
            }
            content_value = json!(sorted_content);
        }

        // Create final output with meta wrapper
        let final_output = json!({
            "meta": MtgjsonMetaObject::new(),
            "data": content_value
        });

        // Write to file
        let mut file = fs::File::create(&write_file)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        let json_string = if pretty_print {
            serde_json::to_string_pretty(&final_output)
        } else {
            serde_json::to_string(&final_output)
        }
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        file.write_all(json_string.as_bytes())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        Ok(())
    }

    /// Helper function to check if a set type is supported
    fn is_supported_set_type(&self, set_type: &str) -> bool {
        // TODO: Import this from constants
        matches!(set_type, "core" | "expansion" | "draft_innovation" | "masters" | "commander" | "planechase" | "archenemy" | "vanguard" | "from_the_vault" | "premium_deck" | "duel_deck" | "starter" | "box" | "promo" | "token" | "memorabilia" | "treasure_chest" | "spellbook" | "arsenal" | "funny" | "un" | "minigame")
    }
}

impl Default for OutputGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for objects that can be converted to JSON
pub trait ToJson {
    fn to_json(&self) -> PyResult<String>;
}

/// Implement ToJson for common types
impl ToJson for Value {
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

impl ToJson for HashMap<String, Value> {
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

impl ToJson for MtgjsonSetObject {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonMetaObject {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonAllPrintings {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonAtomicCards {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonAllIdentifiers {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonTcgplayerSkus {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonCompiledList {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonKeywords {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonCardTypesObject {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonSetObjectList {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonDeckObjectList {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

impl ToJson for MtgjsonEnumValues {
    fn to_json(&self) -> PyResult<String> {
        self.to_json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_output_generator_creation() {
        let generator = OutputGenerator::new();
        assert!(!generator.pretty_print);
        assert!(generator.output_path.exists() || generator.output_path == PathBuf::from("output"));
    }

    #[test]
    fn test_write_to_file() {
        let temp_dir = tempdir().unwrap();
        let mut generator = OutputGenerator::new();
        generator.output_path = temp_dir.path().to_path_buf();

        let test_content = json!({"test": "data"});
        generator.write_to_file("test_file", &test_content, true).unwrap();

        let written_file = temp_dir.path().join("test_file.json");
        assert!(written_file.exists());

        let content = fs::read_to_string(&written_file).unwrap();
        assert!(content.contains("\"test\": \"data\""));
        assert!(content.contains("\"meta\""));
        assert!(content.contains("\"data\""));
    }

    #[test]
    fn test_construct_format_map() {
        let generator = OutputGenerator::new();
        
        // Test with non-existent file
        let format_map = generator.construct_format_map(Some(Path::new("nonexistent.json")), true).unwrap();
        assert!(format_map.contains_key("standard"));
        assert!(format_map.contains_key("modern"));
        assert!(format_map.contains_key("legacy"));
        
        // All format lists should be empty since file doesn't exist
        assert!(format_map.get("standard").unwrap().is_empty());
    }

    #[test]
    fn test_generate_output_file_hashes() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.json");
        
        // Create a test file
        fs::write(&test_file, r#"{"test": "data"}"#).unwrap();
        
        let generator = OutputGenerator::new();
        generator.generate_output_file_hashes(temp_dir.path()).unwrap();
        
        // Check that hash file was created
        let hash_file = temp_dir.path().join("test.json.sha256");
        assert!(hash_file.exists());
        
        // Verify hash content
        let hash_content = fs::read_to_string(&hash_file).unwrap();
        assert!(hash_content.len() == 64); // SHA256 hash length
    }
}
