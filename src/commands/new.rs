use crate::{cli::NewCommand, error::CliResult, find_artisan_home_location, template::Template};

pub fn run(options: &NewCommand) -> CliResult {
    let home = find_artisan_home_location()?;
    let template_dir = home.join("templates");
    let template_file_name = template_dir.join(&options.template);

    let template_file_content = std::fs::read(&template_file_name).map_err(|_| {
        format!(
            "Can't find template {:?} in {:?}",
            options.template,
            template_dir.as_path(),
        )
    })?;

    std::fs::create_dir(&options.name)?;

    let template: Template = toml::from_slice(&template_file_content.as_ref())?;

    template.templates.into_iter().for_each(|template| {
        if let Err(err) = template.generate(&options.name) {
            println!("{}", err);
        }
    });

    Ok(())
}
