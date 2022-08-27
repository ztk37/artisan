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
    #[clap(about = "Create a new folder with a .github/artisan.yml")]
    New,
    #[clap(about = "Manage todos inside this repository")]
    Todo(Todo),
    #[clap(about = "Add or show license")]
    License(License),
}

#[derive(Parser, Debug)]
pub struct Todo {
    #[clap(subcommand)]
    pub sub: TodoCommand,
}

#[derive(Subcommand, Debug)]
pub enum TodoCommand {
    #[clap(about = "TBD")]
    Show,
    #[clap(about = "TBD")]
    Add,
    #[clap(about = "TBD")]
    Delete,
}

#[derive(Parser, Debug)]
pub struct License {
    #[clap(subcommand)]
    pub sub: LicenseCommand,
}

#[derive(Subcommand, Debug)]
pub enum LicenseCommand {
    #[clap(about = "TBD")]
    Show,
    #[clap(about = "TBD")]
    Add,
}

#[derive(Parser, Debug)]
pub struct TodoOptions {
    #[clap(subcommand)]
    pub command: TodoCmd,
}

#[derive(Subcommand, Debug)]
pub enum TodoCmd {
    #[clap(about = "TBD")]
    Show,
    #[clap(about = "TBD")]
    Add,
    #[clap(about = "TBD")]
    Delete,
}
