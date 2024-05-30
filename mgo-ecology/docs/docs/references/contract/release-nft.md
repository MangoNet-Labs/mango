---
title: Release NFT
description: Release NFT
sidebar_position: 2
---

# Create NFT

> The MGO Object Display Standard is a template engine that enables on-chain management and off-chain representation (display) of types. With it, you can replace the object's data with a template string. The standard does not limit the fields you can set. You can access all object properties using the `{property}` syntax and then insert them into the template string.

> **Premise**：You have installed the MGO client and added it to the environment variables; make sure your account has enough tokens, if not please go to the MGO faucet.

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

Create a new `mynft.move` file in the `sources` directory

**3. Open file**

Copy the following code into the file and save it

``` rust
module examples::mynft{
    use mgo::url::{Self, Url};
    use mgo::transfer;
    use mgo::object::{Self,ID, UID};
    use mgo::tx_context::{Self, TxContext};
    use mgo::package::{Self};
    use std::string::{Self,String};
    use mgo::display::{Self};

    struct NFT has key, store{
        id: UID,
        name: String,
        description: String,
        creator: address,
        url: Url,
    }
    struct MYNFT has drop{}

    // Add support for browser object display
    #[allow(lint(share_owned))]
    fun init (otw: MYNFT, ctx: &mut TxContext,){
        let publisher = package::claim(otw, ctx);
        let keys = vector[
            string::utf8(b"name"),
            string::utf8(b"image_url"),
            string::utf8(b"description"),
            string::utf8(b"creator"),
        ];

        let values = vector[
            string::utf8(b"{name}"),
            string::utf8(b"{url}"),
            string::utf8(b"{description}"),
            string::utf8(b"{creator}")
        ];

        let display = display::new_with_fields<NFT>(
            &publisher, 
            keys,
            values,
            ctx,
        );

        display::update_version(&mut display);
        transfer::public_share_object(display);
        transfer::public_transfer(publisher, tx_context::sender(ctx))

    }

    // mint nft
    public entry fun mint(
        name: String,
        description: String,
        url:String,
        ctx: &mut TxContext,
    ){
        let nft = NFT{
            id: object::new(ctx),
            name: name,
            description: description,
            creator: tx_context::sender(ctx),
            url: url::new_unsafe(string::to_ascii(url)),
        };
        transfer::public_transfer(nft, tx_context::sender(ctx))
    }

    /// Transfer "nft" to "receiver"
    public entry fun transfer(
        nft: NFT, recipient: address, _: &mut TxContext
    ) {
        transfer::public_transfer(nft, recipient)
    }
    /// Update the description of "nft" to "new_description"
    public entry fun update_description(
        nft: &mut NFT,
        new_description: vector<u8>,
        _: &mut TxContext
    ) {
        nft.description = string::utf8(new_description)
    }

    /// Permanently delete "nft"
    public entry fun burn(nft: NFT, _: &mut TxContext) {
        let NFT { id, name: _, description: _, url: _ ,creator:_} = nft;
        object::delete(id)
    }
}
```

**4. publish project**

Enter the deployment command `mgo client publish ./ --gas-budget 30000000` on the command line in the root directory of the project

**5. minting tokens**

In the command return value of the previous step, find the attribute `objectChanges`.

Find the value of `packageId` in one of the objects of `type` `published`.

For example, I want to cast an NFT with a name of `name`, a description of `description`, and an image address of `url`.
Enter the command at the command line
``` shell
mgo client call --package {$packageId} --module mycoin --function mint --args {$name} {$description} {$url}
```

**5. View NFT**

The contract calling address `address` when minting tokens

Enter the command at the command line
``` shell
mgo client objects  {$address}
```
View the NFT you obtained