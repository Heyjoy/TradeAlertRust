# Trade Alert Rust

A Rust-based web application (using Axum, SQLite, Askama, Bootstrap, Tokio, etc.) for trade alerts.

## Tech Stack

- **Web Framework:** Axum (modern, fast, type-safe)
- **Database:** SQLite (lightweight, embedded)
- **Async Runtime:** Tokio (standard async ecosystem)
- **HTTP Client:** reqwest (simple HTTP client)
- **Task Scheduling:** tokio-cron-scheduler (for periodic tasks)
- **Config:** config + serde (type-safe configuration)
- **Frontend:** Bootstrap + vanilla JS (with Askama or Tera as template engine)

## Installation

1. Clone the repository (or download the source).
2. Ensure you have Rust (and Cargo) installed (e.g., via rustup).
3. (Optional) If you use dotenv, create a `.env` file (see below).

## Running

- **Build:**  
  Run  
  ```bash
  cargo build
  ```
- **Run:**  
  Run  
  ```bash
  cargo run
  ```
  (The server listens on 127.0.0.1:3000 by default.)

## Database Migrations

- The project uses SQLx migrations (located in the `migrations` folder).  
- On startup, the migrations (e.g., `migrations/20240320000000_initial.sql`) are automatically applied (if the database file (trade_alert.db) is writable).  
- (If you need to manually run migrations, you can use the SQLx CLI.)

## Configuration

- The configuration is read from `config.toml` (and optionally from environment variables prefixed with "TRADE_ALERT").  
- (If you use dotenv, create a `.env` file in the project root.)

## Ignored Files

- (See `.gitignore` for details.)  
- Build artifacts (target), dotenv (if used), SQLite database (trade_alert.db), and IDE files (e.g., .vscode, .idea) are ignored.

---

Happy coding! 