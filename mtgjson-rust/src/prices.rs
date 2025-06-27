use crate::base::JsonObject;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// MTGJSON Singular Prices.Card Object
/// 
/// This struct represents price information for a specific Magic: The Gathering card
/// from a particular provider on a specific date. It contains both buylist prices
/// (what stores pay for the card) and retail prices (what stores sell the card for)
/// in different finishes (normal, foil, etched).
/// 
/// The pricing structure is designed to be flexible and comprehensive, supporting:
/// - Multiple finishing types (normal, foil, etched)
/// - Both buy and sell prices from the same provider
/// - Date-specific pricing for historical tracking
/// - Multiple currencies (though typically USD)
/// - Source attribution for transparency
/// 
/// This matches the Python MtgjsonPricesObject class structure while providing
/// additional type safety and performance optimizations through Rust's type system.
/// 
/// # Examples
/// 
/// ```rust
/// use mtgjson_rust::MtgjsonPrices;
/// 
/// let prices = MtgjsonPrices::new(
///     "paper".to_string(),
///     "tcgplayer".to_string(),
///     "2024-01-15".to_string(),
///     "USD".to_string(),
///     Some(2.50),  // buy_normal
///     Some(3.00),  // buy_foil
///     None,        // buy_etched
///     Some(3.25),  // sell_normal
///     Some(4.50),  // sell_foil
///     None,        // sell_etched
/// );
/// 
/// assert!(prices.has_price_data());
/// assert_eq!(prices.get_buy_prices().len(), 2);  // normal and foil
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[pyclass(name = "MtgjsonPrices")]
pub struct MtgjsonPrices {
    /// The source of the pricing data (e.g., "paper" for physical cards, "mtgo" for online)
    /// 
    /// This field categorizes the market segment for the pricing information:
    /// - "paper": Physical card markets (TCGPlayer, Card Kingdom, etc.)
    /// - "mtgo": Magic: The Gathering Online marketplace
    /// - "arena": MTG Arena (though prices are typically gems/wildcards)
    #[pyo3(get, set)]
    pub source: String,
    
    /// The provider/marketplace that supplied the pricing data
    /// 
    /// Common providers include:
    /// - "tcgplayer": TCGPlayer marketplace
    /// - "cardkingdom": Card Kingdom store
    /// - "cardhoarder": Card Hoarder (MTGO specialist)
    /// - "cardmarket": European Cardmarket platform
    #[pyo3(get, set)]
    pub provider: String,
    
    /// The date when this pricing data was collected (ISO 8601 format: YYYY-MM-DD)
    /// 
    /// This enables historical price tracking and ensures data freshness.
    /// All dates should be in UTC and follow ISO 8601 standard formatting.
    #[pyo3(get, set)]
    pub date: String,
    
    /// The currency code for all price values (typically "USD")
    /// 
    /// Uses ISO 4217 currency codes:
    /// - "USD": US Dollars (most common)
    /// - "EUR": Euros (for European markets)
    /// - "GBP": British Pounds
    /// Note: All prices in a single object should use the same currency
    #[pyo3(get, set)]
    pub currency: String,
    
    /// Buylist price for normal (non-foil) finish cards
    /// 
    /// This is the price that stores/dealers are willing to pay to purchase
    /// the card from customers. None indicates no buylist price available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub buy_normal: Option<f64>,
    
    /// Buylist price for foil finish cards
    /// 
    /// Foil cards typically command higher buylist prices due to their
    /// premium nature and collector appeal. None indicates no foil buylist available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub buy_foil: Option<f64>,
    
    /// Buylist price for etched finish cards
    /// 
    /// Etched foils are a special finish type introduced in recent sets.
    /// None indicates no etched buylist price available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub buy_etched: Option<f64>,
    
    /// Retail/sell price for normal (non-foil) finish cards
    /// 
    /// This is the price at which stores/dealers sell the card to customers.
    /// Typically higher than buylist prices. None indicates unavailable for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub sell_normal: Option<f64>,
    
    /// Retail/sell price for foil finish cards
    /// 
    /// Foil cards typically have significantly higher retail prices than
    /// their non-foil counterparts. None indicates foil version unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub sell_foil: Option<f64>,
    
    /// Retail/sell price for etched finish cards
    /// 
    /// Etched foils often have premium pricing between normal and traditional foil.
    /// None indicates etched version unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub sell_etched: Option<f64>,
}

#[pymethods]
impl MtgjsonPrices {
    /// Create a new MtgjsonPrices instance
    /// 
    /// Initializes a price object with all required metadata (source, provider, date, currency)
    /// and optional price values for different finishes and transaction types.
    /// 
    /// # Arguments
    /// 
    /// * `source` - The market source (e.g., "paper", "mtgo")
    /// * `provider` - The data provider (e.g., "tcgplayer", "cardkingdom")
    /// * `date` - Collection date in ISO 8601 format (YYYY-MM-DD)
    /// * `currency` - Currency code (e.g., "USD", "EUR")
    /// * `buy_normal` - Optional buylist price for normal finish
    /// * `buy_foil` - Optional buylist price for foil finish
    /// * `buy_etched` - Optional buylist price for etched finish
    /// * `sell_normal` - Optional retail price for normal finish
    /// * `sell_foil` - Optional retail price for foil finish
    /// * `sell_etched` - Optional retail price for etched finish
    /// 
    /// # Returns
    /// 
    /// A new MtgjsonPrices instance with the specified values
    /// 
    /// # Examples
    /// 
    /// ```python
    /// # Create a price entry with both buy and sell prices
    /// prices = MtgjsonPrices(
    ///     source="paper",
    ///     provider="tcgplayer", 
    ///     date="2024-01-15",
    ///     currency="USD",
    ///     buy_normal=2.50,
    ///     sell_normal=3.25,
    ///     sell_foil=6.75
    /// )
    /// ```
    #[new]
    #[pyo3(signature = (source, provider, date, currency, buy_normal = None, buy_foil = None, buy_etched = None, sell_normal = None, sell_foil = None, sell_etched = None))]
    pub fn new(
        source: String,
        provider: String,
        date: String,
        currency: String,
        buy_normal: Option<f64>,
        buy_foil: Option<f64>,
        buy_etched: Option<f64>,
        sell_normal: Option<f64>,
        sell_foil: Option<f64>,
        sell_etched: Option<f64>,
    ) -> Self {
        Self {
            source,
            provider,
            date,
            currency,
            buy_normal,
            buy_foil,
            buy_etched,
            sell_normal,
            sell_foil,
            sell_etched,
        }
    }

    /// Get all price items as tuples for iteration compatibility
    /// 
    /// Returns a vector of tuples containing field names and their optional numeric values.
    /// This method provides compatibility with Python's dict-like iteration behavior
    /// while maintaining type safety for numeric values.
    /// 
    /// Note: String fields (source, provider, date, currency) are returned with None
    /// as their "value" since they're not numeric prices.
    /// 
    /// # Returns
    /// 
    /// A Vec of tuples where each tuple contains (field_name, optional_price_value)
    /// 
    /// # Examples
    /// 
    /// ```python
    /// prices = MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD", 
    ///                       buy_normal=2.50, sell_normal=3.25)
    /// 
    /// for field_name, price_value in prices.items():
    ///     if price_value is not None:
    ///         print(f"{field_name}: ${price_value}")
    /// # Output: buy_normal: $2.50, sell_normal: $3.25
    /// ```
    pub fn items(&self) -> Vec<(String, Option<f64>)> {
        vec![
            ("source".to_string(), None), // String fields don't have numeric values
            ("provider".to_string(), None),
            ("date".to_string(), None),
            ("currency".to_string(), None),
            ("buy_normal".to_string(), self.buy_normal),
            ("buy_foil".to_string(), self.buy_foil),
            ("buy_etched".to_string(), self.buy_etched),
            ("sell_normal".to_string(), self.sell_normal),
            ("sell_foil".to_string(), self.sell_foil),
            ("sell_etched".to_string(), self.sell_etched),
        ]
    }

    /// Convert to JSON string representation
    /// 
    /// Note: This differs from the Python version's to_json() method which returns
    /// a complex nested structure. The Rust version returns a simple JSON string
    /// for the object. Use to_json_structure() if you need the Python-compatible
    /// nested format.
    /// 
    /// # Returns
    /// 
    /// A JSON string representation of the price object
    /// 
    /// # Errors
    /// 
    /// Returns a PyValueError if serialization fails
    /// 
    /// # Examples
    /// 
    /// ```python
    /// prices = MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD", buy_normal=2.50)
    /// json_str = prices.to_json()
    /// print(json_str)  # {"source":"paper","provider":"tcgplayer",...}
    /// ```
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Serialization error: {}", e))
        })
    }

    /// Convert to the complex JSON structure expected by MTGJSON format
    /// 
    /// Creates a simplified representation of the pricing data as a JSON string.
    /// This method exists for basic compatibility but doesn't fully replicate
    /// the Python version's complex nested structure with buylist/retail/date hierarchy.
    /// 
    /// For full Python compatibility, this would need to return:
    /// ```json
    /// {
    ///   "source": {
    ///     "provider": {
    ///       "buylist": {"normal": {"date": price}, "foil": {"date": price}},
    ///       "retail": {"normal": {"date": price}, "foil": {"date": price}},
    ///       "currency": "USD"
    ///     }
    ///   }
    /// }
    /// ```
    /// 
    /// # Returns
    /// 
    /// A JSON string with basic price information
    /// 
    /// # Examples
    /// 
    /// ```python
    /// prices = MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD", 
    ///                       buy_normal=2.50, sell_foil=6.75)
    /// structure = prices.to_json_structure()
    /// ```
    pub fn to_json_structure(&self) -> String {
        let mut buy_sell_option: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        
        if let Some(ref buy_normal) = self.buy_normal {
            buy_sell_option.insert("buy_normal".to_string(), format!("{}", buy_normal));
        }
        if let Some(ref buy_foil) = self.buy_foil {
            buy_sell_option.insert("buy_foil".to_string(), format!("{}", buy_foil));
        }
        if let Some(ref buy_etched) = self.buy_etched {
            buy_sell_option.insert("buy_etched".to_string(), format!("{}", buy_etched));
        }
        if let Some(ref sell_normal) = self.sell_normal {
            buy_sell_option.insert("sell_normal".to_string(), format!("{}", sell_normal));
        }
        if let Some(ref sell_foil) = self.sell_foil {
            buy_sell_option.insert("sell_foil".to_string(), format!("{}", sell_foil));
        }
        if let Some(ref sell_etched) = self.sell_etched {
            buy_sell_option.insert("sell_etched".to_string(), format!("{}", sell_etched));
        }

        serde_json::to_string(&buy_sell_option).unwrap_or_default()
    }

    /// Check if this price entry has any actual price data
    /// 
    /// Returns true if at least one price field (buy or sell, any finish) contains a value.
    /// This is useful for filtering out empty price entries that only contain metadata.
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether any price data is present
    /// 
    /// # Examples
    /// 
    /// ```python
    /// # Empty price entry (metadata only)
    /// empty_prices = MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD")
    /// assert not empty_prices.has_price_data()
    /// 
    /// # Price entry with data
    /// prices_with_data = MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD", 
    ///                                  sell_normal=3.25)
    /// assert prices_with_data.has_price_data()
    /// ```
    pub fn has_price_data(&self) -> bool {
        self.buy_normal.is_some() ||
        self.buy_foil.is_some() ||
        self.buy_etched.is_some() ||
        self.sell_normal.is_some() ||
        self.sell_foil.is_some() ||
        self.sell_etched.is_some()
    }

    /// Get all buylist prices by finish type
    /// 
    /// Returns a HashMap containing only the buylist prices that are available,
    /// with finish type as key ("normal", "foil", "etched") and price as value.
    /// 
    /// # Returns
    /// 
    /// HashMap with finish types as keys and buylist prices as values
    /// 
    /// # Examples
    /// 
    /// ```python
    /// prices = MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD",
    ///                       buy_normal=2.50, buy_foil=4.00, sell_normal=3.25)
    /// 
    /// buy_prices = prices.get_buy_prices()
    /// # Returns: {"normal": 2.50, "foil": 4.00}
    /// 
    /// print(f"Normal buylist: ${buy_prices.get('normal', 0)}")
    /// print(f"Foil buylist: ${buy_prices.get('foil', 0)}")
    /// ```
    pub fn get_buy_prices(&self) -> HashMap<String, f64> {
        let mut prices = HashMap::new();
        
        if let Some(price) = self.buy_normal {
            prices.insert("normal".to_string(), price);
        }
        if let Some(price) = self.buy_foil {
            prices.insert("foil".to_string(), price);
        }
        if let Some(price) = self.buy_etched {
            prices.insert("etched".to_string(), price);
        }
        
        prices
    }

    /// Get all retail/sell prices by finish type
    /// 
    /// Returns a HashMap containing only the retail prices that are available,
    /// with finish type as key ("normal", "foil", "etched") and price as value.
    /// 
    /// # Returns
    /// 
    /// HashMap with finish types as keys and retail prices as values
    /// 
    /// # Examples
    /// 
    /// ```python
    /// prices = MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD",
    ///                       buy_normal=2.50, sell_normal=3.25, sell_foil=6.75)
    /// 
    /// sell_prices = prices.get_sell_prices()
    /// # Returns: {"normal": 3.25, "foil": 6.75}
    /// 
    /// print(f"Normal retail: ${sell_prices.get('normal', 0)}")
    /// print(f"Foil retail: ${sell_prices.get('foil', 0)}")
    /// ```
    pub fn get_sell_prices(&self) -> HashMap<String, f64> {
        let mut prices = HashMap::new();
        
        if let Some(price) = self.sell_normal {
            prices.insert("normal".to_string(), price);
        }
        if let Some(price) = self.sell_foil {
            prices.insert("foil".to_string(), price);
        }
        if let Some(price) = self.sell_etched {
            prices.insert("etched".to_string(), price);
        }
        
        prices
    }

    /// Calculate the spread (difference) between buy and sell prices for a given finish
    /// 
    /// The spread represents the profit margin for dealers/stores. A larger spread
    /// indicates higher profit margins, while a smaller spread suggests a more
    /// competitive market.
    /// 
    /// # Arguments
    /// 
    /// * `finish` - The finish type ("normal", "foil", or "etched")
    /// 
    /// # Returns
    /// 
    /// Optional spread value (sell_price - buy_price), or None if either price is missing
    /// 
    /// # Examples
    /// 
    /// ```python
    /// prices = MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD",
    ///                       buy_normal=2.50, sell_normal=3.25)
    /// 
    /// spread = prices.get_spread("normal")
    /// # Returns: Some(0.75)  # $3.25 - $2.50 = $0.75 spread
    /// 
    /// if let Some(spread_value) = spread:
    ///     print(f"Dealer margin: ${spread_value:.2}")
    /// ```
    pub fn get_spread(&self, finish: &str) -> Option<f64> {
        match finish {
            "normal" => {
                if let (Some(sell), Some(buy)) = (self.sell_normal, self.buy_normal) {
                    Some(sell - buy)
                } else {
                    None
                }
            }
            "foil" => {
                if let (Some(sell), Some(buy)) = (self.sell_foil, self.buy_foil) {
                    Some(sell - buy)
                } else {
                    None
                }
            }
            "etched" => {
                if let (Some(sell), Some(buy)) = (self.sell_etched, self.buy_etched) {
                    Some(sell - buy)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get the count of available price points
    /// 
    /// Returns the total number of individual prices (buy/sell combinations)
    /// that have values. Useful for determining the completeness of price data.
    /// 
    /// # Returns
    /// 
    /// The count of non-None price fields (0-6)
    /// 
    /// # Examples
    /// 
    /// ```python
    /// prices = MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD",
    ///                       buy_normal=2.50, sell_normal=3.25, sell_foil=6.75)
    /// 
    /// count = prices.get_price_count()
    /// # Returns: 3 (buy_normal, sell_normal, sell_foil)
    /// 
    /// print(f"Available prices: {count} out of 6 possible")
    /// ```
    pub fn get_price_count(&self) -> usize {
        let prices = [
            self.buy_normal,
            self.buy_foil,
            self.buy_etched,
            self.sell_normal,
            self.sell_foil,
            self.sell_etched,
        ];
        
        prices.iter().filter(|price| price.is_some()).count()
    }
}

impl JsonObject for MtgjsonPrices {}