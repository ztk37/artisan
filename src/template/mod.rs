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
    pub file: String,
    pub template: String,
}

impl TemplateFile {
    pub fn generate(&self, project_name: &String) -> Result<(), String> {
        let file = std::fs::File::create(std::path::PathBuf::from_iter(vec![
            project_name,
            &self.file,
        ]));

        if let Err(err) = file {
            return Err(err.to_string());
        }

        if let Err(err) = std::io::Write::write_all(&mut file.unwrap(), self.template.as_bytes()) {
            return Err(err.to_string());
        }

        Ok(())
    }
}
