// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

mod utils;
use futures::stream::StreamExt;
use mgo_sdk::rpc_types::EventFilter;
use mgo_sdk::MgoClientBuilder;
use utils::{setup_for_write, split_coin_digest};

// This example showcases how to use the Event API.
// At the end of the program it subscribes to the events
// on the Mgo testnet and prints every incoming event to
// the console. The program will loop until it is force
// stopped.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (mgo, active_address, _second_address) = setup_for_write().await?;

    println!(" *** Get events *** ");
    // for demonstration purposes, we set to make a transaction
    let digest = split_coin_digest(&mgo, &active_address).await?;
    let events = mgo.event_api().get_events(digest).await?;
    println!("{:?}", events);
    println!(" *** Get events ***\n ");

    let descending = true;
    let query_events = mgo
        .event_api()
        .query_events(EventFilter::All(vec![]), None, Some(5), descending) // query first 5 events in descending order
        .await?;
    println!(" *** Query events *** ");
    println!("{:?}", query_events);
    println!(" *** Query events ***\n ");

    let ws = MgoClientBuilder::default()
        .ws_url("wss://rpc.testnet.mangonetwork.io:443")
        .build("https://fullnode.testnet.mangonetwork.io:443")
        .await?;
    println!("WS version {:?}", ws.api_version());

    let mut subscribe = ws
        .event_api()
        .subscribe_event(EventFilter::All(vec![]))
        .await?;

    loop {
        println!("{:?}", subscribe.next().await);
    }
}
