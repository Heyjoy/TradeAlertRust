use askama::Template;
use crate::models::{Alert, AlertForTemplate};

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub base: BaseTemplate,
    pub alerts: Vec<AlertForTemplate>,
}

#[derive(Template)]
#[template(path = "alert_form.html")]
pub struct AlertFormTemplate {
    pub base: BaseTemplate,
    pub alert: Option<AlertForTemplate>,
}

impl IndexTemplate {
    pub fn new(alerts: Vec<Alert>) -> Self {
        let alert_templates: Vec<AlertForTemplate> = alerts.into_iter().map(|alert| alert.into()).collect();
        Self {
            base: BaseTemplate {
                title: "预警列表".to_string(),
            },
            alerts: alert_templates,
        }
    }
}

impl AlertFormTemplate {
    pub fn new(alert: Option<Alert>) -> Self {
        Self {
            base: BaseTemplate {
                title: if alert.is_some() { "编辑预警" } else { "创建预警" }.to_string(),
            },
            alert: alert.map(|a| a.into()),
        }
    }
} 