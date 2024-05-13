---
title: Get Mango Address
sidebar_position: 4
description: Get Mango Address
---

An address is a way to uniquely and anonymously identify an account that exists on the Mango blockchain network. In other words, an address is a way for a user to store and use tokens on the Mango network, without providing any personally identifying information (such as email address, phone number, and so on). For example, if you want to purchase a number of MGO tokens to play a game, you must specify an address where these tokens are to be deposited.

The Mango address is unique, similarly to the way a social security number or a personal identification number is unique to one person. However, in Mango you can create and own multiple addresses, all of which are unique. 

In Mango, an address is 32 bytes and is often encoded in base58 with `0x` prefix. For example, this is a valid Mango address: `0x98af42f89adbde3649c025df62e053b52ad92ae4753fc39344611b7957d93bf1`. You can use the [Mango Explorer](https://mgoscan.com/) website to find more information about [this address](https://mgoscan.com/address/0x98af42f89adbde3649c025df62e053b52ad92ae4753fc39344611b7957d93bf1) and the objects it owns.

If you'd like to understand how a Mango address is derived from private keys and other cryptography related topics, see the Keys and Addresses


## How to obtain a Mango address

Mango provides multiple ways to obtain a Mango address. The following are the two most common. 

### Mango Wallet

One of the most straightforward ways to obtain a Mango address for first-time users is through the [Mango Wallet Chrome browser extension](https://api-backned.devnet.mangonetworknet.io/). After you install the extension, there are several ways to create an address. 

Open the Chrome Mango Wallet browser extension and then:
- Click **More Options** → **Create a new passphrase account**. Then follow the on-screen instructions. 

For more information on the Mango Wallet and how to keep it secure, see the Mango Wallet documentation. 

### Command line interface

If you are using the Mango command line interface (CLI) to interact with the Mango network, you can use the `mango client` command to generate a new address. By default, when the Mango CLI runs for the first time it will prompt you to set up your local wallet, and then it generates one Mango address and the associated secret recovery phrase. Make sure you write down the secret recovery phrase and store it in a safe place. 


To generate a new Mango address use `mgo client new-address ed25519`, which specifies the keypair scheme flag to be of type `ed25519`.

For more information, see the [mgo Client CLI](/docs/references/cli/client) documentation.

To see all the generated addresses in the local wallet on your machine, run `mgo keytool list`. For more information about the keytool options, see the [Mango Keytool CLI](/docs/references/cli/client) documentation.

:::danger

The private keys associated with the Mango addresses are stored locally on the machine where the CLI is installed, in the `~/.mango/mango_config/mango.keystore` file. Make sure you do not expose this to anyone, as they can use it to get access to your account.

:::
