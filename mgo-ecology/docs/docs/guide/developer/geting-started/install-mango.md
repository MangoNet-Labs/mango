---
title: Install Mango
description: Install the Mango framework and required prerequisites on your system, including the Mango command line interface to interact with the Mango network.
sidebar_position: 1
---

The quickest way to install Mango is using the binaries delivered with every release. 

## Supported operating systems

Mango supports the following operating systems:

- Linux - Ubuntu version 20.04 (Bionic Beaver)
- macOS - macOS Monterey

## Install from binaries
Each Mango release provides a set of binaries for several operating systems. You can download these binaries from GitHub and use them to install Mango.

1. Go to https://github.com/MangoNet-Labs/mango.
1. In the right pane, find the **Releases** section.
1. Click the release tagged **Latest** to open the release's page.
1. In the **Assets** section of the release, select the .tgz compressed file that corresponds to your operating system.
1. Double-click the downloaded file. If the file doesn't automatically expand, manually unzip the file.
1. Open the expanded folder and double-click the appropriate binary to install, beginning with mgo-`<OS>`-`<ARCHITECTURE>`:
- mgo-faucet-`<OS>`-`<ARCHITECTURE>`: Local faucet to mint coins on local network.
    - mgo-indexer-`<OS>`-`<ARCHITECTURE>`: An indexer for a local Mango network.
    - mgo-`<OS>`-`<ARCHITECTURE>`: Main Mango binary.
    - mgo-node-`<OS>`-`<ARCHITECTURE>`: Run a local node.
    - mgo-test-validator-`<OS>`-`<ARCHITECTURE>`: Run test validators on a local network for development.
    - mgo-tool-`<OS>`-`<ARCHITECTURE>`: Provides utilities for Mango.

The Mango binary takes several minutes to download and install files, so make sure you allocate enough time for it to complete. Actual time to complete depends on your network connection and computer specifications.

### Confirm the installation

To confirm that Mango installed correctly, type `mango` and press Enter. You should see a message about the Mango version installed and help for using Mango commands.

## Next steps {#next-steps}
Now that you have Mango installed, it's time to start developing. Check out the following topics to start working with Mango:

- Read about the [Mango CLI](/docs/references/cli/client), the most straightforward way to start exploring Mango networks.
- [Learn about the available networks](./connect) and connect to one.
- [Get some coins](./get-coins) on a development network.