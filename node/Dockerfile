##############################################################################################################
# Please **do not** use this docker image in production. This is meant to be used for E2E tests.
# docker build -t concordium-node:draft -f Dockerfile .
##############################################################################################################
FROM ubuntu:20.04

ENV BOOTSTRAP_HASKELL_NONINTERACTIVE=1
ENV GHC_VERSION=9.2.5
ENV PROTOC_VERSION=3.15.3
ENV FLATBUFFERS_VERSION=v22.10.26
ENV RUST_TOOLCHAIN_VERSION=1.62
ENV HOME=/root

# install build dependencies
RUN apt-get update -y; \
    DEBIAN_FRONTEND=noninteractive apt-get -y install \
      git \
      curl \
      libprotobuf-dev \
      libssl-dev \
      cmake \
      pkg-config \
      libnuma-dev \
      libgmp-dev \
      liblmdb0 \
      locales \
      liblmdb-dev \
      unzip \
      build-essential \
      clang \
      lldb \
      lld;

WORKDIR /usr/local/src

# Install GHCup
ENV PATH="${HOME}/.local/bin:${HOME}/.ghcup/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh; \
    ghcup install ghc ${GHC_VERSION}; \
    ghcup set ghc ${GHC_VERSION}

# Install protobuf
RUN curl -L https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip -o protoc.zip; \
    unzip protoc.zip bin/protoc -d /usr/; \
    rm protoc.zip;

# Install Rust
ENV PATH="${HOME}/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
    rustup set profile minimal; \
    rustup default "${RUST_TOOLCHAIN_VERSION}";

# Install flatbuffers.
RUN git clone https://github.com/google/flatbuffers.git; \
    ( cd flatbuffers && git checkout "${FLATBUFFERS_VERSION}" && cmake -G "Unix Makefiles" && make -j"$(nproc)" && make install )

# Install concordium-node
## checkout latest release
RUN git clone https://github.com/Concordium/concordium-node.git; \
    cd concordium-node; \
    git checkout 5.2.4-0; \
    git submodule update --init --recursive;

## Build concordium-consensus
WORKDIR /usr/local/src/concordium-node/concordium-consensus
RUN stack build
# include library path so that the concordium-node executable can call the consensus program dynamically.
ENV LD_LIBRARY_PATH="/root/.ghcup/ghc/${GHC_VERSION}/lib/ghc-${GHC_VERSION}/rts/:/usr/local/src/concordium-node/concordium-consensus/.stack-work/dist/x86_64-linux/Cabal-3.6.3.0/build"

## Build concordium-node
WORKDIR /usr/local/src/concordium-node/concordium-node
RUN cargo install --path . --locked

# Install genesis-creator
WORKDIR /usr/local/src
RUN git clone https://github.com/Concordium/concordium-misc-tools.git; \
    cd concordium-misc-tools; \
    git submodule update --init --recursive; \
    cd genesis-creator; \
    cargo install --path . --locked;

WORKDIR /root/workdir

# add configs for genesis data
COPY genesis /root/

# add utility scripts
ADD ./entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

EXPOSE 8000
EXPOSE 10000
EXPOSE 11000

CMD ["entrypoint.sh"]
