mod config;
mod storage;
mod models;

use clap::{Parser, Subcommand};

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

    match cli.command {
        Commands::Push { .. } => println!("push command"),
        Commands::Pull { .. } => println!("pull command"),
        Commands::Ls { .. } => println!("ls command"),
        Commands::Rm { .. } => println!("rm command"),
        Commands::Share { .. } => println!("share command"),
        Commands::Info { .. } => println!("info command"),
        Commands::Copy { .. } => println!("copy command"),
        Commands::Move { .. } => println!("move command"),
        Commands::Cat { .. } => println!("cat command"),
        Commands::Config { .. } => println!("config command"),
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
}
