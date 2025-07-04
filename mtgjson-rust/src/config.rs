use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Once;

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration file not found: {0}")]
    FileNotFound(PathBuf),
    #[error("Configuration parsing error: {0}")]
    ParseError(String),
    #[error("Missing required configuration: {0}")]
    MissingRequired(String),
    #[error("AWS SSM error: {0}")]
    AwsError(String),
}

/// Configuration section for different providers and services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSection {
    pub entries: HashMap<String, String>,
}

impl ConfigSection {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.entries.insert(key, value);
    }
}

/// MTGJSON Configuration - Rust equivalent of Python's MtgjsonConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(name = "MtgjsonConfig")]
pub struct MtgjsonConfig {
    #[pyo3(get, set)]
    pub mtgjson_version: String,
    
    #[pyo3(get, set)]
    pub output_path: PathBuf,
    
    #[pyo3(get, set)]
    pub cache_path: PathBuf,
    
    #[pyo3(get, set)]
    pub resource_path: PathBuf,
    
    /// Configuration sections (equivalent to Python's ConfigParser sections)
    sections: HashMap<String, ConfigSection>,
    
    /// Indicates if this is a singleton instance
    initialized: bool,
}

impl Default for MtgjsonConfig {
    fn default() -> Self {
        Self::new()
    }
}

// Singleton instance management
static mut INSTANCE: Option<MtgjsonConfig> = None;
static INIT: Once = Once::new();

#[pymethods]
impl MtgjsonConfig {
    #[new]
    #[pyo3(signature = (aws_ssm_config_name = None))]
    pub fn new(aws_ssm_config_name: Option<String>) -> Self {
        unsafe {
            INIT.call_once(|| {
                let config = if let Some(ssm_name) = aws_ssm_config_name {
                    Self::from_aws_ssm(&ssm_name).unwrap_or_else(|_| Self::default_config())
                } else {
                    Self::from_file().unwrap_or_else(|_| Self::default_config())
                };
                INSTANCE = Some(config);
            });
            
            INSTANCE.as_ref().unwrap().clone()
        }
    }

    /// Get the singleton instance
    #[staticmethod]
    pub fn get_instance() -> MtgjsonConfig {
        unsafe {
            INSTANCE.as_ref().unwrap_or(&Self::default_config()).clone()
        }
    }

    /// Check if a configuration section exists
    pub fn has_section(&self, section: &str) -> bool {
        self.sections.contains_key(section)
    }

    /// Get a configuration value from a section
    pub fn get(&self, section: &str, key: &str) -> Option<String> {
        self.sections.get(section)?.get(key).cloned()
    }

    /// Set a configuration value in a section
    pub fn set(&mut self, section: &str, key: &str, value: &str) {
        let config_section = self.sections.entry(section.to_string()).or_insert_with(ConfigSection::new);
        config_section.set(key.to_string(), value.to_string());
    }

    /// Get all keys in a section
    pub fn get_section_keys(&self, section: &str) -> Vec<String> {
        self.sections.get(section)
            .map(|s| s.entries.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Add a new section
    pub fn add_section(&mut self, section: &str) {
        self.sections.insert(section.to_string(), ConfigSection::new());
    }

    /// Remove a section
    pub fn remove_section(&mut self, section: &str) -> bool {
        self.sections.remove(section).is_some()
    }

    /// Get the configuration file path
    pub fn get_config_path(&self) -> PathBuf {
        self.get_project_root().join("mtgjson.properties")
    }

    /// Get the project root directory
    pub fn get_project_root(&self) -> PathBuf {
        env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
    }

    /// Validate configuration - equivalent to Python's validate_config_file_in_place
    pub fn validate(&self) -> Result<(), ConfigError> {
        let config_path = self.get_config_path();
        if !config_path.exists() {
            return Err(ConfigError::FileNotFound(config_path));
        }
        Ok(())
    }

    /// Convert to JSON string for PyO3 compatibility
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

impl MtgjsonConfig {
    /// Create default configuration
    fn default_config() -> Self {
        let mut config = Self {
            mtgjson_version: "5.2.0".to_string(),
            output_path: PathBuf::from("output"),
            cache_path: PathBuf::from("cache"),
            resource_path: PathBuf::from("resources"),
            sections: HashMap::new(),
            initialized: true,
        };

        // Add default sections
        config.add_section("Database");
        config.add_section("Providers");
        config.add_section("Scryfall");
        config.add_section("TCGPlayer");
        config.add_section("CardKingdom");
        config.add_section("CardMarket");
        config.add_section("CardHoarder");
        config.add_section("Prices");
        config.add_section("Alerts");
        config.add_section("AWS");

        config
    }

    /// Load configuration from file (equivalent to Python's properties file loading)
    fn from_file() -> Result<Self, ConfigError> {
        let config_path = Self::default_config().get_config_path();
        
        if !config_path.exists() {
            return Err(ConfigError::FileNotFound(config_path));
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;

        let mut config = Self::default_config();
        
        // Parse Java-style properties file
        let mut current_section = "DEFAULT".to_string();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            
            // Check for section headers [section]
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len()-1].to_string();
                config.add_section(&current_section);
                continue;
            }
            
            // Parse key=value pairs
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim();
                let value = line[eq_pos+1..].trim();
                config.set(&current_section, key, value);
            }
        }

        Ok(config)
    }

    /// Load configuration from AWS SSM Parameter Store
    fn from_aws_ssm(parameter_name: &str) -> Result<Self, ConfigError> {
        // TODO: Implement AWS SSM integration
        // For now, return default config with AWS section populated
        let mut config = Self::default_config();
        config.set("AWS", "ssm_parameter_name", parameter_name);
        
        println!("Loading configuration from AWS SSM: {}", parameter_name);
        // In real implementation, would use AWS SDK to fetch parameter value
        // and parse it as configuration data
        
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file(&self, file_path: Option<&Path>) -> Result<(), ConfigError> {
        let path = file_path.unwrap_or(&self.get_config_path());
        
        let mut content = String::new();
        content.push_str("# MTGJSON Configuration File\n");
        content.push_str(&format!("# Version: {}\n", self.mtgjson_version));
        content.push_str("# Generated automatically\n\n");
        
        for (section_name, section) in &self.sections {
            content.push_str(&format!("[{}]\n", section_name));
            
            for (key, value) in &section.entries {
                content.push_str(&format!("{}={}\n", key, value));
            }
            
            content.push('\n');
        }

        fs::write(path, content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
        
        Ok(())
    }

    /// Get database connection string (if configured)
    pub fn get_database_url(&self) -> Option<String> {
        self.get("Database", "url")
    }

    /// Get provider API key
    pub fn get_provider_key(&self, provider: &str) -> Option<String> {
        self.get(provider, "api_key")
            .or_else(|| self.get(provider, "key"))
    }

    /// Check if alerts are enabled
    pub fn are_alerts_enabled(&self) -> bool {
        self.get("Alerts", "enabled")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false)
    }

    /// Get output directory with environment variable override
    pub fn get_output_path(&self) -> PathBuf {
        env::var("MTGJSON_OUTPUT_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| self.output_path.clone())
    }

    /// Get cache directory with environment variable override
    pub fn get_cache_path(&self) -> PathBuf {
        env::var("MTGJSON_CACHE_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| self.cache_path.clone())
    }

    /// Get resource directory with environment variable override
    pub fn get_resource_path(&self) -> PathBuf {
        env::var("MTGJSON_RESOURCE_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| self.resource_path.clone())
    }

    /// Load environment variables into configuration
    pub fn load_env_overrides(&mut self) {
        // Common environment variable patterns
        let env_mappings = [
            ("SCRYFALL_API_KEY", "Scryfall", "api_key"),
            ("TCGPLAYER_API_KEY", "TCGPlayer", "api_key"),
            ("CARDKINGDOM_API_KEY", "CardKingdom", "api_key"),
            ("CARDMARKET_API_KEY", "CardMarket", "api_key"),
            ("CARDHOARDER_API_KEY", "CardHoarder", "api_key"),
            ("DATABASE_URL", "Database", "url"),
            ("ALERTS_ENABLED", "Alerts", "enabled"),
            ("AWS_ACCESS_KEY_ID", "AWS", "access_key_id"),
            ("AWS_SECRET_ACCESS_KEY", "AWS", "secret_access_key"),
            ("AWS_REGION", "AWS", "region"),
        ];

        for (env_var, section, key) in env_mappings {
            if let Ok(value) = env::var(env_var) {
                self.set(section, key, &value);
            }
        }
    }
}

/// Global configuration getter function for easy access
pub fn get_config() -> MtgjsonConfig {
    MtgjsonConfig::get_instance()
}

/// Initialize configuration with validation
pub fn init_config(aws_ssm_config_name: Option<String>) -> Result<(), ConfigError> {
    let config = MtgjsonConfig::new(aws_ssm_config_name);
    config.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = MtgjsonConfig::default_config();
        assert_eq!(config.mtgjson_version, "5.2.0");
        assert!(config.has_section("Database"));
        assert!(config.has_section("Providers"));
    }

    #[test]
    fn test_config_sections() {
        let mut config = MtgjsonConfig::default_config();
        
        // Test adding section
        config.add_section("TestSection");
        assert!(config.has_section("TestSection"));
        
        // Test setting and getting values
        config.set("TestSection", "test_key", "test_value");
        assert_eq!(config.get("TestSection", "test_key"), Some("test_value".to_string()));
        
        // Test removing section
        assert!(config.remove_section("TestSection"));
        assert!(!config.has_section("TestSection"));
    }

    #[test]
    fn test_config_file_operations() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.properties");
        
        // Create test config
        let mut config = MtgjsonConfig::default_config();
        config.set("TestSection", "key1", "value1");
        config.set("TestSection", "key2", "value2");
        
        // Save to file
        config.save_to_file(Some(&config_path)).unwrap();
        
        // Verify file was created
        assert!(config_path.exists());
        
        // Read file content
        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("[TestSection]"));
        assert!(content.contains("key1=value1"));
        assert!(content.contains("key2=value2"));
    }

    #[test]
    fn test_environment_overrides() {
        let mut config = MtgjsonConfig::default_config();
        
        // Set environment variable
        env::set_var("SCRYFALL_API_KEY", "test_key_123");
        
        // Load environment overrides
        config.load_env_overrides();
        
        // Check if the value was loaded
        assert_eq!(config.get("Scryfall", "api_key"), Some("test_key_123".to_string()));
        
        // Clean up
        env::remove_var("SCRYFALL_API_KEY");
    }

    #[test]
    fn test_path_getters() {
        let config = MtgjsonConfig::default_config();
        
        // Test path getters
        assert_eq!(config.get_output_path(), PathBuf::from("output"));
        assert_eq!(config.get_cache_path(), PathBuf::from("cache"));
        assert_eq!(config.get_resource_path(), PathBuf::from("resources"));
        
        // Test environment variable override
        env::set_var("MTGJSON_OUTPUT_PATH", "/custom/output");
        assert_eq!(config.get_output_path(), PathBuf::from("/custom/output"));
        env::remove_var("MTGJSON_OUTPUT_PATH");
    }

    #[test]
    fn test_provider_key_getter() {
        let mut config = MtgjsonConfig::default_config();
        
        // Test getting provider key
        config.set("Scryfall", "api_key", "scryfall_key");
        assert_eq!(config.get_provider_key("Scryfall"), Some("scryfall_key".to_string()));
        
        // Test fallback to "key" field
        config.set("TCGPlayer", "key", "tcgplayer_key");
        assert_eq!(config.get_provider_key("TCGPlayer"), Some("tcgplayer_key".to_string()));
    }

    #[test]
    fn test_alerts_enabled() {
        let mut config = MtgjsonConfig::default_config();
        
        // Test default (should be false)
        assert!(!config.are_alerts_enabled());
        
        // Test enabled
        config.set("Alerts", "enabled", "true");
        assert!(config.are_alerts_enabled());
        
        // Test disabled
        config.set("Alerts", "enabled", "false");
        assert!(!config.are_alerts_enabled());
    }
}