# Build application
FROM rust:1.82-bullseye AS builder
ARG PROFILE=release
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    libpq5 \
    ca-certificates \
    libpq-dev \
    postgresql

# Copy source code
COPY . .

# Build
RUN cargo build --release --bin myso-social-server

# Production Image
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libjemalloc-dev \
    ca-certificates \
    curl \
    libpq5 \
    libpq-dev \
    postgresql

# Use jemalloc as memory allocator
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so

WORKDIR /app
COPY --from=builder /app/target/release/myso-social-server /usr/local/bin

# Copy and install entrypoint script
COPY crates/myso-social-server/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Expose API port (Railway uses PORT; metrics on internal 9186)
EXPOSE 9009

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD ["sh", "-c", "curl -f http://localhost:${PORT:-9009}/health || exit 1"]

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
