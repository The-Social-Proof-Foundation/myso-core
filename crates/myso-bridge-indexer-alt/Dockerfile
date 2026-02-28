# Build application
FROM rust:1.82-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR /app

# Install build dependencies (libpq for Postgres/diesel-async)
RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    ca-certificates \
    pkg-config \
    libssl-dev \
    libpq5 \
    libpq-dev \
    postgresql

# Copy source code
COPY . .

# Build the bridge indexer alt
RUN cargo build --release --bin bridge-indexer-alt

# Production Image
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libjemalloc-dev \
    ca-certificates \
    curl \
    libpq5 \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Use jemalloc as memory allocator
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/bridge-indexer-alt /usr/local/bin/

# Copy and install entrypoint script
COPY crates/myso-bridge-indexer-alt/entrypoint.sh /usr/local/bin/entrypoint.sh
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
