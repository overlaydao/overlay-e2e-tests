# About this project

This project aims to achieve [Overlay](https://overlay.global/) concordium smart contract modules end-to-end test
automation.

# Genesis configuration

This project will launch a local [Concordium node](https://github.com/Concordium/concordium-node).
Concordium node requires to generate `genesis.dat` file before running a node. `genesis.dat` file is a set of
configuration data to run the node.

In this project, we prepare the set of configurations described as below.

🚧 WARNING **Do not use the same credentials for production environment** 🚧

| Name                       | Content                                                                                                          |
|----------------------------|------------------------------------------------------------------------------------------------------------------|
| baker account address      | 4NFzjJZ9hr3CWMTi7s9rrKofchCGeGMDbU9xnRpmynnc2hSKpi                                                               |
| baker account secrets      | [node/genesis/accounts/baker-0.json](node/genesis/accounts/baker-0.json)                                    |
| foundation account address | 4RTrmQ46r2dRgYE5r2b8Cv5dhR5Tc4QAGZQHjRSkaMyUhcFiCy                                                               |
| foundation account secrets | [node/genesis/accounts/foundation-1.json](node/genesis/accounts/foundation-1.json)                               |
| identity provider 0        | [node/genesis/secrets/identity-providers/ip-data-0.json](node/genesis/secrets/identity-providers/ip-data-0.json) |
| identity provider 1        | [node/genesis/secrets/identity-providers/ip-data-1.json](node/genesis/secrets/identity-providers/ip-data-1.json) |
| anonymity revoker 1        | [node/genesis/secrets/anonymity-revokers/ar-data-1.json](node/genesis/secrets/anonymity-revokers/ar-data-1.json) |
| anonymity revoker 2        | [node/genesis/secrets/anonymity-revokers/ar-data-1.json](node/genesis/secrets/anonymity-revokers/ar-data-1.json) |

For more information about genesis configuration, please refer to
[genesis-creator](https://github.com/Concordium/concordium-misc-tools/tree/main/genesis-creator) tool.

# How to launch local concordium node

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
[+] Running 1/0
 ⠿ Container concordium-node  Created                                                                             0.0s
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
concordium-node  | Genesis time is set to 2023-04-19 06:35:43.445 UTC.
concordium-node  | Average block time is set to 5000ms.
concordium-node  | There are 2 accounts in genesis, 1 of which are bakers.
concordium-node  | DONE
concordium-node  | 2023-04-19T06:35:44.036195641Z: INFO: Starting up concordium_node version 5.2.4!
concordium-node  | 2023-04-19T06:35:44.036208744Z: INFO: Application data directory: /root/workdir/node-0
concordium-node  | 2023-04-19T06:35:44.036210456Z: INFO: Application config directory: /root/workdir/node-0
concordium-node  | 2023-04-19T06:35:44.036211350Z: INFO: Network: enabled
concordium-node  | 2023-04-19T06:35:44.036212184Z: INFO: Log level: debug
concordium-node  | 2023-04-19T06:35:44.039391569Z: INFO: My Node ID is 70844257128a36c0
concordium-node  | 2023-04-19T06:35:44.039425116Z: INFO: Listening on 0.0.0.0:8000
concordium-node  | 2023-04-19T06:35:44.058295544Z: INFO: Starting consensus layer
concordium-node  | 2023-04-19T06:35:44.058338389Z: INFO: Starting up the consensus thread
concordium-node  | 2023-04-19T06:35:44.059312094Z: INFO: Starting up the consensus layer
concordium-node  | 2023-04-19T06:35:44.066544487Z: DEBUG: Skov: Attempting to use existing global state.
concordium-node  | 2023-04-19T06:35:44.070474241Z: DEBUG: Skov: No existing global state.
concordium-node  | 2023-04-19T06:35:44.314876469Z: INFO: Runner: Starting new chain at absolute height 0
concordium-node  | 2023-04-19T06:35:44.314950579Z: DEBUG: Skov: Creating new global state.
concordium-node  | 2023-04-19T06:35:44.344866191Z: DEBUG: Skov: Initializing finalization with context = { Ed25519Key = d1fc54608d8325aecaaf95cb2e2c5ef0caf9b36ff0f7e6551eedc3b6e857d5b0, VRFKey = 216fcd303a4447a3e18f7460807fb2537cb367d448b5bc2de0d20c4f1b21a05c, BLSKey = b2082c349b43de5339dd1ea6f6b3f698da6e2fa2a7b82bdd233e3c1de0e3cad1b4d7ea7466fa2a89c97e52ba41c7535c141d205fa9938e8583e07f7e049ec64f4d6c05a79a4e28ff382522c324a07b35bc80074606ac9b16b7841faf88fc3f09}
concordium-node  | 2023-04-19T06:35:44.345056929Z: DEBUG: Skov: Initializing finalization with initial state = BufferedFinalizationState {_bfsFinalization = finIndex: 1 finHeight: 1 currentRound:ActiveCurrentRound roundInput: NoInput roundDelta: 0
concordium-node  |  pendingMessages:[]
concordium-node  |  finQueue: FinalizationQueue {_fqFirstIndex = 1, _fqProofs = fromList []}, _bfsBuffer = FinalizationBuffer {_fbDelays = fromList [], _fbCurrentDelayStep = Nothing}}
concordium-node  | 2023-04-19T06:35:44.345588990Z: INFO: Consensus layer started
concordium-node  | 2023-04-19T06:35:44.345655047Z: INFO: Starting RPC server
concordium-node  | 2023-04-19T06:35:44.345662985Z: INFO: Starting GRPC V2 server listening on 0.0.0.0:11000
concordium-node  | 2023-04-19T06:35:44.345665680Z: DEBUG: GRPC endpoints enabled: ServiceConfig {
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
concordium-node  | 2023-04-19T06:35:44.345731244Z: INFO: Runner: Transaction purging thread started.
concordium-node  | 2023-04-19T06:35:44.346861875Z: INFO: Starting the P2P layer
concordium-node  | 2023-04-19T06:35:44.346935260Z: INFO: Commencing baking
concordium-node  | 2023-04-19T06:35:44.347691008Z: INFO: Runner: Starting baker thread
concordium-node  | 2023-04-19T06:35:44.448304962Z: DEBUG: Regenesis occurred; marking all peers as pending.
concordium-node  | 2023-04-19T06:35:45.433012728Z: INFO: Baker: Won lottery in 7(lottery power: 1 % 1; election difficulty: 5.0e-2)
concordium-node  | 2023-04-19T06:35:45.434585145Z: INFO: Scheduler: Constructed a block in 0.001315441s
concordium-node  | 2023-04-19T06:35:45.434643134Z: INFO: Baker: Baked block
concordium-node  | 2023-04-19T06:35:45.434952095Z: INFO: Skov: Block d1c1d62b48993a0c7edbea7a4ee58749ba7fe9b9b9f772164bf6ba39061284d4 (0) arrived
concordium-node  | 2023-04-19T06:35:45.435155639Z: INFO: Skov: Arrive statistics: blocksVerifiedCount=1 blockLastArrive=1.68188614543491e9 blockArriveLatencyEMA=3.9101450000000003e-4 blockArriveLatencyEMSD=1.1730435e-3 blockArrivePeriodEMA=Nothing blockArrivePeriodEMSD=Nothing transactionsPerBlockEMA=0.0 transactionsPerBlockEMSD=0.0
concordium-node  | 2023-04-19T06:35:45.435255975Z: INFO: Baker: Finished bake block d1c1d62b48993a0c7edbea7a4ee58749ba7fe9b9b9f772164bf6ba39061284d4
concordium-node  | 2023-04-19T06:35:45.435540007Z: DEBUG: Afgjort: Nominating block d1c1d62b48993a0c7edbea7a4ee58749ba7fe9b9b9f772164bf6ba39061284d4
concordium-node  | 2023-04-19T06:35:45.435902791Z: DEBUG: Afgjort: Handling message: [1:0] 0-> Propose d1c1d62b48993a0c7edbea7a4ee58749ba7fe9b9b9f772164bf6ba39061284d4
concordium-node  | 2023-04-19T06:35:45.436316430Z: DEBUG: Afgjort: Handling message: [1:0] 0-> Vote Just d1c1d62b48993a0c7edbea7a4ee58749ba7fe9b9b9f772164bf6ba39061284d4
concordium-node  | 2023-04-19T06:35:45.436731492Z: DEBUG: Afgjort: Handling message: [1:0] 0-> Justified@0: True
concordium-node  | 2023-04-19T06:35:45.437329112Z: DEBUG: Afgjort: Handling message: [1:0] 0-> Seen@0: {top: [0], bot: []}
concordium-node  | 2023-04-19T06:35:45.437656288Z: DEBUG: Afgjort: Handling message: [1:0] 0-> DoneReporting@0
concordium-node  | 2023-04-19T06:35:45.438106079Z: DEBUG: Afgjort: Handling message: [1:0] 0-> Justified@1: True
concordium-node  | 2023-04-19T06:35:45.438716083Z: DEBUG: Afgjort: Handling message: [1:0] 0-> WeAreDone: True
concordium-node  | 2023-04-19T06:35:45.439479574Z: DEBUG: Afgjort: Handling message: [1:0] 0-> Witness: (d1c1d62b48993a0c7edbea7a4ee58749ba7fe9b9b9f772164bf6ba39061284d4,aa2ae94a7df2f5c08851297cd890ea030cb628a4c62006b567a8aefb96aa554cac66d82423ee81fe3a521301f22bdf80)
concordium-node  | 2023-04-19T06:35:45.442016988Z: INFO: Skov: Block d1c1d62b48993a0c7edbea7a4ee58749ba7fe9b9b9f772164bf6ba39061284d4 is finalized at height 1 with finalization delta=0
concordium-node  | 2023-04-19T06:35:45.442088237Z: INFO: Skov: Finalization statistics: finalizationCount=1 lastFinalizedTime=1.681886145442049e9 finalizationPeriodEMA=Nothing finalizationPeriodEMSD=Nothing
concordium-node  | 2023-04-19T06:35:45.446409290Z: DEBUG: Skov: Blocks d1c1d62b48993a0c7edbea7a4ee58749ba7fe9b9b9f772164bf6ba39061284d4 marked finalized
concordium-node  | 2023-04-19T06:35:45.446967664Z: DEBUG: Skov: Processed finalization in 0.004922946s
concordium-node  | 2023-04-19T06:35:45.447244862Z: DEBUG: Afgjort: Starting finalization round: height=2 delta=0
concordium-node  | 2023-04-19T06:35:47.183572626Z: INFO: Baker: Won lottery in 14(lottery power: 1 % 1; election difficulty: 5.0e-2)
concordium-node  | 2023-04-19T06:35:47.185103550Z: INFO: Scheduler: Constructed a block in 0.001179053s
concordium-node  | 2023-04-19T06:35:47.185203861Z: INFO: Baker: Baked block
concordium-node  | 2023-04-19T06:35:47.185784673Z: INFO: Skov: Block 622d342700da631fba726e6a614a510bfa4b04fff4676cb6566a38a1d52f5c58 (0) arrived
concordium-node  | 2023-04-19T06:35:47.186129732Z: INFO: Skov: Arrive statistics: blocksVerifiedCount=2 blockLastArrive=1.681886147185677e9 blockArriveLatencyEMA=8.196079500000001e-4 blockArriveLatencyEMSD=1.700487887651408e-3 blockArrivePeriodEMA=Just 1.750766804 blockArrivePeriodEMSD=Just 0.0 transactionsPerBlockEMA=0.0 transactionsPerBlockEMSD=0.0
concordium-node  | 2023-04-19T06:35:47.186250400Z: INFO: Baker: Finished bake block 622d342700da631fba726e6a614a510bfa4b04fff4676cb6566a38a1d52f5c58
concordium-node  | 2023-04-19T06:35:47.186512823Z: DEBUG: Afgjort: Nominating block 622d342700da631fba726e6a614a510bfa4b04fff4676cb6566a38a1d52f5c58
concordium-node  | 2023-04-19T06:35:47.186915756Z: DEBUG: Afgjort: Handling message: [2:0] 0-> Propose 622d342700da631fba726e6a614a510bfa4b04fff4676cb6566a38a1d52f5c58
concordium-node  | 2023-04-19T06:35:47.187253511Z: DEBUG: Afgjort: Handling message: [2:0] 0-> Vote Just 622d342700da631fba726e6a614a510bfa4b04fff4676cb6566a38a1d52f5c58
concordium-node  | 2023-04-19T06:35:47.188269659Z: DEBUG: Afgjort: Handling message: [2:0] 0-> Justified@0: True
concordium-node  | 2023-04-19T06:35:47.189177320Z: DEBUG: Afgjort: Handling message: [2:0] 0-> Seen@0: {top: [0], bot: []}
concordium-node  | 2023-04-19T06:35:47.190134902Z: DEBUG: Afgjort: Handling message: [2:0] 0-> DoneReporting@0
concordium-node  | 2023-04-19T06:35:47.191247642Z: DEBUG: Afgjort: Handling message: [2:0] 0-> Justified@1: True
concordium-node  | 2023-04-19T06:35:47.192798244Z: DEBUG: Afgjort: Handling message: [2:0] 0-> WeAreDone: True
concordium-node  | 2023-04-19T06:35:47.194646519Z: DEBUG: Afgjort: Handling message: [2:0] 0-> Witness: (622d342700da631fba726e6a614a510bfa4b04fff4676cb6566a38a1d52f5c58,94100122b3fb7d36a7101ab48ae8a30f9a37e86de4bda43484721685bea111620a3d8132e3e5ad7ee776dff77f0c13df)
concordium-node  | 2023-04-19T06:35:47.199170441Z: INFO: Skov: Block 622d342700da631fba726e6a614a510bfa4b04fff4676cb6566a38a1d52f5c58 is finalized at height 2 with finalization delta=0
concordium-node  | 2023-04-19T06:35:47.199376905Z: INFO: Skov: Finalization statistics: finalizationCount=2 lastFinalizedTime=1.681886147199256e9 finalizationPeriodEMA=Just 1.757206961 finalizationPeriodEMSD=Just 0.0
concordium-node  | 2023-04-19T06:35:47.215048180Z: DEBUG: Skov: Blocks 622d342700da631fba726e6a614a510bfa4b04fff4676cb6566a38a1d52f5c58 marked finalized
concordium-node  | 2023-04-19T06:35:47.215910854Z: DEBUG: Skov: Processed finalization in 0.016757646s
concordium-node  | 2023-04-19T06:35:47.216359797Z: DEBUG: Afgjort: Starting finalization round: height=3 delta=0
concordium-node  | 2023-04-19T06:35:48.183424269Z: INFO: Baker: Won lottery in 18(lottery power: 1 % 1; election difficulty: 5.0e-2)
concordium-node  | 2023-04-19T06:35:48.185971820Z: INFO: Scheduler: Constructed a block in 0.001356613s
concordium-node  | 2023-04-19T06:35:48.186121556Z: INFO: Baker: Baked block
concordium-node  | 2023-04-19T06:35:48.186699603Z: INFO: Skov: Block ce62fad196525e2a8adc37f4b46951044e2a7bfd3e0e078a5883f417b923664a (0) arrived
concordium-node  | 2023-04-19T06:35:48.187052283Z: INFO: Skov: Arrive statistics: blocksVerifiedCount=3 blockLastArrive=1.6818861481866071e9 blockArriveLatencyEMA=1.2983563550000002e-3 blockArriveLatencyEMSD=2.1599290423656584e-3 blockArrivePeriodEMA=Just 1.6757831378999999 blockArrivePeriodEMSD=Just 0.22495099829999998 transactionsPerBlockEMA=0.0 transactionsPerBlockEMSD=0.0
concordium-node  | 2023-04-19T06:35:48.187143234Z: INFO: Baker: Finished bake block ce62fad196525e2a8adc37f4b46951044e2a7bfd3e0e078a5883f417b923664a
concordium-node  | 2023-04-19T06:35:48.187320484Z: DEBUG: Afgjort: Nominating block ce62fad196525e2a8adc37f4b46951044e2a7bfd3e0e078a5883f417b923664a
concordium-node  | 2023-04-19T06:35:48.187702852Z: DEBUG: Afgjort: Handling message: [3:0] 0-> Propose ce62fad196525e2a8adc37f4b46951044e2a7bfd3e0e078a5883f417b923664a
concordium-node  | 2023-04-19T06:35:48.188106386Z: DEBUG: Afgjort: Handling message: [3:0] 0-> Vote Just ce62fad196525e2a8adc37f4b46951044e2a7bfd3e0e078a5883f417b923664a
concordium-node  | 2023-04-19T06:35:48.189284548Z: DEBUG: Afgjort: Handling message: [3:0] 0-> Justified@0: True
concordium-node  | 2023-04-19T06:35:48.190467318Z: DEBUG: Afgjort: Handling message: [3:0] 0-> Seen@0: {top: [0], bot: []}
concordium-node  | 2023-04-19T06:35:48.191219377Z: DEBUG: Afgjort: Handling message: [3:0] 0-> DoneReporting@0
concordium-node  | 2023-04-19T06:35:48.192576166Z: DEBUG: Afgjort: Handling message: [3:0] 0-> Justified@1: True
concordium-node  | 2023-04-19T06:35:48.194248199Z: DEBUG: Afgjort: Handling message: [3:0] 0-> WeAreDone: True
concordium-node  | 2023-04-19T06:35:48.196015961Z: DEBUG: Afgjort: Handling message: [3:0] 0-> Witness: (ce62fad196525e2a8adc37f4b46951044e2a7bfd3e0e078a5883f417b923664a,a8f22cbd61f09658760d35ee484c0b9e5fb618ecfe911031e5983712a8676b801b1f1b8376d66e77d25e3d87d64de754)
concordium-node  | 2023-04-19T06:35:48.200841833Z: INFO: Skov: Block ce62fad196525e2a8adc37f4b46951044e2a7bfd3e0e078a5883f417b923664a is finalized at height 3 with finalization delta=0
concordium-node  | 2023-04-19T06:35:48.201068503Z: INFO: Skov: Finalization statistics: finalizationCount=3 lastFinalizedTime=1.6818861482009478e9 finalizationPeriodEMA=Just 1.6816554306 finalizationPeriodEMSD=Just 0.2266545912
concordium-node  | 2023-04-19T06:35:48.213445906Z: DEBUG: Skov: Blocks ce62fad196525e2a8adc37f4b46951044e2a7bfd3e0e078a5883f417b923664a marked finalized
concordium-node  | 2023-04-19T06:35:48.213874195Z: DEBUG: Skov: Processed finalization in 0.013056297s
concordium-node  | 2023-04-19T06:35:48.214579797Z: DEBUG: Afgjort: Starting finalization round: height=4 delta=0
```

# How to build and run overlay-e2e-tests

## Prerequisite

You need to install the following tools to build this smart contract source codes.

1. [rustup](https://rustup.rs/)
2. [cargo-concordium](https://developer.concordium.software/en/mainnet/net/installation/downloads-testnet.html#cargo-concordium-testnet)

Please refer to the [Concordium official Quick start guide](https://developer.concordium.software/en/mainnet/smart-contracts/guides/quick-start.html)
for more information.

## Build

At the top of this project directory, hit the following commands to build.

```shell
% cargo build
```

## How to run

```shell
# create initial account
% cargo overlay-e2e-tests test-account create --identity-provider="$(pwd)/node/genesis/secrets/identity-providers/ip-data-0.json" --anonymity-revoker="$(pwd)/node/genesis/secrets/anonymity-revokers/ar-data-1.json" --output=tmp/test.json

# get account information
% cargo overlay-e2e-tests test-account get --address=4TL3MZVnSrUBcPh5DQKPsjrnmfm9x6XCRS6U9V1XF2e7dLXx4n

# send ccd from foundation account to a test account.
% cargo overlay-e2e-tests test-account send-ccd --from="$(pwd)/node/genesis/accounts/foundation-1.json" --amount=1 --to=4TL3MZVnSrUBcPh5DQKPsjrnmfm9x6XCRS6U9V1XF2e7dLXx4n
```
