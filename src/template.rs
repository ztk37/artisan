use std::path::PathBuf;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Template {
    pub name: String,
    // pub meta: TemplateMeta,
    pub templates: Vec<TemplateFile>,
}

#[derive(Debug, Deserialize)]
pub struct TemplateMeta {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TemplateFile {
    pub file: PathBuf,
    pub template: String,
}

impl TemplateFile {
    pub fn generate(&self, project_name: &String) -> Result<(), String> {
        let project_path = std::path::PathBuf::from(project_name);

        if let Some(sub_folder) = self.file.parent() {
            std::fs::create_dir_all(project_path.join(sub_folder))
                .map_err(|err| err.to_string())?;
        }

        let mut file = std::fs::File::create(project_path.join(self.file.as_path()))
            .map_err(|err| err.to_string())?;

        println!(
            "{} \"{}\"",
            ansi_term::Color::Green.paint("* creating"),
            self.file.as_path().display()
        );

        std::io::Write::write_all(&mut file, self.template.as_bytes())
            .map_err(|err| err.to_string())?;

        Ok(())
    }
}
