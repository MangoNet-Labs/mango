---
title: Connect to a Local Network
sidebar_position: 3
description: Connect to a Local Network
---

Use a Mango local network to test your dApps against the latest changes to Mango, and to prepare for the next Mango release to the Devnet network. To set up a local network, Mango provides the `mgo-test-validator` binary. The `mgo-test-validator` starts a local network that includes a Mango Full node, a Mango validator, and a Mango faucet. You can use the included faucet to get test MANGO to use on the local network.

If you haven't already, you need to [install Mango](./install-mango) on your system.

## Start the local network

To start the local network, run the following command from your `mgo` root folder.

```bash
RUST_LOG="off,mgo_node=info" cargo run --bin mgo-test-validator
```

The command starts the `mgo-test-validator`. The `RUST_LOG`=`off,mgo_node=info` turns off logging for all components except
mango-node. If you want to see more detailed logs, you can remove `RUST_LOG` from the command.

**Important:** Each time you start the `mgo-test-validator`, the network starts as a new network with no previous data. The local network is not persistent.

To customize your local Mango network, such as changing the port used, include additional parameters in the command to start `mgo-test-validator`:

```
OPTIONS:
        --epoch-duration-ms <EPOCH_DURATION_MS>
            The duration for epochs (defaults to one minute) [default: 60000]

        --faucet-port <FAUCET_PORT>
            Port to start the mgo faucet on [default: 9123]

        --fullnode-rpc-port <FULLNODE_RPC_PORT>
            Port to start the Fullnode RPC server on [default: 9000]
```

Use `mgo-test-validator --help` to see these options in your console.

### Access your local Full node

Use the following command to retrieve the total transaction count from your local network:

```bash
curl --location --request POST 'http://127.0.0.1:9000' \
--header 'Content-Type: application/json' \
--data-raw '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "mgo_getTotalTransactionBlocks",
  "params": []
}'
```

If successful, the response resembles the following:

```json
{
    "jsonrpc": "2.0",
    "result": 168,
    "id": 1
}
```

## Connect the Mango Client CLI to your local network

You can use the Mango Client CLI with any Mango network. By default it connects to Mango Devnet. To connect to your local network, create a new environment alias named `local` that sets the RPC URL the client uses to your local network.

```shell
mgo client new-env --alias local --rpc http://127.0.0.1:9000
```

Next, use the following command to set the active environment to the new `local` environment you created.

```
mgo client switch --env local
```

The command returns:

`Active environment switched to [local]`

You can check the current active environment with the following command:

```bash
mgo client active-env
```

The command returns:

`local`

## Show the current active address

The Mango Client CLI uses the active address for command if you don't specify one. Use the following command to show the active address on your local network.

```
mgo client active-address
```

The command returns an address:

`0x78f4925b522779b7ef942714f98f8d6afcb59fa0e77e10327e819be743f4c9bd`

Use the active address to get test MANGO to use on your local network. Use the `mgo client addresses` command to see all of the addresses on your local network.

**Note:** The address returned when you run the command is unique and does not match the one used in this example.

## Use the local faucet {#use-the-local-faucet}

Transactions on your local network require MANGO coins to pay for gas fees just like other networks. To send coins to a Mango Wallet connected to your local network, see [Set up a local Mango Wallet](#set-up-a-local-mango-wallet). You can use the address for the local Mango Wallet with the faucet.

Mango CLI provides the `mgo client faucet` command to get coins from the faucet. In the most basic case, run `mgo client faucet` and wait up to 60 seconds for the coins to reach your wallet. Use `mgo client gas` to check for the new coins.

:::info
The `faucet` command uses the active address and the active network environment by default. If you need to pass in a different address or faucet server URL, check the `help` menu. If you're using a different network than a local network or the public ones (fullnode.devnet.mangonetwork.io), you will have to pass the URL to the faucet server.
:::


### Check the gas coin objects for the active address

After you get coins from the faucet, use the following command to view the coin objects for the address:

```shell
mgo client gas
```

The response resembles the following, but with different IDs:

```shell
╭────────────────────────────────────────────────────────────────────┬────────────╮
│ gasCoinId                                                          │ gasBalance │
├────────────────────────────────────────────────────────────────────┼────────────┤
│ 0x97144a3b8a344fb9e61759a5f37abe01f21fa8b5215a76f2f137b6f9eedc96fd │ 500000000  │
│ 0xba9d92326c87bf44f6e8a50c5b504aab99e277affc95e96555117fb105ca6612 │ 500000000  │ 
╰────────────────────────────────────────────────────────────────────┴────────────╯
```

## Install Mango Wallet and Mango Explorer locally

To install and use the apps locally, you must first install [pnpm](https://pnpm.io/installation). Use the instructions appropriate for your operating system.

After you install `pnpm`, use the following command to install the required dependencies in your workspace:

```shell
pnpm install
```

After the installation completes, run the following command to install Mango Wallet and Mango Explorer:

```shell
pnpm turbo build
```

If you encounter an error from turbo build, confirm that there is no `package-lock.json`. If the file exists, remove it and then run the command again.

### Set up Mango Explorer on your local network

To connect the live Mango Explorer to your local network, open the URL:[https://mgoscan.com/?network=local](https://mgoscan.com/?network=local). The live version of Mango Explorer may not include recent updates added to the `main` branch of the Mango repo. To use Mango Explorer that includes the most recent updates, install and run Mango Explorer from your local clone of the Mango repo.

Run the following command from the `mango` root folder:

**Note:** To run the command you have `pnpm` installed. See [Install Mango Wallet and Mango Explorer locally](#install-mango-wallet-and-mango-explorer-locally) for details.

```bash
pnpm explorer dev
```

After the command completes, open your local Mango Explorer at the following URL: [http://localhost:3000/](http://localhost:3000/).

For more details about Mango Explorer. 

## Set up a local Mango Wallet

You can also use a local Mango Wallet to test with your local network. You can then see transactions executed from your local Mango Wallet on your local Mango Explorer.

**Note:** To run the command you must have `pnpm` installed. See [Install Mango Wallet and Mango Explorer locally](#install-mango-wallet-and-mango-explorer-locally) for details.

Before you start the Mango Wallet app, update its default environment to point to your local network. To do so, first make a copy of `mango/apps/wallet/configs/environment/.env.defaults` and rename it to `.env` in the same directory. In your `.env` file, edit the first line to read `API_ENV=local` and then save the file.

Run the following command from the `mango` root folder to start Mango Wallet on your local network:

```bash
pnpm wallet dev
```

### Add local Mango Wallet to Chrome

After you build your local version of Mango Wallet, you can add the extension to Chrome:

1. Open a Chrome browser to `chrome://extensions`.
1. Click the **Developer mode** toggle to enable, if it's not already on.
1. Click the **Load unpacked** button and select your `mango/apps/wallet/dist` directory.

Consult the Mango Wallet [Readme](https://github.com/MangoNet-Labs/mango/blob/main/apps/wallet/README.md#install-the-extension-to-chrome) for more information on working with a locally built wallet on Chrome.

## Generate example data

Use the TypeScript SDK to add example data to your network.

**Note:** To run the command you must complete the `Pre-requisites for Building Apps locally` section first.

Run the following command from the `mango` root folder:

```bash
pnpm --filter @mangonetwork/mango.js test:e2e
```

For additional information about example data for testing.

## Troubleshooting

If you do not use [Node.js 18](https://nodejs.org/de/blog/announcements/v18-release-announce), you might see the following message:

`Retrying requesting from faucet: Retry failed: fetch is not defined`

To resolve this, switch or update to Node.js 18 and then try again.

## Test with the Mango TypeScript SDK

The published version of the Mango TypeScript SDK might be an earlier version than the version of Mango you installed for your local network. To make sure you're using the latest version of the SDK, use the `experimental`-tagged version (for example, `0.0.0-experimental-20240312184920`) in the [Current Tags](https://www.npmjs.com/package/@mangonetwork/mango.js/v/0.0.0-experimental-20240312184920?activeTab=versions) section of the Mango NPM registry.
