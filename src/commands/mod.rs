use crate::{cli::RootCommand, error::CliError};

pub mod new;

pub fn run(cmd: RootCommand) -> Result<(), CliError> {
    match cmd {
        RootCommand::New(options) => new::run(options),
    }
}
