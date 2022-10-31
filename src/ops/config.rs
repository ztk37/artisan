use crate::cli::Config;

use super::Run;

impl Run for Config {
    fn run(&self) -> crate::error::CliResult {
        match self {
            Config::List => todo!(),
            Config::Get(_) => todo!(),
            Config::Set(_) => todo!(),
        }
    }
}
