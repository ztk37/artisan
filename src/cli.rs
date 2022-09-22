use crate::{commands, error::CliResult};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "artisan", version = "v0.0.0")]
pub enum Cli {
    #[clap(about = "Create a new project from a template")]
    New(NewCommand),
}

impl Cli {
    pub fn run(self) -> CliResult {
        match self {
            Cli::New(options) => commands::new::run(&options),
        }
    }
}

#[derive(Debug, Parser)]
pub struct NewCommand {
    #[clap(long)]
    pub name: String,
    #[clap(long, default_value = "default.toml")]
    pub template: String,
}
