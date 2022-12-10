use std::path::PathBuf;

use serde_derive::Deserialize;

pub const DEFAULT_TEMPLATE: &str = r#"
[[templates]]
file = "README.md"
template = """
# Default Template

> Project description

"""

"#;

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
