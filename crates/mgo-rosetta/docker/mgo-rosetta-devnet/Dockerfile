FROM ubuntu:latest AS chef
WORKDIR mgo
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config curl cmake clang git ca-certificates
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM chef AS builder
RUN git clone https://github.com/MangoNetworkOs/Mango .
RUN git checkout devnet

RUN curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s
RUN curl -fLJO https://github.com/MangoNetworkOs/Mango-genesis/raw/main/devnet/genesis.blob
RUN cargo build --release --bin mgo --bin mgo-rosetta --bin mgo-node

# Production Image
FROM ubuntu:latest AS runtime
WORKDIR mgo
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /mgo/target/release/mgo /usr/local/bin
COPY --from=builder /mgo/target/release/mgo-node /usr/local/bin
COPY --from=builder /mgo/target/release/mgo-rosetta /usr/local/bin
COPY --from=builder /mgo/bin/rosetta-cli /usr/local/bin
COPY --from=builder /mgo/crates/mgo-config/data/fullnode-template.yaml /mgo/devnet/fullnode.yaml
COPY --from=builder /mgo/genesis.blob /mgo/devnet/genesis.blob
RUN /usr/local/bin/mgo genesis

ARG BUILD_DATE
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
