#!/usr/bin/env bash

TAG=${1:-latest}
IMAGE_NAME="concordium-local-node:${TAG}"
echo "let's build ${IMAGE_NAME} docker image."
docker build -t ${IMAGE_NAME} -f Dockerfile .
