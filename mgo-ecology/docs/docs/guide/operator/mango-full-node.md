---
title: Mango Full Node Configuration
description: Operate a Mango Full node to validate blockchain activities, like transactions, checkpoints, and epoch changes.
sidebar_position: 4
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

:::info

These instructions are for advanced users. If you just need a local development environment, you should instead follow the instructions in [Create a Local Mango Network](../developer/geting-started/local-network) to create a local Full node, validators, and faucet.

:::

Mango Full nodes validate blockchain activities, including transactions, checkpoints, and epoch changes. Each Full node stores and services the queries for the blockchain state and history.

This role enables validators to focus on servicing and processing transactions. When a validator commits a new set of transactions (or a block of transactions), the validator pushes that block to all connected Full nodes that then service the queries from clients.

## Features 

Mango Full nodes:
- Track and verify the state of the blockchain, independently and locally.
- Serve read requests from clients.

## State synchronization 
Mango Full nodes sync with validators to receive new transactions on the network.

A transaction requires a few round trips to 2f+1 validators to form a transaction certificate (TxCert).

This synchronization process includes:

1. Following 2f+1 validators and listening for newly committed transactions.
1. Making sure that 2f+1 validators recognize the transaction and that it reaches finality.
1. Executing the transaction locally and updating the local DB.

This synchronization process requires listening to at a minimum 2f+1 validators to ensure that a Full node has properly processed all new transactions. Mango will improve the synchronization process with the introduction of checkpoints and the ability to synchronize with other Full nodes.

## Architecture 

A Mango Full node is essentially a read-only view of the network state. Unlike validator nodes, Full nodes cannot sign transactions, although they can validate the integrity of the chain by re-executing transactions that a quorum of validators previously committed.

Today, a Mango Full node maintains the full history of the chain.

Validator nodes store only the latest transactions on the frontier of the object graph (for example, transactions with >0 unspent output objects).

## Full node setup 

Follow the instructions here to run your own Mango Full.

### Hardware requirements 

Suggested minimum hardware to run a Mango Full node:

- CPUs: 8 physical cores / 16 vCPUs
- RAM: 128 GB
- Storage (SSD): 4 TB NVMe drive

### Software requirements 

Mango recommends running Mango Full nodes on Linux. Mango supports the Ubuntu and Debian distributions. You can also run a Mango Full node on macOS.

Make sure to [update Rust](https://doc.rust-lang.org/book/ch01-01-installation.html#updating-and-uninstalling).

Use the following command to install additional Linux dependencies.

```shell
sudo apt-get update \
&& sudo apt-get install -y --no-install-recommends \
tzdata \
libprotobuf-dev \
ca-certificates \
build-essential \
libssl-dev \
libclang-dev \
libpq-dev \
pkg-config \
openssl \
protobuf-compiler \
git \
clang \
cmake
```

### Setting up a local Mango repository 

You must get the latest source files from the Mango GitHub repository.
1. Set up your fork of the Mango repository:
    1. Go to the [Mango repository](https://github.com/MangoNet-Labs/mango) on GitHub and click the Fork button in the top right-hand corner of the screen.
    1. Clone your personal fork of the Mango repository to your local machine (ensure that you insert your GitHub username into the URL):
    `git clone https://github.com/<YOUR-GITHUB-USERNAME>/mango.git`
1. `cd` into your `mango` repository:
    `cd mango`
1. Set up the Mango repository as a git remote:
    `git remote add upstream https://github.com/MangoNet-Labs/mango`
1. Sync your fork:
    `git fetch upstream`
1. Check out the branch associated with the network version you want to run (for example, `devnet` to run a Devnet Full node):
    `git checkout --track upstream/<BRANCH-NAME>`

### Setting up a Full node from source {#set-up-from-source}

Open a terminal or console to the `mango` directory you downloaded in the previous steps to complete the following:
1. Install the required prerequisites.
1. Make a copy of the [Full node YAML template](https://github.com/MangoNet-Labs/Mango/blob/main/crates/mgo-config/data/fullnode-template.yaml):
    `cp crates/mango-config/data/fullnode-template.yaml fullnode.yaml`
1. Download the genesis blob for the network to use:
    - [Devnet genesis blob](https://github.com/MangoNet-Labs/mango-genesis/raw/main/devnet/genesis.blob):
        `curl -fLJO https://github.com/MangoNet-Labs/mango-genesis/raw/main/devnet/genesis.blob`

    <Tabs groupId="network">

    <TabItem label="Devnet" value="devnet">

    ```shell
        p2p-config:
        seed-peers:
            - address: /dns/nrt-tnt-ssfn-00.testnet.mango.io/udp/8084
            peer-id: 23a1f7cd901b6277cbedaa986b3fc183f171d800cabba863d48f698f518967e1
            - address: /dns/ewr-tnt-ssfn-00.testnet.mango.io/udp/8084
            peer-id: df8a8d128051c249e224f95fcc463f518a0ebed8986bbdcc11ed751181fecd38
            - address: /dns/lax-tnt-ssfn-00.testnet.mango.io/udp/8084
            peer-id: f9a72a0a6c17eed09c27898eab389add704777c03e135846da2428f516a0c11d
            - address: /dns/lhr-tnt-ssfn-00.testnet.mango.io/udp/8084
            peer-id: 9393d6056bb9c9d8475a3cf3525c747257f17c6a698a7062cbbd1875bc6ef71e
            - address: /dns/mel-tnt-ssfn-00.testnet.mango.io/udp/8084
            peer-id: c88742f46e66a11cb8c84aca488065661401ef66f726cb9afeb8a5786d83456e
    ```

    </TabItem>

    </Tabs>
    
1. Optional: Skip this step to accept the default paths to resources. Edit the fullnode.yaml file to use custom paths.
1. Update the `db-path` field with the path to the Full node database.
    `db-path: "/db-files/mango-fullnode"`
1. Update the `genesis-file-location` with the path to genesis.blob.
    ```shell
    genesis:
        genesis-file-location: "/mango-fullnode/genesis.blob"
    ```

### Starting services 

At this point, your Mango Full node is ready to connect to the Mango network.

1. Open a terminal or console to the mango directory.
1. Start the Mango Full node:
    `cargo run --release --bin mango-node -- --config-path fullnode.yaml`
1. Optional: Publish/subscribe to notifications using JSON-RPC via websocket.

If your setup is successful, your Mango Full node is now connected to the appropriate network.

Your Full node serves the read endpoints of the Mango JSON-RPC API at: `http://127.0.0.1:9000`.

### Troubleshooting 
If you receive a `cannot find -lpq` error, you are missing the `libpq` library. Use `sudo apt-get install libpq-dev` to install on Linux, or `brew install libpq` on MacOS. After you install on MacOS, create a Homebrew link using `brew link --force libpq`. For further context, reference the [issue on Stack Overflow](https://stackoverflow.com/questions/70313347/ld-library-not-found-for-lpq-when-build-rust-in-macos?rq=1).

If you receive the following error:

```shell
panicked at error binding to 0.0.0.0:9184: error creating server listener: Address already in use (os error 98)
```

Then update the metrics address in your fullnode.yaml file to use port `9180`.

```shell
metrics-address: "0.0.0.0:9180"
```

## Mango Explorer with your Full node 

[Mango Explorer](https://mgoscan.com/) supports connections to custom RPC URLS and local networks. You can point the Explorer to your local Full node and see the transactions it syncs from the network.
1. Open a browser and go to:https://mgoscan.com/
1. Click **Mainnet** in the network drop-down at the top right-hand corner (or three bars on smaller screens) and select **Local** to connect to a local network, or select **Custom RPC URL** and then enter the URL.

Mango Explorer displays information about the selected network.

## Monitoring 
Monitor your Full node using the instructions at Logging, Tracing, Metrics, and Observability.

The default metrics port is `9184`. To change the port, edit your fullnode.yaml file.

## Update your Full node 

Whenever Mango releases a new version, you must update your Full node with the release to ensure compatibility with the network it connects to. For example, if you use Mango Devnet you should install the version of Mango running on Mango Devnet.


### Update from source 

If you followed the instructions for Building from Source, use the following steps to update your Full node:

1. Shut down your running Full node.
1. `cd` into your local Mango repository:
    ```shell
    cd mango
    ```
1. Remove the database and 'genesis.blob' file:
    ```shell
    rm -r mangodb genesis.blob
    ```
1. Fetch the source from the latest release:
    ```shell
    git fetch upstream
    ```
1. Reset your branch:
    ```shell
    git checkout -B <BRANCH-NAME> --track upstream/<BRANCH-NAME>
    ```
1. Download the latest genesis blob:
    - [Devnet genesis blob](https://github.com/MangoNet-Labs/mango-genesis/raw/main/devnet/genesis.blob):
        ```shell
        curl -fLJO https://github.com/MangoNet-Labs/mango-genesis/raw/main/devnet/genesis.blob
        ```
1. Update your fullnode.yaml configuration file, if needed.
1. Restart your Mango Full node:
    ```shell
    cargo run --release --bin mango-node -- --config-path fullnode.yaml
    ```
    
Your Full node starts on: http://127.0.0.1:9000.

## Object pruning {#object-pruning}

Mango adds new object versions to the database as part of transaction execution. This makes previous versions ready for 
garbage collection. However, without pruning, this can result in database performance degradation and requires large 
amounts of storage space. Mango identifies the objects that are eligible for pruning in each checkpoint, and then performs
the pruning in the background.

You can enable pruning for a Mango node by adding the `authority-store-pruning-config` config to `fullnode.yaml` file:
```yaml
authority-store-pruning-config:
  # Number of epoch dbs to keep 
  # Not relevant for object pruning
  num-latest-epoch-dbs-to-retain: 3
  # The amount of time, in seconds, between running the object pruning task.
  # Not relevant for object pruning
  epoch-db-pruning-period-secs: 3600
  # Number of epochs to wait before performing object pruning.
  # When set to 0, Mango prunes old object versions as soon
  # as possible. This is also called *aggressive pruning*, and results in the most effective
  # garbage collection method with the lowest disk usage possible. 
  # This is the recommended setting for Mango Validator nodes since older object versions aren't
  # necessary to execute transactions.
  # When set to 1, Mango prunes only object versions from transaction checkpoints
  # previous to the current epoch. In general, when set to N (where N >= 1), Mango prunes  
  # only object versions from checkpoints up to `current - N` epoch. 
  # It is therefore possible to have multiple versions of an object present 
  # in the database. This setting is recommended for Mango Full nodes as they might need to serve 
  # RPC requests that require looking up objects by ID and Version (rather than just latest
  # version). However, if your Full node does not serve RPC requests you should then also enable  
  # aggressive pruning.
  num-epochs-to-retain: 0
  # Advanced setting: Maximum number of checkpoints to prune in a batch. The default
  # settings are appropriate for most use cases.
  max-checkpoints-in-batch: 10
  # Advanced setting: Maximum number of transactions in one batch of pruning run. The default
  # settings are appropriate for most use cases.
  max-transactions-in-batch: 1000
```
## Transaction pruning {#transaction-pruning}

Transaction pruning removes previous transactions and effects from the database.
Mango periodically creates checkpoints. Each checkpoint contains the transactions that occurred during the checkpoint and their associated effects.

Mango performs transaction pruning in the background after checkpoints complete.

You can enable transaction pruning for your Full node or Validator node by adding  `num-epochs-to-retain-for-checkpoints`
to the `authority-store-pruning-config` config for the node:

```yaml
authority-store-pruning-config:
  num-latest-epoch-dbs-to-retain: 3
  epoch-db-pruning-period-secs: 3600
  num-epochs-to-retain: 0
  max-checkpoints-in-batch: 10
  max-transactions-in-batch: 1000
  # Number of epochs to wait before performing transaction pruning.
  # When this is N (where N >= 2), Mango prunes transactions and effects from 
  # checkpoints up to the `current - N` epoch. Mango never prunes transactions and effects from the current and
  # immediately prior epoch. N = 2 is a recommended setting for Mango Validator nodes and Mango Full nodes that don't 
  # serve RPC requests.
  num-epochs-to-retain-for-checkpoints: 2
  # Ensures that individual database files periodically go through the compaction process.
  # This helps reclaim disk space and avoid fragmentation issues
  periodic-compaction-threshold-days: 1
```
