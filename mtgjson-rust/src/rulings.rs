use crate::base::JsonObject;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// MTGJSON Singular Card.Rulings Object
/// 
/// This struct represents a single ruling or clarification for a Magic: The Gathering card.
/// Rulings provide official clarifications from Wizards of the Coast about how cards
/// interact with game rules, other cards, or specific situations.
/// 
/// Each ruling contains:
/// - A date when the ruling was issued or updated
/// - The text of the ruling explaining the card interaction or clarification
/// 
/// Rulings are essential for understanding complex card interactions and are used by
/// judges, players, and deck builders to ensure correct gameplay.
/// 
/// This struct matches the Python MtgjsonRulingObject class while providing
/// additional type safety and performance optimizations through Rust's type system.
/// 
/// # Examples
/// 
/// ```rust
/// use mtgjson_rust::MtgjsonRuling;
/// 
/// let ruling = MtgjsonRuling::new(
///     "2021-06-18".to_string(),
///     "If Lightning Bolt targets a creature, that creature takes 3 damage.".to_string()
/// );
/// 
/// assert!(ruling.is_valid());
/// assert_eq!(ruling.date, "2021-06-18");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[pyclass(name = "MtgjsonRuling")]
pub struct MtgjsonRuling {
    /// The date when this ruling was issued or last updated
    /// 
    /// Format should be ISO 8601 (YYYY-MM-DD) for consistency with other MTGJSON dates.
    /// This represents when Wizards of the Coast issued or updated the ruling.
    /// 
    /// # Examples
    /// - "2021-06-18" - June 18, 2021
    /// - "2020-01-24" - January 24, 2020
    #[pyo3(get, set)]
    pub date: String,
    
    /// The text content of the ruling
    /// 
    /// Contains the official ruling text explaining card interactions, clarifications,
    /// or specific situations. This text is sourced from Wizards of the Coast's
    /// official rulings database (formerly Gatherer, now integrated into other systems).
    /// 
    /// The text should be complete and self-contained, providing clear guidance
    /// on the card's behavior in the described situation.
    #[pyo3(get, set)]
    pub text: String,
}

#[pymethods]
impl MtgjsonRuling {
    /// Create a new MtgjsonRuling instance
    /// 
    /// Initializes a ruling with the specified date and text content.
    /// Both parameters are required as they represent the essential
    /// components of any ruling.
    /// 
    /// # Arguments
    /// 
    /// * `date` - The ruling date in ISO 8601 format (YYYY-MM-DD)
    /// * `text` - The ruling text content
    /// 
    /// # Returns
    /// 
    /// A new MtgjsonRuling instance with the specified date and text
    /// 
    /// # Examples
    /// 
    /// ```python
    /// ruling = MtgjsonRuling(
    ///     date="2021-06-18",
    ///     text="If Lightning Bolt targets a creature, that creature takes 3 damage."
    /// )
    /// ```
    #[new]
    pub fn new(date: String, text: String) -> Self {
        Self { date, text }
    }

    /// Convert the ruling to a JSON string representation
    /// 
    /// Serializes the ruling object to JSON format, which can be used for
    /// storage, transmission, or integration with other systems.
    /// 
    /// # Returns
    /// 
    /// A JSON string representation of the ruling
    /// 
    /// # Errors
    /// 
    /// Returns a PyValueError if serialization fails (though this is rare for simple structs)
    /// 
    /// # Examples
    /// 
    /// ```python
    /// ruling = MtgjsonRuling("2021-06-18", "Card ruling text here")
    /// json_str = ruling.to_json()
    /// print(json_str)  # {"date":"2021-06-18","text":"Card ruling text here"}
    /// ```
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Serialization error: {}", e))
        })
    }

    /// Check if the ruling is valid (contains both date and text)
    /// 
    /// A ruling is considered valid if both the date and text fields contain
    /// non-empty values. This is useful for data validation and filtering.
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether the ruling has valid content
    /// 
    /// # Examples
    /// 
    /// ```python
    /// # Valid ruling
    /// valid_ruling = MtgjsonRuling("2021-06-18", "This is a ruling")
    /// assert valid_ruling.is_valid()
    /// 
    /// # Invalid ruling (empty text)
    /// invalid_ruling = MtgjsonRuling("2021-06-18", "")
    /// assert not invalid_ruling.is_valid()
    /// 
    /// # Invalid ruling (empty date)
    /// invalid_ruling2 = MtgjsonRuling("", "This is a ruling")
    /// assert not invalid_ruling2.is_valid()
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.date.is_empty() && !self.text.is_empty()
    }

    /// Get a summary of the ruling (first 100 characters of text)
    /// 
    /// Returns a truncated version of the ruling text for display purposes,
    /// such as in lists or previews where space is limited. The full text
    /// remains available in the `text` field.
    /// 
    /// # Returns
    /// 
    /// A string containing up to 100 characters of the ruling text,
    /// with "..." appended if truncated
    /// 
    /// # Examples
    /// 
    /// ```python
    /// long_ruling = MtgjsonRuling(
    ///     "2021-06-18",
    ///     "This is a very long ruling that explains complex interactions between multiple cards and various game states in great detail."
    /// )
    /// 
    /// summary = long_ruling.get_summary()
    /// print(summary)  # "This is a very long ruling that explains complex interactions between multiple cards and va..."
    /// 
    /// short_ruling = MtgjsonRuling("2021-06-18", "Short ruling")
    /// print(short_ruling.get_summary())  # "Short ruling"
    /// ```
    pub fn get_summary(&self) -> String {
        if self.text.len() <= 100 {
            self.text.clone()
        } else {
            format!("{}...", &self.text[..97])
        }
    }

    /// Compare rulings by date for sorting purposes
    /// 
    /// Compares two rulings based on their date strings. This is useful for
    /// chronological sorting of ruling collections. Uses string comparison
    /// which works correctly for ISO 8601 date format (YYYY-MM-DD).
    /// 
    /// # Arguments
    /// 
    /// * `other` - Another MtgjsonRuling to compare against
    /// 
    /// # Returns
    /// 
    /// An integer indicating the comparison result:
    /// - `-1` if this ruling's date is earlier than the other
    /// - `0` if both rulings have the same date  
    /// - `1` if this ruling's date is later than the other
    /// 
    /// # Examples
    /// 
    /// ```python
    /// ruling1 = MtgjsonRuling("2021-01-15", "First ruling")
    /// ruling2 = MtgjsonRuling("2021-06-18", "Second ruling")
    /// 
    /// comparison = ruling1.compare_by_date(ruling2)
    /// print(comparison)  # -1 (ruling1 is earlier)
    /// 
    /// comparison = ruling2.compare_by_date(ruling1)
    /// print(comparison)  # 1 (ruling2 is later)
    /// 
    /// ruling3 = MtgjsonRuling("2021-01-15", "Same date")
    /// comparison = ruling1.compare_by_date(ruling3)
    /// print(comparison)  # 0 (same date)
    /// ```
    pub fn compare_by_date(&self, other: &MtgjsonRuling) -> i32 {
        match self.date.cmp(&other.date) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    /// Get the word count of the ruling text
    /// 
    /// Counts the number of words in the ruling text, which can be useful
    /// for analysis, display formatting, or content categorization.
    /// Words are separated by whitespace.
    /// 
    /// # Returns
    /// 
    /// The number of words in the ruling text
    /// 
    /// # Examples
    /// 
    /// ```python
    /// ruling = MtgjsonRuling("2021-06-18", "This ruling has five words")
    /// word_count = ruling.get_word_count()
    /// print(word_count)  # 5
    /// 
    /// empty_ruling = MtgjsonRuling("2021-06-18", "")
    /// print(empty_ruling.get_word_count())  # 0
    /// ```
    pub fn get_word_count(&self) -> usize {
        if self.text.is_empty() {
            0
        } else {
            self.text.split_whitespace().count()
        }
    }

    /// Check if the ruling text contains a specific keyword or phrase
    /// 
    /// Performs a case-insensitive search for a keyword or phrase within
    /// the ruling text. This is useful for filtering or categorizing rulings
    /// based on their content.
    /// 
    /// # Arguments
    /// 
    /// * `keyword` - The keyword or phrase to search for
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether the keyword was found in the ruling text
    /// 
    /// # Examples
    /// 
    /// ```python
    /// ruling = MtgjsonRuling("2021-06-18", "This creature enters the battlefield tapped")
    /// 
    /// assert ruling.contains_keyword("creature")
    /// assert ruling.contains_keyword("BATTLEFIELD")  # Case insensitive
    /// assert ruling.contains_keyword("enters the battlefield")  # Phrase search
    /// assert not ruling.contains_keyword("graveyard")
    /// ```
    pub fn contains_keyword(&self, keyword: &str) -> bool {
        self.text.to_lowercase().contains(&keyword.to_lowercase())
    }

    /// Get the character count of the ruling text
    /// 
    /// Returns the total number of characters in the ruling text,
    /// including spaces and punctuation. This can be useful for
    /// display formatting or content analysis.
    /// 
    /// # Returns
    /// 
    /// The number of characters in the ruling text
    /// 
    /// # Examples
    /// 
    /// ```python
    /// ruling = MtgjsonRuling("2021-06-18", "Hello")
    /// char_count = ruling.get_character_count()
    /// print(char_count)  # 5
    /// ```
    pub fn get_character_count(&self) -> usize {
        self.text.len()
    }
}

impl JsonObject for MtgjsonRuling {}