# Dockerfile for MCP Web Search Server
# Uses Rust 1.92 for building
# Uses rustls-tls instead of OpenSSL for zero system dependencies

# Build stage
FROM rust:1.92-slim AS builder

WORKDIR /build

# Copy Cargo files
COPY Cargo.toml ./
COPY Cargo.lock ./

# Create a dummy main.rs to pre-build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the project
RUN touch src/main.rs && cargo build --release

# Runtime stage - minimal scratch-like image
FROM debian:bookworm-slim

WORKDIR /app

# Install minimal runtime dependencies (only ca-certificates for HTTPS)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

# Copy the binary from builder
COPY --from=builder /build/target/release/mcp-websearch /app/mcp-websearch

# Make the binary executable
RUN chmod +x /app/mcp-websearch

# Use non-root user
RUN useradd -m -u 1000 appuser && chown -R appuser:appuser /app
USER appuser

# Set the entrypoint
ENTRYPOINT ["/app/mcp-websearch"]
