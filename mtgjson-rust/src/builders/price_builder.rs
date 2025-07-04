// MTGJSON price builder - price data processing and compression
use chrono::{DateTime, Utc, Duration};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use crate::config::get_config;
use crate::providers::{
    CardHoarderProvider, CardKingdomProvider, CardMarketProvider, 
    MultiverseBridgeProvider, TCGPlayerProvider
};

/// MTGJSON Price Builder - Exact Python API compatibility
#[derive(Debug)]
#[pyclass(name = "PriceBuilder")]
pub struct PriceBuilder {
    #[pyo3(get, set)]
    pub providers: Vec<PyObject>, // List of AbstractProvider instances
    #[pyo3(get, set)]
    pub all_printings_path: Option<PathBuf>,
}

#[pymethods]
impl PriceBuilder {
    #[new]
    #[pyo3(signature = (*_args, all_printings_path=None))]
    pub fn new(_args: &Bound<'_, PyTuple>, all_printings_path: Option<PathBuf>) -> Self {
        // Convert providers tuple to Vec<PyObject>
        let provider_list = if _args.len() == 0 {
            // Default providers (would be actual provider instances in real implementation)
            vec![]
        } else {
            Python::with_gil(|py| {
                _args.iter().map(|item: Bound<'_, PyAny>| item.unbind()).collect::<Vec<_>>()
            })
            .unwrap() // Convert Vec<PyObject> to Bound<'_, PyList>
        };
        Self {
            providers: provider_list,
            all_printings_path,
        }
    }

    /// Build today's prices from upstream sources and combine them together
    /// Returns: Dict[str, Any] - Today's prices to be merged into archive
    pub fn build_today_prices(&self) -> PyResult<HashMap<String, Value>> {
        let mut final_results = HashMap::new();

        // Check if AllPrintings exists
        if let Some(ref path) = self.all_printings_path {
            if !path.exists() {
                return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(
                    format!(
                        "Unable to build prices. AllPrintings not found in {:?}",
                        path
                    ),
                ));
            }
        } else {
            let config = get_config();
            let default_path = config.get_output_path().join("AllPrintings.json");
            if !default_path.exists() {
                return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(
                    format!(
                        "Unable to build prices. AllPrintings not found in {}",
                        default_path.display()
                    ),
                ));
            }
        }

        // Generate prices from each provider
        if self.providers.is_empty() {
            // Use default providers if none specified
            let default_providers = vec![
                "CardHoarder",
                "TCGPlayer", 
                "CardMarket",
                "CardKingdom",
                "MultiverseBridge"
            ];

            for provider_name in default_providers {
                match self.generate_prices_for_provider(provider_name) {
                    Ok(provider_prices) => {
                        self.merge_price_data(&mut final_results, provider_prices);
                    }
                    Err(e) => {
                        eprintln!("Failed to compile for {} with error: {}", provider_name, e);
                    }
                }
            }
        } else {
            // Use provided providers
            Python::with_gil(|py| {
                for provider in &self.providers {
                    match provider.call_method1(
                        py,
                        "generate_today_price_dict",
                        (self.all_printings_path.as_ref(),),
                    ) {
                        Ok(provider_result) => {
                            if let Ok(json_str) = provider_result.extract::<String>(py) {
                                if let Ok(provider_data) = serde_json::from_str::<Value>(&json_str) {
                                    if let Some(provider_map) = provider_data.as_object() {
                                        for (key, value) in provider_map {
                                            final_results.insert(key.clone(), value.clone());
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Provider failed to generate prices: {}", e);
                        }
                    }
                }
            });
        }

        if final_results.is_empty() {
            eprintln!("Warning: No price data generated from any provider");
        }

        Ok(final_results)
    }

    /// The full build prices operation - Prune & Update remote database
    /// Returns: Tuple[Dict[str, Any], Dict[str, Any]] - (archive_prices, today_prices)
    pub fn build_prices(&self) -> PyResult<(HashMap<String, Value>, HashMap<String, Value>)> {
        println!("Prices Build - Building Prices");

        // Check if AllPrintings.json exists, download if necessary
        let all_printings_path = self.all_printings_path.as_ref()
            .unwrap_or(&get_config().get_output_path().join("AllPrintings.json"));
        
        if !all_printings_path.exists() {
            println!("AllPrintings not found, attempting to download");
            self.download_old_all_printings()?;
        }

        // Get today's price database
        println!("Building new price data");
        let today_prices = self.build_today_prices()?;
        
        if today_prices.is_empty() {
            eprintln!("Warning: Pricing information failed to generate");
            return Ok((HashMap::new(), HashMap::new()));
        }

        let config = get_config();
        
        // Check if we have price configuration
        if !config.has_section("Prices") {
            return Ok((today_prices.clone(), today_prices));
        }

        // Get bucket configuration
        let bucket_name = config.get("Prices", "bucket_name")
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing bucket_name in Prices config"))?;
        let bucket_object_path = config.get("Prices", "bucket_object_path")
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing bucket_object_path in Prices config"))?;

        // Download and merge with archive
        let mut archive_prices = Self::get_price_archive_data(bucket_name.clone(), bucket_object_path.clone())?;

        // Update local copy of database
        println!("Merging old and new price data");
        self.merge_price_data(&mut archive_prices, today_prices.clone());

        // Prune local copy of database
        println!("Pruning price data");
        Self::prune_prices_archive_static(&mut archive_prices, 3)?;

        // Compress and upload
        println!("Compressing price data");
        let cache_path = config.get_cache_path();
        fs::create_dir_all(&cache_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let local_zip_file = cache_path.join(&bucket_object_path);
        Self::write_price_archive_data_static(local_zip_file.clone(), &archive_prices)?;

        // Upload to S3 (placeholder - would need AWS SDK)
        println!("Uploading price data to S3");
        // TODO: Implement actual S3 upload using AWS SDK

        // Clean up local file
        if local_zip_file.exists() {
            fs::remove_file(&local_zip_file)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        }

        Ok((archive_prices, today_prices))
    }

    /// Prune entries from the MTGJSON database that are older than `months` old
    #[staticmethod]
    #[pyo3(signature = (_content, months=3))]
    pub fn prune_prices_archive(_content: Bound<'_, PyDict>, months: i32) -> PyResult<()> {
        Python::with_gil(|py| {
            // Convert PyDict to Rust structure, prune, and update
            let mut rust_data: HashMap<String, Value> = HashMap::new();
            
            // Extract data from PyDict
            for (key, value) in _content.iter() {
                let key_str = key.extract::<String>()?;
                let value_str = value.str()?.extract::<String>()?;
                if let Ok(parsed_value) = serde_json::from_str::<Value>(&value_str) {
                    rust_data.insert(key_str, parsed_value);
                }
            }
            
            // Prune the data
            Self::prune_prices_archive_static(&mut rust_data, months)?;
            
            // Update the original PyDict
            _content.clear();
            for (key, value) in rust_data {
                let value_str = serde_json::to_string(&value)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
                _content.set_item(key, value_str)?;
            }
            
            Ok(())
        })
    }

    /// Download compiled MTGJSON price data from S3/remote storage
    #[staticmethod]
    pub fn get_price_archive_data(
        bucket_name: String,
        bucket_object_path: String,
    ) -> PyResult<HashMap<String, Value>> {
        println!("Downloading Current Price Data File from S3");
        
        let config = get_config();
        let cache_path = config.get_cache_path();
        fs::create_dir_all(&cache_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let temp_zip_file = cache_path.join("temp.tar.xz");
        
        // TODO: Implement actual S3 download using AWS SDK
        // For now, create an empty file or return empty data
        if !temp_zip_file.exists() {
            eprintln!("Warning: Download of current price data failed - no S3 implementation yet");
            return Ok(HashMap::new());
        }
        
        // Decompress and read the file
        let output = Command::new("xz")
            .arg("-d")
            .arg("-c")
            .arg(&temp_zip_file)
            .output()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Failed to decompress file: {}", e)
            ))?;
        
        if !output.status.success() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Decompression failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }
        
        let content: HashMap<String, Value> = serde_json::from_slice(&output.stdout)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        
        // Clean up temp file
        if temp_zip_file.exists() {
            fs::remove_file(&temp_zip_file)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        }
        
        Ok(content)
    }

    /// Write price data to a compressed archive file (xz format)
    #[staticmethod]
    pub fn write_price_archive_data(
        local_save_path: PathBuf,
        price_data: Bound<'_, PyDict>,
    ) -> PyResult<()> {
        // Convert PyDict to Rust structure
        let mut rust_data: HashMap<String, Value> = HashMap::new();
        for (key, value) in price_data.iter() {
            let key_str = key.extract::<String>()?;
            let value_str = value.str()?.extract::<String>()?;
            if let Ok(parsed_value) = serde_json::from_str::<Value>(&value_str) {
                rust_data.insert(key_str, parsed_value);
            }
        }
        
        Self::write_price_archive_data_static(local_save_path, &rust_data)
    }

    /// Download the hosted version of AllPrintings from MTGJSON for future consumption
    pub fn download_old_all_printings(&self) -> PyResult<()> {
        println!("Downloading AllPrintings.json from MTGJSON");
        
        // Use reqwest or similar HTTP client (placeholder for now)
        // This would implement:
        // 1. HTTP download from https://mtgjson.com/api/v5/AllPrintings.json.xz
        // 2. XZ decompression using Command::new("xz") or lzma crate
        // 3. Writing to self.all_printings_path
        
        let url = "https://mtgjson.com/api/v5/AllPrintings.json.xz";
        let output_path = self.all_printings_path.as_ref()
            .unwrap_or(&get_config().get_output_path().join("AllPrintings.json"));
        
        // Create output directory
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        }
        
        // Download using curl (placeholder - would use proper HTTP client in production)
        let temp_file = output_path.with_extension("json.xz");
        
        let download_result = Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(&temp_file)
            .arg(url)
            .output();
        
        match download_result {
            Ok(output) if output.status.success() => {
                // Decompress the file
                let decompress_result = Command::new("xz")
                    .arg("-d")
                    .arg("-c")
                    .arg(&temp_file)
                    .output()
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                        format!("Failed to decompress: {}", e)
                    ))?;
                
                if decompress_result.status.success() {
                    fs::write(output_path, &decompress_result.stdout)
                        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
                    
                    // Clean up compressed file
                    if temp_file.exists() {
                        fs::remove_file(&temp_file)
                            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
                    }
                    
                    println!("Successfully downloaded and decompressed AllPrintings.json");
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                        format!("Decompression failed: {}", String::from_utf8_lossy(&decompress_result.stderr))
                    ));
                }
            }
            Ok(output) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Download failed: {}", String::from_utf8_lossy(&output.stderr))
                ));
            }
            Err(e) => {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Failed to execute curl: {}", e)
                ));
            }
        }
        
        Ok(())
    }
}

impl PriceBuilder {
    /// Helper method to generate prices from a provider by name
    fn generate_prices_for_provider(&self, provider_name: &str) -> PyResult<HashMap<String, Value>> {
        println!("Generating prices from provider: {}", provider_name);
        
        // Placeholder implementation - would integrate with actual provider APIs
        let mut prices = HashMap::new();
        
        // In real implementation, would:
        // 1. Create provider instance
        // 2. Call generate_today_price_dict method
        // 3. Return parsed JSON data
        
        match provider_name {
            "CardHoarder" => {
                // Placeholder - would call CardHoarderProvider
                prices.insert("cardhoarder".to_string(), json!({}));
            }
            "TCGPlayer" => {
                // Placeholder - would call TCGPlayerProvider
                prices.insert("tcgplayer".to_string(), json!({}));
            }
            "CardMarket" => {
                // Placeholder - would call CardMarketProvider
                prices.insert("cardmarket".to_string(), json!({}));
            }
            "CardKingdom" => {
                // Placeholder - would call CardKingdomProvider
                prices.insert("cardkingdom".to_string(), json!({}));
            }
            "MultiverseBridge" => {
                // Placeholder - would call MultiverseBridgeProvider
                prices.insert("multiverse_bridge".to_string(), json!({}));
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Unknown provider: {}", provider_name)
                ));
            }
        }
        
        Ok(prices)
    }

    /// Helper method to merge price data from multiple providers
    fn merge_price_data(&self, target: &mut HashMap<String, Value>, source: HashMap<String, Value>) {
        for (key, value) in source {
            // Deep merge logic - for now, simple overwrite
            // In real implementation, would do deep merge of nested objects
            target.insert(key, value);
        }
    }

    /// Static version of prune_prices_archive for internal use
    fn prune_prices_archive_static(content: &mut HashMap<String, Value>, months: i32) -> PyResult<()> {
        let prune_date = Utc::now() - Duration::days(months as i64 * 30);
        let cutoff_str = prune_date.format("%Y-%m-%d").to_string();
        let mut keys_pruned = 0;

        fn prune_recursive(obj: &mut Value, depth: i32, cutoff: &str, keys_pruned: &mut i32) {
            if depth == 5 {
                // At the date level, remove old entries
                if let Some(obj_map) = obj.as_object_mut() {
                    let keys_to_remove: Vec<String> = obj_map.keys()
                        .filter(|&date| date < cutoff)
                        .cloned()
                        .collect();
                    
                    for key in keys_to_remove {
                        obj_map.remove(&key);
                        *keys_pruned += 1;
                    }
                }
            } else if let Some(obj_map) = obj.as_object_mut() {
                let keys_to_remove: Vec<String> = obj_map.iter()
                    .filter_map(|(key, value)| {
                        if let Some(inner_map) = value.as_object() {
                            if inner_map.is_empty() {
                                Some(key.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                
                // First prune recursively
                for value in obj_map.values_mut() {
                    prune_recursive(value, depth + 1, cutoff, keys_pruned);
                }
                
                // Then remove empty objects
                for key in keys_to_remove {
                    obj_map.remove(&key);
                    *keys_pruned += 1;
                }
            }
        }

        println!("Determining keys to prune");
        for value in content.values_mut() {
            prune_recursive(value, 0, &cutoff_str, &mut keys_pruned);
        }
        println!("Pruned {} structs", keys_pruned);

        Ok(())
    }

    /// Static version of write_price_archive_data for internal use
    fn write_price_archive_data_static(
        local_save_path: PathBuf, 
        price_data: &HashMap<String, Value>
    ) -> PyResult<()> {
        // Create parent directories
        if let Some(parent) = local_save_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        }

        let tmp_save_path = local_save_path.with_extension("");
        
        println!("Dumping price data to {:?}", tmp_save_path);
        
        // Write JSON data to temporary file
        let json_data = serde_json::to_string(price_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        
        fs::write(&tmp_save_path, json_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let file_size = tmp_save_path.metadata()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?
            .len();
        
        println!("Finished writing to {:?} (Size = {} bytes)", tmp_save_path, file_size);

        // Compress the file using xz
        println!("Compressing {:?} for upload", tmp_save_path);
        
        let compress_result = Command::new("xz")
            .arg(tmp_save_path.to_str().unwrap())
            .output()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Failed to execute xz: {}", e)
            ))?;

        if !compress_result.status.success() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Compression failed: {}", String::from_utf8_lossy(&compress_result.stderr))
            ));
        }

        let compressed_size = local_save_path.metadata()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?
            .len();
        
        println!("Finished compressing content to {:?} (Size = {} bytes)", local_save_path, compressed_size);

        Ok(())
    }
}

impl Default for PriceBuilder {
    fn default() -> Self {
        Python::with_gil(|py| {
            let empty_tuple = PyTuple::new(py, []);
            Self::new(&empty_tuple.bind(py), None)
        })
    }
}
