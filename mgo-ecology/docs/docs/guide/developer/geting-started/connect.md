---
title: Connect to a Mango Network
description: Connect to a Mango Network. 
sidebar_position: 2
---

Mango has  Devnet networks available. You can use  Devnet , to experiment with the version of Mango running on that network. You can also spin up a [local Mango network](./local-network) for local development. 

The Mango Devnet networks consist of:

 * Four validator nodes operated by MangoNetworkOs. Clients send transactions and read requests via this endpoint: `https://fullnode.<MANGO-NETWORK-VERSION>.mango.io:443` using [JSON-RPC](/docs/references/mango-api).
 * A public network [Mango Explorer](https://mgoscan.com/) for browsing transaction history.

## Tools
Mango provides the following tools to interact with Mango networks:

 * [Mango command line interface (CLI)](/docs/references/cli/client)
     * Create and manage your private keys
     * Create example NFTs
     * Call and publish Move modules
 * [Mango Explorer](https://mgoscan.com/) to view transactions and objects on the network

## Environment set up

First, [Install Mango](./install-mango#install-binaries). run the following command:

```shell
which mgo
```

If Mango is installed, the command returns the path to the Mango binary. If Mango is not installed, it returns `mgo not found`.

See the [Mango Releases](https://github.com/MangoNet-Labs/mango/releases) page to view the changes in each Mango release.

## Configure Mango client

If you previously ran `mgo genesis` to create a local network, it created a Mango client configuration file (client.yaml) that connects to `localhost` at `http://0.0.0.0:9000`. See [Connect to a custom RPC endpoint](#connect-to-a-custom-rpc-endpoint) to update the client.yaml file.

To connect the Mango client to a network, run the following command:

```shell
mgo client
```

If you receive the `mgo-client` help output in the console, you already have a client.yaml file. See [Connect to a custom RPC endpoint](#connect-to-a-custom-rpc-endpoint) to add a new environment alias or to switch the currently active network.

The first time you start Mango client without having a client.yaml file, the console displays the following message:

```
Config file ["<PATH-TO-FILE>/client.yaml"] doesn't exist, do you want to connect to a mgo Full node server [y/N]?
```

Press **y** and then press **Enter**. The process then requests the RPC server URL: 

```
mgo Full node server URL (Defaults to Mango Devnet if not specified) :
```

Press **Enter** to connect to Mango Devnet. To use a custom RPC server, Mango Testnet, or Mango Mainnet, enter the URL to the correct RPC endpoint and then press **Enter**.

If you enter a URL, the process prompts for an alias for the environment:

```
Environment alias for [<URL-ENTERED>] :
```
Type an alias name and press **Enter**.

```
Select key scheme to generate keypair (0 for ed25519, 1 for secp256k1, 2 for secp256r1):
```

Press **0**, **1**, or **2** to select a key scheme and the press **Enter**.

Mango returns a message similar to the following (depending on the key scheme you selected) that includes the address and 12-word recovery phrase for the address:

```
Generated new keypair for address with scheme "ed25519" [0x78f4925b522779b7ef942714f98f8d6afcb59fa0e77e10327e819be743f4c9bd]
Secret Recovery Phrase : [camp climb many line human lazy few solid bored proud speed grocery]
```

### Connect to a custom RPC endpoint

If you previously used `mgo genesis` with the force option (`-f` or `--force`), your client.yaml file already includes two RPC endpoints: `localnet` at `http://0.0.0.0:9000` and `devnet` at `https://fullnode.devnet.mango.io:443`. You can view the defined environments with the `mango client envs` command, and switch between them with the `mgo client switch` command.

If you previously installed a Mango client that connected to a Mango network, or created a local network, you can modify your existing client.yaml file to change the configured RPC endpoint. The `mgo client` commands that relate to environments read from and write to the client.yaml file.

To check currently available environment aliases, run the following command: 

```sh
mgo client envs
```

The command outputs the available environment aliases, with `(active)` denoting the currently active network.
```sh
localnet => http://0.0.0.0:9000 (active)
devnet => https://fullnode.devnet.mangonetwork.io
```

To add a new alias for a custom RPC endpoint, run the following command. Replace values in `<` `>` with values for your installation:

```shell
mgo client new-env --alias <ALIAS> --rpc <RPC-SERVER-URL>
```

To switch the active network, run the following command:

```shell
mgo client switch --env <ALIAS>
```

If you encounter an issue, delete the Mango configuration directory (`~/.mango/mango_config`) and reinstall the latest [Mango binaries](./install-mango#install-mango-binaries).
