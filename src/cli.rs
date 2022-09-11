use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "artisan", version = "v0.0.0")]
pub struct Options {
    #[clap(subcommand)]
    pub command: RootCommand,
}

#[derive(Subcommand, Debug)]
pub enum RootCommand {
    #[clap(about = "Create a new project from a template")]
    New(NewOptions),
}

#[derive(Parser, Debug)]
pub struct NewOptions {
    #[clap(long)]
    pub name: String,
    #[clap(long, default_value = "default.toml")]
    pub template: String,
}
