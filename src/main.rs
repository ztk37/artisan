use clap::{Parser, Subcommand};

const VERSION: &str = "v0.0.0";

#[derive(Parser, Debug)]
#[clap(
    name = "repo",
    version = VERSION
)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
    Version
}

fn main() {
    let args = Arguments::parse();

    match args.command {
        Command::Init => todo!(),
        Command::Version => println!("{}", VERSION)
    }
}
