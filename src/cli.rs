use clap::{Parser, Subcommand};

use crate::cmds;

#[derive(Parser, Debug)]
#[clap(
    name = "repo",
    version = "v0.0.0",
    about = "A tool for doing chore work inside your repositories."
)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(about = "Init repo in the current directory")]
    Init,
}

pub fn parse_args() -> Arguments {
    return Arguments::parse()
}

pub fn run(args: Arguments) -> Result<(), String> {
    match args.command {
        Command::Init => cmds::init().map_err(|_err| String::from("")),
    }
}
