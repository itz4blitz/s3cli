use std::sync::Arc;

use crate::config::Config;
use crate::storage::Storage;
use super::CmdResult;

pub async fn remove_file(
    config: &Config,
    id: String,
) -> CmdResult<()> {
    let storage = get_storage(config).await?;

    storage.delete(&id).await?;

    println!("Removed: {}", id);
    Ok(())
}

async fn get_storage(config: &Config) -> CmdResult<Arc<dyn Storage>> {
    use crate::storage::S3Storage;
    
    let storage = S3Storage::new(config).await?;
    Ok(Arc::new(storage))
}

#[cfg(test)]
mod tests {
    // Simple passthrough to storage delete
}
