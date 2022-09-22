use std::path::PathBuf;

pub mod cli;
pub mod commands;
pub mod error;
pub mod template;

pub fn find_artisan_home_location() -> Result<PathBuf, String> {
    if let Ok(env_artisan_home) = std::env::var("ARTISAN_HOME") {
        return Ok(PathBuf::from(env_artisan_home));
    }

    let home = std::env::var("HOME").map(PathBuf::from).map_err(|_| {
        "The environment variable HOME not set. Can't find artisan home folder".to_string()
    })?;

    let artisan_config_dir = home.join(".config").join("artisan");

    if artisan_config_dir.exists() {
        return Ok(artisan_config_dir);
    }

    let artisan_home_dir = home.join(".artisan");

    if artisan_home_dir.exists() {
        return Ok(artisan_home_dir);
    }

    Err("Can't find artisan home directory".to_string())
}
