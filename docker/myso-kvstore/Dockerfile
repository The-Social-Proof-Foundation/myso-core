FROM rust:1.92-bullseye as build

ARG PROFILE=release
WORKDIR /work

RUN apt-get update && apt-get install -y cmake clang

COPY .git/ .git/
COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY myso-execution myso-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} --bin myso-kvstore-alt

FROM debian:12-slim as deploy
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libjemalloc-dev
COPY --from=build --chmod=755 /work/target/release/myso-kvstore-alt /opt/myso/bin/myso-kvstore-alt
