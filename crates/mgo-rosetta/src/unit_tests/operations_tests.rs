// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::annotated_value::MoveTypeLayout;
use mgo_json_rpc_types::MgoCallArg;
use mgo_types::base_types::{ObjectDigest, ObjectID, SequenceNumber, MgoAddress};
use mgo_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use mgo_types::transaction::{CallArg, TransactionData, TEST_ONLY_GAS_UNIT_FOR_TRANSFER};

use crate::operations::Operations;
use crate::types::ConstructionMetadata;

#[tokio::test]
async fn test_operation_data_parsing() -> Result<(), anyhow::Error> {
    let gas = (
        ObjectID::random(),
        SequenceNumber::new(),
        ObjectDigest::random(),
    );

    let sender = MgoAddress::random_for_testing_only();

    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder
            .pay_mgo(vec![MgoAddress::random_for_testing_only()], vec![10000])
            .unwrap();
        builder.finish()
    };
    let gas_price = 10;
    let data = TransactionData::new_programmable(
        sender,
        vec![gas],
        pt,
        TEST_ONLY_GAS_UNIT_FOR_TRANSFER * gas_price,
        gas_price,
    );

    let ops: Operations = data.clone().try_into()?;
    let metadata = ConstructionMetadata {
        sender,
        coins: vec![gas],
        objects: vec![],
        total_coin_value: 0,
        gas_price,
        budget: TEST_ONLY_GAS_UNIT_FOR_TRANSFER * gas_price,
    };
    let parsed_data = ops.into_internal()?.try_into_data(metadata)?;
    assert_eq!(data, parsed_data);

    Ok(())
}
#[tokio::test]
async fn test_mgo_json() {
    let arg1 = CallArg::Pure(bcs::to_bytes(&1000000u64).unwrap());
    let arg2 = CallArg::Pure(bcs::to_bytes(&30215u64).unwrap());
    let json1 = MgoCallArg::try_from(arg1, Some(&MoveTypeLayout::U64)).unwrap();
    let json2 = MgoCallArg::try_from(arg2, Some(&MoveTypeLayout::U64)).unwrap();
    println!("{:?}, {:?}", json1, json2);
}
