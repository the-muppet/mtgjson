use std::collections::HashMap;

/// Build HTTP header for Scryfall
pub fn build_http_header() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    
    // In a real implementation, you'd read from MtgjsonConfig
    // For now, just return basic headers
    headers.insert("User-Agent".to_string(), "MTGJSON-Rust/1.0".to_string());
    headers.insert("Connection".to_string(), "Keep-Alive".to_string());
    
    // TODO: Add proper configuration support
    // if MtgjsonConfig().has_section("Scryfall") {
    //     if let Some(client_secret) = MtgjsonConfig().get("Scryfall", "client_secret") {
    //         headers.insert("Authorization".to_string(), format!("Bearer {}", client_secret));
    //     }
    // }
    
    headers
}