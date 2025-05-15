FROM rust:1.75-slim as builder

WORKDIR /usr/src/mdict-rs
COPY . .
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