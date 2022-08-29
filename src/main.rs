use std::process::exit;

use clap::Parser;

use repo::{
    cmds,
    options::{Options, RootCommand},
};

fn main() {
    let opts = Options::from_args();

    let exit_code = match run_command(opts.command) {
        Err((code, message)) => {
            println!("exited with code {} - {}", code, message);
            code
        }
        Ok(()) => 0,
    };

    exit(exit_code);
}

fn run_command(cmd: RootCommand) -> Result<(), (i32, String)> {
    match cmd {
        RootCommand::Init => cmds::init::run().map_err(|err| (1, format!("{:?}", err))),
        RootCommand::New(_) => cmds::new::run(),
        RootCommand::Release(_) => cmds::release::run(),
    }
}
