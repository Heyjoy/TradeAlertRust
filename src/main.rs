mod config;
mod db;
mod templates;
mod models;

use axum::{
    routing::get,
    Router,
    extract::{State, Path, Json},
    response::{IntoResponse, Html},
    http::StatusCode,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;
use crate::models::{CreateAlertRequest, AlertResponse};
use crate::templates::{IndexTemplate, AlertFormTemplate};
use askama::Template;

// 应用程序状态
#[derive(Clone)]
struct AppState {
    db: Arc<db::Database>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = config::Config::load()?;

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.logging.level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let db = Arc::new(db::Database::new(&config.database.url).await?);

    // Create application state
    let state = AppState { db };

    // Build our application with a route
    let app = Router::new()
        .route("/", get(index_page))
        .route("/alerts/new", get(new_alert_form))
        .route("/api/alerts", get(list_alerts).post(create_alert))
        .route("/api/alerts/:id", get(get_alert).delete(delete_alert))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Run it
    let addr = config.server_addr();
    tracing::info!("listening on {}", addr);
    
    // Use tokio's Server instead of axum's
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// 首页处理函数
async fn index_page(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.db.list_alerts().await {
        Ok(alerts) => {
            let template = IndexTemplate::new(alerts);
            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    tracing::error!("Failed to render template: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render template").into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to list alerts: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list alerts").into_response()
        }
    }
}

// API handlers
async fn create_alert(
    State(state): State<AppState>,
    Json(payload): Json<CreateAlertRequest>,
) -> impl IntoResponse {
    match state.db.create_alert(&payload).await {
        Ok(alert) => (StatusCode::CREATED, Json(AlertResponse::from(alert))).into_response(),
        Err(e) => {
            tracing::error!("Failed to create alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create alert").into_response()
        }
    }
}

async fn list_alerts(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.db.list_alerts().await {
        Ok(alerts) => Json(alerts.into_iter().map(AlertResponse::from).collect::<Vec<_>>()).into_response(),
        Err(e) => {
            tracing::error!("Failed to list alerts: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list alerts").into_response()
        }
    }
}

async fn get_alert(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match state.db.get_alert(id).await {
        Ok(Some(alert)) => Json(AlertResponse::from(alert)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to get alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get alert").into_response()
        }
    }
}

async fn delete_alert(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match state.db.delete_alert(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to delete alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete alert").into_response()
        }
    }
}

// 新建预警表单
async fn new_alert_form() -> impl IntoResponse {
    let template = AlertFormTemplate::new(None);
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Failed to render template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render template").into_response()
        }
    }
} 