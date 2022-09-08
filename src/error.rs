#[derive(Debug)]
pub enum CliError {
    Io(std::io::Error),
    Msg(String),
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err)
    }
}

impl From<String> for CliError {
    fn from(msg: String) -> Self {
        CliError::Msg(msg)
    }
}
