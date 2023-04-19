#!/bin/bash

GENESIS_DIR=/root/workdir/genesis
SECRETS_DIR=/root/secrets
DATADIR=/root/workdir/node-0

# generate genesis files
if [ ! -e "${GENESIS_DIR}/genesis.dat" ]; then
  echo "genesis.data file is missing. let's create one by generator."
  genesis-creator generate --config "/root/genesis.toml"
fi

# create node data directory
if [ ! -d "${DATADIR}" ]; then
  mkdir -p "${DATADIR}"
fi

concordium-node \
   --genesis-data-file ${GENESIS_DIR}/genesis.dat \
   --no-bootstrap= \
   --listen-address 0.0.0.0 \
   --listen-port 8000 \
   --rpc-server-addr 0.0.0.0 \
   --rpc-server-port 10000 \
   --grpc2-listen-addr 0.0.0.0 \
   --grpc2-listen-port 11000 \
   --data-dir ${DATADIR} \
   --config-dir ${DATADIR} \
   --baker-credentials-file ${SECRETS_DIR}/bakers/baker-0-credentials.json \
   --debug=
