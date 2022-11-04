use crate::cli::Cli;
use crate::error::Error;

pub mod config;
pub mod init;
pub mod new;

pub trait Run {
    fn run(&self) -> Result<(), Error>;
}

impl Run for Cli {
    fn run(&self) -> Result<(), Error> {
        match self {
            Cli::New(cmd) => cmd.run(),
            Cli::Config(cmd) => cmd.run(),
            Cli::Init(cmd) => cmd.run(),
        }
    }
}
