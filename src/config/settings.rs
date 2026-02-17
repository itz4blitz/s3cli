use super::Provider;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_provider")]
    pub provider: Provider,

    #[serde(default)]
    pub endpoint: Option<String>,

    #[serde(default = "default_region")]
    pub region: String,

    #[serde(default)]
    pub bucket: String,

    #[serde(default)]
    pub access_key: Option<String>,

    #[serde(default)]
    pub secret_key: Option<String>,

    #[serde(default)]
    pub path: Option<String>,

    #[serde(default = "default_expiry")]
    pub default_expiry: String,

    #[serde(default = "default_true")]
    pub verify_ssl: bool,

    #[serde(default)]
    pub color: String,

    #[serde(default = "default_concurrency")]
    pub upload_concurrency: usize,

    #[serde(default = "default_concurrency")]
    pub download_concurrency: usize,

    #[serde(default = "default_part_size")]
    pub part_size: String,
}

fn default_provider() -> Provider {
    Provider::S3
}

fn default_region() -> String {
    "us-east-1".to_string()
}

fn default_expiry() -> String {
    "7d".to_string()
}

fn default_true() -> bool {
    true
}

fn default_concurrency() -> usize {
    4
}

fn default_part_size() -> String {
    "8MB".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: Provider::S3,
            endpoint: None,
            region: default_region(),
            bucket: String::new(),
            access_key: None,
            secret_key: None,
            path: None,
            default_expiry: default_expiry(),
            verify_ssl: default_true(),
            color: "auto".to_string(),
            upload_concurrency: default_concurrency(),
            download_concurrency: default_concurrency(),
            part_size: default_part_size(),
        }
    }
}

impl Config {
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("s3cli"))
    }

    pub fn config_file() -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join("config.toml"))
    }

    pub fn load_config_file() -> Result<Option<Config>, ConfigError> {
        let config_path = Self::config_file();

        if let Some(path) = config_path {
            if path.exists() {
                let contents = std::fs::read_to_string(&path)
                    .map_err(|e| ConfigError::IoError(e.to_string()))?;

                let config: Config = toml::from_str(&contents)
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?;

                return Ok(Some(config));
            }
        }

        Ok(None)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.bucket.is_empty() {
            return Err(ConfigError::MissingField("bucket".to_string()));
        }

        if self.provider.requires_endpoint() && self.endpoint.is_none() {
            return Err(ConfigError::MissingEndpoint(self.provider));
        }

        if !matches!(self.provider, Provider::Local) {
            if self.access_key.is_none() || self.secret_key.is_none() {
                return Err(ConfigError::MissingCredentials);
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(String),

    #[error("Failed to parse config: {0}")]
    ParseError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Missing credentials")]
    MissingCredentials,

    #[error("Provider {0} requires an endpoint URL")]
    MissingEndpoint(Provider),
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.provider, Provider::S3);
        assert_eq!(config.region, "us-east-1");
    }

    #[test]
    fn test_provider_from_str() {
        assert_eq!("s3".parse::<Provider>().unwrap(), Provider::S3);
        assert_eq!("r2".parse::<Provider>().unwrap(), Provider::R2);
        assert_eq!("minio".parse::<Provider>().unwrap(), Provider::MinIO);
        assert_eq!("local".parse::<Provider>().unwrap(), Provider::Local);
    }

    #[test]
    fn test_config_to_toml() {
        let config = Config::default();
        let toml = toml::to_string(&config).unwrap();
        assert!(toml.contains("provider"));
        assert!(toml.contains("bucket"));
    }

    #[test]
    fn test_config_from_toml() {
        let toml_str = r#"
provider = "r2"
endpoint = "https://test.r2.cloudflarestorage.com"
region = "auto"
bucket = "test-bucket"
access_key = "test-key"
secret_key = "test-secret"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.provider, Provider::R2);
        assert_eq!(config.bucket, "test-bucket");
    }

    #[test]
    fn test_config_validate_missing_bucket() {
        let config = Config::default();
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validate_r2_requires_endpoint() {
        let mut config = Config::default();
        config.provider = Provider::R2;
        config.bucket = "test".to_string();
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validate_local_no_credentials() {
        let mut config = Config::default();
        config.provider = Provider::Local;
        config.bucket = "test".to_string();
        let result = config.validate();
        assert!(result.is_ok());
    }
}
