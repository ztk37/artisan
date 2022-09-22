use artisan::cli::Cli;
use clap::Parser;
use std::process::exit;

fn main() {
    let opts = Cli::from_args();

    let exit_code = match opts.run() {
        Err(err) => {
            let code = 1;
            println!("exited with code {} - {:?}", code, err);
            code
        }
        Ok(()) => 0,
    };

    exit(exit_code);
}
