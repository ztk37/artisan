use crate::{cli::NewOptions, error::CliResult, template::Template};
use std::path::PathBuf;

pub fn run(options: &NewOptions) -> CliResult {
    std::fs::create_dir(&options.name)?;

    let file_content = std::fs::read(PathBuf::from("templates").join(&options.template))?;

    let template: Template = toml::from_slice(file_content.as_ref())?;

    template.templates.into_iter().for_each(|template| {
        if let Err(err) = template.generate(&options.name) {
            println!("{}", err);
        }
    });

    Ok(())
}
