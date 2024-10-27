use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct TemplateBuilder<T>
where
    T: Serialize,
{
    pub template: String,
    pub data: Option<T>,
}

impl<T> TemplateBuilder<T>
where
    T: Serialize,
{
    pub fn new(template: String, data: Option<T>) -> Self {
        Self { template, data }
    }

    pub fn build(&self) -> crate::Result<String> {
        let handle_bar = handlebars::Handlebars::new();
        Ok(handle_bar.render_template(&self.template, &self.data)?)
    }
}
