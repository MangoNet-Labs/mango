---
title: Release Tokens
description: Release Tokens
sidebar_position: 1
---

# Release Tokens

> Releasing a token on MGO is almost as easy as releasing a new type. The main difference is that a one-time witness is required when creating a token.

> **premise**：You have installed the MGO client and added it to the environment variables; make sure your account has enough tokens, if not please go to the MGO faucet.

## Specific steps
**1. Create project**

Open the command line and enter the command:

Create move project
``` shell
mgo move new examples
```
Enter the newly created project directory
``` shell
cd examples
```
**2. Create code file**

Create a new `mycoin.move` file in the `sources` directory

**3. open a file**

Copy the following code into the file and save it

``` rust
module examples::mycoin {
    use std::option;
    use mgo::coin::{Self, Coin, TreasuryCap};
    use mgo::transfer;
    use mgo::tx_context::{Self, TxContext};

    /// The name of the coin. By convention, this type has the same name as its parent module.
    /// And there are no fields. The complete coin type defined by this module will be `COIN<MYCOIN>`.
    struct MYCOIN has drop {}

    /// Sign up for managed currency to get it“TreasuryCap”。
    /// Because this is a module initializer, it ensures that the currency only gets registered once。
    fun init(witness: MYCOIN, ctx: &mut TxContext) {
        let (treasury, metadata) = coin::create_currency(witness, 6, b"MYCOIN", b"", b"", option::none(), ctx);
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury, tx_context::sender(ctx))
    }
    /// Admins can mint tokens
    public entry fun mint(
        treasury_cap: &mut TreasuryCap<MYCOIN>, amount: u64, recipient: address, ctx: &mut TxContext
    ) {
        coin::mint_and_transfer(treasury_cap, amount, recipient, ctx)
    }

    /// Administrators can destroy tokens
    public entry fun burn(treasury_cap: &mut TreasuryCap<MYCOIN>, coin: Coin<MYCOIN>) {
        coin::burn(treasury_cap, coin);
    }
}
```
**4. publish project**

Enter the deployment command at the command line in the root directory of the project `mgo client publish ./ --gas-budget 30000000`

**5. minting tokens**

In the command return value of the previous step, find the attribute `objectChanges`.

Find the value of `packageId` in one of the objects of `type` `published`.

And the value of `objectId` in one of the objects whose `objectType` is `0x2::coin::TreasuryCap<{$packageId}::examples::MYCOIN>`.

For example, I want to mint `number` amount of tokens at `address` address
Enter the command at the command line
``` shell
mgo client call --package {$packageId} --module mycoin --function mint --args {$objectId} {$number} {$address}
```

**5. View tokens**

Enter the command at the command line
``` shell
mgo client objects  {$address}
```
Check the tokens you have obtained
