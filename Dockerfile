FROM rust:1.88-slim AS builder

# Install target and cargo-auditable
RUN rustup target add wasm32-wasip1 && \
    cargo install cargo-auditable --version 0.6.3

WORKDIR /workspace
COPY . .

# Fetch dependencies and build with lockfile for reproducibility
RUN cargo fetch
RUN cargo auditable build --release --target wasm32-wasip1 --locked

# Final minimal image with OCI metadata
FROM scratch
WORKDIR /

# OCI-compliant labels for registry metadata
LABEL org.opencontainers.image.title="Ark MCP Test Hash plugin"
LABEL org.opencontainers.image.description="WASM test plugin for Ark MCP server"
LABEL org.opencontainers.image.version="v0.0.1"
LABEL org.opencontainers.image.authors="vpopescu"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/vpopescu/ark-mcp-plugin-hash"

COPY --from=builder /workspace/target/wasm32-wasip1/release/plugin.wasm /plugin.wasm

