use clap::{Parser, Subcommand}; // 4.0.26

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = None,
    long_about = None
)]
pub enum Cli {
    /// Create a new project based on the given template
    New(NewCommand),
    /// Manage artisans global config
    #[command(subcommand)]
    Global(Global),
}

#[derive(Debug, Parser)]
pub struct NewCommand {
    /// Project name
    #[clap(short, long)]
    pub name: String,
    /// Template name
    #[clap(short, long, default_value = "default.toml")]
    pub template: String,
}

#[derive(Debug, Subcommand)]
pub enum Global {
    /// Init
    Init(InitCommand),
    /// List config values
    List,
    /// Get config value by key
    Get(GetCommand),
    /// Set config value by key-value pairs
    Set(SetCommand),
}

#[derive(Debug, Parser)]
pub struct InitCommand {
    /// Use config directory for initialisation
    #[arg(long)]
    pub use_config_dir: bool
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
