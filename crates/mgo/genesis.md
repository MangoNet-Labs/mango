# Genesis Ceremony

This document lays out the step-by-step process for orchestrating a Mango Genesis Ceremony.

## Prerequisites 

Each validator participating in the ceremony will need the following:

- Ed25519 Public key
- Mango network address // WAN
- Narwhal_primary_to_primary network address // WAN
- Narwhal_worker_to_primary network address // LAN
- Narwhal_primary_to_worker network address // LAN
- Narwhal_worker_to_worker network address // WAN
- Narwhal_consensus_address network address // LAN

Note:
- Network addresses should be Multiaddrs in the form of `/ip4/{ip4 name}/tcp/{port}/http` and
only the addresses marked WAN need to be publicly accessible by the wider internet.
- An Ed25519 key can be created using `mgo keytool generate`
- Four validators are required for a single server, and each validator's port number cannot be reused

#### Generate key file
Each validator has 4 keys that need to be configured：

| key | scheme     | purpose                           |
| ------------- | ---------------- | --------------------------------- |
| protocol.key      | bls12381          | transactions, narwhal consensus    |
| account.key      | ed25519 | controls assets for staking         |
| network.key      | ed25519 | narwhal primary, mgo state sync          |
| worker.key      | ed25519        | validate narwhal workers          |

There are currently two ways to generate these four keys, one is to use official command-line tools mgo：

```
- generate bls12381
- mgo keytool generate bls12381
- generate ed25519
- mgo keytool generate ed25519
  
Ed25519 is the standard method for generating Mango wallet accounts, which can be interchanged
Another way is to use official command-line tools validator

mgo validator make-validator-info [OPTIONS] <NAME> <DESCRIPTION> <IMAGE_URL> <PROJECT_URL> <HOST_NAME> <GAS_PRICE>
```

## Ceremony

![blob](https://image.devnet.mangonetwork.io/img/blob.png "eg")

1. Creation of a shared workspace

To start, you'll need to create a shared workspace where all validators will be able to share their
information. For these instructions, we'll assume that such a shared workspace is created and managed
using a git repository hosted on git hosting provider.

The MC (Master of Ceremony) will create a new git repository and initialize the directory:

```
$ git init genesis && cd genesis
$ mgo genesis-ceremony init
$ git add .
$ git commit -m "init genesis"
$ git push
```

2. Contribute Validator information

Once the shared workspace has been initialized, each validator can contribute their information:

```
$ git clone <url to genesis repo> && cd genesis
$ mgo genesis-ceremony add-validator \
    --name <human-readable validator name> \
    --key-file <path to key file> \
    --network-address <multiaddr> \
    --narwhal-primary-to-primary <multiaddr> \
    --narwhal-worker-to-primary <multiaddr> \
    --narwhal-primary-to-worker <multiaddr> \
    --narwhal-worker-to-worker <multiaddr> \
    --narwhal-consensus-address <multiaddr>

$ git add .
$ git commit -m "add validator"
$ git push 

eg: Each validator needs to be added once

mgo genesis-ceremony add-validator \
--name "node1" \
--validator-key-file "/$(pwd)/protocol.key" \
--worker-key-file "/$(pwd)/worker.key" \
--account-key-file "/$(pwd)/account.key" \
--network-key-file "/$(pwd)/network.key" \
--network-address "/ip4/127.0.0.1/tcp/41659/http" \
--p2p-address "/ip4/127.0.0.1/udp/45161" \
--narwhal-primary-address "/ip4/127.0.0.1/udp/45661" \
--narwhal-worker-address "/ip4/127.0.0.1/udp/37573" \
--description "node one" \
--image-url "" \
--project-url ""
...
```

3. Build Genesis

Once all validators and gas objects have been added, the MC can build the genesis object:

```
$ mgo genesis-ceremony build-unsigned-checkpoint

Can modify  token-distribution-schedule files 
Please delete the unsigned-genesis  file after modification
Re execute  mgo genesis-ceremony build-unsigned-checkpoint

$ git add .
$ git commit -m "build genesis"
$ git push
```
![Build Genesis](https://image.devnet.mangonetwork.io/img/token.png "Build Genesis")

4. Verify and Sign Genesis

Once genesis is built each validator will need to verify and sign genesis:

```
$ mgo genesis-ceremony verify-and-sign \
    --key-file <path to key file>
$ git add .
$ git commit -m "sign genesis"
$ git push

eg : Each verifier needs to verify their signature once

mgo genesis-ceremony verify-and-sign \
--key-file "/$(pwd)/protocol.key"
...
```

5. Finalize Genesis

Once all validators have successfully verified and signed genesis, the MC can finalize the ceremony
and then the genesis state can be distributed:

```
$ mgo genesis-ceremony finalize
$ git add .
$ git commit -m "add finalize"
$ git push
```
