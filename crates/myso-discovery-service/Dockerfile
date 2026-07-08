# Discovery Service

FROM rust:1.85-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    ca-certificates \
    cmake \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Workspace build requires the full repo context (consensus/, external-crates/, etc.).
COPY . .
RUN cargo build --release --bin myso-discovery-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/myso-discovery-service /usr/local/bin/myso-discovery-service
COPY crates/myso-discovery-service/config/discovery/sources.localnet.yaml /etc/myso/discovery/sources.localnet.yaml
ENV DISCOVERY_SOURCES_CONFIG=/etc/myso/discovery/sources.localnet.yaml
EXPOSE 8096
CMD ["myso-discovery-service"]
