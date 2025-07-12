# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TradeAlert is a Rust-based trading alert system that monitors stock prices across multiple markets (A-shares, US stocks, crypto) and sends email notifications when price conditions are met. The system features a web interface, REST API, and automated price monitoring service.

## Development Commands

### Environment Setup
```bash
# Set required environment variables
$env:DATABASE_URL = "sqlite:data/trade_alert.db"
$env:SQLX_OFFLINE = "false"

# Load .env file (create from config/_env.example)
# Use double underscore format: TRADE_ALERT__EMAIL__SMTP_SERVER=smtp.gmail.com
```

### Development Workflow
```bash
# Start development server (runs migrations automatically)
.\scripts\dev_start.ps1

# Create new database migration
.\scripts\new_migration.ps1 <migration_name>

# Run migrations manually
.\scripts\dev_migrate.ps1

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy

# Build release
cargo build --release
```

### Testing
```bash
# Test API endpoints
.\scripts\test\testing\test_api.ps1

# Test email functionality 
.\scripts\test\testing\test_email.ps1

# Test Yahoo Finance API
.\scripts\test\testing\test_yahoo_api.ps1
```

## Architecture

### Core Components
- **Web Server**: Axum-based HTTP server with REST API and HTML templates
- **Database**: SQLite with SQLx for async database operations
- **Price Service**: Yahoo Finance API integration with intelligent caching
- **Email Notifier**: SMTP email notifications with HTML templates
- **Alert Monitor**: Background service that checks price conditions every 30 seconds

### Module Structure
```
src/
├── main.rs              # Entry point, HTTP routes, dependency injection
├── lib.rs               # Module exports
├── config/              # Configuration management with env var support
├── handlers/            # HTTP request handlers (market, strategy)
├── models/              # Data models and DTOs
├── services/            # Business logic
│   ├── db.rs           # Database operations and connection management
│   ├── email.rs        # SMTP email notification service
│   └── fetcher.rs      # Price fetching and alert monitoring
├── templates/           # Askama HTML templates
└── utils/               # Utility functions
```

### Key Design Patterns
- **Repository Pattern**: Database operations abstracted through `Database` service
- **Dependency Injection**: All services injected via `AppState` 
- **Error Handling**: Uses `anyhow` for application errors, `thiserror` for custom error types
- **Async/Await**: Full async architecture with Tokio runtime
- **Configuration**: Environment-based config with double underscore notation

### Price Data Strategy
The system uses a hybrid approach for price data:
1. Check database cache (valid for 1 hour)
2. If cache miss, fetch real-time data from Yahoo Finance API
3. Store fetched data in database for future requests
4. Background service updates all monitored symbols every 30 seconds

## Configuration

### Environment Variables
Use double underscore format for nested configuration:
```bash
TRADE_ALERT__EMAIL__SMTP_SERVER=smtp.gmail.com
TRADE_ALERT__EMAIL__SMTP_PORT=587
TRADE_ALERT__EMAIL__SMTP_USERNAME=your_email@gmail.com
TRADE_ALERT__EMAIL__SMTP_PASSWORD=your_app_password
TRADE_ALERT__DATABASE__URL=sqlite:data/trade_alert.db
TRADE_ALERT__LOGGING__LEVEL=info
```

### Database Migrations
- Located in `migrations/` directory
- Use `sqlx migrate` commands
- Always test migrations in development first
- Migration files follow format: `YYYYMMDDHHMMSS_description.sql`

## API Endpoints

### Alerts
- `GET /api/alerts` - List all alerts
- `POST /api/alerts` - Create new alert 
- `GET /api/alerts/{id}` - Get specific alert
- `PUT /api/alerts/{id}` - Update alert
- `DELETE /api/alerts/{id}` - Delete alert

### Price Data
- `GET /api/prices/{symbol}/latest` - Get latest price (real-time)
- `GET /api/prices/{symbol}` - Get price history

### Utilities
- `GET /api/test-email` - Send test email
- `GET /` - Dashboard web interface

## Important Implementation Details

### Email Configuration
- Support for multiple SMTP providers (Gmail, QQ, 163, Outlook)
- Gmail requires app-specific passwords
- HTML email templates using Askama
- Connection timeout issues may require trying different ports (587 vs 465)

### Alert Processing
- Background task runs every 30 seconds
- Checks all active alerts against current prices
- Updates alert status to 'triggered' when conditions met
- Sends email notifications automatically
- Prevents duplicate notifications for same alert

### Error Handling
- Never use `unwrap()` or `expect()` in production code
- All external API calls wrapped in proper error handling
- Database operations use transactions where appropriate
- Comprehensive logging with structured tracing

### Testing Strategy
- Unit tests for business logic
- Integration tests for API endpoints
- Mock external dependencies (Yahoo Finance API)
- Test email functionality separately

## Development Guidelines

### Code Style
- Follow Rust naming conventions (snake_case, PascalCase)
- Use `cargo fmt` for consistent formatting
- Add `#[derive(Debug)]` to all custom types
- Document public APIs with `///` comments

### Security
- Never log sensitive data (API keys, passwords)
- Use environment variables for all configuration
- Validate all input data
- Use HTTPS for external API calls

### Performance
- Use Arc<T> for shared state in async contexts
- Implement proper connection pooling for database
- Cache frequently accessed data appropriately
- Monitor external API rate limits