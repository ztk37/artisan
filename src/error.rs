use std::fmt;
use std::io;

#[derive(Debug)]
pub struct Error {
    pub code: i32,
    pub msg: String,
}

impl Error {
    pub fn new(code: i32, msg: String) -> Self {
        Error { code, msg }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "error: {}", self.msg)?;

        Ok(())
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error { code: 1, msg }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            code: err.raw_os_error().unwrap_or(1),
            msg: err.to_string(),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::from(err.to_string())
    }
}

impl From<liquid::Error> for Error {
    fn from(err: liquid::Error) -> Self {
        Error::from(err.to_string())
    }
}
