use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = None,
    long_about = None
)]
pub enum Cli {
    /// Initialise artisans home directory
    Init(Init),
    /// Create a new project from a template
    New(NewCommand),
    /// Manage artisans global config
    #[command(subcommand)]
    Config(Config),
}

#[derive(Debug, Parser)]
pub struct NewCommand {
    #[clap(long)]
    pub name: String,
    #[clap(long, default_value = "default.toml")]
    pub template: String,
}

#[derive(Debug, Subcommand)]
pub enum Config {
    /// List config values
    List,
    /// Get config value by key
    Get(GetCommand),
    /// Set config value by key-value pairs
    Set(SetCommand),
}

#[derive(Debug, Parser)]
pub struct GetCommand {
    /// Config key
    #[arg(long)]
    key: String,
}

#[derive(Debug, Parser)]
pub struct SetCommand {
    /// Config key
    #[arg(long)]
    key: String,
    /// Config value
    #[arg(long)]
    value: String,
}
#[derive(Debug, Parser)]
pub struct Init {}
