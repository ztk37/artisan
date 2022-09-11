pub type CliResult = Result<(), CliError>;

#[derive(Debug)]
pub enum CliError {
    Io(std::io::Error),
    Plain(String),
    Toml(String),
}

impl From<String> for CliError {
    fn from(msg: String) -> Self {
        CliError::Plain(msg)
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err)
    }
}

impl From<toml::de::Error> for CliError {
    fn from(err: toml::de::Error) -> Self {
        CliError::Toml(err.to_string())
    }
}
