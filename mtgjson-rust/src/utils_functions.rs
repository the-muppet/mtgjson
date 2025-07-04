// PyO3 wrapper functions for utility functions
use pyo3::prelude::*;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Convert string to camelCase (PyO3 wrapper)
#[pyfunction]
#[pyo3(signature = ())]
pub fn to_camel_case(string: &str) -> PyResult<String> {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for (i, ch) in string.chars().enumerate() {
        if ch == '_' || ch == '-' || ch == ' ' {
            capitalize_next = true;
        } else if i == 0 {
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        } else if capitalize_next {
            result.push(ch.to_uppercase().next().unwrap_or(ch));
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
}

/// Make a Windows-safe filename (PyO3 wrapper)
#[pyfunction]
#[pyo3(signature = ())]
pub fn make_windows_safe_filename(filename: &str) -> PyResult<String> {
    let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let mut safe_filename = String::new();
    
    for ch in filename.chars() {
        if invalid_chars.contains(&ch) {
            safe_filename.push('_');
        } else {
            safe_filename.push(ch);
        }
    }
    
    // Handle reserved names
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
    ];
    
    let uppercase_name = safe_filename.to_uppercase();
    if reserved_names.contains(&uppercase_name.as_str()) {
        safe_filename.push('_');
    }
    
    // Remove trailing dots and spaces
    safe_filename = safe_filename.trim_end_matches('.').trim_end().to_string();
    
    // Ensure not empty
    if safe_filename.is_empty() {
        safe_filename = "unnamed".to_string();
    }
    
    Ok(safe_filename)
}

/// Clean card number string (PyO3 wrapper)
#[pyfunction]
#[pyo3(signature = ())]
pub fn clean_card_number(card_number: &str) -> PyResult<String> {
    // Remove common prefixes and suffixes that clutter card numbers
    let mut cleaned = card_number.trim().to_string();
    
    // Remove leading zeros but preserve single zero
    if cleaned.len() > 1 {
        cleaned = cleaned.trim_start_matches('0').to_string();
        if cleaned.is_empty() {
            cleaned = "0".to_string();
        }
    }
    
    // Handle special characters commonly found in card numbers
    cleaned = cleaned.replace("★", "*");  // Replace star character
    cleaned = cleaned.replace("†", ""); // Remove dagger
    
    Ok(cleaned)
}

/// Calculate SHA256 hash of a file - equivalent to Python's get_file_hash
pub fn get_file_hash(file_path: &Path) -> Option<String> {
    let file = File::open(file_path).ok()?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => hasher.update(&buffer[..n]),
            Err(_) => return None,
        }
    }

    let result = hasher.finalize();
    Some(hex::encode(result))
}

/// Initialize logger - equivalent to Python's init_logger
#[pyfunction]
pub fn init_logger() {
    env_logger::init();
}

/// Send push notification - placeholder for now
#[pyfunction]
pub fn send_push_notification(message: String) -> PyResult<()> {
    println!("Push notification: {}", message);
    Ok(())
}

/// Load local set data - placeholder implementation
#[pyfunction]  
pub fn load_local_set_data() -> PyResult<std::collections::HashMap<String, serde_json::Value>> {
    // In real implementation, would load from resources/additional_sets.json
    Ok(std::collections::HashMap::new())
}

/// URL keygen function - placeholder implementation
#[pyfunction]
pub fn url_keygen(url: String) -> PyResult<String> {
    // Simple URL key generation - in real implementation would be more sophisticated
    Ok(url.replace("https://", "").replace("http://", "").replace("/", "_"))
}

/// Get string or None helper
pub fn get_str_or_none(value: Option<&str>) -> Option<String> {
    value.map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_get_file_hash() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Create a test file
        fs::write(&file_path, "Hello, world!").unwrap();
        
        // Calculate hash
        let hash = get_file_hash(&file_path);
        assert!(hash.is_some());
        
        // Verify it's a valid SHA256 hash (64 hex characters)
        let hash_str = hash.unwrap();
        assert_eq!(hash_str.len(), 64);
        assert!(hash_str.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_get_file_hash_nonexistent() {
        let hash = get_file_hash(Path::new("nonexistent_file.txt"));
        assert!(hash.is_none());
    }

    #[test]
    fn test_url_keygen() {
        let result = url_keygen("https://api.scryfall.com/cards".to_string()).unwrap();
        assert_eq!(result, "api.scryfall.com_cards");
    }

    #[test]
    fn test_get_str_or_none() {
        assert_eq!(get_str_or_none(Some("test")), Some("test".to_string()));
        assert_eq!(get_str_or_none(None), None);
    }
}