use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    #[default]
    S3,
    R2,
    Backblaze,
    MinIO,
    Local,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Provider::S3 => write!(f, "s3"),
            Provider::R2 => write!(f, "r2"),
            Provider::Backblaze => write!(f, "backblaze"),
            Provider::MinIO => write!(f, "minio"),
            Provider::Local => write!(f, "local"),
        }
    }
}

impl FromStr for Provider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "s3" | "aws" => Ok(Provider::S3),
            "r2" | "cloudflare" => Ok(Provider::R2),
            "backblaze" | "b2" => Ok(Provider::Backblaze),
            "minio" => Ok(Provider::MinIO),
            "local" | "filesystem" => Ok(Provider::Local),
            _ => Err(format!("Unknown provider: {}", s)),
        }
    }
}

impl Provider {
    pub fn default_endpoint(&self) -> Option<&'static str> {
        match self {
            Provider::S3 => None, // AWS uses proper DNS
            Provider::R2 => Some("https://<account_id>.r2.cloudflarestorage.com"),
            Provider::Backblaze => Some("https://s3.<region>.backblazeb2.com"),
            Provider::MinIO => Some("http://localhost:9000"),
            Provider::Local => None,
        }
    }

    pub fn requires_endpoint(&self) -> bool {
        match self {
            Provider::S3 => false,
            Provider::R2 => true,
            Provider::Backblaze => true,
            Provider::MinIO => true,
            Provider::Local => false,
        }
    }
}
