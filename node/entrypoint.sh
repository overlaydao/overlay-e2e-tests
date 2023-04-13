#!/bin/bash

WORKDIR=/root/workdir
DATADIR=/root/workdir/node-0

# generate genesis files
if [ ! -e "${WORKDIR}/genesis.dat" ]; then
  echo "genesis.data file is missing. let's create one by generator."
  genesis-creator generate --config "${WORKDIR}/genesis.toml"
fi

# create node data directory
if [ ! -d "${DATADIR}" ]; then
  mkdir -p "${DATADIR}"
fi

concordium-node \
   --genesis-data-file ${WORKDIR}/genesis.dat \
   --no-bootstrap= \
   --listen-port 8000 \
   --rpc-server-port 10000 \
   --grpc2-listen-addr 0.0.0.0 \
   --grpc2-listen-port 11000 \
   --data-dir ${DATADIR} \
   --config-dir ${DATADIR} \
   --baker-credentials-file ${WORKDIR}/bakers/baker-0-credentials.json \
   --debug=
