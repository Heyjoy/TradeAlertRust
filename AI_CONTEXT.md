# AI Context File

> Quick project context guide for AI assistants
> Read this file first when starting a new conversation

## Project Overview

**TradeAlertRust** - Stock price alert system built with Rust
- Core features: Stock price monitoring, alerts, email notifications
- Tech stack: Rust + Axum + SQLite + Yahoo Finance API
- Current status: Basic features complete, developing market anomaly monitoring

## Essential Files to Read

### 1. Project Structure
- docs/PROJECT_STRUCTURE.md - Project directory structure
- docs/development/DEVELOPMENT_PLAN.md - Development plan and progress
- README.md - Basic project information

### 2. Current Features
- src/main.rs - Main API and functionality
- docs/development/user_feedback.md - User feedback and resolved issues
- docs/guides/PRD.md - Product requirements
- migrations/ - Database structure (latest migration files)

### 3. Available Tools (DO NOT RECREATE)
- docs/development/DATABASE_MIGRATION_GUIDE.md - Migration workflow
- scripts/development/ - Development tools
- scripts/testing/ - Testing tools  
- scripts/deployment/ - Deployment tools

## Completed Tools (Avoid Duplication)

### Development Tools (scripts/development/)
- new_migration.ps1 - Create database migration files
- dev_migrate.ps1 - Run database migrations
- dev_start.ps1 - Start development environment (auto-migrate + start app)
- start.ps1 / start.bat - Simple startup scripts

### Testing Tools (scripts/testing/)
- test_email.ps1 - Email functionality test
- test_api.ps1 - API functionality test
- test_yahoo_api.ps1 - Yahoo Finance API test

### Deployment Tools (scripts/deployment/)
- deploy_to_railway.ps1 - Railway platform deployment
- cross_compile_arm.ps1 - ARM cross-compilation
- deploy_nas.sh / deploy_nas_direct.sh - NAS deployment scripts

## Current Development Focus

1. Market anomaly monitoring system - Based on new database tables
2. User experience optimization - Based on user feedback
3. Feature enhancement - Technical indicators, news analysis

## Recommended Conversation Starter

```
Hello! I want to continue developing the TradeAlertRust project.

Please first read AI_CONTEXT.md to understand the project overview, 
then check the recommended key files, especially:
- docs/development/DEVELOPMENT_PLAN.md (development plan)
- docs/development/user_feedback.md (user feedback)
- src/main.rs (current features)

Based on this information, help me plan today's development tasks.
Please use existing development tools (scripts/development/) to avoid duplication.
```

## Quick Commands

```powershell
# Start development environment
.\scripts\development\dev_start.ps1

# Create new migration
.\scripts\development\new_migration.ps1 "feature_name"

# Test functionality
.\scripts\testing\test_api.ps1
```

**Goal**: Help AI get up to speed quickly, avoid duplicate work, focus on core features!
