// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use jsonrpsee::rpc_params;
use tracing::info;

use mgo_json_rpc_types::MgoTransactionBlockResponse;
use mgo_types::{
    base_types::{ObjectID, MgoAddress},
    crypto::{get_key_pair, AccountKeyPair},
    object::Owner,
};

use crate::{
    helper::{BalanceChangeChecker, ObjectChecker},
    TestCaseImpl, TestContext,
};

pub struct NativeTransferTest;

#[async_trait]
impl TestCaseImpl for NativeTransferTest {
    fn name(&self) -> &'static str {
        "NativeTransfer"
    }

    fn description(&self) -> &'static str {
        "Test tranferring MGO coins natively"
    }

    async fn run(&self, ctx: &mut TestContext) -> Result<(), anyhow::Error> {
        info!("Testing gas coin transfer");
        let mut mgo_objs = ctx.get_mgo_from_faucet(Some(1)).await;
        let gas_obj = ctx.get_mgo_from_faucet(Some(1)).await.swap_remove(0);

        let signer = ctx.get_wallet_address();
        let (recipient_addr, _): (_, AccountKeyPair) = get_key_pair();
        // Test transfer object
        let obj_to_transfer: ObjectID = *mgo_objs.swap_remove(0).id();
        let params = rpc_params![
            signer,
            obj_to_transfer,
            Some(*gas_obj.id()),
            (2_000_000).to_string(),
            recipient_addr
        ];
        let data = ctx
            .build_transaction_remotely("unsafe_transferObject", params)
            .await?;
        let mut response = ctx.sign_and_execute(data, "coin transfer").await;

        Self::examine_response(ctx, &mut response, signer, recipient_addr, obj_to_transfer).await;

        let mut mgo_objs_2 = ctx.get_mgo_from_faucet(Some(1)).await;
        // Test transfer mgo
        let obj_to_transfer_2 = *mgo_objs_2.swap_remove(0).id();
        let params = rpc_params![
            signer,
            obj_to_transfer_2,
            (2_000_000).to_string(),
            recipient_addr,
            None::<u64>
        ];
        let data = ctx
            .build_transaction_remotely("unsafe_transferMgo", params)
            .await?;
        let mut response = ctx.sign_and_execute(data, "coin transfer").await;

        Self::examine_response(ctx, &mut response, signer, recipient_addr, obj_to_transfer).await;
        Ok(())
    }
}

impl NativeTransferTest {
    async fn examine_response(
        ctx: &TestContext,
        response: &mut MgoTransactionBlockResponse,
        signer: MgoAddress,
        recipient: MgoAddress,
        obj_to_transfer_id: ObjectID,
    ) {
        let balance_changes = &mut response.balance_changes.as_mut().unwrap();
        // for transfer we only expect 2 balance changes, one for sender and one for recipient.
        assert_eq!(
            balance_changes.len(),
            2,
            "Expect 2 balance changes emitted, but got {}",
            balance_changes.len()
        );
        // Order of balance change is not fixed so need to check who's balance come first.
        // this make sure recipient always come first
        if balance_changes[0].owner.get_owner_address().unwrap() == signer {
            balance_changes.reverse()
        }
        BalanceChangeChecker::new()
            .owner(Owner::AddressOwner(recipient))
            .coin_type("0x2::mgo::MGO")
            .check(&balance_changes.remove(0));
        BalanceChangeChecker::new()
            .owner(Owner::AddressOwner(signer))
            .coin_type("0x2::mgo::MGO")
            .check(&balance_changes.remove(0));
        // Verify fullnode observes the txn
        ctx.let_fullnode_sync(vec![response.digest], 5).await;

        let _ = ObjectChecker::new(obj_to_transfer_id)
            .owner(Owner::AddressOwner(recipient))
            .check(ctx.get_fullnode_client())
            .await;
    }
}
