#[path = "src/cli.rs"]
mod cli;

use std::env;
use std::io;

use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::shells::{Bash, Zsh};

fn main() -> io::Result<()> {
    let bin_name = env!("CARGO_PKG_NAME");
    let out_dir = "completions";

    let mut cmd = cli::Cli::command();

    generate_to(Bash, &mut cmd, bin_name, out_dir)?;
    generate_to(Zsh, &mut cmd, bin_name, out_dir)?;

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write("artisan.1", buffer)?;

    Ok(())
}
