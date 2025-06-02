mod config;
mod db;
mod templates;

use axum::{
    routing::get,
    Router,
    extract::State,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;

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
        .route("/", get(hello_world))
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

async fn hello_world(
    State(state): State<AppState>,
) -> &'static str {
    // 现在我们可以使用 state.db 来访问数据库
    "Hello, World!"
} 