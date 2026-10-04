# ============================================================
# AI credit oracle — Dockerfile
#
# Build from the myso-core repo root. Cargo.toml uses workspace deps.
#     docker build -f crates/myso-ai-credit-oracle/Dockerfile .
#
# Railway Root Directory must stay "/" so the workspace is the context.
# The pricing catalog path is compiled as CARGO_MANIFEST_DIR/config,
# so that directory is copied to the same absolute path in the runtime image.
# ============================================================

# Tag: rust:1.85-bookworm  |  arch: linux/amd64  |  pinned: 2026-04-16
FROM rust:1.85-bookworm@sha256:bf7d87666c4da6eace19e06d21bc4859c6e2a5c97a21ac273b0e082112753cf0 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release --locked -p myso-ai-credit-oracle

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash appuser
WORKDIR /app

COPY --from=builder --chown=appuser:appuser \
    /app/target/release/myso-ai-credit-oracle ./myso-ai-credit-oracle
COPY --from=builder --chown=appuser:appuser \
    /app/crates/myso-ai-credit-oracle/config ./crates/myso-ai-credit-oracle/config

USER appuser

ENV PORT=8095
EXPOSE 8095

CMD ["sh", "-c", "export AI_CREDIT_ORACLE_LISTEN=0.0.0.0:${PORT:-8095}; exec ./myso-ai-credit-oracle"]
