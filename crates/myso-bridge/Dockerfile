# MySo Bridge Dockerfile for Railway Deployment
FROM rust:1.81-bullseye

WORKDIR /work

# Install system dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace
COPY . .

# Copy config files
COPY crates/myso-bridge/bridge-config.yaml /app/bridge-config.yaml
COPY crates/myso-bridge/bridge-config-validator-1.yaml /app/bridge-config-validator-1.yaml
COPY crates/myso-bridge/bridge-config-validator-2.yaml /app/bridge-config-validator-2.yaml
COPY crates/myso-bridge/bridge-config-validator-3.yaml /app/bridge-config-validator-3.yaml

# Set GIT_REVISION for compile-time (bin_version macro; avoids panic when .git unavailable)
ARG RAILWAY_GIT_COMMIT_SHA
RUN GIT_REVISION="${RAILWAY_GIT_COMMIT_SHA:-$(git rev-parse HEAD 2>/dev/null || echo 'unknown')}" \
    cargo build --release -p myso-bridge

# Expose ports
EXPOSE 8080 9090

# Default command - Railway will override this via railway.toml
CMD ["sleep", "infinity"] 