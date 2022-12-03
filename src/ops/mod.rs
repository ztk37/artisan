use crate::{cli::Cli, error::Error, context::Context};

mod new;

pub trait Run {
    fn run(&self, ctx: &Context) -> Result<(), Error>;
}

impl Run for Cli {
    fn run(&self, ctx: &Context) -> Result<(), Error> {
        match self {
            Cli::New(cmd) => cmd.run(ctx),
        }
    }
}
