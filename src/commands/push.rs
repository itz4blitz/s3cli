use std::io::{self, Read};
use std::path::Path;
use std::sync::Arc;

use crate::config::Config;
use crate::models::FileMetadata;
use crate::storage::Storage;
use super::CmdResult;

pub async fn push(
    config: &Config,
    file: Option<String>,
    public: bool,
    expires: Option<String>,
    content_type: Option<String>,
) -> CmdResult<String> {
    let storage = get_storage(config).await?;

    let data: Vec<u8> = match file.as_deref() {
        Some("-") | None => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            buffer
        }
        Some(path) => {
            std::fs::read(path)?
        }
    };

    let key = if let Some(ref f) = file {
        if f == "-" {
            "stdin"
        } else {
            Path::new(f)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
        }
    } else {
        "stdin"
    };

    let mut metadata = FileMetadata {
        original_name: key.to_string(),
        content_type: content_type.or_else(|| guess_content_type(key)),
        is_public: public,
        ..Default::default()
    };

    if let Some(ref exp) = expires {
        metadata.expires_after = Some(parse_duration(exp)?);
    }

    let data = io::Cursor::new(data);
    let reader = Box::new(data) as Box<dyn Read + Send + Sync>;

    let entry = storage.put(key, reader, &metadata).await?;

    println!("{}", entry.key);
    Ok(entry.key)
}

async fn get_storage(config: &Config) -> CmdResult<Arc<dyn Storage>> {
    use crate::storage::S3Storage;
    
    let storage = S3Storage::new(config).await?;
    Ok(Arc::new(storage))
}

fn guess_content_type(path: &str) -> Option<String> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())?;

    Some(match ext.to_lowercase().as_str() {
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "md" => "text/markdown",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        _ => "application/octet-stream",
    }.to_string())
}

fn parse_duration(s: &str) -> CmdResult<std::time::Duration> {
    let s = s.trim();
    
    if s.is_empty() {
        return Err(super::CommandError::InvalidInput("Empty duration".to_string()));
    }

    let (num_str, unit) = s.split_at(s.len() - 1);
    let num: u64 = num_str.parse()
        .map_err(|_| super::CommandError::InvalidInput(format!("Invalid duration: {}", s)))?;

    let secs = match unit {
        "s" => num,
        "m" => num * 60,
        "h" => num * 3600,
        "d" => num * 86400,
        "w" => num * 604800,
        _ => return Err(super::CommandError::InvalidInput(format!("Unknown unit: {}", unit))),
    };

    Ok(std::time::Duration::from_secs(secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("30s").unwrap().as_secs(), 30);
        assert_eq!(parse_duration("5m").unwrap().as_secs(), 300);
        assert_eq!(parse_duration("2h").unwrap().as_secs(), 7200);
        assert_eq!(parse_duration("1d").unwrap().as_secs(), 86400);
        assert_eq!(parse_duration("1w").unwrap().as_secs(), 604800);
    }

    #[test]
    fn test_parse_duration_invalid() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("xyz").is_err());
        assert!(parse_duration("10x").is_err());
    }

    #[test]
    fn test_guess_content_type() {
        assert_eq!(guess_content_type("test.png"), Some("image/png".to_string()));
        assert_eq!(guess_content_type("test.JPEG"), Some("image/jpeg".to_string()));
        assert_eq!(guess_content_type("test.mp4"), Some("video/mp4".to_string()));
        assert_eq!(guess_content_type("test.unknown"), Some("application/octet-stream".to_string()));
    }
}
