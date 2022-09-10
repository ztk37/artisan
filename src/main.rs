use std::process::exit;

use clap::Parser;

use repo::{
    cli::{Options, RootCommand},
    commands,
    error::CliError,
};

fn main() {
    let opts = Options::from_args();

    let exit_code = match run_command(opts.command) {
        Err(message) => {
            let code = 1;
            println!("exited with code {} - {:?}", code, message);
            code
        }
        Ok(()) => 0,
    };

    exit(exit_code);
}

fn run_command(cmd: RootCommand) -> Result<(), CliError> {
    match cmd {
        RootCommand::New(options) => commands::new::run(options),
    }
}
