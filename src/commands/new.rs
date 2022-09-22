use clap::Parser;

use crate::{error::CliResult, find_artisan_home_location, template::Template};

#[derive(Debug, Parser)]
pub struct NewCommand {
    #[clap(long)]
    pub name: String,
    #[clap(long, default_value = "default.toml")]
    pub template: String,
}

impl NewCommand {
    pub fn run(&self) -> CliResult {
        let home = find_artisan_home_location()?;
        let template_dir = home.join("templates");
        let template_file_name = template_dir.join(&self.template);

        let template_file_content = std::fs::read(&template_file_name).map_err(|_| {
            format!(
                "Can't find template {:?} in {:?}",
                self.template,
                template_dir.as_path(),
            )
        })?;

        std::fs::create_dir(&self.name)?;

        let template: Template = toml::from_slice(&template_file_content.as_ref())?;

        template.templates.into_iter().for_each(|template| {
            if let Err(err) = template.generate(&self.name) {
                println!("{}", err);
            }
        });

        Ok(())
    }
}
