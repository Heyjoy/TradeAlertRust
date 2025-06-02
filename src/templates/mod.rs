use askama::Template;
use crate::models::Alert;

#[derive(Template)]
#[template(path = "base.html", escape = "html")]
pub struct BaseTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "index.html", escape = "html")]
pub struct IndexTemplate {
    pub base: BaseTemplate,
    pub alerts: Vec<Alert>,
}

impl IndexTemplate {
    pub fn new(alerts: Vec<Alert>) -> Self {
        Self {
            base: BaseTemplate {
                title: "预警列表".to_string(),
            },
            alerts,
        }
    }
} 