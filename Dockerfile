# Multi-stage build for GhostWire - Optimized for First Ping Test
# Build: docker build -t ghostwire-node:latest -f Dockerfile .

# =============================================================================
# Stage 1: Builder
# =============================================================================
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libdbus-1-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files first (better layer caching)
COPY ghostwire/Cargo.toml ghostwire/Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release && rm -rf src

# Create dummy frontend dist for RustEmbed (since we are skipping frontend build)
RUN mkdir -p /webui/dist && touch /webui/dist/index.html

# Copy source code
COPY ghostwire/src ./src
COPY ghostwire/Cargo.toml ./
COPY ghostwire/build.rs ./

# Build the application
RUN cargo build --release

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
# Using Bookworm for newer GLIBC compatibility
# =============================================================================
FROM debian:bookworm-slim

# Install ONLY runtime dependencies (minimal attack surface)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libdbus-1-3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user for security
RUN useradd -r -s /bin/false ghostwire

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/ghostwire /usr/local/bin/ghostwire

# Create config directory and default config
RUN mkdir -p /app/config && \
    echo '[server]' > /app/config.toml && \
    echo 'host = "0.0.0.0"' >> /app/config.toml && \
    echo 'port = 9000' >> /app/config.toml

# Set ownership
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
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -sf http://localhost:9000/api/v2/health || exit 1

# Set environment variables
ENV RUST_LOG=ghostwire=info,libp2p=warn,tower_http=error
ENV GHOSTWIRE_HOST=0.0.0.0
ENV GHOSTWIRE_PORT=9000

# Start the application
CMD ["ghostwire", "--web", "--api-host", "0.0.0.0", "--port", "9000"] 