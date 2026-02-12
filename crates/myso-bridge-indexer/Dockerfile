# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rustlang/rust:nightly-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/myso"

# bridge-indexer needs postgres libpq5 and ca-certificates
RUN apt update && apt install -y libpq5 ca-certificates libpq-dev postgresql

RUN apt-get update && apt-get install -y cmake clang

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY myso-execution myso-execution
COPY external-crates external-crates

RUN cargo build --release --package myso-bridge-indexer --bin bridge-indexer

# Production Image
FROM debian:bullseye-slim AS runtime
# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y libjemalloc-dev ca-certificates curl
ENV LD_PRELOAD /usr/lib/x86_64-linux-gnu/libjemalloc.so
WORKDIR "$WORKDIR/myso"
COPY --from=builder /myso/target/release/bridge-indexer /usr/local/bin
COPY crates/myso-bridge-indexer/config.yaml /myso/config.yaml
RUN apt update && apt install -y libpq5 ca-certificates libpq-dev postgresql

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
