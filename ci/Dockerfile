FROM docker.io/library/rust:1.75.0-bookworm@sha256:87f3b2f93b82995443a1a558c234212dafe79cfdc3af956539610560369ddcd0
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    cmake clang pkg-config libssl-dev protobuf-compiler libclang-dev libpq-dev \
    && rm -rf /var/lib/apt/lists/*
ENV RUSTUP_TOOLCHAIN=1.75.0
