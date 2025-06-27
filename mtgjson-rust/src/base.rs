use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Base trait for all MTGJSON objects, equivalent to Python's JsonObject abstract base class
/// 
/// This trait provides the core functionality required by all MTGJSON data structures,
/// including JSON serialization, key filtering, and standard transformations.
/// It mirrors the behavior of the Python JsonObject class while leveraging Rust's
/// type system and performance characteristics.
/// 
/// # Design Philosophy
/// 
/// - **Consistency**: Matches the Python JsonObject interface for seamless interoperability 
/// - **Performance**: Leverages Rust's zero-cost abstractions and compile-time optimizations
/// - **Safety**: Uses Rust's type system to prevent common serialization errors
/// - **Flexibility**: Provides customization points for specialized serialization needs
/// 
/// # Default Implementations
/// 
/// The trait provides sensible defaults for most methods, requiring implementing types
/// to only override specific behavior when needed.
/// 
/// # Examples
/// 
/// ```rust
/// use crate::base::JsonObject;
/// use serde::{Serialize, Deserialize};
/// use std::collections::HashSet;
/// 
/// #[derive(Serialize, Deserialize)]
/// struct MyObject {
///     pub name: String,
///     pub value: i32,
/// }
/// 
/// impl JsonObject for MyObject {
///     fn build_keys_to_skip(&self) -> HashSet<String> {
///         // Skip serializing if value is negative
///         if self.value < 0 {
///             let mut skip = HashSet::new();
///             skip.insert("value".to_string());
///             skip
///         } else {
///             HashSet::new()
///         }
///     }
/// }
/// ```
pub trait JsonObject {
    /// Determine what keys should be avoided in the JSON dump
    /// 
    /// This method allows implementing types to specify which fields should be
    /// excluded from JSON serialization based on runtime conditions. This is
    /// equivalent to the Python JsonObject.build_keys_to_skip() method.
    /// 
    /// By default, no keys are skipped. Override this method to implement
    /// custom filtering logic.
    /// 
    /// # Returns
    /// 
    /// A HashSet containing the string names of fields to skip during serialization
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use std::collections::HashSet;
    /// 
    /// // Skip empty optional fields
    /// fn build_keys_to_skip(&self) -> HashSet<String> {
    ///     let mut skip = HashSet::new();
    ///     if self.optional_field.is_none() {
    ///         skip.insert("optional_field".to_string());
    ///     }
    ///     skip
    /// }
    /// ```
    fn build_keys_to_skip(&self) -> HashSet<String> {
        HashSet::new()
    }
    
    /// Convert the object to a JSON string representation
    /// 
    /// Serializes the implementing type to a JSON string using serde_json.
    /// This method provides a standardized way to convert MTGJSON objects
    /// to their string representation for storage or transmission.
    /// 
    /// # Returns
    /// 
    /// A Result containing either the JSON string or a serialization error
    /// 
    /// # Errors
    /// 
    /// Returns a serde_json::Error if:
    /// - The object contains values that cannot be serialized to JSON
    /// - Circular references are detected (though this is rare with MTGJSON data)
    /// - Memory allocation fails during serialization
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let obj = MyObject { name: "Lightning Bolt".to_string(), value: 3 };
    /// let json_string = obj.to_json_string().unwrap();
    /// println!("JSON: {}", json_string);
    /// // Output: {"name":"Lightning Bolt","value":3}
    /// ```
    fn to_json_string(&self) -> Result<String, serde_json::Error>
    where
        Self: Serialize,
    {
        serde_json::to_string(self)
    }
    
    /// Convert the object to a JSON value for further manipulation
    /// 
    /// Serializes the implementing type to a serde_json::Value, which can be
    /// further manipulated, merged with other JSON structures, or selectively
    /// serialized. This is useful for complex JSON transformations.
    /// 
    /// # Returns
    /// 
    /// A Result containing either the JSON Value or a serialization error
    /// 
    /// # Errors
    /// 
    /// Returns a serde_json::Error if:
    /// - The object contains values that cannot be serialized to JSON
    /// - Memory allocation fails during serialization
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let obj = MyObject { name: "Lightning Bolt".to_string(), value: 3 };
    /// let json_value = obj.to_json_value().unwrap();
    /// 
    /// // Can now manipulate the JSON value
    /// if let serde_json::Value::Object(mut map) = json_value {
    ///     map.insert("added_field".to_string(), serde_json::Value::Bool(true));
    /// }
    /// ```
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error>
    where
        Self: Serialize,
    {
        serde_json::to_value(self)
    }
}

/// Convert snake_case string to camelCase
/// 
/// This utility function transforms snake_case identifiers to camelCase,
/// which is the standard naming convention used in MTGJSON output files.
/// This ensures consistency between Rust field names (which use snake_case
/// by convention) and the JSON output format.
/// 
/// The conversion follows these rules:
/// - First word remains lowercase
/// - Subsequent words have their first letter capitalized
/// - Underscores are removed
/// - Empty strings and single words are returned unchanged
/// 
/// This is equivalent to the Python `to_camel_case` function in utils.py.
/// 
/// # Arguments
/// 
/// * `snake_str` - A string slice in snake_case format
/// 
/// # Returns
/// 
/// A String in camelCase format
/// 
/// # Examples
/// 
/// ```rust
/// use crate::base::to_camel_case;
/// 
/// assert_eq!(to_camel_case("card_kingdom_id"), "cardKingdomId");
/// assert_eq!(to_camel_case("mtg_arena_id"), "mtgArenaId");
/// assert_eq!(to_camel_case("name"), "name");  // Single word unchanged
/// assert_eq!(to_camel_case(""), "");         // Empty string unchanged
/// ```
/// 
/// # Performance
/// 
/// This function performs a single pass through the input string, making it
/// O(n) in time complexity. It allocates a new String for the result.
pub fn to_camel_case(snake_str: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for c in snake_str.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap_or(c));
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Serde serializer helper that skips empty/falsy values
/// 
/// This function can be used with serde's `skip_serializing_if` attribute
/// to exclude fields that contain their default value. This is useful for
/// keeping JSON output clean by omitting fields that don't add information.
/// 
/// # Type Parameters
/// 
/// * `T` - The type being checked, which must implement Default and PartialEq
/// 
/// # Arguments
/// 
/// * `value` - An optional value to check
/// 
/// # Returns
/// 
/// `true` if the value should be skipped (is None or equals the default), `false` otherwise
/// 
/// # Examples
/// 
/// ```rust
/// use serde::Serialize;
/// use crate::base::skip_if_empty;
/// 
/// #[derive(Serialize)]
/// struct Example {
///     #[serde(skip_serializing_if = "skip_if_empty")]
///     pub optional_count: Option<i32>,
/// }
/// 
/// let example = Example { optional_count: Some(0) };
/// // This field will be skipped since 0 is the default for i32
/// ```
pub fn skip_if_empty<T>(value: &Option<T>) -> bool
where
    T: Default + PartialEq,
{
    match value {
        Some(v) => *v == T::default(),
        None => true,
    }
}

/// Serde serializer helper that skips empty vectors
/// 
/// This function can be used with serde's `skip_serializing_if` attribute
/// to exclude Vec fields that are empty. This helps keep JSON output clean
/// by not including empty arrays.
/// 
/// # Type Parameters
/// 
/// * `T` - The element type of the vector
/// 
/// # Arguments
/// 
/// * `value` - A reference to the vector to check
/// 
/// # Returns
/// 
/// `true` if the vector is empty (should be skipped), `false` otherwise
/// 
/// # Examples
/// 
/// ```rust
/// use serde::Serialize;
/// use crate::base::skip_if_empty_vec;
/// 
/// #[derive(Serialize)]
/// struct CardList {
///     #[serde(skip_serializing_if = "skip_if_empty_vec")]
///     pub rulings: Vec<String>,
/// }
/// 
/// let card = CardList { rulings: vec![] };
/// // The rulings field will be omitted from JSON output
/// ```
pub fn skip_if_empty_vec<T>(value: &Vec<T>) -> bool {
    value.is_empty()
}

/// Serde serializer helper that skips empty strings
/// 
/// This function can be used with serde's `skip_serializing_if` attribute
/// to exclude String fields that are empty. This is commonly used for
/// optional text fields that may not have content.
/// 
/// # Arguments
/// 
/// * `value` - A reference to the string to check
/// 
/// # Returns
/// 
/// `true` if the string is empty (should be skipped), `false` otherwise
/// 
/// # Examples
/// 
/// ```rust
/// use serde::Serialize;
/// use crate::base::skip_if_empty_string;
/// 
/// #[derive(Serialize)]
/// struct CardText {
///     pub name: String,
///     #[serde(skip_serializing_if = "skip_if_empty_string")]
///     pub flavor_text: String,
/// }
/// 
/// let card = CardText { 
///     name: "Lightning Bolt".to_string(), 
///     flavor_text: "".to_string() 
/// };
/// // flavor_text will be omitted from JSON since it's empty
/// ```
pub fn skip_if_empty_string(value: &str) -> bool {
    value.is_empty()
}

/// Serde serializer helper that skips empty optional strings
/// 
/// This function combines the behavior of checking for None values and
/// empty strings. It's the most commonly used skip function for optional
/// string fields in MTGJSON structures.
/// 
/// # Arguments
/// 
/// * `value` - A reference to the optional string to check
/// 
/// # Returns
/// 
/// `true` if the value is None or contains an empty string (should be skipped), `false` otherwise
/// 
/// # Examples
/// 
/// ```rust
/// use serde::Serialize;
/// use crate::base::skip_if_empty_optional_string;
/// 
/// #[derive(Serialize)]
/// struct CardIdentifiers {
///     #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
///     pub scryfall_id: Option<String>,
///     #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
///     pub multiverse_id: Option<String>,
/// }
/// 
/// let identifiers = CardIdentifiers {
///     scryfall_id: Some("abc123".to_string()),  // Will be included
///     multiverse_id: Some("".to_string()),      // Will be skipped (empty)
/// };
/// ```
pub fn skip_if_empty_optional_string(value: &Option<String>) -> bool {
    match value {
        Some(s) => s.is_empty(),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("snake_case"), "snakeCase");
        assert_eq!(to_camel_case("already_camel"), "alreadyCamel");
        assert_eq!(to_camel_case("single"), "single");
        assert_eq!(to_camel_case(""), "");
        assert_eq!(to_camel_case("multiple_underscore_words"), "multipleUnderscoreWords");
        assert_eq!(to_camel_case("trailing_underscore_"), "trailingUnderscore");
    }

    #[test]
    fn test_skip_if_empty() {
        assert!(skip_if_empty(&None::<i32>));
        assert!(skip_if_empty(&Some(0i32))); // 0 is default for i32
        assert!(!skip_if_empty(&Some(42i32)));
        
        assert!(skip_if_empty(&None::<String>));
        assert!(skip_if_empty(&Some(String::new()))); // Empty string is default
        assert!(!skip_if_empty(&Some("content".to_string())));
    }

    #[test]
    fn test_skip_if_empty_vec() {
        assert!(skip_if_empty_vec(&Vec::<i32>::new()));
        assert!(!skip_if_empty_vec(&vec![1, 2, 3]));
    }

    #[test]
    fn test_skip_if_empty_string() {
        assert!(skip_if_empty_string(""));
        assert!(!skip_if_empty_string("content"));
    }

    #[test]
    fn test_skip_if_empty_optional_string() {
        assert!(skip_if_empty_optional_string(&None));
        assert!(skip_if_empty_optional_string(&Some("".to_string())));
        assert!(!skip_if_empty_optional_string(&Some("content".to_string())));
    }
}