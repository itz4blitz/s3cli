use std::sync::Arc;

use crate::config::Config;
use crate::storage::Storage;
use super::CmdResult;

pub async fn move_file(
    config: &Config,
    source: String,
    dest: String,
) -> CmdResult<()> {
    let storage = get_storage(config).await?;

    storage.move_to(&source, &dest).await?;

    println!("Moved {} -> {}", source, dest);
    Ok(())
}

async fn get_storage(config: &Config) -> CmdResult<Arc<dyn Storage>> {
    use crate::storage::S3Storage;
    
    let storage = S3Storage::new(config).await?;
    Ok(Arc::new(storage))
}

#[cfg(test)]
mod tests {
    // Passthrough to storage move_to
}
