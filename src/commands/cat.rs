use std::io::{self, Write};
use std::sync::Arc;

use crate::config::Config;
use crate::storage::Storage;
use super::CmdResult;

pub async fn cat_file(
    config: &Config,
    id: String,
) -> CmdResult<()> {
    let storage = get_storage(config).await?;

    let data = storage.get(&id).await?;

    io::stdout().write_all(&data.data)?;
    Ok(())
}

async fn get_storage(config: &Config) -> CmdResult<Arc<dyn Storage>> {
    use crate::storage::S3Storage;
    
    let storage = S3Storage::new(config).await?;
    Ok(Arc::new(storage))
}

#[cfg(test)]
mod tests {
    // Passthrough to storage get
}
