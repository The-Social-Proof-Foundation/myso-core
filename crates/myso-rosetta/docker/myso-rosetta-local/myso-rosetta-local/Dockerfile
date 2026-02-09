FROM ubuntu:latest AS chef
WORKDIR myso
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config curl cmake clang ca-certificates
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Build application
FROM chef AS builder

RUN curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY myso-execution myso-execution
COPY external-crates external-crates
RUN cargo build --release --bin myso --bin myso-rosetta

# Production Image
FROM ubuntu:latest AS runtime
WORKDIR myso
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /myso/target/release/myso /usr/local/bin
COPY --from=builder /myso/target/release/myso-rosetta /usr/local/bin
COPY --from=builder /myso/bin/rosetta-cli /usr/local/bin
COPY --from=builder /myso/crates/myso-config/data/fullnode-template.yaml /myso/devnet/fullnode.yaml
RUN /usr/local/bin/myso genesis

ARG BUILD_DATE
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
