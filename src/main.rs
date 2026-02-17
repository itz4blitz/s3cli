mod config;
mod storage;
mod models;
mod commands;

use clap::{Parser, Subcommand};
use tokio::runtime::Runtime;

#[derive(Parser)]
#[command(name = "s3cli")]
#[command(version = "0.1.0")]
#[command(about = "CLI-first S3 storage for developers and AI agents", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Upload a file to S3 storage
    Push {
        /// File to upload (use - for stdin)
        file: Option<String>,

        /// Make file publicly accessible
        #[arg(short, long)]
        public: bool,

        /// Expiration for file (e.g., 7d, 24h)
        #[arg(short, long)]
        expires: Option<String>,

        /// Set Content-Type
        #[arg(short, long, value_name = "TYPE")]
        content_type: Option<String>,
    },

    /// Download a file from S3 storage
    Pull {
        /// File ID or S3 key
        id: String,

        /// Output path
        #[arg(short, long, value_name = "PATH")]
        output: Option<String>,

        /// Overwrite existing file
        #[arg(long)]
        force: bool,
    },

    /// List stored files
    Ls {
        /// Filter by key pattern (glob)
        pattern: Option<String>,

        /// Long format with details
        #[arg(short, long)]
        long: bool,

        /// Maximum results
        #[arg(long, default_value = "100")]
        limit: usize,
    },

    /// Remove file(s)
    Rm {
        /// File ID or S3 key
        id: String,
    },

    /// Generate a presigned URL for a file
    Share {
        /// File ID or S3 key
        id: String,

        /// Expiration time (e.g., 7d, 24h)
        #[arg(short, long, default_value = "7d")]
        expires: String,

        /// Force download (Content-Disposition)
        #[arg(short, long)]
        download: bool,

        /// Copy URL to clipboard
        #[arg(short, long)]
        copy: bool,
    },

    /// Show file metadata
    Info {
        /// File ID or S3 key
        id: String,
    },

    /// Copy file within bucket
    Copy {
        /// Source file ID or key
        source: String,

        /// Destination key
        dest: String,
    },

    /// Move file within bucket
    Move {
        /// Source file ID or key
        source: String,

        /// Destination key
        dest: String,
    },

    /// Stream file to stdout
    Cat {
        /// File ID or S3 key
        id: String,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Get config value
    Get {
        /// Config key
        key: String,
    },
    /// Set config value
    Set {
        /// Config key
        key: String,
        /// Config value
        value: String,
    },
    /// Show all config
    List,
    /// Initialize new config
    Init,
}

fn main() {
    let cli = Cli::parse();
    let runtime = Runtime::new().expect("Failed to create runtime");

    if let Err(e) = run(runtime, cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(runtime: Runtime, cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Push { file, public, expires, content_type } => {
            runtime.block_on(async {
                let config = load_config()?;
                commands::push(&config, file, public, expires, content_type).await
            })?;
        }
        Commands::Pull { id, output, force } => {
            runtime.block_on(async {
                let config = load_config()?;
                commands::pull(&config, id, output, force).await
            })?;
        }
        Commands::Ls { pattern, long, limit } => {
            runtime.block_on(async {
                let config = load_config()?;
                commands::list_files(&config, pattern, long, limit).await
            })?;
        }
        Commands::Rm { id } => {
            runtime.block_on(async {
                let config = load_config()?;
                commands::remove_file(&config, id).await
            })?;
        }
        Commands::Share { id, expires, download, copy } => {
            runtime.block_on(async {
                let config = load_config()?;
                commands::share_file(&config, id, expires, download, copy).await
            })?;
        }
        Commands::Info { id } => {
            runtime.block_on(async {
                let config = load_config()?;
                commands::file_info(&config, id).await
            })?;
        }
        Commands::Copy { source, dest } => {
            runtime.block_on(async {
                let config = load_config()?;
                commands::copy_file(&config, source, dest).await
            })?;
        }
        Commands::Move { source, dest } => {
            runtime.block_on(async {
                let config = load_config()?;
                commands::move_file(&config, source, dest).await
            })?;
        }
        Commands::Cat { id } => {
            runtime.block_on(async {
                let config = load_config()?;
                commands::cat_file(&config, id).await
            })?;
        }
        Commands::Config { command } => {
            match command {
                ConfigCommands::Get { key } => {
                    commands::config_get(key)?;
                }
                ConfigCommands::Set { key, value } => {
                    commands::config_set(key, value)?;
                }
                ConfigCommands::List => {
                    commands::config_list()?;
                }
                ConfigCommands::Init => {
                    commands::config_init()?;
                }
            }
        }
    }
    Ok(())
}

fn load_config() -> Result<config::Config, config::ConfigError> {
    match config::Config::load_config_file()? {
        Some(config) => {
            let mut config = config;
            if let Err(e) = config.validate() {
                eprintln!("Warning: {}", e);
            }
            Ok(config)
        }
        None => {
            let config = config::Config::default();
            config.validate()?;
            Ok(config)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_push() {
        let cli = Cli::parse_from(["s3cli", "push", "test.mp4"]);
        match cli.command {
            Commands::Push { file, .. } => {
                assert_eq!(file, Some("test.mp4".to_string()));
            }
            _ => panic!("Expected Push command"),
        }
    }

    #[test]
    fn test_cli_parse_pull() {
        let cli = Cli::parse_from(["s3cli", "pull", "abc123"]);
        match cli.command {
            Commands::Pull { id, .. } => {
                assert_eq!(id, "abc123");
            }
            _ => panic!("Expected Pull command"),
        }
    }

    #[test]
    fn test_cli_parse_ls() {
        let cli = Cli::parse_from(["s3cli", "ls", "--long", "--limit", "50"]);
        match cli.command {
            Commands::Ls { long, limit, .. } => {
                assert!(long);
                assert_eq!(limit, 50);
            }
            _ => panic!("Expected Ls command"),
        }
    }

    #[test]
    fn test_cli_parse_share() {
        let cli = Cli::parse_from(["s3cli", "share", "abc123", "--expires", "30d"]);
        match cli.command {
            Commands::Share { id, expires, .. } => {
                assert_eq!(id, "abc123");
                assert_eq!(expires, "30d");
            }
            _ => panic!("Expected Share command"),
        }
    }

    #[test]
    fn test_cli_config_get() {
        let cli = Cli::parse_from(["s3cli", "config", "get", "provider"]);
        match cli.command {
            Commands::Config { command } => {
                if let ConfigCommands::Get { key } = command {
                    assert_eq!(key, "provider");
                } else {
                    panic!("Expected Get command");
                }
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_cli_parse_copy() {
        let cli = Cli::parse_from(["s3cli", "copy", "source.txt", "dest.txt"]);
        match cli.command {
            Commands::Copy { source, dest } => {
                assert_eq!(source, "source.txt");
                assert_eq!(dest, "dest.txt");
            }
            _ => panic!("Expected Copy command"),
        }
    }

    #[test]
    fn test_cli_parse_move() {
        let cli = Cli::parse_from(["s3cli", "move", "old.txt", "new.txt"]);
        match cli.command {
            Commands::Move { source, dest } => {
                assert_eq!(source, "old.txt");
                assert_eq!(dest, "new.txt");
            }
            _ => panic!("Expected Move command"),
        }
    }

    #[test]
    fn test_cli_parse_cat() {
        let cli = Cli::parse_from(["s3cli", "cat", "abc123"]);
        match cli.command {
            Commands::Cat { id } => {
                assert_eq!(id, "abc123");
            }
            _ => panic!("Expected Cat command"),
        }
    }

    #[test]
    fn test_cli_parse_rm() {
        let cli = Cli::parse_from(["s3cli", "rm", "abc123"]);
        match cli.command {
            Commands::Rm { id } => {
                assert_eq!(id, "abc123");
            }
            _ => panic!("Expected Rm command"),
        }
    }
}
