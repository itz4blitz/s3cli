use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: String,
    pub key: String,
    pub size: u64,
    pub etag: String,
    pub content_type: String,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub storage_class: Option<String>,
}

impl FileEntry {
    pub fn new(id: String, key: String, size: u64, content_type: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            key,
            size,
            etag: String::new(),
            content_type,
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
            expires_at: None,
            storage_class: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub original_name: String,
    pub content_type: Option<String>,
    pub cache_control: Option<String>,
    pub content_disposition: Option<String>,
    pub metadata: HashMap<String, String>,
    pub expires_after: Option<std::time::Duration>,
    pub is_public: bool,
}

impl Default for FileMetadata {
    fn default() -> Self {
        Self {
            original_name: String::new(),
            content_type: None,
            cache_control: None,
            content_disposition: None,
            metadata: HashMap::new(),
            expires_after: None,
            is_public: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub continuation_token: Option<String>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: Some(100),
            offset: None,
            continuation_token: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    Name,
    Size,
    Date,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_entry_new() {
        let entry = FileEntry::new(
            "abc123".to_string(),
            "uploads/test.mp4".to_string(),
            1024,
            "video/mp4".to_string(),
        );

        assert_eq!(entry.id, "abc123");
        assert_eq!(entry.key, "uploads/test.mp4");
        assert_eq!(entry.size, 1024);
        assert_eq!(entry.content_type, "video/mp4");
    }

    #[test]
    fn test_file_metadata_default() {
        let metadata = FileMetadata::default();
        assert!(metadata.original_name.is_empty());
        assert!(!metadata.is_public);
    }
}
