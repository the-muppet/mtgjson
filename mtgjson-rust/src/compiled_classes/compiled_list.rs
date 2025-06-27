use crate::classes::base::JsonObject;
use crate::compiled_classes::structures::MtgjsonStructuresObject;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// MTGJSON CompiledList Object
/// Rust equivalent of MtgjsonCompiledListObjectObject
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass(name = "MtgjsonCompiledListObject")]
pub struct MtgjsonCompiledListObject {
    #[pyo3(get, set)]
    pub files: Vec<String>,
}

#[pymethods]
impl MtgjsonCompiledListObject {
    #[new]
    pub fn new() -> Self {
        let structures = MtgjsonStructuresObject::new();
        let files = structures.get_compiled_list_files();
        
        Self { files }
    }

    /// Create from custom file list
    #[staticmethod]
    pub fn from_files(files: Vec<String>) -> Self {
        let mut sorted_files = files;
        sorted_files.sort();
        Self { files: sorted_files }
    }

    /// Add a file to the list
    pub fn add_file(&mut self, file_name: String) {
        if !self.files.contains(&file_name) {
            self.files.push(file_name);
            self.files.sort();
        }
    }

    /// Remove a file from the list
    pub fn remove_file(&mut self, file_name: &str) -> bool {
        if let Some(pos) = self.files.iter().position(|x| x == file_name) {
            self.files.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if a file is in the list
    pub fn contains_file(&self, file_name: &str) -> bool {
        self.files.contains(&file_name.to_string())
    }

    /// Get file count
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.files).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Serialization error: {}", e))
        })
    }

    /// Get the files list (for JSON serialization)
    pub fn get_files(&self) -> Vec<String> {
        self.files.clone()
    }
}

impl Default for MtgjsonCompiledListObject {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonObject for MtgjsonCompiledListObject {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiled_list_creation() {
        let compiled_list = MtgjsonCompiledListObject::new();
        assert!(!compiled_list.files.is_empty());
        assert!(compiled_list.contains_file("AllPrintings"));
        assert!(compiled_list.contains_file("AtomicCards"));
    }

    #[test]
    fn test_add_remove_files() {
        let mut compiled_list = MtgjsonCompiledListObject::new();
        let initial_count = compiled_list.file_count();
        
        compiled_list.add_file("TestFile".to_string());
        assert_eq!(compiled_list.file_count(), initial_count + 1);
        assert!(compiled_list.contains_file("TestFile"));
        
        assert!(compiled_list.remove_file("TestFile"));
        assert_eq!(compiled_list.file_count(), initial_count);
        assert!(!compiled_list.contains_file("TestFile"));
    }

    #[test]
    fn test_from_files() {
        let files = vec!["FileB".to_string(), "FileA".to_string(), "FileC".to_string()];
        let compiled_list = MtgjsonCompiledListObject::from_files(files);
        
        assert_eq!(compiled_list.files, vec!["FileA", "FileB", "FileC"]);
    }
}