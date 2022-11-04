use artisan::cli::Cli;
use artisan::ops::Run;
use clap::Parser;
use std::process::exit;

fn main() {
    let opts = Cli::parse();

    let exit_code = match opts.run() {
        Err(err) => {
            println!("{}", err);
            err.code
        }
        Ok(()) => 0,
    };

    exit(exit_code);
}
