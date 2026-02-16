use async_trait::async_trait;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE, CONTENT_LENGTH}};
use crate::config::{Config, Provider};
use crate::models::{FileEntry, FileMetadata, Pagination};
use crate::storage::{StorageError, StreamedData, ListResult};

type HmacSha256 = Hmac<Sha256>;

pub struct S3Storage {
    client: Client,
    config: Config,
    bucket: String,
    endpoint: String,
}

impl S3Storage {
    pub async fn new(config: &Config) -> Result<Self, StorageError> {
        let client = Client::builder()
            .build()
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        let endpoint = config.endpoint.clone()
            .unwrap_or_else(|| format!("https://{}.s3.{}.amazonaws.com", config.bucket, config.region));
        
        Ok(Self {
            client,
            config: config.clone(),
            bucket: config.bucket.clone(),
            endpoint,
        })
    }
    
    fn generate_id() -> String {
        use uuid::Uuid;
        let uuid = Uuid::new_v4();
        let bytes = uuid.as_bytes();
        crockford_encode(bytes)
    }
    
    fn sign_request(
        &self,
        method: &str,
        path: &str,
        query: &str,
        headers: &mut HeaderMap,
        payload_hash: &str,
    ) -> Result<(), StorageError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        let date = chrono::Utc::now();
        let amz_date = date.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = date.format("%Y%m%d").to_string();
        
        headers.insert("x-amz-date", HeaderValue::from_str(&amz_date).unwrap());
        headers.insert("x-amz-content-sha256", HeaderValue::from_str(payload_hash).unwrap());
        
        let host = self.endpoint.replace("https://", "").replace("http://", "");
        headers.insert("host", HeaderValue::from_str(&host).unwrap());
        
        let canonical_headers = format!(
            "host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n",
            host, payload_hash, amz_date
        );
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";
        
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method,
            path,
            query,
            canonical_headers,
            signed_headers,
            payload_hash
        );
        
        let canonical_request_hash = sha256_hex(canonical_request.as_bytes());
        
        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.config.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, canonical_request_hash
        );
        
        let k_date = hmac_sha256(format!("AWS4{}", self.config.secret_key.as_ref().unwrap()).as_bytes(), date_stamp.as_bytes());
        let k_region = hmac_sha256(&k_date, self.config.region.as_bytes());
        let k_service = hmac_sha256(&k_region, b"s3");
        let k_signing = hmac_sha256(&k_service, b"aws4_request");
        let signature = hmac_sha256_hex(&k_signing, string_to_sign.as_bytes());
        
        let auth_header = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.config.access_key.as_ref().unwrap(),
            credential_scope,
            signed_headers,
            signature
        );
        
        headers.insert("Authorization", HeaderValue::from_str(&auth_header).unwrap());
        
        Ok(())
    }
    
    async fn request(
        &self,
        method: &str,
        path: &str,
        query: &str,
        body: Option<Vec<u8>>,
        content_type: Option<&str>,
    ) -> Result<reqwest::Response, StorageError> {
        let mut headers = HeaderMap::new();
        
        let payload_hash = if let Some(ref b) = body {
            sha256_hex(b)
        } else {
            sha256_hex(b"")
        };
        
        if let Some(ct) = content_type {
            headers.insert(CONTENT_TYPE, HeaderValue::from_str(ct).unwrap());
        }
        
        if let Some(ref b) = body {
            headers.insert(CONTENT_LENGTH, HeaderValue::from_str(&b.len().to_string()).unwrap());
        }
        
        self.sign_request(method, path, query, &mut headers, &payload_hash)?;
        
        let url = format!("{}{}?{}", self.endpoint, path, query);
        
        let mut request = self.client.request(
            reqwest::Method::from_bytes(method.as_bytes()).unwrap(),
            &url
        ).headers(headers);
        
        if let Some(b) = body {
            request = request.body(b);
        }
        
        request.send().await.map_err(|e| StorageError::NetworkError(e.to_string()))
    }
}

fn crockford_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    let mut result = String::with_capacity(bytes.len() * 8 / 5 + 1);
    
    let mut buffer: u64 = 0;
    let mut bits_in_buffer = 0;
    
    for &byte in bytes {
        buffer = (buffer << 8) | (byte as u64);
        bits_in_buffer += 8;
        
        while bits_in_buffer >= 5 {
            bits_in_buffer -= 5;
            let index = ((buffer >> bits_in_buffer) & 0x1F) as usize;
            result.push(ALPHABET[index] as char);
        }
    }
    
    if bits_in_buffer > 0 {
        let index = ((buffer << (5 - bits_in_buffer)) & 0x1F) as usize;
        result.push(ALPHABET[index] as char);
    }
    
    result
}

fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

fn hmac_sha256_hex(key: &[u8], data: &[u8]) -> String {
    hex::encode(hmac_sha256(key, data))
}

#[async_trait]
impl crate::storage::Storage for S3Storage {
    async fn put(
        &self, 
        key: &str, 
        data: Box<dyn Read + Send + Sync>, 
        metadata: &FileMetadata
    ) -> Result<FileEntry, StorageError> {
        let mut buffer = Vec::new();
        let mut reader = data;
        reader.read_to_end(&mut buffer).map_err(|e| StorageError::IoError(e.to_string()))?;
        
        let content_type = metadata.content_type.clone()
            .unwrap_or_else(|| "application/octet-stream".to_string());
        
        let path = format!("/{}", key);
        
        let response = self.request(
            "PUT",
            &path,
            "",
            Some(buffer.clone()),
            Some(&content_type),
        ).await?;
        
        if !response.status().is_success() {
            return Err(StorageError::ProviderError(format!(
                "Upload failed: {}",
                response.status()
            )));
        }
        
        let size = buffer.len() as u64;
        
        Ok(FileEntry::new(
            Self::generate_id(),
            key.to_string(),
            size,
            content_type,
        ))
    }
    
    async fn get(&self, key: &str) -> Result<StreamedData, StorageError> {
        let path = format!("/{}", key);
        
        let response = self.request("GET", &path, "", None, None).await?;
        
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(StorageError::NotFound(key.to_string()));
        }
        
        if !response.status().is_success() {
            return Err(StorageError::ProviderError(format!(
                "Download failed: {}",
                response.status()
            )));
        }
        
        let content_type = response.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();
        
        let content_length = response.headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        
        let data = response.bytes().await.map_err(|e| StorageError::NetworkError(e.to_string()))?;
        
        Ok(StreamedData {
            data,
            content_type,
            content_length,
        })
    }
    
    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let path = format!("/{}", key);
        
        let response = self.request("DELETE", &path, "", None, None).await?;
        
        if !response.status().is_success() && response.status() != reqwest::StatusCode::NOT_FOUND {
            return Err(StorageError::ProviderError(format!(
                "Delete failed: {}",
                response.status()
            )));
        }
        
        Ok(())
    }
    
    async fn list(&self, prefix: Option<&str>, pagination: &Pagination) -> Result<ListResult, StorageError> {
        let mut query = String::new();
        query.push_str("list-type=2");
        
        if let Some(p) = prefix {
            query.push_str(&format!("&prefix={}", urlencoding::encode(p)));
        }
        
        if let Some(limit) = pagination.limit {
            query.push_str(&format!("&max-keys={}", limit));
        }
        
        let path = "/";
        
        let response = self.request("GET", path, &query, None, None).await?;
        
        if !response.status().is_success() {
            return Err(StorageError::ProviderError(format!(
                "List failed: {}",
                response.status()
            )));
        }
        
        let body = response.text().await.map_err(|e| StorageError::NetworkError(e.to_string()))?;
        
        let entries = parse_list_response(&body);
        
        Ok(ListResult {
            entries,
            next_continuation_token: None,
        })
    }
    
    async fn presign(&self, key: &str, expires: std::time::Duration) -> Result<String, StorageError> {
        let expires_secs = expires.as_secs() as i64;
        
        let path = format!("/{}", key);
        let query = format!("X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential={}%2F{}%2F{}%2Fs3%2Faws4_request&X-Amz-Date={}&X-Amz-Expires={}&X-Amz-SignedHeaders=host",
            urlencoding::encode(self.config.access_key.as_ref().unwrap()),
            chrono::Utc::now().format("%Y%m%d").to_string(),
            self.config.region,
            chrono::Utc::now().format("%Y%m%dT%H%M%SZ"),
            expires_secs
        );
        
        let url = format!("{}{}?{}", self.endpoint, path, query);
        
        Ok(url)
    }
    
    async fn head(&self, key: &str) -> Result<FileMetadata, StorageError> {
        let path = format!("/{}", key);
        
        let response = self.request("HEAD", &path, "", None, None).await?;
        
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(StorageError::NotFound(key.to_string()));
        }
        
        if !response.status().is_success() {
            return Err(StorageError::ProviderError(format!(
                "Head failed: {}",
                response.status()
            )));
        }
        
        Ok(FileMetadata {
            original_name: key.split('/').last().unwrap_or(key).to_string(),
            content_type: response.headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string()),
            cache_control: response.headers()
                .get("cache-control")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string()),
            content_disposition: None,
            metadata: std::collections::HashMap::new(),
            expires_after: None,
            is_public: false,
        })
    }
    
    async fn copy(&self, src: &str, dest: &str) -> Result<(), StorageError> {
        let path = format!("/{}", dest);
        let copy_source = format!("/{}/{}", self.bucket, src);
        
        let mut headers = HeaderMap::new();
        headers.insert("x-amz-copy-source", HeaderValue::from_str(&copy_source).unwrap());
        
        let url = format!("{}{}?{}", self.endpoint, path, "");
        
        let response = self.client.put(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| StorageError::NetworkError(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(StorageError::ProviderError(format!(
                "Copy failed: {}",
                response.status()
            )));
        }
        
        Ok(())
    }
    
    async fn move_to(&self, src: &str, dest: &str) -> Result<(), StorageError> {
        self.copy(src, dest).await?;
        self.delete(src).await?;
        Ok(())
    }
    
    fn bucket(&self) -> &str {
        &self.bucket
    }
    
    fn provider_name(&self) -> &str {
        "s3"
    }
}

fn parse_list_response(body: &str) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    
    if let Some(contents_start) = body.find("<Contents>") {
        let contents = &body[contents_start..];
        
        let mut search_pos = 0;
        while let Some(key_start) = contents[search_pos..].find("<Key>") {
            let key_start = search_pos + key_start;
            if let Some(key_end) = contents[key_start..].find("</Key>") {
                let key = &contents[key_start + 5..key_start + key_end];
                
                let size = if let Some(size_start) = contents[key_start..].find("<Size>") {
                    let size_start = key_start + size_start + 7;
                    if let Some(size_end) = contents[size_start..].find("</Size>") {
                        contents[size_start..size_start + size_end].parse().unwrap_or(0)
                    } else {
                        0
                    }
                } else {
                    0
                };
                
                entries.push(FileEntry::new(
                    S3Storage::generate_id(),
                    key.to_string(),
                    size,
                    "application/octet-stream".to_string(),
                ));
                
                search_pos = key_start + key_end + 6;
            } else {
                break;
            }
        }
    }
    
    entries
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut result = String::new();
        for byte in s.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                _ => {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
        result
    }
}
