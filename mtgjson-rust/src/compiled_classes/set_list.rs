use crate::classes::base::JsonObject;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MTGJSON SetList Object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(name = "MtgjsonSetListObject")]
pub struct MtgjsonSetListObject {
    #[pyo3(get, set)]
    pub set_list: Vec<HashMap<String, String>>,
}

#[pymethods]
impl MtgjsonSetListObject {
    #[new]
    pub fn new() -> Self {
        Self {
            set_list: Vec::new(),
        }
    }
}

impl Default for MtgjsonSetListObject {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonObject for MtgjsonSetListObject {}