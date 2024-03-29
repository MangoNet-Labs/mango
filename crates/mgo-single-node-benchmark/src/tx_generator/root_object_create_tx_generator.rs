// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::mock_account::Account;
use crate::tx_generator::TxGenerator;
use mgo_test_transaction_builder::TestTransactionBuilder;
use mgo_types::base_types::ObjectID;
use mgo_types::transaction::{CallArg, Transaction, DEFAULT_VALIDATOR_GAS_PRICE};

pub struct RootObjectCreateTxGenerator {
    move_package: ObjectID,
    child_per_root: u64,
}

impl RootObjectCreateTxGenerator {
    pub fn new(move_package: ObjectID, child_per_root: u64) -> Self {
        Self {
            move_package,
            child_per_root,
        }
    }
}

impl TxGenerator for RootObjectCreateTxGenerator {
    fn generate_tx(&self, account: Account) -> Transaction {
        TestTransactionBuilder::new(
            account.sender,
            account.gas_objects[0],
            DEFAULT_VALIDATOR_GAS_PRICE,
        )
        .move_call(
            self.move_package,
            "benchmark",
            "generate_dynamic_fields",
            vec![CallArg::Pure(bcs::to_bytes(&self.child_per_root).unwrap())],
        )
        .build_and_sign(account.keypair.as_ref())
    }

    fn name(&self) -> &'static str {
        "Root Object Creation Transaction Generator"
    }
}
