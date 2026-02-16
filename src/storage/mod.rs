use async_trait::async_trait;
use std::io::Read;
use crate::models::{FileEntry, FileMetadata, Pagination};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn put(
        &self, 
        key: &str, 
        data: Box<dyn Read + Send + Sync>, 
        metadata: &FileMetadata
    ) -> Result<FileEntry, StorageError>;
    
    async fn get(&self, key: &str) -> Result<StreamedData, StorageError>;
    
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
    
    async fn list(&self, prefix: Option<&str>, pagination: &Pagination) -> Result<ListResult, StorageError>;
    
    async fn presign(&self, key: &str, expires: std::time::Duration) -> Result<String, StorageError>;
    
    async fn head(&self, key: &str) -> Result<FileMetadata, StorageError>;
    
    async fn copy(&self, src: &str, dest: &str) -> Result<(), StorageError>;
    
    async fn move_to(&self, src: &str, dest: &str) -> Result<(), StorageError>;
    
    fn bucket(&self) -> &str;
    
    fn provider_name(&self) -> &str;
}

pub struct StreamedData {
    pub data: bytes::Bytes,
    pub content_type: String,
    pub content_length: u64,
}

#[derive(Debug)]
pub struct ListResult {
    pub entries: Vec<FileEntry>,
    pub next_continuation_token: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Object not found: {0}")]
    NotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Provider error: {0}")]
    ProviderError(String),
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_streamed_data() {
        let data = StreamedData {
            data: bytes::Bytes::from("test content"),
            content_type: "text/plain".to_string(),
            content_length: 12,
        };
        
        assert_eq!(data.content_length, 12);
    }
    
    #[test]
    fn test_list_result() {
        let result = ListResult {
            entries: vec![],
            next_continuation_token: None,
        };
        
        assert!(result.entries.is_empty());
    }
}
