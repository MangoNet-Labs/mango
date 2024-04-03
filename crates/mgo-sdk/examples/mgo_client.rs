// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use mgo_sdk::MgoClientBuilder;

// This example shows the few basic ways to connect to a Mgo network.
// There are several in-built methods for connecting to the
// Mgo devnet, tesnet, and localnet (running locally),
// as well as a custom way for connecting to custom URLs.
// The example prints out the API versions of the different networks,
// and finally, it prints the list of available RPC methods
// and the list of subscriptions.
// Note that running this code will fail if there is no Mgo network
// running locally on the default address: 127.0.0.1:9000

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mgo = MgoClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("Mgo local network version: {}", mgo.api_version());

    // local Mgo network, like the above one but using the dedicated function
    let mgo_local = MgoClientBuilder::default().build_localnet().await?;
    println!("Mgo local network version: {}", mgo_local.api_version());

    // Mgo devnet -- https://fullnode.devnet.mangonetwork.io:443
    let mgo_devnet = MgoClientBuilder::default().build_devnet().await?;
    println!("Mgo devnet version: {}", mgo_devnet.api_version());

    // Mgo testnet -- https://fullnode.testnet.mangonetwork.io:443
    let mgo_testnet = MgoClientBuilder::default().build_testnet().await?;
    println!("Mgo testnet version: {}", mgo_testnet.api_version());

    println!("{:?}", mgo_local.available_rpc_methods());
    println!("{:?}", mgo_local.available_subscriptions());

    Ok(())
}
