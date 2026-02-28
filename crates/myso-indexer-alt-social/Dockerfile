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
    postgresql

# Copy source code
COPY . .

# Build
RUN cargo build --release --bin myso-social-indexer

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
    gettext-base

# Use jemalloc as memory allocator
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so

WORKDIR /app
COPY --from=builder /app/target/release/myso-social-indexer /usr/local/bin
COPY --from=builder /app/crates/myso-indexer-alt-social-schema/migrations /app/migrations

# Create entrypoint script with forced migration
RUN echo '#!/bin/sh\n\
echo "=== ENTRYPOINT DEBUG ==="\n\
echo "Checking environment variables..."\n\
\n\
# Validate required environment variables\n\
if [ -z "$DATABASE_URL" ]; then\n\
  echo "ERROR: DATABASE_URL is not set"\n\
  exit 1\n\
fi\n\
\n\
# Mask sensitive values for logging\n\
if [ -n "$DATABASE_URL" ]; then\n\
  MASKED_DB_URL=$(echo "$DATABASE_URL" | sed '\''s/:[^@]*@/:****@/'\'')\n\
  echo "DATABASE_URL: ${MASKED_DB_URL:-NOT_SET}"\n\
fi\n\
\n\
echo "PORT: ${PORT:-8080}"\n\
echo "=============================="\n\
\n\
# Force migrations by setting environment variable\n\
echo "=== FORCING DATABASE MIGRATIONS ==="\n\
export FORCE_MIGRATIONS=true\n\
export RESET_BLOCKING_SCHEMA=true\n\
echo "Setting FORCE_MIGRATIONS=true to ensure migrations run"\n\
echo "Setting RESET_BLOCKING_SCHEMA=true to force blocking schema update"\n\
echo "Migrations will be handled by the application startup process"\n\
echo "=== MIGRATIONS WILL BE FORCED ==="\n\
\n\
echo "Starting myso-social-indexer..."\n\
echo ""\n\
\n\
# Build optional args for gRPC (streaming + ingestion fallback)\n\
EXTRA_ARGS=""\n\
if [ -n "$STREAMING_URL" ]; then\n\
  EXTRA_ARGS="$EXTRA_ARGS --streaming-url $STREAMING_URL"\n\
  echo "STREAMING_URL (gRPC): $STREAMING_URL"\n\
fi\n\
RPC_URL="${RPC_API_URL:-$RPC_URL}"\n\
if [ -n "$RPC_URL" ]; then\n\
  EXTRA_ARGS="$EXTRA_ARGS --rpc-api-url $RPC_URL"\n\
  echo "RPC_API_URL (gRPC ingestion): $RPC_URL"\n\
fi\n\
\n\
exec myso-social-indexer --metrics-address "0.0.0.0:${PORT:-8080}" $EXTRA_ARGS' > /usr/local/bin/entrypoint.sh && \
    chmod +x /usr/local/bin/entrypoint.sh

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Expose API port
EXPOSE 8080

# Use the entrypoint script
ENTRYPOINT /usr/local/bin/entrypoint.sh