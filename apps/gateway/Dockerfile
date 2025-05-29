# 第一阶段：编译
FROM rust:1.86.0 AS builder

# 创建工作目录
WORKDIR /app

COPY .cargo ./.cargo
COPY Cargo.lock .
COPY Cargo.toml .

RUN mkdir ./apps/gateway/src -p && echo "fn main() {}" > apps/gateway/src/main.rs
RUN mkdir ./apps/auth/src -p && echo "fn main() {}" > apps/auth/src/main.rs
RUN mkdir ./apps/note/src -p && echo "fn main() {}" > apps/note/src/main.rs
RUN mkdir ./packages/server-common/src -p && echo "fn main() {}" > packages/server-common/src/lib.rs

COPY apps/gateway/Cargo.toml apps/gateway/Cargo.toml
COPY apps/auth/Cargo.toml apps/auth/Cargo.toml
COPY apps/note/Cargo.toml apps/note/Cargo.toml
COPY packages/server-common/Cargo.toml packages/server-common/Cargo.toml

RUN cargo fetch

COPY apps/gateway ./apps/gateway
COPY apps/auth ./apps/auth
COPY apps/note ./apps/note
COPY packages/server-common ./packages/server-common


# 构建 release 版本的 gateway package
RUN cargo build -p gateway --release

# 第二阶段：最小化运行环境（使用更小的基础镜像）
FROM debian:12.10-slim

# 安装运行时依赖（如果你的程序没有特别依赖也可以不装）
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户（可选）
RUN useradd -m appuser

# 从 builder 拷贝可执行文件
COPY --from=builder /app/target/release/gateway /usr/local/bin/gateway

# 切换到非 root 用户（可选）
USER appuser

EXPOSE 11000

# 设置容器启动命令
ENTRYPOINT ["/usr/local/bin/gateway"]
