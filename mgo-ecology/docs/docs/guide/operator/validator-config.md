---
title: Mango Validator Node Configuration
description: Learn how to set up, configure, and manage a Mango Validator node, including staking, reference gas price, and tallying rules.
sidebar_position: 5
---


# Become a Validator

Validators on the Mango network run special nodes and have additional tasks and responsibilities beyond those of Full node operators..

### More precisely

A validator candidate must accrue at least 30M MGO of stake before they can request to join the validator set.
If an active validator’s stake falls below 20M MGO, they have seven epochs of grace period to gain back the stake before being removed from the validator set.
If an active validator’s stake falls below 15M MGO, they are removed from the validator set at the end of the current epoch boundary. Mango uses 1-hour epochs.

### Prepare what you need

```
root@iZt4n0h02f2iwp3mlskZ:~/mango# ll
total 231760
drwxr-xr-x  2 root root      4096 Mar 19 15:12 ./
drwx------ 11 root root      4096 Mar 20 09:55 ../
-rwxr-xr-x  1 root root 135485240 Mar 19 11:33 mgo*
-rwxr-xr-x  1 root root 101822080 Mar 19 11:33 mgo-node*
-rw-r--r--  1 root root    256633 Mar 20 09:37 genesis.blob

Usage: mgo client switch [OPTIONS]

Options:
      --address <ADDRESS>  An address to be used as the active address for subsequent commands. It accepts also the
                           alias of the address
      --env <ENV>          The RPC server URL (e.g., local rpc server, devnet rpc server, etc) to be used for subsequent
                           commands
      --json               Return command outputs in json format
  -h, --help               Print help
  -V, --version            Print version
```

```

Granted permission

chmod +x mgo
chmod +x mgo-node
export PATH=/root/mango:$PATH

Add network
mgo client new-env  --alias node --rpc https://fullnode.devnet.mangonetwork.io:443

Switch network
mgo client switch --env node

Generate address
mgo client new-address ed25519 node

Switch to generated address
mgo client switch --address <ADDRESS>
```
### To become a validator candidate

1. This will generate a validator.info file and key pair files.

```
mgo validator make-validator-info [OPTIONS] <NAME> <DESCRIPTION> <IMAGE_URL> <PROJECT_URL> <HOST_NAME> <GAS_PRICE>

Usage: mgo validator make-validator-info [OPTIONS] <NAME> <DESCRIPTION> <IMAGE_URL> <PROJECT_URL> <HOST_NAME> <GAS_PRICE>

Arguments:
  <NAME>         
  <DESCRIPTION>  
  <IMAGE_URL>    
  <PROJECT_URL>  
  <HOST_NAME>    
  <GAS_PRICE>    

Options:
      --json     Return command outputs in json format
  -h, --help     Print help
  -V, --version  Print version

```

2. To submit an on-chain transaction to become a validator candidate

```
mgo validator become-candidate [OPTIONS] <validator-info-path>

Usage: mgo validator become-candidate [OPTIONS] <validator-info-path>

Arguments:
  <validator-info-path>  

Options:
      --gas-budget <gas-budget>  
      --json                     Return command outputs in json format
  -h, --help                     Print help
  -V, --version                  Print version

mgo validator become-candidate ./validator.info --gas-budget 100000000
```

3. At this point you are validator candidate and can start to accept self staking and delegated staking.
4. 
![switch](https://image.devnet.mangonetwork.io/img/gas.png)

```
mgo client call --package 0x3 --module mgo_system --function request_add_stake  \
--args 0x5 gasCoinId  Pool_Address --gas-budget 10000000

Pool_Address : Validator address

```
4. start a fullnode now to catch up with the network.

```
mgo-node --config-path ./validator.yaml

---
protocol-key-pair:
  value: H4AO+JqCaZ3tpK/rSKyl/iP63m2Ew+7CAk=
worker-key-pair:
  value: AGDwR+LuSqIhSBf7vtT25VQeFH0F5rf1GABI
account-key-pair:
  value: ANlSpledCgdY9H7DvOo5IwviGOd21mciO5SN
network-key-pair:
  value: AMGOUrmMYi5PmS6A3OvEjTHsYwWdvRItkmV
db-path: /root/validator/authorities_db/validator
network-address: /ip4/0.0.0.0/tcp/8080/http
json-rpc-address: "0.0.0.0:9202"
enable-experimental-rest-api: true
metrics-address: "0.0.0.0:37151"
admin-interface-port: 34736
consensus-config:
  address: /ip4/0.0.0.0/tcp/34879/http
  db-path: /root/validator/consensus_db/validator
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
      socket_addr: /ip4/127.0.0.1/tcp/35211/http
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
  external-address: /ip4/<ip>/udp/8084
  seed-peers:
    - peer-id: 9eed9adca100cdf5d4cf71078bdcbf752a4eb3001e550b5f643be6fea2a80168
      address: /dns/validator1.devnet.mangonetwork.io/udp/8084/http
    - peer-id: ff550ebd44ba4b4521289045f1beb99447c60cda7606ac7cb059a11e14394f70
      address: /dns/validator2.devnet.mangonetwork.io/udp/8084/http
    - peer-id: 87671fae328e75d22bef6aa5790f42844dddfb1abe486351a289d649453aeb53
      address: /dns/validator3.devnet.mangonetwork.io/udp/8084/http
    - peer-id: a8ff5bac98c0626979276c32dd002c8c460e927fee7f48d45b2bcc9dab7d74a3
      address: /dns/validator4.devnet.mangonetwork.io/udp/8084/http
genesis:
  genesis-file-location: /root/mango/genesis.blob
authority-store-pruning-config:
  num-latest-epoch-dbs-to-retain: 3
  epoch-db-pruning-period-secs: 3600
  num-epochs-to-retain: 0
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

```

5. Once you collect enough staking amount

```

mgo validator join-committee --gas-budget 1000000000

Usage: mgo validator join-committee [OPTIONS]

Options:
      --gas-budget <gas-budget>  Gas budget for this transaction
      --json                     Return command outputs in json format
  -h, --help                     Print help
  -V, --version                  Print version

```
to become a pending validator. A pending validator will become active and join the committee starting from next epoch.
