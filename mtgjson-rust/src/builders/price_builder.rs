// MTGJSON price builder - price data processing and compression
use chrono::{DateTime, Utc, Duration};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::{Write, Read, BufReader, BufRead};
use std::path::PathBuf;
use std::process::Command;
use reqwest;
use tokio;
use futures::StreamExt;

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
        if !self.all_printings_path.is_file() {
            eprintln!(
                "Unable to build prices. AllPrintings not found in {}",
                get_config().get_output_path().display()
            );
            return Ok(HashMap::new());
        }

        let mut final_results = HashMap::new();
        let mut provider_results = Vec::new();

        // Process each provider
        for provider in &self.providers {
            match self.generate_prices(provider) {
                Ok(prices) => provider_results.push(prices),
                Err(e) => {
                    eprintln!("Failed to generate prices: {}", e);
                }
            }
        }

        // Merge all provider results using mergedeep-like functionality
        for prices in provider_results {
            self.merge_deep(&mut final_results, prices);
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

        // Push changes to remote database
        println!("Uploading price data");
        let s3_handler = crate::s3_handler::MtgjsonS3Handler::new();
        s3_handler.upload_file(
            local_zip_file.to_string_lossy().to_string(),
            bucket_name,
            bucket_object_path,
        )?;
        
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
        println!("Downloading Current Price Data File");
        
        let config = get_config();
        let cache_path = config.get_cache_path();
        fs::create_dir_all(&cache_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let temp_zip_file = cache_path.join("temp.tar.xz");
        
        // Download file using S3 handler
        let s3_handler = crate::s3_handler::MtgjsonS3Handler::new();
        let downloaded_successfully = s3_handler.download_file(
            bucket_name,
            bucket_object_path,
            temp_zip_file.to_string_lossy().to_string(),
        )?;
        
        if !downloaded_successfully {
            eprintln!("Warning: Download of current price data failed");
            return Ok(HashMap::new());
        }
        
        // Decompress and read the file using xz2 crate
        let file = fs::File::open(&temp_zip_file)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let decoder = xz2::read::XzDecoder::new(file);
        let content: HashMap<String, Value> = serde_json::from_reader(decoder)
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
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async {
            self.download_old_all_printings_async().await
        })
    }

    async fn download_old_all_printings_async(&self) -> PyResult<()> {
        let mut file_bytes = Vec::new();
        let url = "https://mtgjson.com/api/v5/AllPrintings.json.xz";
        
        let client = reqwest::Client::new();
        let mut response = client
            .get(url)
            .send()
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyConnectionError, _>(e.to_string()))?;

        if !response.status().is_success() {
            return Err(PyErr::new::<pyo3::exceptions::PyConnectionError, _>(
                format!("HTTP error: {}", response.status())
            ));
        }

        while let Some(chunk) = response.chunk().await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyConnectionError, _>(e.to_string()))? {
            file_bytes.extend_from_slice(&chunk);
        }

        // Create output directory
        let output_path = get_config().get_output_path();
        fs::create_dir_all(&output_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        // Decompress using xz2 crate
        let decompressed = xz2::read::XzDecoder::new(&file_bytes[..]);
        let mut reader = BufReader::new(decompressed);
        let mut content = String::new();
        reader.read_to_string(&mut content)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        // Write to file
        fs::write(&self.all_printings_path, content)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        Ok(())
    }
}

impl PriceBuilder {
    /// Generate the prices for a source and prepare them for merging with other entities
    /// Equivalent to Python's _generate_prices method
    fn generate_prices(&self, provider: &PyObject) -> PyResult<HashMap<String, Value>> {
        Python::with_gil(|py| {
            match provider.call_method1(py, "generate_today_price_dict", (&self.all_printings_path,)) {
                Ok(preprocess_prices) => {
                    // Convert to JSON and back to ensure serialization like Python's json.loads(json.dumps(...))
                    let json_str = preprocess_prices.call_method0(py, "to_json")
                        .unwrap_or_else(|_| preprocess_prices.str(py).unwrap())
                        .extract::<String>(py)?;
                    
                    let final_prices: HashMap<String, Value> = serde_json::from_str(&json_str)
                        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
                    
                    Ok(final_prices)
                }
                Err(exception) => {
                    let provider_name = provider.getattr(py, "__class__")
                        .and_then(|cls| cls.getattr(py, "__name__"))
                        .and_then(|name| name.extract::<String>(py))
                        .unwrap_or_else(|_| "Unknown".to_string());
                    
                    eprintln!("Failed to compile for {} with error: {}", provider_name, exception);
                    Ok(HashMap::new())
                }
            }
        })
    }

    /// Deep merge functionality equivalent to Python's mergedeep.merge
    fn merge_deep(&self, target: &mut HashMap<String, Value>, source: HashMap<String, Value>) {
        for (key, value) in source {
            match target.get_mut(&key) {
                Some(existing_value) => {
                    // If both are objects, merge recursively
                    if let (Some(existing_obj), Some(source_obj)) = (existing_value.as_object_mut(), value.as_object()) {
                        for (inner_key, inner_value) in source_obj {
                            existing_obj.insert(inner_key.clone(), inner_value.clone());
                        }
                    } else {
                        // Otherwise, overwrite
                        *existing_value = value;
                    }
                }
                None => {
                    // Key doesn't exist, insert new value
                    target.insert(key, value);
                }
            }
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
        
        fs::write(&tmp_save_path, &json_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let file_size = tmp_save_path.metadata()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?
            .len();
        
        println!("Finished writing to {:?} (Size = {} bytes)", tmp_save_path, file_size);

        // Compress the file using xz2 crate
        println!("Compressing {:?} for upload", tmp_save_path);
        
        let input_file = fs::File::open(&tmp_save_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let output_file = fs::File::create(&local_save_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        let mut encoder = xz2::write::XzEncoder::new(output_file, 6);
        std::io::copy(&mut std::io::BufReader::new(input_file), &mut encoder)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        encoder.finish()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        // Remove temporary uncompressed file
        fs::remove_file(&tmp_save_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

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
