use std::process::exit;

use clap::Parser;

use artisan::{cli::Options, commands};

fn main() {
    let opts = Options::from_args();

    let exit_code = match commands::run(opts.command) {
        Err(err) => {
            let code = 1;
            println!("exited with code {} - {:?}", code, err);
            code
        }
        Ok(()) => 0,
    };

    exit(exit_code);
}
