use crate::template::item::Template;
use crate::util::string::trim_duplicate_lines;
use anyhow::Result;
use std::ops::Deref;

/// Represents a list of .gitignore templates
#[derive(Debug, Clone)]
pub struct TemplateList {
    pub items: Vec<Template>,
}

impl Deref for TemplateList {
    type Target = Vec<Template>;
    fn deref(&self) -> &Vec<Template> {
        &self.items
    }
}

impl TemplateList {
    pub fn new(templates: Vec<Template>) -> Self {
        Self { items: templates }
    }

    /// Get the number of templates in the list
    pub fn count(&self) -> usize {
        self.len()
    }

    /// Get the preview content of the template list (not used)
    pub fn content_preview(&self) -> Result<String> {
        if self.count() == 1 {
            return self[0].content_body();
        }

        Ok(self
            .iter()
            .map(|t| t.content(None))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n\n"))
    }

    /// Get the content of the templates in the list joined together
    pub fn content(&self) -> Result<String> {
        if self.count() == 1 {
            return self[0].content(None);
        }

        // If more than 1 template in list, trim duplicate lines
        let templates_content_body = self
            .iter()
            .map(|tmpl| tmpl.content_body())
            .collect::<Result<Vec<String>>>()?;

        let templates_content_body = trim_duplicate_lines(templates_content_body);

        let templates_content = self
            .iter()
            .enumerate()
            .map(|(i, tmpl)| tmpl.content(Some(templates_content_body[i].as_str())))
            .collect::<Result<Vec<String>>>()?;

        Ok(templates_content.join("\n\n"))
    }

    /// Get the CLI command needed to generate the content of the templates
    pub fn command(&self) -> Result<String> {
        let cmds = self
            .iter()
            .map(|t| {
                t.value
                    .name()
                    .map(|name| format!("{}{}", t.value.prefix(), name))
            })
            .collect::<Result<Vec<String>>>()?;

        let cmds = cmds.join(" ");

        Ok(format!("gitnr create {}", cmds).trim().to_string())
    }
}
