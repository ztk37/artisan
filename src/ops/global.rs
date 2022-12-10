use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::cli::{Global, InitCommand};
use crate::context::{ConfigPaths, Context};
use crate::error::Error;
use crate::ops::Run;
use crate::template::DEFAULT_TEMPLATE;

impl Run for Global {
    fn run(&self, ctx: &Context) -> Result<(), Error> {
        match self {
            Global::Init(cmd) => global_init(ctx, cmd),
            Global::List => todo!(),
            Global::Get(_cmd) => todo!(),
            Global::Set(_cmd) => todo!(),
        }
    }
}

fn global_init(ctx: &Context, cmd: &InitCommand) -> Result<(), Error> {
    let home =
        std::env::var("HOME").map_err(|_| "Environment variable \"HOME\" not set.".to_string())?;

    let config_paths =
        &ctx.config_paths
            .clone()
            .unwrap_or(ConfigPaths::from(PathBuf::from(&home).join(
                if cmd.use_config_dir {
                    PathBuf::from_iter(&[&home, ".config", "artisan"])
                } else {
                    PathBuf::from_iter(&[&home, ".artisan"])
                },
            )));

    fs::create_dir_all(&config_paths.base_dir_path)?;

    let mut config_file = fs::File::create(&config_paths.config_file_path)?;
    config_file.write_all(b"")?;

    fs::create_dir(&config_paths.template_dir_path)?;
    let mut default_template_file =
        fs::File::create(&config_paths.template_dir_path.join("default.toml"))?;
    default_template_file.write(DEFAULT_TEMPLATE.as_bytes())?;

    Ok(())
}

#[allow(dead_code)]
fn global_list(_ctx: &Context) -> Result<(), Error> {
    todo!()
}

#[allow(dead_code)]
fn global_get(_ctx: &Context) -> Result<(), Error> {
    todo!()
}

#[allow(dead_code)]
fn global_set(_ctx: &Context) -> Result<(), Error> {
    todo!()
}
