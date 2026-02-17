use std::sync::Arc;

use crate::config::Config;
use crate::storage::Storage;
use super::CmdResult;

pub async fn copy_file(
    config: &Config,
    source: String,
    dest: String,
) -> CmdResult<()> {
    let storage = get_storage(config).await?;

    storage.copy(&source, &dest).await?;

    println!("Copied {} -> {}", source, dest);
    Ok(())
}

async fn get_storage(config: &Config) -> CmdResult<Arc<dyn Storage>> {
    use crate::storage::S3Storage;
    
    let storage = S3Storage::new(config).await?;
    Ok(Arc::new(storage))
}

#[cfg(test)]
mod tests {
    // Passthrough to storage copy
}
