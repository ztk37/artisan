use crate::cli::Config;
use crate::error::Error;
use crate::ops::Run;

impl Run for Config {
    fn run(&self) -> Result<(), Error> {
        match self {
            Config::List => todo!(),
            Config::Get(_) => todo!(),
            Config::Set(_) => todo!(),
        }
    }
}
