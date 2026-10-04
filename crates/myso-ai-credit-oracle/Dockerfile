# Build from repo root (recommended):
#   docker build -f crates/myso-ai-credit-oracle/Dockerfile .
#
# Also works when the build context is only crates/myso-ai-credit-oracle (fetches full tree via git).

FROM rust:1.94-bookworm AS builder

ARG MYSO_CORE_GIT_URL=https://github.com/The-Social-Proof-Foundation/myso-core.git
ARG MYSO_CORE_GIT_REF=main
ARG RAILWAY_GIT_COMMIT_SHA

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    ca-certificates \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . /scratch/

RUN set -eux; \
    if grep -q '^\[workspace\]' /scratch/Cargo.toml 2>/dev/null; then \
        cp -a /scratch/. /app/; \
    else \
        git clone --depth 1 "${MYSO_CORE_GIT_URL}" /app; \
        cd /app; \
        if [ -n "${RAILWAY_GIT_COMMIT_SHA:-}" ]; then \
            git fetch --depth 1 origin "${RAILWAY_GIT_COMMIT_SHA}"; \
            git checkout "${RAILWAY_GIT_COMMIT_SHA}"; \
        elif [ "${MYSO_CORE_GIT_REF}" != "main" ]; then \
            git fetch --depth 1 origin "${MYSO_CORE_GIT_REF}"; \
            git checkout "${MYSO_CORE_GIT_REF}"; \
        fi; \
    fi

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

RUN mkdir -p /app/data && chown -R appuser:appuser /app

USER appuser

ENV PORT=8095
ENV AI_CREDIT_RECEIPT_STORE=/app/data/ai_credit_receipts.json
EXPOSE 8095

CMD ["sh", "-c", "export AI_CREDIT_ORACLE_LISTEN=0.0.0.0:${PORT:-8095}; exec ./myso-ai-credit-oracle"]
