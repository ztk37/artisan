use std::process::exit;

mod cli;
mod cmds;

fn main() {
    let args = cli::parse_args();

    match cli::run(args) {
        Ok(()) => exit(0),
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    }
}
