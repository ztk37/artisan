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
    #[clap(about = "Init artisan in the current directory")]
    Init,
    #[clap(about = "TBD")]
    New(NewOptions),
    #[clap(about = "TBD")]
    Release(Release),
}

#[derive(Parser, Debug)]
pub struct NewOptions {}

#[derive(Parser, Debug)]
pub struct Release {
    #[clap(subcommand)]
    pub command: ReleaseCommand,
}

#[derive(Subcommand, Debug)]
pub enum ReleaseCommand {
    #[clap(about = "TBD")]
    Create,
}
