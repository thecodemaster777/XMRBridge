# Dockerfile for XMRBridge gateway
FROM rust:1.81 as builder

WORKDIR /app

# Install SQLx CLI (for migrations if needed)
RUN cargo install sqlx-cli --no-default-features --features postgres

# Copy manifests first for caching deps
COPY Cargo.toml Cargo.lock ./
COPY gateway/Cargo.toml gateway/Cargo.toml
COPY crates/monero-rpc/Cargo.toml crates/monero-rpc/Cargo.toml
COPY crates/security-utils/Cargo.toml crates/security-utils/Cargo.toml
COPY crates/price-engine/Cargo.toml crates/price-engine/Cargo.toml

# Pre-fetch dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy full project
COPY . .

# Build release binary
RUN cargo build --workspace --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/gateway /usr/local/bin/gateway

# Expose port
EXPOSE 8080

CMD ["gateway"]
