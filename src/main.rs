mod cmds;

use clap::{Parser, Subcommand};

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

fn main() {
    let args = Arguments::parse();

    match args.command {
        Command::Init => {
            if let Err(err) = cmds::init() {
                println!("{:?}", err)
            }
        }
    }
}
