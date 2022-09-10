use crate::{error::CliResult, cli::NewOptions};

pub fn run(options: NewOptions) -> CliResult {
    println!("{:?}", options);
    Ok(())
}
