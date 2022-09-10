use std::process::exit;

use clap::Parser;

use repo::{cli::Options, commands};

fn main() {
    let opts = Options::from_args();

    let exit_code = match commands::run(opts.command) {
        Err(message) => {
            let code = 1;
            println!("exited with code {} - {:?}", code, message);
            code
        }
        Ok(()) => 0,
    };

    exit(exit_code);
}
