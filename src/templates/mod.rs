use askama::Template;
use crate::models::{Alert, AlertForTemplate};

#[derive(Template)]
#[template(path = "base.html", escape = "html")]
pub struct BaseTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "index.html", escape = "html")]
pub struct IndexTemplate {
    pub base: BaseTemplate,
    pub alerts: Vec<AlertForTemplate>,
}

#[derive(Template)]
#[template(path = "alert_form.html", escape = "html")]
pub struct AlertFormTemplate {
    pub base: BaseTemplate,
    pub alert: Option<AlertForTemplate>,
}

impl IndexTemplate {
    pub fn new(alerts: Vec<Alert>) -> Self {
        Self {
            base: BaseTemplate {
                title: "预警列表".to_string(),
            },
            alerts: alerts.into_iter().map(AlertForTemplate::from).collect(),
        }
    }
}

impl AlertFormTemplate {
    pub fn new(alert: Option<Alert>) -> Self {
        Self {
            base: BaseTemplate {
                title: if alert.is_some() { "编辑预警" } else { "创建预警" }.to_string(),
            },
            alert: alert.map(AlertForTemplate::from),
        }
    }
} 