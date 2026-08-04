# Multi-stage build for GhostWire - Security hardened
# Build: docker build -t ghostwire-node:latest -f Dockerfile .

# =============================================================================
# Stage 1: Builder
# =============================================================================
ARG RUST_VERSION=1.88
FROM rust:${RUST_VERSION}-bookworm AS builder

# ort_sys downloads ONNX Runtime binaries built against glibc 2.38+.
# Bookworm has 2.36 — install Trixie's libc6 to provide __isoc23_strtol etc.
RUN echo "deb http://deb.debian.org/debian trixie main" > /etc/apt/sources.list.d/trixie.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends -t trixie libc6 \
    && rm -rf /var/lib/apt/lists/*

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libdbus-1-dev \
    && rm -rf /var/lib/apt/lists/*

# Override host cargo config — mold linker doesn't exist in this container
RUN mkdir -p /root/.cargo && printf '[build]\njobs = "n"\n\n[target.x86_64-unknown-linux-gnu]\nlinker = "cc"\n' > /root/.cargo/config.toml

# Set working directory
WORKDIR /app

# Copy Cargo files first (better layer caching)
COPY ghostwire/Cargo.toml ghostwire/Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release --features reticulum && rm -rf src

# Create dummy frontend dist for RustEmbed (since we are skipping frontend build)
RUN mkdir -p /webui/dist && touch /webui/dist/index.html

# Copy source code
COPY ghostwire/src ./src
COPY ghostwire/Cargo.toml ./
COPY ghostwire/build.rs ./

# Build the application
RUN cargo build --release --features reticulum

# =============================================================================
# Stage 2: Frontend Build (OPTIONAL - Commented out for first ping test)
# Frontend has TypeScript errors - will fix after first ping validation
# =============================================================================
# FROM node:18-alpine as frontend-builder
# WORKDIR /app
# COPY webui/package*.json ./
# RUN npm ci --only=production
# COPY webui/ ./
# RUN npm run build

# =============================================================================
# Stage 3: Runtime (Minimal)
# Trixie provides glibc 2.40 — required by ONNX Runtime prebuilt binaries
# =============================================================================
FROM debian:trixie-slim

# Install ONLY runtime dependencies (minimal attack surface)
# SECURITY FIX: Use wget instead of curl (smaller footprint, no shell needed)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    wget \
    libdbus-1-3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean \
    && rm -rf /tmp/* /var/tmp/*

# Create non-root user for security
RUN groupadd -r ghostwire && useradd -r -g ghostwire -d /home/ghostwire -s /bin/false ghostwire \
    && mkdir -p /home/ghostwire/.ghostwire \
    && chown -R ghostwire:ghostwire /home/ghostwire

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/ghostwire /usr/local/bin/ghostwire

# Create config directory with proper ownership
RUN mkdir -p /app/config && chown -R ghostwire:ghostwire /app

# Set ownership of app directory
RUN chown -R ghostwire:ghostwire /app

# Switch to non-root user
USER ghostwire

# Expose ports
# 3000 - Web UI
# 9000 - API server
# 4001 - libp2p TCP
# 4001/udp - libp2p QUIC (if enabled)
EXPOSE 3000 9000 4001 4001/udp

# Health check using new /api/v2/health endpoint
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD wget -qO- http://localhost:9000/api/v2/health || exit 1

# Set environment variables
ENV RUST_LOG=ghostwire=info,libp2p=warn,tower_http=error
ENV GHOSTWIRE_HOST=0.0.0.0
ENV GHOSTWIRE_PORT=9000
ENV HOME=/home/ghostwire

# Start the application
CMD ["ghostwire", "--web", "--api-host", "0.0.0.0", "--port", "9000"]