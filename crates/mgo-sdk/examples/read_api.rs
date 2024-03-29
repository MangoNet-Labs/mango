// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod utils;
use mgo_sdk::rpc_types::{
    MgoGetPastObjectRequest, MgoObjectDataOptions, MgoTransactionBlockResponseOptions,
};
use mgo_sdk::types::base_types::ObjectID;
use utils::{setup_for_write, split_coin_digest};

// This example uses the Read API to get owned objects of an address,
// the dynamic fields of an object,
// past objects, information about the chain
// and the protocol configuration,
// the transaction data after executing a transaction,
// and finally, the number of transaction blocks known to the server.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (mgo, active_address, _) = setup_for_write().await?;

    // ************ READ API ************ //
    println!("// ************ READ API ************ //\n");
    // Owned Objects
    let owned_objects = mgo
        .read_api()
        .get_owned_objects(active_address, None, None, Some(5))
        .await?;
    println!(" *** Owned Objects ***");
    println!("{:?}", owned_objects);
    println!(" *** Owned Objects ***\n");

    // Dynamic Fields
    let parent_object_id = ObjectID::from_address(active_address.into());
    let dynamic_fields = mgo
        .read_api()
        .get_dynamic_fields(parent_object_id, None, None)
        .await?;
    println!(" *** Dynamic Fields ***");
    println!("{:?}", dynamic_fields);
    println!(" *** Dynamic Fields ***\n");
    if let Some(dynamic_field_info) = dynamic_fields.data.into_iter().next() {
        println!(" *** First Dynamic Field ***");
        let dynamic_field = mgo
            .read_api()
            .get_dynamic_field_object(parent_object_id, dynamic_field_info.name)
            .await?;
        println!("{dynamic_field:?}");
        println!(" *** First Dynamic Field ***\n");
    }

    let object = owned_objects
        .data
        .first()
        .unwrap_or_else(|| panic!("No object data for this address {}", active_address));
    let object_data = object
        .data
        .as_ref()
        .unwrap_or_else(|| panic!("No object data for this MgoObjectResponse {:?}", object));
    let object_id = object_data.object_id;
    let version = object_data.version;

    let mgo_data_options = MgoObjectDataOptions {
        show_type: true,
        show_owner: true,
        show_previous_transaction: true,
        show_display: true,
        show_content: true,
        show_bcs: true,
        show_storage_rebate: true,
    };

    let past_object = mgo
        .read_api()
        .try_get_parsed_past_object(object_id, version, mgo_data_options.clone())
        .await?;
    println!(" *** Past Object *** ");
    println!("{:?}", past_object);
    println!(" *** Past Object ***\n");

    let mgo_get_past_object_request = past_object.clone().into_object()?;
    let multi_past_object = mgo
        .read_api()
        .try_multi_get_parsed_past_object(
            vec![MgoGetPastObjectRequest {
                object_id: mgo_get_past_object_request.object_id,
                version: mgo_get_past_object_request.version,
            }],
            mgo_data_options.clone(),
        )
        .await?;
    println!(" *** Multi Past Object *** ");
    println!("{:?}", multi_past_object);
    println!(" *** Multi Past Object ***\n");

    // Object with options
    let object_with_options = mgo
        .read_api()
        .get_object_with_options(mgo_get_past_object_request.object_id, mgo_data_options)
        .await?;

    println!(" *** Object with Options *** ");
    println!("{:?}", object_with_options);
    println!(" *** Object with Options ***\n");

    println!(" *** Chain identifier *** ");
    println!("{:?}", mgo.read_api().get_chain_identifier().await?);
    println!(" *** Chain identifier ***\n ");

    println!(" *** Protocol Config *** ");
    println!("{:?}", mgo.read_api().get_protocol_config(None).await?);
    println!(" *** Protocol Config ***\n ");

    // we make a dummy transaction which returns a transaction digest
    let tx_digest = split_coin_digest(&mgo, &active_address).await?;
    println!(" *** Transaction data *** ");
    let tx_response = mgo
        .read_api()
        .get_transaction_with_options(
            tx_digest,
            MgoTransactionBlockResponseOptions {
                show_input: true,
                show_raw_input: true,
                show_effects: true,
                show_events: true,
                show_object_changes: true,
                show_balance_changes: true,
                show_raw_effects: true,
            },
        )
        .await?;
    println!("Transaction succeeded: {:?}\n\n", tx_response.status_ok());

    println!("Transaction data: {:?}", tx_response);

    let tx_blocks = mgo.read_api().get_total_transaction_blocks().await?;
    println!("Total transaction blocks {tx_blocks}");
    // ************ END OF READ API ************ //

    Ok(())
}
