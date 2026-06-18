# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.92-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR /app

# librocksdb-sys can produce a malformed static archive when C/C++ builds run with
# high parallelism on memory-constrained builders (e.g. Railway).
ENV CARGO_BUILD_JOBS=1
ENV CMAKE_BUILD_PARALLEL_LEVEL=1

# Install dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    libpq5 \
    ca-certificates \
    libpq-dev \
    postgresql \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace (build context must be repo root)
COPY . .

# Build librocksdb-sys first so its static archive is created before other crates
# compile in parallel.
RUN cargo build --release -p librocksdb-sys

# Rust crates can compile with more parallelism once rocksdb is built.
ENV CARGO_BUILD_JOBS=4
RUN cargo build --release --bin myso-indexer-alt-graphql

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
    && rm -rf /var/lib/apt/lists/*

# Use jemalloc as memory allocator
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so

WORKDIR /app
COPY --from=builder /app/target/release/myso-indexer-alt-graphql /usr/local/bin
COPY crates/myso-indexer-alt-graphql/config/indexer-config.toml /app/config/indexer-config.toml

# Create entrypoint script
RUN echo '#!/bin/sh\n\
echo "=== ENTRYPOINT DEBUG ==="\n\
echo "Checking environment variables..."\n\
\n\
if [ -z "$DATABASE_URL" ]; then\n\
  echo "ERROR: DATABASE_URL is not set"\n\
  exit 1\n\
fi\n\
\n\
if [ -z "$RPC_URL" ]; then\n\
  echo "ERROR: RPC_URL is not set"\n\
  exit 1\n\
fi\n\
\n\
if [ -n "$DATABASE_URL" ]; then\n\
  MASKED_DB_URL=$(echo "$DATABASE_URL" | sed '\''s/:[^@]*@/:****@/'\'')\n\
  echo "DATABASE_URL: ${MASKED_DB_URL:-NOT_SET}"\n\
fi\n\
\n\
if [ -n "$RPC_URL" ]; then\n\
  echo "RPC_URL: [SET]"\n\
fi\n\
\n\
if [ -n "$CONSISTENT_STORE_URL" ]; then\n\
  echo "CONSISTENT_STORE_URL: [SET]"\n\
fi\n\
\n\
if [ -n "$SOCIAL_DATABASE_URL" ]; then\n\
  echo "SOCIAL_DATABASE_URL: [SET]"\n\
fi\n\
\n\
echo "PORT: ${PORT:-8080}"\n\
echo "Starting myso-indexer-alt-graphql..."\n\
echo ""\n\
\n\
exec myso-indexer-alt-graphql rpc \\\n\
    --database-url "$DATABASE_URL" \\\n\
    --fullnode-rpc-url "$RPC_URL" \\\n\
    ${CONSISTENT_STORE_URL:+--consistent-store-url "$CONSISTENT_STORE_URL" }\\\n\
    ${SOCIAL_DATABASE_URL:+--social-database-url "$SOCIAL_DATABASE_URL" }\\\n\
    --indexer-config /app/config/indexer-config.toml \\\n\
    --rpc-listen-address "0.0.0.0:${PORT:-8080}"' > /usr/local/bin/entrypoint.sh && \
    chmod +x /usr/local/bin/entrypoint.sh

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Use the entrypoint script
ENTRYPOINT /usr/local/bin/entrypoint.sh
