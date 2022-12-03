mod cli;
mod context;
mod error;
mod ops;
mod template;
mod utils;

pub mod prelude {
    pub use crate::cli::Cli;
    pub use crate::context::Context;
    pub use crate::ops::Run;
    pub use clap::Parser;
}
