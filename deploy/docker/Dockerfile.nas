# NAS优化的Rust应用Docker镜像
FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .

# 构建优化配置
ENV CARGO_TERM_COLOR=never
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 编译发布版本
RUN cargo build --release --bin trade_alert_rust

# 运行时镜像
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && adduser --disabled-password --gecos '' appuser

# 复制二进制文件
COPY --from=builder /app/target/release/trade_alert_rust /usr/local/bin/
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static

# 创建数据目录
RUN mkdir -p /app/data && chown -R appuser:appuser /app

USER appuser
WORKDIR /app

# 暴露端口
EXPOSE 8000

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# 启动命令
CMD ["trade_alert_rust"] 