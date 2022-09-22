use crate::{cli::NewCommand, error::CliResult, find_artisan_home_location, template::Template};

pub fn run(options: &NewCommand) -> CliResult {
    let home = find_artisan_home_location()?;

    let template_file_content = std::fs::read(home.join("templates").join(&options.template))?;

    std::fs::create_dir(&options.name)?;

    let template: Template = toml::from_slice(&template_file_content.as_ref())?;

    template.templates.into_iter().for_each(|template| {
        if let Err(err) = template.generate(&options.name) {
            println!("{}", err);
        }
    });

    Ok(())
}
