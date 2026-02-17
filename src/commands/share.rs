use std::sync::Arc;
use std::time::Duration;

use crate::config::Config;
use crate::storage::Storage;
use super::CmdResult;

pub async fn share_file(
    config: &Config,
    id: String,
    expires: String,
    download: bool,
    copy_clipboard: bool,
) -> CmdResult<()> {
    let storage = get_storage(config).await?;

    let duration = parse_duration(&expires)?;
    
    let mut url = storage.presign(&id, duration).await?;

    if download {
        url.push_str("&ResponseContentDisposition=attachment");
    }

    if copy_clipboard {
        copy_to_clipboard(&url)?;
        println!("Copied to clipboard!");
    } else {
        println!("{}", url);
    }
    Ok(())
}

fn copy_to_clipboard(text: &str) -> CmdResult<()> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| super::CommandError::Io(format!("Clipboard error: {}", e)))?;
    clipboard.set_text(text)
        .map_err(|e| super::CommandError::Io(format!("Clipboard error: {}", e)))?;
    Ok(())
}

async fn get_storage(config: &Config) -> CmdResult<Arc<dyn Storage>> {
    use crate::storage::S3Storage;
    
    let storage = S3Storage::new(config).await?;
    Ok(Arc::new(storage))
}

fn parse_duration(s: &str) -> CmdResult<Duration> {
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

    Ok(Duration::from_secs(secs))
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
    }

    #[test]
    fn test_parse_duration_invalid() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("xyz").is_err());
    }
}
