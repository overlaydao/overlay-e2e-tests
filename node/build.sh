#!/usr/bin/env bash

TAG=${TAG:-latest}

docker build -t concordium-local-node:${TAG} -f Dockerfile .
