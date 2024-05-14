---
title: Genesis
description: Genesis
sidebar_position: 1
---

Genesis is the initial state of the Mango blockchain. To launch a network, the initial committee of validators collaborate by providing their validator information (public keys, network addresses, and so on) to a shared workspace. After all of the initial validators have contributed their information, Mango generates the initial, unsigned genesis checkpoint (checkpoint with sequence number 0) and each validator provides their signature. Mango aggregates these signatures to form a certificate on the genesis checkpoint. Mango bundles this checkpoint, as well as the initial objects, together into a single genesis.blob file that is used to initialize the state when running the `mgo-node` binary for both validators and Full nodes.

## Genesis blob locations

The genesis.blob files for each network are in the [mango-genesis](https://github.com/MangoNet-Labs/mango-genesis) repository.

See [Mango Full Node](./mango-full-node#set-up-from-source) for how to get the genesis.blob file for each network.
