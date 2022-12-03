use std::process;

// The prelude module includes everything used in the main function
use artisan::prelude::{Cli, Context, Parser, Run};

fn main() {
    let ctx = Context::init();

    let exit_code = match Cli::parse().run(&ctx) {
        Err(err) => {
            println!("{}", err);
            err.code
        }
        Ok(()) => 0,
    };

    process::exit(exit_code);
}
