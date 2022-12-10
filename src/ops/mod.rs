use crate::{cli::Cli, context::Context, error::Error};

mod global;
mod new;

pub trait Run {
    fn run(&self, ctx: &Context) -> Result<(), Error>;
}

impl Run for Cli {
    fn run(&self, ctx: &Context) -> Result<(), Error> {
        match self {
            Cli::New(cmd) => cmd.run(ctx),
            Cli::Global(cmd) => cmd.run(ctx),
        }
    }
}
