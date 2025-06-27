use crate::base::JsonObject;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MTGJSON AtomicCards Object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(name = "MtgjsonAtomicCardsObject")]
pub struct MtgjsonAtomicCardsObject {
    #[pyo3(get, set)]
    pub atomic_cards_dict: HashMap<String, Vec<String>>,
}

#[pymethods]
impl MtgjsonAtomicCardsObject {
    #[new]
    #[pyo3(signature = (cards_data=None))]
    pub fn new(cards_data: Option<HashMap<String, Vec<String>>>) -> Self {
        Self {
            atomic_cards_dict: cards_data.unwrap_or_default(),
        }
    }
}

impl Default for MtgjsonAtomicCardsObject {
    fn default() -> Self {
        Self::new(None)
    }
}

impl JsonObject for MtgjsonAtomicCardsObject {}