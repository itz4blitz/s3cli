use std::io::{self, Write};

use crate::config::{Config, ConfigError};

pub fn config_get(key: String) -> Result<String, ConfigError> {
    let config = load_config()?;

    let config = config.ok_or_else(|| ConfigError::MissingField("no config found".to_string()))?;

    let value = match key.as_str() {
        "provider" => config.provider.to_string(),
        "endpoint" => config.endpoint.unwrap_or_default(),
        "region" => config.region,
        "bucket" => config.bucket,
        "default_expiry" => config.default_expiry,
        "verify_ssl" => config.verify_ssl.to_string(),
        "color" => config.color,
        _ => return Err(ConfigError::MissingField(key)),
    };

    println!("{}", value);
    Ok(value)
}

pub fn config_set(key: String, value: String) -> Result<(), ConfigError> {
    let mut config = load_config()?.unwrap_or_default();

    match key.as_str() {
        "provider" => {
            config.provider = value
                .parse()
                .map_err(|e: String| ConfigError::ParseError(e))?;
        }
        "endpoint" => config.endpoint = Some(value.clone()),
        "region" => config.region = value.clone(),
        "bucket" => config.bucket = value.clone(),
        "access_key" => config.access_key = Some(value.clone()),
        "secret_key" => config.secret_key = Some(value.clone()),
        "default_expiry" => config.default_expiry = value.clone(),
        "verify_ssl" => config.verify_ssl = value.parse().unwrap_or(true),
        "color" => config.color = value.clone(),
        _ => return Err(ConfigError::MissingField(key)),
    }

    save_config(&config)?;
    println!("Updated {} = {}", key, value);
    Ok(())
}

pub fn config_list() -> Result<(), ConfigError> {
    let config = load_config()?;

    if let Some(config) = config {
        println!("provider = {}", config.provider);
        if let Some(ep) = config.endpoint {
            println!("endpoint = {}", ep);
        }
        println!("region = {}", config.region);
        println!("bucket = {}", config.bucket);
        if let Some(ak) = &config.access_key {
            println!("access_key = {}", ak);
        }
        if let Some(sk) = &config.secret_key {
            println!("secret_key = ***");
        }
        println!("default_expiry = {}", config.default_expiry);
        println!("verify_ssl = {}", config.verify_ssl);
        println!("color = {}", config.color);
    } else {
        println!("No configuration found. Run 's3cli config init' to create one.");
    }

    Ok(())
}

pub fn config_init() -> Result<Config, ConfigError> {
    println!("Creating new s3cli configuration...\n");

    let mut config = Config::default();

    print!("Provider (s3, r2, minio, local) [s3]: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if !input.trim().is_empty() {
        config.provider = input
            .trim()
            .parse()
            .map_err(|e: String| ConfigError::ParseError(e))?;
    }

    if config.provider.requires_endpoint() {
        print!("Endpoint URL: ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        config.endpoint = Some(input.trim().to_string());
    }

    print!("Region [us-east-1]: ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    if !input.trim().is_empty() {
        config.region = input.trim().to_string();
    }

    print!("Bucket name: ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    config.bucket = input.trim().to_string();

    if !matches!(config.provider, crate::config::Provider::Local) {
        print!("Access Key: ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        config.access_key = Some(input.trim().to_string());

        print!("Secret Key: ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        config.secret_key = Some(input.trim().to_string());
    }

    save_config(&config)?;
    println!(
        "\nConfiguration saved to {:?}",
        Config::config_file().unwrap()
    );

    Ok(config)
}

fn load_config() -> Result<Option<Config>, ConfigError> {
    Config::load_config_file()
}

fn save_config(config: &Config) -> Result<(), ConfigError> {
    let config_path = Config::config_file()
        .ok_or_else(|| ConfigError::IoError("Cannot determine config path".to_string()))?;

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| ConfigError::IoError(e.to_string()))?;
    }

    let toml =
        toml::to_string_pretty(config).map_err(|e| ConfigError::ParseError(e.to_string()))?;

    std::fs::write(&config_path, toml).map_err(|e| ConfigError::IoError(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_key_validation() {
        // Test that invalid keys return error
        let result = config_get("invalid_key".to_string());
        assert!(result.is_err());
    }
}
