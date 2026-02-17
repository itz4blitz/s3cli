mod push;
mod pull;
mod ls;
mod rm;
mod share;
mod info;
mod copy;
mod move_cmd;
mod cat;
mod config_cmd;

pub use push::push;
pub use pull::pull;
pub use ls::list_files;
pub use rm::remove_file;
pub use share::share_file;
pub use info::file_info;
pub use copy::copy_file;
pub use move_cmd::move_file;
pub use cat::cat_file;

pub use config_cmd::{config_get, config_set, config_list, config_init};

use crate::storage::StorageError;

pub type CmdResult<T> = Result<T, CommandError>;

#[derive(Debug)]
pub enum CommandError {
    Storage(StorageError),
    Io(String),
    Config(String),
    NotFound(String),
    InvalidInput(String),
}

impl From<StorageError> for CommandError {
    fn from(err: StorageError) -> Self {
        CommandError::Storage(err)
    }
}

impl From<std::io::Error> for CommandError {
    fn from(err: std::io::Error) -> Self {
        CommandError::Io(err.to_string())
    }
}

impl From<crate::config::ConfigError> for CommandError {
    fn from(err: crate::config::ConfigError) -> Self {
        CommandError::Config(err.to_string())
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Storage(e) => write!(f, "Storage error: {}", e),
            CommandError::Io(e) => write!(f, "IO error: {}", e),
            CommandError::Config(e) => write!(f, "Config error: {}", e),
            CommandError::NotFound(e) => write!(f, "Not found: {}", e),
            CommandError::InvalidInput(e) => write!(f, "Invalid input: {}", e),
        }
    }
}

impl std::error::Error for CommandError {}
