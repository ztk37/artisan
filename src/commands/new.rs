use crate::{
    cli::NewOptions,
    error::{CliError, CliResult},
    template::Template,
};

pub fn run(options: &NewOptions) -> CliResult {
    if let Err(err) = std::fs::create_dir(&options.name) {
        return Err(CliError::from(err));
    }

    let readed = std::fs::read_to_string(std::path::PathBuf::from_iter(vec![
        &"templates".to_string(),
        &options.template,
    ]));

    if let Err(err) = readed {
        return Err(CliError::from(err));
    }

    let template: Result<Template, CliError> =
        toml::from_str(readed.unwrap().as_str()).map_err(CliError::from);

    if let Err(err) = template {
        return Err(err);
    }

    template
        .unwrap()
        .templates
        .into_iter()
        .for_each(|template| {
            if let Err(err) = template.generate(&options.name) {
                println!("{}", err);
            }
        });

    Ok(())
}
