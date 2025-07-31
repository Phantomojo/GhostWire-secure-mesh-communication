# Multi-stage build for GhostWire
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY ghostwire/Cargo.toml ghostwire/Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Copy source code
COPY ghostwire/src ./src
COPY ghostwire/Cargo.toml ./

# Build the application
RUN cargo build --release

# Frontend build stage
FROM node:18-alpine as frontend-builder

WORKDIR /app

# Copy frontend files
COPY webui/package*.json ./
RUN npm ci --only=production

COPY webui/ ./
RUN npm run build

# Final stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false ghostwire

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/ghostwire /usr/local/bin/ghostwire

# Copy frontend build
COPY --from=frontend-builder /app/dist ./webui/dist

# Copy configuration files
COPY ghostwire/config.toml ./config.toml

# Set ownership
RUN chown -R ghostwire:ghostwire /app

# Switch to non-root user
USER ghostwire

# Expose ports
EXPOSE 3000 9000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/api/status || exit 1

# Start the application
CMD ["ghostwire", "--web", "--host", "0.0.0.0", "--port", "3000"] 