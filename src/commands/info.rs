use std::sync::Arc;

use crate::config::Config;
use crate::storage::Storage;
use super::CmdResult;

pub async fn file_info(
    config: &Config,
    id: String,
) -> CmdResult<()> {
    let storage = get_storage(config).await?;

    let metadata = storage.head(&id).await?;

    println!("Key:          {}", id);
    println!("Original Name: {}", metadata.original_name);
    
    if let Some(ct) = metadata.content_type {
        println!("Content-Type:  {}", ct);
    }
    
    if let Some(cc) = metadata.cache_control {
        println!("Cache-Control: {}", cc);
    }
    
    if let Some(cd) = metadata.content_disposition {
        println!("Disposition:   {}", cd);
    }
    
    if metadata.is_public {
        println!("Public:        Yes");
    }

    if !metadata.metadata.is_empty() {
        println!("\nMetadata:");
        for (key, value) in &metadata.metadata {
            println!("  {}: {}", key, value);
        }
    }

    Ok(())
}

async fn get_storage(config: &Config) -> CmdResult<Arc<dyn Storage>> {
    use crate::storage::S3Storage;
    
    let storage = S3Storage::new(config).await?;
    Ok(Arc::new(storage))
}

#[cfg(test)]
mod tests {
    // Passthrough to storage head
}
