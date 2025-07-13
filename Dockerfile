# Railway optimized Rust application Docker image
FROM rust:1.78-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Set build environment variables for Railway
ENV CARGO_TERM_COLOR=never
ENV SQLX_OFFLINE=true
ENV CARGO_PROFILE_RELEASE_LTO=false
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
ENV CARGO_PROFILE_RELEASE_PANIC=abort
ENV CARGO_PROFILE_RELEASE_STRIP=true

# Build the application
RUN cargo build --release --bin trade_alert_rust --jobs 2

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && adduser --disabled-password --gecos '' appuser

# Create app directory and copy files
WORKDIR /app
COPY --from=builder /app/target/release/trade_alert_rust /usr/local/bin/
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static

# Create data directory and set permissions
RUN mkdir -p /app/data && chown -R appuser:appuser /app

USER appuser

# Expose port (Railway will override this)
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Start command
CMD ["trade_alert_rust"]