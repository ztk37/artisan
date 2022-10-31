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

    let cmd = &mut cli::Cli::command();

    generate_to(Bash, cmd, bin_name, out_dir)?;
    generate_to(Zsh, cmd, bin_name, out_dir)?;

    Ok(())
}
