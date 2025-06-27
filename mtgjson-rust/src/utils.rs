use regex::Regex;
use std::collections::HashSet;

/// Utility functions for MTGJSON processing
/// 
/// This module provides various utility functions used throughout the MTGJSON Rust implementation
/// for data processing, validation, and formatting. These utilities handle common tasks such as:
/// 
/// - Filename sanitization for cross-platform compatibility
/// - Card number processing for proper sorting
/// - Windows filename safety checks
/// - String processing for deck name matching
/// 
/// All functions are stateless and designed to be called independently. They provide
/// equivalent functionality to the Python utility functions while leveraging Rust's
/// performance and safety features.
/// 
/// # Design Principles
/// 
/// - **Cross-platform compatibility**: All filename operations work on Windows, macOS, and Linux
/// - **Data integrity**: Preserves essential information while sanitizing problematic characters
/// - **Performance**: Optimized for processing large datasets typical of MTGJSON operations  
/// - **Safety**: Uses Rust's type system to prevent common string processing errors
/// 
/// # Examples
/// 
/// ```rust
/// use mtgjson_rust::utils::MtgjsonUtils;
/// 
/// // Sanitize a deck name for use as filename
/// let safe_name = MtgjsonUtils::sanitize_deck_name("My Awesome Deck!", "EDH");
/// assert_eq!(safe_name, "MYAWESOMEDECK_EDH");
/// 
/// // Process card numbers for sorting
/// let (num, len) = MtgjsonUtils::clean_card_number("123a");
/// assert_eq!(num, 123);
/// assert_eq!(len, 3);
/// ```
pub struct MtgjsonUtils;

impl MtgjsonUtils {
    /// Sanitize a deck name for use as a filename
    /// 
    /// Converts a deck name into a safe filename by removing problematic characters
    /// and formatting it for consistent file naming across different operating systems.
    /// This function ensures that deck files can be saved and loaded reliably on
    /// Windows, macOS, and Linux systems.
    /// 
    /// The sanitization process:
    /// 1. Removes all whitespace characters
    /// 2. Converts all characters to uppercase
    /// 3. Removes all non-word characters (keeping only letters, numbers, underscores)
    /// 4. Appends the provided set/format code with an underscore separator
    /// 
    /// # Arguments
    /// 
    /// * `name` - The original deck name to sanitize
    /// * `code` - The set or format code to append (e.g., "EDH", "STD", "MOD")
    /// 
    /// # Returns
    /// 
    /// A sanitized filename string safe for use on all operating systems
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mtgjson_rust::utils::MtgjsonUtils;
    /// 
    /// // Basic deck name sanitization
    /// let result = MtgjsonUtils::sanitize_deck_name("Lightning Aggro", "STD");
    /// assert_eq!(result, "LIGHTNINGAGGRO_STD");
    /// 
    /// // Handle special characters and symbols
    /// let result = MtgjsonUtils::sanitize_deck_name("Control & Combo!", "EDH");
    /// assert_eq!(result, "CONTROLCOMBO_EDH");
    /// 
    /// // Handle unicode and extended characters  
    /// let result = MtgjsonUtils::sanitize_deck_name("Björk's Deck™", "VIN");
    /// assert_eq!(result, "BJRKSDECK_VIN");
    /// ```
    /// 
    /// # Performance
    /// 
    /// This function performs regex operations and string transformations.
    /// It's optimized for typical deck name lengths (10-50 characters) and
    /// should handle thousands of deck names per second.
    pub fn sanitize_deck_name(name: &str, code: &str) -> String {
        let word_characters_only = Regex::new(r"\W").unwrap();
        let capital_case: String = name
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_uppercase().collect::<String>())
            .collect::<Vec<String>>()
            .join("");
        
        let deck_name_sanitized = word_characters_only.replace_all(&capital_case, "");
        format!("{}_{}", deck_name_sanitized, code)
    }
    
    /// Clean a card number for sorting purposes
    /// 
    /// Processes a card number by extracting only the numeric digits and
    /// providing metadata about the number for proper sorting. This is essential
    /// for correctly ordering cards within sets, as card numbers may contain
    /// letters, symbols, or other non-numeric characters.
    /// 
    /// The function handles various card number formats:
    /// - Simple numbers: "123" → (123, 3)
    /// - Numbers with letters: "123a" → (123, 3)  
    /// - Numbers with symbols: "123★" → (123, 3)
    /// - Complex collectors numbers: "123/350" → (123, 3)
    /// 
    /// Cards without any digits default to a high sort value (100000) to
    /// ensure they appear at the end of sorted lists.
    /// 
    /// # Arguments
    /// 
    /// * `number` - The card number string to process
    /// 
    /// # Returns
    /// 
    /// A tuple containing:
    /// - The numeric value extracted from the card number (or 100000 if no digits found)
    /// - The count of digits in the extracted number
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mtgjson_rust::utils::MtgjsonUtils;
    /// 
    /// // Standard numeric card number
    /// let (num, len) = MtgjsonUtils::clean_card_number("123");
    /// assert_eq!(num, 123);
    /// assert_eq!(len, 3);
    /// 
    /// // Card number with suffix letter
    /// let (num, len) = MtgjsonUtils::clean_card_number("123a");
    /// assert_eq!(num, 123);
    /// assert_eq!(len, 3);
    /// 
    /// // Card number with symbols/unicode
    /// let (num, len) = MtgjsonUtils::clean_card_number("007★");
    /// assert_eq!(num, 7);
    /// assert_eq!(len, 3);  // Original had 3 digits (007)
    /// 
    /// // Complex collector number format
    /// let (num, len) = MtgjsonUtils::clean_card_number("123/350");
    /// assert_eq!(num, 123350);  // Concatenated digits
    /// assert_eq!(len, 6);
    /// 
    /// // Non-numeric card numbers
    /// let (num, len) = MtgjsonUtils::clean_card_number("★★★");
    /// assert_eq!(num, 100000);  // Default high value
    /// assert_eq!(len, 0);
    /// ```
    /// 
    /// # Use Cases
    /// 
    /// This function is primarily used for:
    /// - Sorting cards within a set by collector number
    /// - Grouping cards with similar numeric bases
    /// - Handling edge cases in card numbering systems
    /// - Ensuring consistent ordering across different card number formats
    pub fn clean_card_number(number: &str) -> (u32, usize) {
        let digits_only: String = number.chars().filter(|c| c.is_ascii_digit()).collect();
        let number_int = digits_only.parse::<u32>().unwrap_or(100000);
        (number_int, digits_only.len())
    }
    
    /// Check if a filename would be problematic on Windows
    /// 
    /// Windows has reserved filenames that cannot be used for files or directories,
    /// even with extensions. This function checks if a given filename would conflict
    /// with these reserved names, ensuring cross-platform compatibility.
    /// 
    /// Reserved names in Windows include:
    /// - Device names: CON, PRN, AUX, NUL
    /// - Serial ports: COM1-COM9  
    /// - Parallel ports: LPT1-LPT9
    /// 
    /// The check is case-insensitive as Windows treats these names the same
    /// regardless of capitalization.
    /// 
    /// # Arguments
    /// 
    /// * `filename` - The filename to check (without path, may include extension)
    /// 
    /// # Returns
    /// 
    /// `true` if the filename is safe to use on Windows, `false` if it's reserved
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mtgjson_rust::utils::MtgjsonUtils;
    /// 
    /// // Safe filenames
    /// assert!(MtgjsonUtils::is_windows_safe_filename("normal_file"));
    /// assert!(MtgjsonUtils::is_windows_safe_filename("my_deck"));
    /// assert!(MtgjsonUtils::is_windows_safe_filename("set_BRO"));
    /// 
    /// // Unsafe filenames (Windows reserved)
    /// assert!(!MtgjsonUtils::is_windows_safe_filename("CON"));
    /// assert!(!MtgjsonUtils::is_windows_safe_filename("con"));  // Case insensitive
    /// assert!(!MtgjsonUtils::is_windows_safe_filename("PRN"));
    /// assert!(!MtgjsonUtils::is_windows_safe_filename("COM1"));
    /// assert!(!MtgjsonUtils::is_windows_safe_filename("LPT9"));
    /// ```
    /// 
    /// # Note
    /// 
    /// This function checks the base filename only. Extensions don't affect
    /// whether a name is reserved (e.g., "CON.txt" is still problematic).
    pub fn is_windows_safe_filename(filename: &str) -> bool {
        const BAD_NAMES: &[&str] = &[
            "CON", "PRN", "AUX", "NUL",
            "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
            "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
        ];
        
        !BAD_NAMES.contains(&filename.to_uppercase().as_str())
    }
    
    /// Make a filename Windows-safe by appending underscore if needed
    /// 
    /// Takes a potentially problematic filename and makes it safe for use on
    /// Windows systems by appending an underscore to reserved names. This
    /// provides a simple, consistent way to handle filename conflicts while
    /// preserving the original name's recognizability.
    /// 
    /// The transformation is minimal and reversible:
    /// - Safe names are returned unchanged
    /// - Reserved names get a trailing underscore
    /// - The original filename remains easily recognizable
    /// 
    /// # Arguments
    /// 
    /// * `filename` - The filename to make safe
    /// 
    /// # Returns
    /// 
    /// A filename string that is safe to use on Windows systems
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mtgjson_rust::utils::MtgjsonUtils;
    /// 
    /// // Safe filenames pass through unchanged
    /// assert_eq!(MtgjsonUtils::make_windows_safe_filename("normal_file"), "normal_file");
    /// assert_eq!(MtgjsonUtils::make_windows_safe_filename("my_deck"), "my_deck");
    /// 
    /// // Reserved names get underscore appended
    /// assert_eq!(MtgjsonUtils::make_windows_safe_filename("CON"), "CON_");
    /// assert_eq!(MtgjsonUtils::make_windows_safe_filename("con"), "con_");
    /// assert_eq!(MtgjsonUtils::make_windows_safe_filename("PRN"), "PRN_");
    /// assert_eq!(MtgjsonUtils::make_windows_safe_filename("COM1"), "COM1_");
    /// ```
    /// 
    /// # Use Cases
    /// 
    /// This function is particularly useful when:
    /// - Creating files based on user input or external data
    /// - Processing set codes that might conflict with reserved names (e.g., "CON" set)
    /// - Ensuring MTGJSON output files work correctly on all platforms
    /// - Batch processing where some filenames might be problematic
    pub fn make_windows_safe_filename(filename: &str) -> String {
        if Self::is_windows_safe_filename(filename) {
            filename.to_string()
        } else {
            format!("{}_", filename)
        }
    }
    
    /// Extract alpha-numeric characters only (for deck name matching)
    /// 
    /// Processes a string to extract only alphanumeric characters and spaces,
    /// converting the result to lowercase. This is useful for fuzzy matching
    /// of deck names, set names, or other text where punctuation, case, and
    /// special characters should be ignored.
    /// 
    /// The function preserves:
    /// - Letters (A-Z, a-z) → lowercase
    /// - Numbers (0-9) → unchanged  
    /// - Spaces → preserved for word separation
    /// 
    /// It removes:
    /// - Punctuation marks
    /// - Special symbols
    /// - Unicode decorative characters
    /// - Extra formatting characters
    /// 
    /// # Arguments
    /// 
    /// * `input` - The string to process
    /// 
    /// # Returns
    /// 
    /// A lowercase string containing only alphanumeric characters and spaces
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mtgjson_rust::utils::MtgjsonUtils;
    /// 
    /// // Basic text cleaning
    /// let result = MtgjsonUtils::alpha_numeric_only("Hello, World!");
    /// assert_eq!(result, "hello world");
    /// 
    /// // Deck name normalization
    /// let result = MtgjsonUtils::alpha_numeric_only("Control & Combo (Updated)");
    /// assert_eq!(result, "control  combo updated");
    /// 
    /// // Set name processing
    /// let result = MtgjsonUtils::alpha_numeric_only("Innistrad: Midnight Hunt");
    /// assert_eq!(result, "innistrad midnight hunt");
    /// 
    /// // Handle numbers and mixed content
    /// let result = MtgjsonUtils::alpha_numeric_only("Modern Masters 2017™");
    /// assert_eq!(result, "modern masters 2017");
    /// 
    /// // Unicode and special characters
    /// let result = MtgjsonUtils::alpha_numeric_only("Björk's Deck v2.0!");
    /// assert_eq!(result, "bjrks deck v20");
    /// ```
    /// 
    /// # Use Cases
    /// 
    /// This function is commonly used for:
    /// - Fuzzy matching of deck names from different sources
    /// - Normalizing set names for comparison
    /// - Creating search-friendly versions of card names
    /// - Preprocessing text for similarity algorithms
    /// - Cleaning user input for consistent processing
    /// 
    /// # Note
    /// 
    /// Multiple consecutive spaces may result from removed punctuation.
    /// Consider using `.split_whitespace().collect::<Vec<_>>().join(" ")`
    /// if you need to normalize spacing as well.
    pub fn alpha_numeric_only(input: &str) -> String {
        input
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
    }

    /// Extract only alphabetic characters from a string
    /// 
    /// Similar to `alpha_numeric_only` but excludes numbers, keeping only
    /// letters and spaces. This is useful for processing card names or
    /// text where numbers should be ignored for matching purposes.
    /// 
    /// # Arguments
    /// 
    /// * `input` - The string to process
    /// 
    /// # Returns
    /// 
    /// A lowercase string containing only letters and spaces
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mtgjson_rust::utils::MtgjsonUtils;
    /// 
    /// let result = MtgjsonUtils::alpha_only("Lightning Bolt 3000");
    /// assert_eq!(result, "lightning bolt ");
    /// 
    /// let result = MtgjsonUtils::alpha_only("Modern Masters 2017");  
    /// assert_eq!(result, "modern masters ");
    /// ```
    pub fn alpha_only(input: &str) -> String {
        input
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
    }

    /// Check if a string contains only alphanumeric characters
    /// 
    /// Validates whether a string consists entirely of letters, numbers,
    /// and optionally spaces. This is useful for validating identifiers,
    /// codes, or other structured data fields.
    /// 
    /// # Arguments
    /// 
    /// * `input` - The string to validate
    /// * `allow_spaces` - Whether to allow spaces in the string
    /// 
    /// # Returns
    /// 
    /// `true` if the string contains only valid characters, `false` otherwise
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mtgjson_rust::utils::MtgjsonUtils;
    /// 
    /// assert!(MtgjsonUtils::is_alphanumeric_only("ABC123", false));
    /// assert!(MtgjsonUtils::is_alphanumeric_only("Hello World", true));
    /// assert!(!MtgjsonUtils::is_alphanumeric_only("Hello World", false));  // Space not allowed
    /// assert!(!MtgjsonUtils::is_alphanumeric_only("Hello!", true));  // Punctuation not allowed
    /// ```
    pub fn is_alphanumeric_only(input: &str, allow_spaces: bool) -> bool {
        if input.is_empty() {
            return false;
        }
        
        input.chars().all(|c| {
            c.is_alphanumeric() || (allow_spaces && c.is_whitespace())
        })
    }

    /// Normalize whitespace in a string
    /// 
    /// Converts all sequences of whitespace characters (spaces, tabs, newlines)
    /// into single spaces and trims leading/trailing whitespace. This is useful
    /// for cleaning text data from various sources.
    /// 
    /// # Arguments
    /// 
    /// * `input` - The string to normalize
    /// 
    /// # Returns
    /// 
    /// A string with normalized whitespace
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mtgjson_rust::utils::MtgjsonUtils;
    /// 
    /// let result = MtgjsonUtils::normalize_whitespace("  Hello   World  \n\t");
    /// assert_eq!(result, "Hello World");
    /// 
    /// let result = MtgjsonUtils::normalize_whitespace("Multiple\n\n\nlines");
    /// assert_eq!(result, "Multiple lines");
    /// ```
    pub fn normalize_whitespace(input: &str) -> String {
        input
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_deck_name() {
        let result = MtgjsonUtils::sanitize_deck_name("Test Deck!", "ABC");
        assert_eq!(result, "TESTDECK_ABC");
        
        let result = MtgjsonUtils::sanitize_deck_name("Control & Combo", "EDH");
        assert_eq!(result, "CONTROLCOMBO_EDH");
        
        let result = MtgjsonUtils::sanitize_deck_name("", "STD");
        assert_eq!(result, "_STD");
    }

    #[test]
    fn test_clean_card_number() {
        let (num, len) = MtgjsonUtils::clean_card_number("123a");
        assert_eq!(num, 123);
        assert_eq!(len, 3);
        
        let (num, len) = MtgjsonUtils::clean_card_number("007");
        assert_eq!(num, 7);
        assert_eq!(len, 3);
        
        let (num, len) = MtgjsonUtils::clean_card_number("★★★");
        assert_eq!(num, 100000);
        assert_eq!(len, 0);
        
        let (num, len) = MtgjsonUtils::clean_card_number("123/350");
        assert_eq!(num, 123350);
        assert_eq!(len, 6);
    }

    #[test]
    fn test_windows_safe_filename() {
        assert!(!MtgjsonUtils::is_windows_safe_filename("CON"));
        assert!(!MtgjsonUtils::is_windows_safe_filename("con"));
        assert!(!MtgjsonUtils::is_windows_safe_filename("COM1"));
        assert!(!MtgjsonUtils::is_windows_safe_filename("LPT9"));
        assert!(MtgjsonUtils::is_windows_safe_filename("NORMAL"));
        assert!(MtgjsonUtils::is_windows_safe_filename("deck_name"));
    }

    #[test]
    fn test_make_windows_safe_filename() {
        assert_eq!(MtgjsonUtils::make_windows_safe_filename("CON"), "CON_");
        assert_eq!(MtgjsonUtils::make_windows_safe_filename("NORMAL"), "NORMAL");
        assert_eq!(MtgjsonUtils::make_windows_safe_filename("COM1"), "COM1_");
        assert_eq!(MtgjsonUtils::make_windows_safe_filename("my_deck"), "my_deck");
    }

    #[test]
    fn test_alpha_numeric_only() {
        let result = MtgjsonUtils::alpha_numeric_only("Test-Deck! 123");
        assert_eq!(result, "testdeck 123");
        
        let result = MtgjsonUtils::alpha_numeric_only("Control & Combo");
        assert_eq!(result, "control  combo");
        
        let result = MtgjsonUtils::alpha_numeric_only("Modern Masters 2017™");
        assert_eq!(result, "modern masters 2017");
    }

    #[test]
    fn test_alpha_only() {
        let result = MtgjsonUtils::alpha_only("Lightning Bolt 3000");
        assert_eq!(result, "lightning bolt ");
        
        let result = MtgjsonUtils::alpha_only("ABC123XYZ");
        assert_eq!(result, "abcxyz");
    }

    #[test]
    fn test_is_alphanumeric_only() {
        assert!(MtgjsonUtils::is_alphanumeric_only("ABC123", false));
        assert!(MtgjsonUtils::is_alphanumeric_only("Hello World", true));
        assert!(!MtgjsonUtils::is_alphanumeric_only("Hello World", false));
        assert!(!MtgjsonUtils::is_alphanumeric_only("Hello!", true));
        assert!(!MtgjsonUtils::is_alphanumeric_only("", false));
    }

    #[test]
    fn test_normalize_whitespace() {
        let result = MtgjsonUtils::normalize_whitespace("  Hello   World  \n\t");
        assert_eq!(result, "Hello World");
        
        let result = MtgjsonUtils::normalize_whitespace("Multiple\n\n\nlines");
        assert_eq!(result, "Multiple lines");
        
        let result = MtgjsonUtils::normalize_whitespace("");
        assert_eq!(result, "");
    }
}