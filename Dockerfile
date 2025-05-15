FROM rust:latest as builder

WORKDIR /usr/src/mdict-rs
COPY Cargo.toml Cargo.lock ./
# Create an empty main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the actual source code and build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/mdict-rs/target/release/mdict-rs /usr/local/bin/mdict-rs
COPY --from=builder /usr/src/mdict-rs/resources/static /app/static

# Create directories for dictionary files
RUN mkdir -p /app/dicts/builtin /app/dicts/user

# Copy the large dictionary files from Git LFS
COPY --from=builder /usr/src/mdict-rs/resources/mdx /app/dicts/builtin

ENV RUST_LOG=info
ENV STATIC_PATH=/app/static
ENV BUILTIN_DICT_DIR=/app/dicts/builtin
ENV USER_DICT_DIR=/app/dicts/user
ENV HOST=127.0.0.1
ENV PORT=8181

# Define volumes for dictionary files and static files
VOLUME ["/app/dicts/user", "/app/static"]
# Note: We removed /app/dicts/builtin from VOLUME to avoid hiding the built-in dictionaries

EXPOSE 8181
CMD ["mdict-rs"]