use clap::Parser;

const DEFAULT_PROJECT_TEMPLATE: &str = r#"
[[inputs]]
name = "fullname"
prompt = "What is your full name?"
default = "John Doe"

[[inputs]]
name = "year"
prompt = "What is the current year?"
default = "2023"

[[inputs]]
name = "headline"
prompt = "What is the project headline?"
default = "My Project"

[[templates]]
file = "README.md"
content = """
# {{headline}}
"""

[[templates]]
file = "LICENSE"
content = """
MIT License

Copyright (c) {{year}} {{fullname}}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""
"#;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectTemplate {
    inputs: Vec<TemplateInput>,
    templates: Vec<TemplateFile>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TemplateFile {
    file: String,
    content: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TemplateInput {
    name: String,
    prompt: String,
    #[serde(rename = "default")]
    default_value: Option<String>,
}

#[derive(Debug, Parser)]
pub enum Cli {
    /// Create a new project
    New {
        name: String,
        #[clap(short, long)]
        template: Option<String>,
    },
}

pub fn template() -> anyhow::Result<()> {
    let cmd = Cli::parse();

    println!("{:?}", cmd);

    match cmd {
        Cli::New { name, template } => {
            let template_content = if let Some(template) = &template {
                std::fs::read_to_string(template)?
            } else {
                DEFAULT_PROJECT_TEMPLATE.to_owned()
            };

            let project_template: ProjectTemplate = toml::from_str(&template_content)?;

            let mut tera = tera::Tera::default();

            for template in &project_template.templates {
                tera.add_raw_template(&template.file, &template.content)?;
            }

            let mut ctx = tera::Context::new();

            for input in &project_template.inputs {
                let value = if let Some(default_value) = &input.default_value {
                    inquire::Text::new(&input.prompt)
                        .with_default(default_value)
                        .prompt()?
                } else {
                    inquire::Text::new(&input.prompt).prompt()?
                };

                ctx.insert(&input.name, &value);
            }

            std::fs::create_dir_all(&name)?;

            for template in &project_template.templates {
                let file =
                    std::fs::File::create(std::path::PathBuf::from(&name).join(&template.file))?;

                tera.render_to(&template.file, &ctx, &file)?;
            }

            Ok(())
        }
    }
}
