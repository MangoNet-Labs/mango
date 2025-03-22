# Mango for Node Operators

## Overview

This document is focused on running the Mango Node software as a Validator.

## Requirements

To run a Mango Validator a machine with the following is required:
- CPU: 24 physical cores (or 48 virtual cores)
- Memory: 128 GB
- Storage: 4 TB NVME
- Network: 1 Gbps

## Configuration

Mango Node runs with a single configuration file provided as an argument, example:

```

validator.yaml
---
protocol-key-pair:
  value: CizYW5zYfTE3zPiKIhtTUW6wTcOLzJr5FDscw5yckNQ=
worker-key-pair:
  value: AP4osc3oYVlGx7GyB3BYoZhv1SBXEomXZStxCeAGT3Ox
account-key-pair:
  value: AC6o+NhGiACICv47udvDc1tiZehGUDrfu/0yKuXBB0P0
network-key-pair:
  value: AGzVTQg3S8UCTrVImHhnTsWr/RkaWWypKDFPxb/efnoS
db-path: /root/mgo/db/authorities_db/a3b5d168f135
network-address: /ip4/0.0.0.0/tcp/8080/http
json-rpc-address: "127.0.0.1:57082"
enable-experimental-rest-api: true
metrics-address: "127.0.0.1:8085"
admin-interface-port: 57080
consensus-config:
  address: /ip4/127.0.0.1/tcp/8083/http
  db-path: /root/mgo/db/consensus_db/a3b5d168f135
  internal-worker-address: ~
  max-pending-transactions: ~
  max-submit-position: ~
  submit-delay-step-override-millis: ~
  narwhal-config:
    header_num_of_batches_threshold: 32
    max_header_num_of_batches: 1000
    max_header_delay: 1000ms
    min_header_delay: 500ms
    gc_depth: 50
    sync_retry_delay: 5000ms
    sync_retry_nodes: 3
    batch_size: 5000000
    max_batch_delay: 100ms
    max_concurrent_requests: 500000
    prometheus_metrics:
      socket_addr: /ip4/127.0.0.1/tcp/8086/http
    network_admin_server:
      primary_network_admin_server_port: 8081
      worker_network_admin_server_base_port: 8082
    anemo:
      send_certificate_rate_limit: ~
      report_batch_rate_limit: ~
      request_batches_rate_limit: ~
enable-event-processing: false
enable-index-processing: true
grpc-load-shed: ~
grpc-concurrency-limit: 20000000000
p2p-config:
  listen-address: "0.0.0.0:8084"
  external-address: /ip4/127.0.0.1/udp/8084/http
  seed-peers:    
  - peer-id: ...
    address: /dns/.../udp/8084/http 
genesis:
  genesis-file-location: /root/genesis.blob
authority-store-pruning-config:
  num-latest-epoch-dbs-to-retain: 3
  epoch-db-pruning-period-secs: 3600
  num-epochs-to-retain: 18446744073709551615
  num-epochs-to-retain-for-checkpoints: 720
  max-checkpoints-in-batch: 10
  max-transactions-in-batch: 1000
end-of-epoch-broadcast-channel-capacity: 128
checkpoint-executor-config:
  checkpoint-execution-max-concurrency: 200
  local-execution-timeout-sec: 30
db-checkpoint-config:
  perform-db-checkpoints-at-epoch-end: false
indirect-objects-threshold: 18446744073709551615
expensive-safety-check-config:
  enable-epoch-mgo-conservation-check: false
  enable-deep-per-tx-mgo-conservation-check: false
  force-disable-epoch-mgo-conservation-check: false
  enable-state-consistency-check: false
  force-disable-state-consistency-check: false
  enable-secondary-index-checks: false
transaction-deny-config:
  package-publish-disabled: false
  package-upgrade-disabled: false
  shared-object-disabled: false
  user-transaction-disabled: false
  receiving-objects-disabled: false
  zklogin-sig-disabled: false
  zklogin-disabled-providers: []
certificate-deny-config: {}
state-debug-dump-config: {}
state-archive-write-config:
  concurrency: 0
  use-for-pruning-watermark: false
state-archive-read-config: []
state-snapshot-write-config:
  concurrency: 0
indexer-max-subscriptions: ~
transaction-kv-store-read-config:
  base-url: ""
jwk-fetch-interval-seconds: 3600
zklogin-oauth-providers:
  Mainnet:
    - Facebook
    - Google
    - Twitch
  Testnet:
    - Facebook
    - Google
    - Twitch
  Unknown:
    - Apple
    - Facebook
    - Google
    - Kakao
    - Slack
    - Twitch
overload-threshold-config:
  max-txn-age-in-queue:
    secs: 1
    nanos: 0
  overload-monitor-interval:
    secs: 10
    nanos: 0
  execution-queue-latency-soft-limit:
    secs: 1
    nanos: 0
  execution-queue-latency-hard-limit:
    secs: 10
    nanos: 0
  max-load-shedding-percentage: 95
  min-load-shedding-percentage-above-hard-limit: 50
---

mgo-node --config-path ./validator.yaml

```

## Connectivity

Mango Node uses the following ports by default:

| protocol/port | reachability     | purpose                           |
| ------------- | ---------------- | --------------------------------- |
| TCP/8080      | inbound          | protocol/transaction interface    |
| UDP/8081      | inbound/outbound | narwhal primary interface         |
| UDP/8082      | inbound/outbound | narwhal worker interface          |
| TCP/8083      | localhost        | mgo -> narwhal interface          |
| UDP/8084      | inbound/outbound | peer to peer state sync interface |
| TCP/8443      | outbound         | metrics pushing                   |
| TCP/9184      | localhost        | metrics scraping                  |

To run a validator successfully it is critical that ports 8080-8084 are open as outlined above, including the specific protocol (TCP/UDP).

## Key Management

The following keys are used by Mango Node:

| key          | scheme   | purpose                         |
| ------------ | -------- | ------------------------------- |
| protocol.key | bls12381 | transactions, narwhal consensus |
| account.key  | ed25519  | controls assets for staking     |
| network.key  | ed25519  | narwhal primary, mgo state sync |
| worker.key   | ed25519  | validate narwhal workers        |

## Monitoring

### Metrics

Mango Node exposes metrics via a local HTTP interface. These can be scraped for use in a central monitoring system as well as viewed directly from the node.

- View all metrics:

```shell
curl -s http://localhost:9184/metrics
```

- Search for a particular metric:

```shell
curl http://localhost:9184/metrics | grep <METRIC>
```

Mango Node also pushes metrics to a central Mango metrics proxy.

### Logs

Logs are controlled using the `RUST_LOG` environment variable.

The `RUST_LOG_JSON=1` environment variable can optionally be set to enable logging in JSON structured format.

To view and follow the Mango Node logs:

```shell
journalctl -u mgo-node -f
```

To search for a particular match

```shell
journalctl -u mgo-node -g <SEARCH_TERM>
```

- If using Docker Compose, look at the examples [here]()

It is possible to change the logging configuration while a node is running using the admin interface.

To view the currently configured logging values:

```shell
curl localhost:1337/logging
```

To change the currently configured logging values:

```shell
curl localhost:1337/logging -d "info"
```




## State Sync

Checkpoints in Mango contain the permanent history of the network. They are comparable to blocks in other blockchains with one big difference being that they are lagging instead of leading. All transactions are final and executed prior to being included in a checkpoint.

These checkpoints are synchronized between validators and fullnodes via a dedicated peer to peer state sync interface.

Inter-validator state sync is always permitted however there are controls available to limit what fullnodes are allowed to sync from a specific validator.

The default and recommended `max-concurrent-connections: 0` configuration does not affect inter-validator state sync, but will restrict all fullnodes from syncing. The Mango Node [configuration](#configuration) can be modified to allow a known fullnode to sync from a validator:

```shell
p2p-config:
  anemo-config:
    max-concurrent-connections: 0
  seed-peers:
    - address: <multiaddr>  # The p2p address of the fullnode
      peer-id: <peer-id>    # hex encoded network public key of the node
    - address: ...          # another permitted peer
      peer-id: ...
```

## Chain Operations

The following chain operations are executed using the `mgo` CLI. This binary is built and provided as a release similar to `mgo-node`, examples:

```shell
wget xxxxxxxx/mgo
chmod +x mgo
```

```shell
curl xxxxxxx/mgo -o mgo
chmod +x mgo
```

It is recommended and often required that the `mgo` binary release/version matches that of the deployed network.

### Updating On-chain Metadata

You can leverage [Validator Tool](validator.md) to perform majority of the following tasks.

An active/pending validator can update its on-chain metadata by submitting a transaction. Some metadata changes take effect immediately, including:

- name
- description
- image url
- project url

Other metadata (keys, addresses etc) only come into effect at the next epoch.

To update metadata, a validator makes a MoveCall transaction that interacts with the System Object. For example:

1. to update name to `new_validator_name`, use the Mango Client CLI to call `mgo_system::update_validator_name`:

```
mgo client call --package 0x3 --module mgo_system --function update_validator_name --args 0x5 \"new_validator_name\" --gas-budget 10000
```

2. to update p2p address starting from next epoch to `/ip4/192.168.10.1`, use the Mango Client CLI to call `mgo_system::update_validator_next_epoch_p2p_address`:

```
mgo client call --package 0x3 --module mgo_system --function update_validator_next_epoch_p2p_address --args 0x5 "[4, 192, 168, 1, 1]" --gas-budget 1000000000
```

See the full list of metadata update functions here

### Operation Cap

To avoid touching account keys too often and allowing them to be stored off-line, validators can delegate the operation ability to another address. This address can then update the reference gas price and tallying rule on behalf of the validator.

Upon creating a `Validator`, an `UnverifiedValidatorOperationCap` is created as well and transferred to the validator address. The holder of this `Cap` object (short for "Capability") therefore could perform operational actions for this validator. To authorize another address to conduct these operations, a validator transfers the object to another address that they control. The transfer can be done by using Mango Client CLI: `mgo client transfer`.

To rotate the delegatee address or revoke the authorization, the current holder of `Cap` transfers it to another address. In the event of compromised or lost keys, the validator could create a new `Cap` object to invalidate the incumbent one. This is done by calling `mgo_system::rotate_operation_cap`:

```
mgo client call --package 0x3 --module mgo_system --function rotate_operation_cap --args 0x5 --gas-budget 10000
```

By default the new `Cap` object is transferred to the validator address, which then could be transferred to the new delegatee address. At this point, the old `Cap` becomes invalidated and no longer represents eligibility.

To get the current valid `Cap` object's ID of a validator, use the Mango Client CLI `mgo client objects` command after setting the holder as the active address. Or go to the [explorer]() and look for `operation_cap_id` of that validator in the `validators` module.

### Updating the Gas Price Survey Quote

To update the Gas Price Survey Quote of a validator, which is used to calculate the Reference Gas Price at the end of the epoch, the sender needs to hold a valid [`UnverifiedValidatorOperationCap`](#operation-cap). The sender could be the validator itself, or a trusted delegatee. To do so, call `mgo_system::request_set_gas_price`:

```
mgo client call --package 0x3 --module mgo_system --function request_set_gas_price --args 0x5 {cap_object_id} {new_gas_price} --gas-budget 1000000000
```

### Reporting/Un-reporting Validators

To report a validator or undo an existing reporting, the sender needs to hold a valid [`UnverifiedValidatorOperationCap`](#operation-cap). The sender could be the validator itself, or a trusted delegatee. To do so, call `mgo_system::report_validator/undo_report_validator`:

```
mgo client call --package 0x3 --module mgo_system --function report_validator/undo_report_validator --args 0x5 {cap_object_id} {reportee_address} --gas-budget 1000000000
```

Once a validator is reported by `2f + 1` other validators by voting power, their staking rewards will be slashed.

### Joining the Validator Set

In order for a Mango address to join the validator set, they need to first sign up as a validator candidate by calling `mgo_system::request_add_validator_candidate` with their metadata and initial configs:

```
mgo client call --package 0x3 --module mgo_system --function request_add_validator_candidate --args 0x5 {protocol_pubkey_bytes} {network_pubkey_bytes} {worker_pubkey_bytes} {proof_of_possession} {name} {description} {image_url} {project_url} {net_address}
{p2p_address} {primary_address} {worker_address} {gas_price} {commission_rate} --gas-budget 1000000000
```

After an address becomes a validator candidate, any address (including the candidate address itself) can start staking with the candidate's staking pool. Refer to our dedicated staking FAQ on how staking works. Once a candidate's staking pool has accumulated at least `mgo_system::MIN_VALIDATOR_JOINING_STAKE` amount of stake, the candidate can call `mgo_system::request_add_validator` to officially add themselves to the next epoch's active validator set:

```
mgo client call --package 0x3 --module mgo_system --function request_add_validator --args 0x5 --gas-budget 100000000
```

### Leaving the Validator Set

To leave the validator set starting the next epoch, the sender needs to be an active validator in the current epoch and should call `mgo_system::request_remove_validator`:

```
mgo client call --package 0x3 --module mgo_system --function request_remove_validator --args 0x5 --gas-budget 1000000000
```

After the validator is removed at the next epoch change, the staking pool will become inactive and stakes can only be withdrawn from an inactive pool.
