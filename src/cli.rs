use clap::Parser; // 4.0.26

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = None,
    long_about = None
)]
pub enum Cli {
    /// Create a new project based on the given template
    New(NewCommand)
}

#[derive(Debug, Parser)]
pub struct NewCommand {
    /// Project name
    #[clap(short, long)]
    pub name: String,
    /// Template name
    #[clap(short, long, default_value = "default.toml")]
    pub template: String,
}
