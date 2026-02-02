# Dockerfile for MCP Web Search Server
# Uses Rust 1.92 for building
# Uses rustls-tls instead of OpenSSL for zero system dependencies
# Uses Alpine for minimal runtime image

# Build stage - use Alpine to build musl binary
FROM rust:1.92-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

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

# Build the project with static linking
RUN touch src/main.rs && RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

# Runtime stage - Alpine (minimal)
FROM alpine:latest

WORKDIR /app

# Install ca-certificates for HTTPS
RUN apk add --no-cache ca-certificates

# Copy the binary from builder
COPY --from=builder /build/target/release/mcp-websearch /app/mcp-websearch

# Make the binary executable
RUN chmod +x /app/mcp-websearch

# Use non-root user
RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser && \
    chown -R appuser:appuser /app
USER appuser

# Set the entrypoint
ENTRYPOINT ["/app/mcp-websearch"]
