use askama::Template;

#[derive(Template)]
#[template(path = "base.html", escape = "html")]
pub struct BaseTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "index.html", escape = "html")]
pub struct IndexTemplate {
    pub base: BaseTemplate,
}

impl IndexTemplate {
    pub fn new() -> Self {
        Self {
            base: BaseTemplate {
                title: "Home".to_string(),
            },
        }
    }
} 