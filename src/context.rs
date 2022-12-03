use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Context {
    pub config_paths: Option<ConfigPaths>,
}

impl Context {
    pub fn init() -> Self {
        Context {
            config_paths: init_config_paths(),
        }
    }
}

#[derive(Debug)]
pub struct ConfigPaths {
    pub config_file_path: PathBuf,
    pub template_dir_path: PathBuf,
}

impl From<String> for ConfigPaths {
    fn from(base_path: String) -> Self {
        ConfigPaths {
            config_file_path: PathBuf::from_iter(&[&base_path, "config.toml"]),
            template_dir_path: PathBuf::from_iter(&[&base_path, "templates"]),
        }
    }
}

impl From<PathBuf> for ConfigPaths {
    fn from(base_path: PathBuf) -> Self {
        ConfigPaths {
            config_file_path: base_path.join("config.toml"),
            template_dir_path: base_path.join("templates"),
        }
    }
}

pub fn init_config_paths() -> Option<ConfigPaths> {
    if let Ok(artisan_home) = std::env::var("ARTISAN_HOME") {
        return Some(ConfigPaths::from(artisan_home));
    }

    // Return None if HOME environment varibale is not set
    let home = std::env::var("HOME").ok()?;

    let artisan_config_dir = PathBuf::from_iter(&[&home, ".config", "artisan"]);

    if artisan_config_dir.exists() {
        return Some(ConfigPaths::from(artisan_config_dir));
    }

    let artisan_home_dir = PathBuf::from_iter(&[&home, ".artisan"]);

    if artisan_home_dir.exists() {
        return Some(ConfigPaths::from(artisan_home_dir));
    }

    None
}

#[allow(dead_code)]
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
