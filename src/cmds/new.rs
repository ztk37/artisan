use crate::{cli::NewOptions, error::CliResult};

pub fn run(options: NewOptions) -> CliResult {
    println!("{:?}", options);
    Ok(())
}
