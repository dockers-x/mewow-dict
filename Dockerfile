FROM rust:latest as builder

WORKDIR /usr/src/mdict-rs
COPY Cargo.toml Cargo.lock ./
# 创建一个空的main.rs来构建依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 现在复制实际的源代码
COPY . .
# 重新构建项目
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/mdict-rs/target/release/mdict-rs /usr/local/bin/mdict-rs
COPY --from=builder /usr/src/mdict-rs/resources/static /app/static

# Create directories for dictionary files
RUN mkdir -p /app/dicts/builtin /app/dicts/user

ENV RUST_LOG=info
ENV STATIC_PATH=/app/static
ENV BUILTIN_DICT_DIR=/app/dicts/builtin
ENV USER_DICT_DIR=/app/dicts/user

# Define volumes for dictionary files and static files
VOLUME ["/app/dicts/builtin", "/app/dicts/user", "/app/static"]

EXPOSE 8080

CMD ["mdict-rs"] 