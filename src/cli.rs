use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "artisan",
    version = "v0.0.0",
    about = "A tool for doing chore work inside your repositories."
)]
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
    #[clap(long, default_value="default.toml")]
    pub template: String,
}
