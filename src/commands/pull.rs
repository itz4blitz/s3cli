use std::io::{self, Write};
use std::path::Path;
use std::sync::Arc;

use crate::config::Config;
use crate::storage::Storage;
use super::CmdResult;

pub async fn pull(
    config: &Config,
    id: String,
    output: Option<String>,
    force: bool,
) -> CmdResult<()> {
    let storage = get_storage(config).await?;

    let data = storage.get(&id).await?;

    let output_path = match output {
        Some(ref p) => p.clone(),
        None => {
            Path::new(&id)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output")
                .to_string()
        }
    };

    let path = Path::new(&output_path);

    if path.exists() && !force {
        return Err(super::CommandError::InvalidInput(
            format!("File '{}' already exists. Use --force to overwrite.", output_path)
        ));
    }

    if output.is_none() || output_path == "-" {
        io::stdout().write_all(&data.data)?;
    } else {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(&output_path, &data.data)?;
        println!("Saved to {}", output_path);
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
    use super::*;

    #[test]
    fn test_output_path_from_id() {
        let id = "uploads/test.png";
        let output = None;
        
        let output_path = output.unwrap_or_else(|| {
            Path::new(id)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output")
                .to_string()
        });
        
        assert_eq!(output_path, "test.png");
    }

    #[test]
    fn test_output_path_with_explicit() {
        let id = "uploads/test.png";
        let output = Some("custom/path/file.png".to_string());
        
        let output_path = output.unwrap_or_else(|| {
            Path::new(id)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output")
                .to_string()
        });
        
        assert_eq!(output_path, "custom/path/file.png");
    }
}
