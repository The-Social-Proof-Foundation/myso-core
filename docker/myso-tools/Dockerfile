# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.92-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/myso"
RUN apt-get update && apt-get install -y cmake clang libpq5 libpq-dev

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY myso-execution myso-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} \
    --bin myso-faucet \
    --bin myso-bridge \
    --bin bridge-indexer \
    --bin myso-bridge-cli \
    --bin myso \
    --bin myso-cluster-test \
    --bin myso-tool

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/myso"

# myso-tool needs libpq at runtime and uses git when fetching move dependencies during build
RUN apt-get update && apt-get install -y libpq5 libpq-dev ca-certificates git

COPY --from=builder /myso/target/release/myso-faucet /usr/local/bin
COPY --from=builder /myso/target/release/myso-bridge /usr/local/bin
COPY --from=builder /myso/target/release/bridge-indexer /usr/local/bin
COPY --from=builder /myso/target/release/myso-bridge-cli /usr/local/bin
COPY --from=builder /myso/target/release/myso /usr/local/bin
COPY --from=builder /myso/target/release/myso-cluster-test /usr/local/bin
COPY --from=builder /myso/target/release/myso-tool /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
