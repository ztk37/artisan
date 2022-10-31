use crate::cli::Cli;
use crate::error::CliResult;

pub mod config;
pub mod init;
pub mod new;

pub trait Run {
    fn run(&self) -> CliResult;
}

impl Run for Cli {
    fn run(&self) -> CliResult {
        match self {
            Cli::New(cmd) => cmd.run(),
            Cli::Config(cmd) => cmd.run(),
            Cli::Init(cmd) => cmd.run(),
        }
    }
}
