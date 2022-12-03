use std::io::Write;

use crate::cli::NewCommand;
use crate::context::Context;
use crate::error::Error;
use crate::template::Template;
use crate::utils::find_artisan_home_location;

use crate::ops::Run;

pub fn engine() -> liquid::Parser {
    liquid::ParserBuilder::new()
        .build()
        .expect("no partial templates supported - no panic")
}

impl Run for NewCommand {
    fn run(&self, _ctx: &Context) -> Result<(), Error> {
        let home = find_artisan_home_location()?;
        let template_dir = home.join("templates");
        let template_file_name = template_dir.join(&self.template);

        let project_path = std::path::PathBuf::from(&self.name);

        let template_file_content = std::fs::read(&template_file_name).map_err(|_| {
            format!(
                "Can't find template {:?} in {:?}",
                self.template,
                template_dir.as_path(),
            )
        })?;

        std::fs::create_dir(&self.name)?;

        let template: Template = toml::from_slice(&template_file_content)?;

        let engine = engine();

        let mut globals = liquid::Object::new();

        globals.insert("name".into(), liquid::model::Value::scalar("example"));

        let mut errors: Vec<liquid::Error> = Vec::new();

        for tpl in template.templates.iter() {
            let parsed = engine.parse(&tpl.template);

            if let Err(ref err) = parsed {
                errors.push(err.to_owned());
            }

            let rendered = parsed.unwrap().render(&globals);

            if let Err(ref err) = rendered {
                errors.push(err.to_owned());
            }

            let file =
                std::fs::File::create(project_path.join(&tpl.file)).map_err(|err| err.to_string());

            match file.unwrap().write_all(rendered.unwrap().as_bytes()) {
                Ok(()) => println!(
                    "{} \"{}\"",
                    ansi_term::Color::Green.paint("* creating"),
                    tpl.file.as_path().display()
                ),
                Err(err) => println!("{:?}", err),
            }
        }

        Ok(())
    }
}
