use clap::{Parser, Subcommand};

use crate::cmds;

#[derive(Parser, Debug)]
#[clap(
    name = "repo",
    version = "v0.0.0",
    about = "A tool for doing chore work inside your repositories."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: RootCommand,
}

#[derive(Subcommand, Debug)]
pub enum RootCommand {
    #[clap(about = "Init repo in the current directory")]
    Init,
    #[clap(about = "Create a new folder with a .github/repo.yml")]
    New,
    #[clap(about = "Manage todos inside this repo")]
    Todo(Todo),
    #[clap(about = "Add or show license")]
    License(License),
}

#[derive(Parser, Debug)]
pub struct Todo {
    #[clap(subcommand)]
    sub: TodoCommand,
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
    sub: LicenseCommand,
}

#[derive(Subcommand, Debug)]
pub enum LicenseCommand {
    #[clap(about = "TBD")]
    Show,
    #[clap(about = "TBD")]
    Add,
}

pub fn parse_args() -> Cli {
    return Cli::parse();
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

pub fn run(args: Cli) -> Result<(), String> {
    match args.command {
        RootCommand::Init => cmds::init().map_err(|_err| String::from("")),
        RootCommand::New => cmds::not_implemented_yet(),
        RootCommand::License(command) => match command.sub {
            LicenseCommand::Add => cmds::not_implemented_yet(),
            LicenseCommand::Show => cmds::not_implemented_yet(),
        },
        RootCommand::Todo(command) => match command.sub {
            TodoCommand::Show => cmds::not_implemented_yet(),
            TodoCommand::Add => cmds::not_implemented_yet(),
            TodoCommand::Delete => cmds::not_implemented_yet(),
        },
    }
}
