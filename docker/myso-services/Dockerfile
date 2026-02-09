FROM rust:1.92-bullseye AS chef
WORKDIR myso
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y cmake clang libpq5 ca-certificates libpq-dev postgresql

# Build application
FROM chef AS builder 

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY myso-execution myso-execution
COPY external-crates external-crates
RUN cargo build --release \
    --bin myso-oracle
RUN mkdir /myso/bin/
RUN find /myso/target/release/ -maxdepth 1 -type f -executable -print 
RUN find /myso/target/release/ -maxdepth 1 -type f -executable -print | xargs cp -t /myso/bin/

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR myso
COPY --from=builder /myso/bin /tmp/myso-bin
RUN find /tmp/myso-bin -maxdepth 1 -type f -executable -print | xargs cp -t /usr/local/bin/
RUN rm -rf /tmp/myso-bin
RUN apt update && apt install -y libpq5 libpq-dev postgresql

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

