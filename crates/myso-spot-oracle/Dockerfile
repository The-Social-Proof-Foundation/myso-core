# Build stage
FROM rust:1.75-slim as builder

# clang/libclang needed for native build deps (zstd-sys bindgen, etc.)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    ca-certificates \
    cmake \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the entire workspace (spot-oracle depends on multiple local crates).
COPY . .

RUN cargo build --release --bin myso-spot-oracle

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false oracle
RUN mkdir -p /app/data && chown oracle:oracle /app/data

COPY --from=builder /app/target/release/myso-spot-oracle /usr/local/bin/myso-spot-oracle
RUN chmod +x /usr/local/bin/myso-spot-oracle

USER oracle
WORKDIR /app

EXPOSE 8097

HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:8097/health || exit 1

CMD ["myso-spot-oracle"]
