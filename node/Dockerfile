##############################################################################################################
# This dockerfile is highly inspired by the following docker file.
# * [static-binaries.Dockerfile](https://github.com/Concordium/concordium-node/blob/main/scripts/static-binaries/static-binaries.Dockerfile)
# Please **do not** use this docker image in production. This is meant to be used for E2E tests.
##############################################################################################################

# Build static consensus libraries.
FROM concordium/static-libraries:rust-1.68.2_ghc-9.2.7 as static-builder
RUN apt-get update -y; \
    apt-get install libtinfo-dev;
RUN git clone https://github.com/Concordium/concordium-node.git; \
    cd concordium-node; \
    git checkout 5.2.4-0; \
    git submodule update --init --recursive; \
    cd ..; \
    mv concordium-node /build \
WORKDIR /build
ENV GHC_VERSION=9.2.7
RUN /build/scripts/static-libraries/build-static-libraries.sh

# Build static node binaries.
# FROM ubuntu:20.04
