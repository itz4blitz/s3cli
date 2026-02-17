use std::sync::Arc;

use crate::config::Config;
use crate::models::Pagination;
use crate::storage::Storage;
use super::CmdResult;

pub async fn list_files(
    config: &Config,
    pattern: Option<String>,
    long: bool,
    limit: usize,
) -> CmdResult<()> {
    let storage = get_storage(config).await?;

    let pagination = Pagination {
        limit: Some(limit),
        ..Default::default()
    };

    let prefix = pattern.as_deref();
    let result = storage.list(prefix, &pagination).await?;

    if result.entries.is_empty() {
        println!("No files found.");
        return Ok(());
    }

    if long {
        println!("{:<40} {:>12} {}", "KEY", "SIZE", "CREATED");
        println!("{}", "-".repeat(80));
        
        for entry in &result.entries {
            let size = format_size(entry.size);
            let date = entry.created_at.format("%Y-%m-%d %H:%M");
            println!("{:<40} {:>12} {}", entry.key, size, date);
        }
    } else {
        for entry in &result.entries {
            println!("{}", entry.key);
        }
    }

    if let Some(token) = result.next_continuation_token {
        println!("\n--more--");
    }

    Ok(())
}

async fn get_storage(config: &Config) -> CmdResult<Arc<dyn Storage>> {
    use crate::storage::S3Storage;
    
    let storage = S3Storage::new(config).await?;
    Ok(Arc::new(storage))
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500B");
        assert_eq!(format_size(1024), "1.0K");
        assert_eq!(format_size(1536), "1.5K");
        assert_eq!(format_size(1048576), "1.0M");
        assert_eq!(format_size(1073741824), "1.0G");
    }
}
