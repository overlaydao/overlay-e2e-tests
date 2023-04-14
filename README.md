# Overlay E2E Tests

This project aims to achieve Overlay concordium smart contract modules end-to-end test automation.


## How to launch local concordium node

* At first, build concordium node docker image by the following steps.

```shell
% cd node
% ./build.sh
# You will have to wait looong time to complete docker build.(Be patient!)
# then you can get `concordium-local-node:latest` image on your machine.
```

**note**

> If you want to try any patch of concordium node itself, you can do it by customising `node/Dockerfile`.

* Then, run `docker-compose up` on the root directory.

```shell
% docker-compose up
[+] Running 1/1
 ⠿ Container concordium-node  Recreated                                                                                                          0.1s
Attaching to concordium-node
concordium-node  | genesis.data file is missing. let's create one by generator.
concordium-node  | Deleting any existing directories.
concordium-node  | Account keys will be generated in /root/workdir/genesis/accounts
concordium-node  | Chain update keys will be generated in /root/workdir/genesis/update-keys
concordium-node  | Identity providers will be generated in /root/workdir/genesis/idps
concordium-node  | Anonymity revokers will be generated in /root/workdir/genesis/ars
concordium-node  | Baker keys will be generated in /root/workdir/genesis/bakers
concordium-node  | Cryptographic parameter will be generated in /root/workdir/genesis/global
concordium-node  | The genesis data will be stored in /root/workdir/genesis/genesis.dat
concordium-node  | The genesis hash will be written to /root/workdir/genesis/genesis_hash
concordium-node  | Genesis time is set to 2023-04-14 05:06:43.619 UTC.
concordium-node  | Average block time is set to 5000ms.
concordium-node  | There are 6 accounts in genesis, 5 of which are bakers.
concordium-node  | DONE
concordium-node  | 2023-04-14T05:06:44.187237437Z: INFO: Starting up concordium_node version 5.2.4!
concordium-node  | 2023-04-14T05:06:44.187302869Z: INFO: Application data directory: /root/workdir/node-0
concordium-node  | 2023-04-14T05:06:44.187305707Z: INFO: Application config directory: /root/workdir/node-0
concordium-node  | 2023-04-14T05:06:44.187307012Z: INFO: Network: enabled
concordium-node  | 2023-04-14T05:06:44.187308178Z: INFO: Log level: debug
concordium-node  | 2023-04-14T05:06:44.190012826Z: INFO: My Node ID is 47bd189ece76d0df
concordium-node  | 2023-04-14T05:06:44.190054619Z: INFO: Listening on 0.0.0.0:8000
concordium-node  | 2023-04-14T05:06:44.211365037Z: INFO: Starting consensus layer
concordium-node  | 2023-04-14T05:06:44.211398729Z: INFO: Starting up the consensus thread
concordium-node  | 2023-04-14T05:06:44.212165601Z: INFO: Starting up the consensus layer
concordium-node  | 2023-04-14T05:06:44.218649939Z: DEBUG: Skov: Attempting to use existing global state.
concordium-node  | 2023-04-14T05:06:44.220167151Z: DEBUG: Skov: No existing global state.
concordium-node  | 2023-04-14T05:06:44.497358446Z: INFO: Runner: Starting new chain at absolute height 0
concordium-node  | 2023-04-14T05:06:44.497542415Z: DEBUG: Skov: Creating new global state.
concordium-node  | 2023-04-14T05:06:44.561196304Z: DEBUG: Skov: Initializing finalization with context = { Ed25519Key = faf9d66865c065ece81a1f2b1408d02e56c26ad918909edb7d4710151faba0cb, VRFKey = b2a18921a1365fa598e1c3448008de08af68908c3cd664dd0267eb0b9932f020, BLSKey = afe1f1901e88a0563a42e53fc2f49c11d0ce45c993e89a84b34b3cdbfa060bafd323b8f13f9ce415b1c8668a19a3fe840fa52c0acf6d0700b6fe39a88f9e575d2b9767e276d321718cee99e3fba82e3ee52ba66c8ee82248c1960789ec9d4462}
concordium-node  | 2023-04-14T05:06:44.561366982Z: DEBUG: Skov: Initializing finalization with initial state = BufferedFinalizationState {_bfsFinalization = finIndex: 1 finHeight: 1 currentRound:ActiveCurrentRound roundInput: NoInput roundDelta: 0
concordium-node  |  pendingMessages:[]
concordium-node  |  finQueue: FinalizationQueue {_fqFirstIndex = 1, _fqProofs = fromList []}, _bfsBuffer = FinalizationBuffer {_fbDelays = fromList [], _fbCurrentDelayStep = Nothing}}
concordium-node  | 2023-04-14T05:06:44.561956157Z: INFO: Consensus layer started
concordium-node  | 2023-04-14T05:06:44.562031742Z: INFO: Starting RPC server
concordium-node  | 2023-04-14T05:06:44.562043688Z: INFO: Starting GRPC V2 server listening on 0.0.0.0:11000
concordium-node  | 2023-04-14T05:06:44.562046029Z: DEBUG: GRPC endpoints enabled: ServiceConfig {
concordium-node  |     get_finalized_blocks: true,
concordium-node  |     get_blocks: true,
concordium-node  |     get_account_list: true,
concordium-node  |     get_account_info: true,
concordium-node  |     get_module_list: true,
concordium-node  |     get_module_source: true,
concordium-node  |     get_instance_list: true,
concordium-node  |     get_instance_info: true,
concordium-node  |     get_instance_state: true,
concordium-node  |     instance_state_lookup: true,
concordium-node  |     get_next_account_sequence_number: true,
concordium-node  |     get_consensus_info: true,
concordium-node  |     get_ancestors: true,
concordium-node  |     get_block_item_status: true,
concordium-node  |     invoke_instance: true,
concordium-node  |     get_cryptographic_parameters: true,
concordium-node  |     get_block_info: true,
concordium-node  |     get_baker_list: true,
concordium-node  |     get_pool_info: true,
concordium-node  |     get_passive_delegation_info: true,
concordium-node  |     get_blocks_at_height: true,
concordium-node  |     get_tokenomics_info: true,
concordium-node  |     get_pool_delegators: true,
concordium-node  |     get_pool_delegators_reward_period: true,
concordium-node  |     get_passive_delegators: true,
concordium-node  |     get_passive_delegators_reward_period: true,
concordium-node  |     get_branches: true,
concordium-node  |     get_election_info: true,
concordium-node  |     get_identity_providers: true,
concordium-node  |     get_anonymity_revokers: true,
concordium-node  |     get_account_non_finalized_transactions: true,
concordium-node  |     get_block_transaction_events: true,
concordium-node  |     get_block_special_events: true,
concordium-node  |     get_block_pending_updates: true,
concordium-node  |     get_next_update_sequence_numbers: true,
concordium-node  |     get_block_chain_parameters: true,
concordium-node  |     get_block_finalization_summary: true,
concordium-node  |     shutdown: true,
concordium-node  |     peer_connect: true,
concordium-node  |     peer_disconnect: true,
concordium-node  |     get_banned_peers: true,
concordium-node  |     ban_peer: true,
concordium-node  |     unban_peer: true,
concordium-node  |     dump_start: true,
concordium-node  |     dump_stop: true,
concordium-node  |     get_peers_info: true,
concordium-node  |     get_node_info: true,
concordium-node  |     send_block_item: true,
concordium-node  |     get_account_transaction_sign_hash: true,
concordium-node  |     get_block_items: true,
concordium-node  | }
concordium-node  | 2023-04-14T05:06:44.562070157Z: INFO: Runner: Transaction purging thread started.
concordium-node  | 2023-04-14T05:06:44.563335164Z: INFO: Starting the P2P layer
concordium-node  | 2023-04-14T05:06:44.563342940Z: INFO: Commencing baking
concordium-node  | 2023-04-14T05:06:44.563899553Z: INFO: Runner: Starting baker thread
concordium-node  | 2023-04-14T05:06:44.665303105Z: DEBUG: Regenesis occurred; marking all peers as pending.
concordium-node  | 2023-04-14T05:07:04.386239853Z: INFO: Baker: Won lottery in 82(lottery power: 1 % 5; election difficulty: 5.0e-2)
concordium-node  | 2023-04-14T05:07:04.388999871Z: INFO: Scheduler: Constructed a block in 0.00256006s
concordium-node  | 2023-04-14T05:07:04.389088954Z: INFO: Baker: Baked block
concordium-node  | 2023-04-14T05:07:04.389293725Z: INFO: Skov: Block eaa89656ff2060bbefe510266364c03a9a05bdda6a30e9af8e602b46f6922384 (0) arrived
concordium-node  | 2023-04-14T05:07:04.389496276Z: INFO: Skov: Arrive statistics: blocksVerifiedCount=1 blockLastArrive=1.68144882438927e9 blockArriveLatencyEMA=5.270052000000001e-4 blockArriveLatencyEMSD=1.5810156e-3 blockArrivePeriodEMA=Nothing blockArrivePeriodEMSD=Nothing transactionsPerBlockEMA=0.0 transactionsPerBlockEMSD=0.0
concordium-node  | 2023-04-14T05:07:04.389535608Z: INFO: Baker: Finished bake block eaa89656ff2060bbefe510266364c03a9a05bdda6a30e9af8e602b46f6922384
concordium-node  | 2023-04-14T05:07:04.389618761Z: DEBUG: Afgjort: Nominating block eaa89656ff2060bbefe510266364c03a9a05bdda6a30e9af8e602b46f6922384
concordium-node  | 2023-04-14T05:07:04.389871962Z: DEBUG: Afgjort: Handling message: [1:0] 0-> Propose eaa89656ff2060bbefe510266364c03a9a05bdda6a30e9af8e602b46f6922384
```