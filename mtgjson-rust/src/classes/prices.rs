use crate::base::{skip_if_empty_optional_string, JsonObject};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MTGJSON Singular Prices.Card Object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[pyclass(name = "MtgjsonPricesObject")]
pub struct MtgjsonPricesObject {
    #[pyo3(get, set)]
    pub source: String,
    
    #[pyo3(get, set)]
    pub provider: String,
    
    #[pyo3(get, set)]
    pub date: String,
    
    #[pyo3(get, set)]
    pub currency: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub buy_normal: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub buy_foil: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub buy_etched: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub sell_normal: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub sell_foil: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub sell_etched: Option<f64>,
}

#[pymethods]
impl MtgjsonPricesObject {
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

    /// Get all price items as tuples
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

    /// Convert to the complex JSON structure expected by MTGJSON
    pub fn to_json(&self) -> PyResult<String> {
        let result = self.to_json_structure();
        serde_json::to_string(&result).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Serialization error: {}", e))
        })
    }

    /// Convert to the complex JSON structure
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
    pub fn has_price_data(&self) -> bool {
        self.buy_normal.is_some() ||
        self.buy_foil.is_some() ||
        self.buy_etched.is_some() ||
        self.sell_normal.is_some() ||
        self.sell_foil.is_some() ||
        self.sell_etched.is_some()
    }

    /// Get all buy prices
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

    /// Get all sell prices
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
}

impl JsonObject for MtgjsonPricesObject {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prices_creation() {
        let prices = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "2023-12-31".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some(10.0),
            Some(5.0),
            Some(15.0),
            Some(8.0),
            Some(12.0),
            Some(7.0),
        );
        
        assert_eq!(prices.date, "2023-01-01");
        assert_eq!(prices.uuid, "");
        assert_eq!(prices.currency, "USD");
        assert_eq!(prices.buylist_foil, Some(10.0));
        assert_eq!(prices.buylist_normal, Some(5.0));
        assert_eq!(prices.retail_foil, Some(15.0));
        assert_eq!(prices.retail_normal, Some(8.0));
        assert_eq!(prices.selllist_foil, Some(12.0));
        assert_eq!(prices.selllist_normal, Some(7.0));
    }

    #[test]
    fn test_prices_default() {
        let prices = MtgjsonPricesObject::default();
        assert_eq!(prices.date, "");
        assert_eq!(prices.uuid, "");
        assert_eq!(prices.currency, "USD");
        assert_eq!(prices.buylist_foil, None);
        assert_eq!(prices.buylist_normal, None);
        assert_eq!(prices.retail_foil, None);
        assert_eq!(prices.retail_normal, None);
        assert_eq!(prices.selllist_foil, None);
        assert_eq!(prices.selllist_normal, None);
    }

    #[test]
    fn test_prices_from_py_dict() {
        // Create a Python dict-like structure using HashMap
        let mut py_dict = std::collections::HashMap::new();
        py_dict.insert("date".to_string(), serde_json::Value::String("2023-01-01".to_string()));
        py_dict.insert("currency".to_string(), serde_json::Value::String("EUR".to_string()));
        py_dict.insert("buylistFoil".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(10.5).unwrap()));
        py_dict.insert("retailNormal".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(8.25).unwrap()));
        
        let prices = MtgjsonPricesObject::from_py_dict(&py_dict);
        
        assert_eq!(prices.date, "2023-01-01");
        assert_eq!(prices.currency, "EUR");
        assert_eq!(prices.buylist_foil, Some(10.5));
        assert_eq!(prices.retail_normal, Some(8.25));
        assert_eq!(prices.buylist_normal, None);
    }

    #[test]
    fn test_prices_json_serialization() {
        let prices = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "2023-12-31".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some(10.0),
            Some(5.0),
            Some(15.0),
            Some(8.0),
            Some(12.0),
            Some(7.0),
        );
        
        let json_result = prices.to_json();
        assert!(json_result.is_ok());
        
        let json_string = json_result.unwrap();
        assert!(json_string.contains("2023-01-01"));
        assert!(json_string.contains("USD"));
        assert!(json_string.contains("10.0") || json_string.contains("10"));
        assert!(json_string.contains("15.0") || json_string.contains("15"));
    }

    #[test]
    fn test_prices_string_representations() {
        let mut prices = MtgjsonPricesObject::default();
        prices.date = "2023-01-01".to_string();
        prices.currency = "USD".to_string();
        
        let str_repr = prices.__str__();
        assert!(str_repr.contains("2023-01-01"));
        assert!(str_repr.contains("USD"));
        
        let repr = prices.__repr__();
        assert!(repr.contains("2023-01-01"));
        assert!(repr.contains("USD"));
    }

    #[test]
    fn test_prices_equality() {
        let prices1 = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "uuid1".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some(10.0),
            None,
            None,
            None,
            None,
            None,
        );
        
        let prices2 = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "uuid1".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some(10.0),
            None,
            None,
            None,
            None,
            None,
        );
        
        assert!(prices1.__eq__(&prices2));
        
        let mut prices3 = prices2.clone();
        prices3.buylist_foil = Some(15.0);
        assert!(!prices1.__eq__(&prices3));
    }

    #[test]
    fn test_prices_hash() {
        let mut prices = MtgjsonPricesObject::default();
        prices.date = "2023-01-01".to_string();
        prices.uuid = "test-uuid".to_string();
        
        let hash1 = prices.__hash__();
        let hash2 = prices.__hash__();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_prices_partial_data() {
        let prices = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "uuid".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some(10.0),
            None,
            Some(15.0),
            None,
            None,
            Some(7.0),
        );
        
        // Test that only provided values are set
        assert_eq!(prices.buylist_foil, Some(10.0));
        assert_eq!(prices.buylist_normal, None);
        assert_eq!(prices.retail_foil, Some(15.0));
        assert_eq!(prices.retail_normal, None);
        assert_eq!(prices.selllist_foil, None);
        assert_eq!(prices.selllist_normal, Some(7.0));
    }

    #[test]
    fn test_prices_different_currencies() {
        let usd_prices = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "uuid".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some(10.0),
            Some(5.0),
            None,
            None,
            None,
            None,
        );
        
        let eur_prices = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "uuid".to_string(),
            "EUR".to_string(),
            "EUR".to_string(),
            Some(8.5),
            Some(4.25),
            None,
            None,
            None,
            None,
        );
        
        assert_eq!(usd_prices.currency, "USD");
        assert_eq!(eur_prices.currency, "EUR");
        assert_ne!(usd_prices.buylist_foil, eur_prices.buylist_foil);
    }

    #[test]
    fn test_prices_edge_cases() {
        // Test with zero values
        let zero_prices = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "uuid".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some(0.0),
            Some(0.0),
            Some(0.0),
            Some(0.0),
            Some(0.0),
            Some(0.0),
        );
        
        assert_eq!(zero_prices.buylist_foil, Some(0.0));
        assert_eq!(zero_prices.retail_normal, Some(0.0));
        
        // Test with very large values
        let large_prices = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "uuid".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some(999999.99),
            Some(1000000.0),
            None,
            None,
            None,
            None,
        );
        
        assert_eq!(large_prices.buylist_foil, Some(999999.99));
        assert_eq!(large_prices.buylist_normal, Some(1000000.0));
    }

    #[test]
    fn test_json_object_trait() {
        let prices = MtgjsonPricesObject::default();
        let keys_to_skip = prices.build_keys_to_skip();
        
        // Keys to skip should contain uuid as it's marked as skip
        assert!(keys_to_skip.contains("uuid"));
    }

    #[test]
    fn test_prices_clone() {
        let original = MtgjsonPricesObject::new(
            "2023-01-01".to_string(),
            "uuid".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some(10.0),
            Some(5.0),
            Some(15.0),
            Some(8.0),
            Some(12.0),
            Some(7.0),
        );
        
        let cloned = original.clone();
        
        assert_eq!(original.date, cloned.date);
        assert_eq!(original.currency, cloned.currency);
        assert_eq!(original.buylist_foil, cloned.buylist_foil);
        assert_eq!(original.retail_normal, cloned.retail_normal);
    }

    #[test]
    fn test_from_py_dict_missing_fields() {
        // Test with minimal data
        let mut py_dict = std::collections::HashMap::new();
        py_dict.insert("date".to_string(), serde_json::Value::String("2023-01-01".to_string()));
        
        let prices = MtgjsonPricesObject::from_py_dict(&py_dict);
        
        assert_eq!(prices.date, "2023-01-01");
        assert_eq!(prices.currency, "USD"); // Should default to USD
        assert_eq!(prices.buylist_foil, None);
        assert_eq!(prices.retail_normal, None);
    }

    #[test]
    fn test_from_py_dict_invalid_data() {
        // Test with invalid number data
        let mut py_dict = std::collections::HashMap::new();
        py_dict.insert("date".to_string(), serde_json::Value::String("2023-01-01".to_string()));
        py_dict.insert("buylistFoil".to_string(), serde_json::Value::String("not-a-number".to_string()));
        
        let prices = MtgjsonPricesObject::from_py_dict(&py_dict);
        
        assert_eq!(prices.date, "2023-01-01");
        assert_eq!(prices.buylist_foil, None); // Should be None when conversion fails
    }
}