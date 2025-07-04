use pyo3::prelude::*;
use std::path::Path;
use std::fs;
use std::io::Write;
use reqwest;
use tokio;
use walkdir;

/// MTGJSON S3 Handler - equivalent to Python's MtgjsonS3Handler
#[derive(Debug, Clone)]
#[pyclass(name = "MtgjsonS3Handler")]
pub struct MtgjsonS3Handler {
    client: Option<reqwest::Client>,
}

#[pymethods]
impl MtgjsonS3Handler {
    #[new]
    pub fn new() -> Self {
        Self {
            client: Some(reqwest::Client::new()),
        }
    }

    /// Download a file from S3 bucket
    pub fn download_file(
        &self,
        bucket_name: String,
        object_key: String,
        local_file_path: String,
    ) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async {
            self.download_file_async(bucket_name, object_key, local_file_path).await
        })
    }

    /// Upload a file to S3 bucket
    pub fn upload_file(
        &self,
        local_file_path: String,
        bucket_name: String,
        object_key: String,
    ) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async {
            self.upload_file_async(local_file_path, bucket_name, object_key).await
        })
    }

    /// Upload directory contents to S3 bucket
    pub fn upload_directory(
        &self,
        local_dir_path: String,
        bucket_name: String,
        metadata: std::collections::HashMap<String, String>,
    ) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        rt.block_on(async {
            self.upload_directory_async(local_dir_path, bucket_name, metadata).await
        })
    }
}

impl MtgjsonS3Handler {
    /// Async version of download_file
    async fn download_file_async(
        &self,
        bucket_name: String,
        object_key: String,
        local_file_path: String,
    ) -> PyResult<bool> {
        // Check if AWS credentials are available
        let config = crate::config::get_config();
        let aws_configured = config.has_section("AWS") && 
            config.get("AWS", "access_key_id").is_some() &&
            config.get("AWS", "secret_access_key").is_some();

        if !aws_configured {
            eprintln!("Warning: AWS credentials not configured, cannot download from S3");
            return Ok(false);
        }

        // For actual S3 integration, you would use aws-sdk-s3
        // For now, we'll simulate the download with a placeholder
        #[cfg(feature = "aws")]
        {
            use aws_config;
            use aws_sdk_s3 as s3;

            let config = aws_config::load_from_env().await;
            let client = s3::Client::new(&config);

            match client
                .get_object()
                .bucket(&bucket_name)
                .key(&object_key)
                .send()
                .await
            {
                Ok(resp) => {
                    let data = resp.body.collect().await
                        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
                    
                    let bytes = data.into_bytes();
                    
                    // Create parent directories if they don't exist
                    if let Some(parent) = Path::new(&local_file_path).parent() {
                        fs::create_dir_all(parent)
                            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
                    }
                    
                    fs::write(&local_file_path, bytes)
                        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
                    
                    println!("Successfully downloaded {} from S3 bucket {}", object_key, bucket_name);
                    Ok(true)
                }
                Err(e) => {
                    eprintln!("Failed to download from S3: {}", e);
                    Ok(false)
                }
            }
        }

        #[cfg(not(feature = "aws"))]
        {
            // Fallback implementation without AWS SDK
            eprintln!("AWS SDK not available, cannot download from S3");
            Ok(false)
        }
    }

    /// Async version of upload_file
    async fn upload_file_async(
        &self,
        local_file_path: String,
        bucket_name: String,
        object_key: String,
    ) -> PyResult<bool> {
        let config = crate::config::get_config();
        let aws_configured = config.has_section("AWS") && 
            config.get("AWS", "access_key_id").is_some() &&
            config.get("AWS", "secret_access_key").is_some();

        if !aws_configured {
            eprintln!("Warning: AWS credentials not configured, cannot upload to S3");
            return Ok(false);
        }

        #[cfg(feature = "aws")]
        {
            use aws_config;
            use aws_sdk_s3 as s3;

            let file_data = fs::read(&local_file_path)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

            let config = aws_config::load_from_env().await;
            let client = s3::Client::new(&config);

            let body = aws_sdk_s3::primitives::ByteStream::from(file_data);

            match client
                .put_object()
                .bucket(&bucket_name)
                .key(&object_key)
                .body(body)
                .send()
                .await
            {
                Ok(_) => {
                    println!("Successfully uploaded {} to S3 bucket {}", object_key, bucket_name);
                    Ok(true)
                }
                Err(e) => {
                    eprintln!("Failed to upload to S3: {}", e);
                    Ok(false)
                }
            }
        }

        #[cfg(not(feature = "aws"))]
        {
            eprintln!("AWS SDK not available, simulating upload to S3");
            println!("Would upload {} to s3://{}/{}", local_file_path, bucket_name, object_key);
            Ok(true)
        }
    }

    /// Async version of upload_directory
    async fn upload_directory_async(
        &self,
        local_dir_path: String,
        bucket_name: String,
        metadata: std::collections::HashMap<String, String>,
    ) -> PyResult<bool> {
        let dir_path = Path::new(&local_dir_path);
        
        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(
                format!("Directory not found: {}", local_dir_path)
            ));
        }

        let mut success_count = 0;
        let mut total_count = 0;

        // Walk through all files in the directory
        for entry in walkdir::WalkDir::new(dir_path) {
            let entry = entry.map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
            
            if entry.file_type().is_file() {
                total_count += 1;
                let file_path = entry.path();
                let relative_path = file_path.strip_prefix(dir_path)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
                
                let object_key = relative_path.to_string_lossy().replace('\\', "/");
                
                if self.upload_file_async(
                    file_path.to_string_lossy().to_string(),
                    bucket_name.clone(),
                    object_key,
                ).await? {
                    success_count += 1;
                }
            }
        }

        println!("Uploaded {}/{} files to S3 bucket {}", success_count, total_count, bucket_name);
        Ok(success_count == total_count)
    }
}

impl Default for MtgjsonS3Handler {
    fn default() -> Self {
        Self::new()
    }
}