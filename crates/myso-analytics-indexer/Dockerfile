# Build application
FROM rust:1.82-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    libpq5 \
    ca-certificates \
    libpq-dev \
    postgresql \
    pkg-config \
    libssl-dev

# Copy source code
COPY . .

# Build the analytics indexer
RUN cargo build --release --bin myso-analytics-indexer

# Production Image
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libjemalloc-dev \
    ca-certificates \
    curl \
    libpq5 \
    libpq-dev \
    postgresql \
    gettext-base \
    libssl1.1 \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Use jemalloc as memory allocator
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/myso-analytics-indexer /usr/local/bin/

# Create directories for storage
RUN mkdir -p /tmp/analytics \
    && mkdir -p /opt/myso/db/package_cache \
    && mkdir -p /app/credentials

# Copy and install entrypoint script
COPY crates/myso-analytics-indexer/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Build metadata
ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Expose metrics port (9184 default; Railway uses PORT)
EXPOSE 9184

# Health check endpoint for metrics (uses PORT when set by Railway)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD sh -c 'curl -f http://localhost:${PORT:-9184}/metrics || exit 1'

# Use the entrypoint script
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"] 