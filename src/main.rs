mod config;
mod db;
mod email;
mod fetcher;
mod models;
mod templates;

use crate::models::{AlertResponse, CreateAlertRequest};
use crate::templates::{AlertFormTemplate, IndexTemplate};
use askama::Template;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 应用程序状态
#[derive(Clone)]
struct AppState {
    db: Arc<db::Database>,
    email_notifier: Arc<email::EmailNotifier>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载.env文件（如果存在）- 必须在配置加载之前
    dotenvy::dotenv().ok();

    // Load configuration
    let config = config::Config::load()?;

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.logging.level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let db = Arc::new(db::Database::new(&config.database.url).await?);

    // Initialize email notifier
    let email_notifier = Arc::new(email::EmailNotifier::new(config.email.clone())?);

    // Initialize price service with Arc
    let price_service = Arc::new(fetcher::PriceService::new(
        db.pool().clone(),
        &config.price_fetcher,
        email_notifier.clone(),
    ));
    let price_config = Arc::new(config.price_fetcher.clone());
    price_service.start_price_updater(price_config).await;

    // Create application state
    let state = AppState {
        db: db.clone(),
        email_notifier,
    };

    // Build our application with a route
    let app = Router::new()
        .route("/", get(index_page))
        .route("/alerts/new", get(new_alert_form))
        .route("/alerts/:id/edit", get(edit_alert_form))
        .route("/api/alerts", get(list_alerts).post(create_alert))
        .route(
            "/api/alerts/:id",
            get(get_alert).delete(delete_alert).put(update_alert),
        )
        .route("/api/prices/:symbol", get(get_price_history))
        .route("/api/prices/:symbol/latest", get(get_latest_price))
        .route("/api/test-email", get(send_test_email))
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
async fn index_page(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_alerts().await {
        Ok(alerts) => {
            let template = IndexTemplate::new(alerts);
            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    tracing::error!("Failed to render template: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to render template",
                    )
                        .into_response()
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

async fn list_alerts(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_alerts().await {
        Ok(alerts) => Json(
            alerts
                .into_iter()
                .map(AlertResponse::from)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Failed to list alerts: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list alerts").into_response()
        }
    }
}

async fn get_alert(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match state.db.get_alert(id).await {
        Ok(Some(alert)) => Json(AlertResponse::from(alert)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to get alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get alert").into_response()
        }
    }
}

async fn delete_alert(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match state.db.delete_alert(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to delete alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete alert").into_response()
        }
    }
}

async fn update_alert(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateAlertRequest>,
) -> impl IntoResponse {
    match state.db.update_alert(id, &payload).await {
        Ok(Some(alert)) => Json(AlertResponse::from(alert)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to update alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update alert").into_response()
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
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render template",
            )
                .into_response()
        }
    }
}

// 编辑预警表单
async fn edit_alert_form(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match state.db.get_alert(id).await {
        Ok(Some(alert)) => {
            let template = AlertFormTemplate::new(Some(alert));
            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    tracing::error!("Failed to render template: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to render template",
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to get alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get alert").into_response()
        }
    }
}

// 获取股票价格历史
async fn get_price_history(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
        SELECT price, volume, timestamp, created_at
        FROM price_history
        WHERE symbol = ?
        ORDER BY timestamp DESC
        LIMIT 100
        "#,
        symbol
    )
    .fetch_all(state.db.pool())
    .await;

    match result {
        Ok(prices) => {
            let price_data: Vec<_> = prices
                .into_iter()
                .map(|row| {
                    serde_json::json!({
                        "price": row.price,
                        "volume": row.volume,
                        "timestamp": row.timestamp,
                        "created_at": row.created_at
                    })
                })
                .collect();

            Json(serde_json::json!({
                "symbol": symbol,
                "prices": price_data
            }))
            .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get price history for {}: {}", symbol, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get price history",
            )
                .into_response()
        }
    }
}

// 获取股票最新价格
async fn get_latest_price(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
        SELECT price, volume, timestamp, created_at
        FROM price_history
        WHERE symbol = ?
        ORDER BY timestamp DESC
        LIMIT 1
        "#,
        symbol
    )
    .fetch_optional(state.db.pool())
    .await;

    match result {
        Ok(Some(row)) => Json(serde_json::json!({
            "symbol": symbol,
            "price": row.price,
            "volume": row.volume,
            "timestamp": row.timestamp,
            "created_at": row.created_at
        }))
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "No price data found").into_response(),
        Err(e) => {
            tracing::error!("Failed to get latest price for {}: {}", symbol, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get latest price",
            )
                .into_response()
        }
    }
}

async fn send_test_email(State(state): State<AppState>) -> impl IntoResponse {
    match state.email_notifier.send_test_email().await {
        Ok(_) => {
            tracing::info!("测试邮件发送成功");
            Json(serde_json::json!({
                "success": true,
                "message": "测试邮件发送成功，请检查您的邮箱"
            }))
            .into_response()
        }
        Err(e) => {
            tracing::error!("测试邮件发送失败: {}", e);
            Json(serde_json::json!({
                "success": false,
                "message": format!("测试邮件发送失败: {}", e)
            }))
            .into_response()
        }
    }
}
