# Multi-stage Dockerfile for FortiChain Server
# Stage 1: Build stage - Using nightly to support edition 2024
FROM rust:1.83-alpine AS builder

# Install nightly toolchain for edition 2024 support
RUN rustup install nightly

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    postgresql-dev \
    build-base

# Add musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Create app directory
WORKDIR /app

# Copy dependency files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Pre-fetch dependencies to leverage Docker cache
RUN cargo +nightly fetch

# Copy source code
COPY src ./src
COPY tests ./tests

# Build the application with musl (static linking) for minimal runtime dependencies
RUN cargo +nightly build --release --target x86_64-unknown-linux-musl

# Stage 2: Runtime stage (minimal)
FROM alpine:3.19 AS runtime

# Install minimal runtime dependencies
RUN apk add --no-cache ca-certificates wget

# Create a non-root user for security
RUN adduser -D -u 1000 appuser

# Create app directory and set ownership
WORKDIR /app
RUN chown appuser:appuser /app

# Copy the statically linked binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/fortichain_server /app/fortichain_server

# Change ownership of the binary
RUN chown appuser:appuser /app/fortichain_server

# Switch to non-root user
USER appuser

# Expose the port (default is 8000, but configurable via PORT env var)
EXPOSE 8000

# Health check (using wget since it's available in Alpine)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:${PORT:-8000}/health_check || exit 1

# Set the binary as entrypoint
ENTRYPOINT ["./fortichain_server"]
