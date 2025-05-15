FROM rust:latest AS builder

# Install git-lfs
RUN apt-get update && \
    apt-get install -y git-lfs && \
    git lfs install

WORKDIR /usr/src/mewow-dict

# Copy the source code first
COPY . .

# Pull LFS files explicitly
RUN git lfs pull

# List the dictionary files to verify they exist
RUN ls -la resources/mdx || echo "Dictionary directory not found"

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/mewow-dict/target/release/mewow-dict /usr/local/bin/mewow-dict
COPY --from=builder /usr/src/mewow-dict/resources/static /app/static

# Create directories for dictionary files
RUN mkdir -p /app/dicts/builtin /app/dicts/user

# Copy the large dictionary files from Git LFS
COPY --from=builder /usr/src/mewow-dict/resources/mdx/* /app/dicts/builtin/
RUN ls -la /app/dicts/builtin || echo "No dictionaries were copied"

ENV RUST_LOG=info
ENV STATIC_PATH=/app/static
ENV BUILTIN_DICT_DIR=/app/dicts/builtin
ENV USER_DICT_DIR=/app/dicts/user
ENV HOST=0.0.0.0
ENV PORT=8181

# Define volumes for dictionary files and static files
VOLUME ["/app/dicts/user", "/app/static"]
# Note: We removed /app/dicts/builtin from VOLUME to avoid hiding the built-in dictionaries

EXPOSE 8181
CMD ["mewow-dict"]