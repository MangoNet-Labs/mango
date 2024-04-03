// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use futures::stream::StreamExt;
use mgo_json_rpc_types::TransactionFilter;
use mgo_sdk::MgoClientBuilder;

// This example showcases how to use the Read API to listen
// for transactions. It subscribes to the transactions that
// transfer MGO on the Mgo testnet and prints every incoming
// transaction to the console. The program will loop until it
// is force stopped.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ws = MgoClientBuilder::default()
        .ws_url("wss://rpc.testnet.mangonetwork.io:443")
        .build("https://fullnode.testnet.mangonetwork.io:443")
        .await?;
    println!("WS version {:?}", ws.api_version());

    let mut subscribe = ws
        .read_api()
        .subscribe_transaction(TransactionFilter::MoveFunction {
            package: "0x2".parse()?,
            module: Some("mgo".to_owned()),
            function: Some("transfer".to_owned()),
        })
        .await?;

    loop {
        println!("{:?}", subscribe.next().await);
    }
}
