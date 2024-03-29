# Validator Tool

## Overview

This document is focused on using Validator Tool.

**Caveat: this tool only supports Pending Validators and Active Validators at the moment.**

## Preparation

1. Make sure you have completed all the prerequisites

2. Build the `mango` binary, which you will need for the genesis ceremony. This step can be done on any machine you like. It does not have to be done on the machine on which you will run the validator.

 Remember the path to your binary:

    export PATH=$(pwd)/target/release:$PATH


1. Run the following command to set up your Mango account and CLI environment. 

    1. If this is the first time running this program, it will ask you to provide a Mango Fullnode Server URL and a meaningful environment alias. It will also generate a random key pair in `mgo.keystore` and a config `client.yaml`. Swap in your validator account key if you already have one.

    2. If you already set it up, simply make sure 
      a. `rpc` is correct in `client.yaml`. 
      b. `active_address` is correct in `client.yaml`.
      b. `mgo.keystore` contains your account key pair.

    If at this point you can't find where `client.yaml` or `mgo.keystore` is or have other questions, read Mango Client CLI tutorial.

``` bash
$mgo client
```

4. To test you are connected to the network and configured your config correctly, run the following command to display your validator info.

``` bash
$mgo validator display-metadata
```



## Using Validator Tool

#### Print Help Info
``` bash
$mgo validator --help

A tool for validators and validator candidates
Usage: mgo validator [OPTIONS] [COMMAND]
Commands:
  make-validator-info               
  become-candidate                  
  join-committee                    
  leave-committee                   
  display-metadata                  
  update-metadata                   
  update-gas-price                  Update gas price that is used to calculate Reference Gas Price
  report-validator                  Report or un-report a validator
  serialize-payload-pop             Serialize the payload that is used to generate Proof of Possession. This is
                                        useful to take the payload offline for an Authority protocol keypair to sign
  display-gas-price-update-raw-txn  Print out the serialized data of a transaction that sets the gas price quote
                                        for a validator
  help                              Print this message or the help of the given subcommand(s)

Options:
      --client.config <CONFIG>  Sets the file storing the state of our user accounts (an empty one will be created
                                if missing)
      --json                    Return command outputs in json format
  -y, --yes                     
  -h, --help                    Print help
  -V, --version                 Print version
```

#### Display Validator Metadata
``` bash
$mgo validator display-metadata
```

or 

``` bash
$mgo validator display-metadata <validator-address>
```
to print another validator's information.

#### Update Validator Metadata
Run the following to see how to update validator metadata. Read description carefully about when the change will take effect.
``` bash
$mgo validator update-metadata --help
Usage: mgo validator update-metadata [OPTIONS] <COMMAND>
Commands:
  name              Update name. Effectuate immediately
  description       Update description. Effectuate immediately
  image-url         Update Image URL. Effectuate immediately
  project-url       Update Project URL. Effectuate immediately
  network-address   Update Network Address. Effectuate from next epoch
  primary-address   Update Primary Address. Effectuate from next epoch
  worker-address    Update Worker Address. Effectuate from next epoch
  p2p-address       Update P2P Address. Effectuate from next epoch
  network-pub-key   Update Network Public Key. Effectuate from next epoch
  worker-pub-key    Update Worker Public Key. Effectuate from next epoch
  protocol-pub-key  Update Protocol Public Key and Proof and Possession. Effectuate from next epoch
  help              Print this message or the help of the given subcommand(s)

Options:
      --gas-budget <gas-budget>  Gas budget for this transaction
      --json                     Return command outputs in json format
  -h, --help                     Print help
  -V, --version                  Print version
```

You can update the following on-chain metadata:
1. name
2. description
3. image URL
4. project URL
5. network address
6. p2p address
7. primary address
8. worker address
9. protocol public key
10. network public key
11. worker public key

Notably, only the first 4 metadata listed above take effect immediately.

If you change any metadata from points 5 to 11, they will be changed only after the next epoch - **for these, you'll want to restart the validator program immediately after the next epoch, with the new key files and/or updated `validator.yaml` config. Particularly, make sure the new address is not behind a firewall.**

Run the following to see how to update each metadata.
``` bash
$mgo validator update-metadata --help
```

#### Operation Cap
Operation Cap allows a validator to authorizer another account to perform certain actions on behalf of this validator. Read about [Operation Cap here](mgo_for_node_operators.md#operation-cap).

The Operation Cap holder (either the valdiator itself or the delegatee) updates its Gas Price and reports validator peers with the Operation Cap.

#### Update Gas Price
To update Gas Price, run

```bash
$mgo validator update-gas-price <gas-price>
```

if the account itself is a validator and holds the Operation Cap. Or 

```bash
$mgo validator update-gas-price --operation-cap-id <operation-cap-id> <gas-price>
```

if the account is a delegatee.

#### Report Validators
To report validators peers, run

```bash
$mgo validator report-validator <reportee-address>
```

Add `--undo-report false` if it intents to undo an existing report.

Similarly, if the account is a delegatee, add `--operation-cap-id <operation-cap-id>` option to the command.

if the account itself is a validator and holds the Operation Cap. Or 

```bash
$mgo validator update-gas-price --operation-cap-id <operation-cap-id> <gas-price>
```

if the account is a delegatee.


#### Become a Validator / Join Committee
To become a validator candidate, first run

```bash
$mgo validator make-validator-info <name> <description> <image-url> <project-url> <host-name> <gas_price>
```

This will generate a `validator.info` file and key pair files. The output of this command includes:
  1. Four key pair files (Read [more here](mgo_for_node_operators.md#key-management)). ==Set their permissions with the minimal visibility (chmod 600, for example) and store them securely==. They are needed when running the validator node as covered below.
    a. If you follow this guide thoroughly, this key pair is actually copied from your `mgo.keystore` file.
  2. `validator.info` file that contains your validator info. **Double check all information is correct**.

Then run 

``` bash
$mgo validator become-candidate {path-to}validator.info --gas-budget 10000000
```

to submit an on-chain transaction to become a validator candidate. The parameter is the file path to the validator.info generated in the previous step. **Make sure the transaction succeeded (printed in the output).**

At this point you are validator candidate and can start to accept self staking and delegated staking. 

**If you haven't, start a fullnode now to catch up with the network. When you officially join the committee but is not fully up-to-date, you cannot make meaningful contribution to the network and may be subject to peer reporting hence face the risk of reduced staking rewards for you and your delegators.**


### self staking and delegated staking.

Add self pledge ==*request_add_stake*==

``` bash
$mgo client call [OPTIONS] --package <PACKAGE> --module <MODULE> --function <FUNCTION> --gas-budget <GAS_BUDGET>
Options:
      --package <PACKAGE>               Object ID of the package, which contains the module
      --module <MODULE>                 The name of the module in the package
      --function <FUNCTION>             Function name in module
      --type-args <TYPE_ARGS>...        Type arguments to the generic function being called. All must be specified,
                                        or the call will fail
      --args <ARGS>...                  Simplified ordered args like in the function syntax ObjectIDs, Addresses
                                        must be hex strings
      --gas <GAS>                       ID of the gas object for gas payment, in 20 bytes Hex string If not
                                        provided, a gas object with at least gas_budget value will be selected
      --gas-budget <GAS_BUDGET>         Gas budget for this call
      --serialize-unsigned-transaction  Instead of executing the transaction, serialize the bcs bytes of the
                                        unsigned transaction data (TransactionData) using base64 encoding, and print
                                        out the string <TX_BYTES>. The string can be used to execute transaction
                                        with `mgo client execute-signed-tx --tx-bytes <TX_BYTES>`
      --serialize-signed-transaction    Instead of executing the transaction, serialize the bcs bytes of the signed
                                        transaction data (SenderSignedData) using base64 encoding, and print out the
                                        string <SIGNED_TX_BYTES>. The string can be used to execute transaction with
                                        `mgo client execute-combined-signed-tx --signed-tx-bytes <SIGNED_TX_BYTES>`

```
Pledge execution
``` bash
$mgo client call --package 0x3 --module mgo_system --function request_add_stake --args 0x5 gasCoinId validator_address --gas-budget 10000000

Parameter Introduction
gasCoinId: My own mgo objectId
validator_address: The address of the validator pool
```
![gasCoinId](https://image.devnet.mangonetwork.io/img/gas.png "gasCoinId")


##### Extract pledge

Extract pledge execution ==*request_withdraw_stake*==

``` bash
$mgo client call --package 0x3 --module mgo_system --function request_withdraw_stake --args 0x5 stakedMangoId --gas-budget 10000000
```
![stakedMangoId](https://image.devnet.mangonetwork.io/img/stakedMangoId.png "stakedMangoId")


Once you collect enough staking amount, run

``` bash
$mgo validator join-committee --gas-budget 10000000
```

to become a pending validator. A pending validator will become active and join the committee starting from next epoch.

#### Leave Committee

To leave committee, run

``` bash
$mgo validator leave-committee --gas-budget 10000000
```

Then you will be removed from committee starting from next epoch.

### Generate the payload to create PoP

Serialize the payload that is used to generate Proof of Possession. This is allows the signer to take the payload offline for an Authority protocol BLS keypair to sign.

``` bash
$mgo validator serialize-payload-pop --account-address $ACCOUNT_ADDRESS --protocol-public-key $BLS_PUBKEY
Serialized payload: $PAYLOAD_TO_SIGN
```
