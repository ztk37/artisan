// use std::fs::File;
// use std::path::Path;

use std::{env::current_dir, fs::File, path::Path};

#[derive(Debug)]
pub enum InitialisationError {
    GenericError(String),
    AlreadyInitialized,
}

pub fn init() -> Result<(), InitialisationError> {
    // TODO: refactor with chaining
    match current_dir() {
        Ok(cwd) => {
            let config_path = Path::join(&cwd, Path::new(".github/repo.yml"));

            if Path::exists(config_path.as_path()) {
                Err(InitialisationError::AlreadyInitialized)
            } else {
                match std::fs::create_dir_all(".github") {
                    Ok(()) => {}
                    Err(err) => println!("{:?}", err),
                }

                match File::create(config_path) {
                    Ok(_file) => Ok(()),
                    Err(err) => Err(InitialisationError::GenericError(err.to_string())),
                }
            }
        }
        Err(err) => Err(InitialisationError::GenericError(err.to_string())),
    }
}
