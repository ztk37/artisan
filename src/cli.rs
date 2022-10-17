use crate::{commands::new::NewCommand, error::CliResult};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about = None, long_about = None)]
pub enum Cli {
    /// Create a new project from a template
    New(NewCommand),
    /// Manage artisans global config
    #[command(subcommand)]
    Config(Config),
    /// Manage artisans home directory
    #[command(subcommand)]
    Home(Home),
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

#[derive(Debug, Subcommand)]
pub enum Home {
    /// Create new home folder for artisan
    Init,
}

impl Cli {
    pub fn parse_args() -> Cli {
        Self::parse()
    }

    pub fn run(self) -> CliResult {
        match self {
            Self::New(cmd) => cmd.run(),
            Self::Config(cmd) => match cmd {
                Config::List => todo!(),
                Config::Get(_) => todo!(),
                Config::Set(_) => todo!(),
            },
            Self::Home(cmd) => match cmd {
                Home::Init => todo!(),
            },
        }
    }
}
