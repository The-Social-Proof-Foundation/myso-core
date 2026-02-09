FROM ubuntu:latest AS chef
WORKDIR myso
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config curl cmake clang git ca-certificates
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM chef AS builder
RUN git clone https://github.comthe-social-proof-foundation/myso-core .
RUN git checkout devnet

RUN curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s
RUN curl -fLJO https://github.com/the-social-proof-foundation/myso-core-genesis/raw/main/devnet/genesis.blob
RUN cargo build --release --bin myso --bin myso-rosetta --bin myso-node

# Production Image
FROM ubuntu:latest AS runtime
WORKDIR myso
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /myso/target/release/myso /usr/local/bin
COPY --from=builder /myso/target/release/myso-node /usr/local/bin
COPY --from=builder /myso/target/release/myso-rosetta /usr/local/bin
COPY --from=builder /myso/bin/rosetta-cli /usr/local/bin
COPY --from=builder /myso/crates/myso-config/data/fullnode-template.yaml /myso/devnet/fullnode.yaml
COPY --from=builder /myso/genesis.blob /myso/devnet/genesis.blob
RUN /usr/local/bin/myso genesis

ARG BUILD_DATE
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
