use std::{env::current_dir, fs::File, path::Path};

#[derive(Debug)]
pub enum InitialisationError {
    GenericError(String),
    AlreadyInitialized,
}

pub fn init() -> Result<(), InitialisationError> {
    let config_path_buf = current_dir()
        .map(|cur_dir| Path::join(&cur_dir, ".github/repo.yml"))
        .map_err(|io_err| InitialisationError::GenericError(io_err.to_string()));

    config_path_buf.and_then(|config_path| {
        if Path::exists(config_path.as_path()) {
            Err(InitialisationError::AlreadyInitialized)
        } else {
            if let Err(err) = std::fs::create_dir_all(".github") {
                return Err(InitialisationError::GenericError(err.to_string()));
            }

            if let Err(err) = File::create(config_path) {
                return Err(InitialisationError::GenericError(err.to_string()));
            }

            Ok(())
        }
    })
}

pub fn not_implemented_yet() -> Result<(), String> {
    Err(String::from("command not implemented yet"))
}
