// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::cluster::new_wallet_context_from_cluster;

use super::Cluster;
use shared_crypto::intent::Intent;
use mgo_keys::keystore::AccountKeystore;
use mgo_sdk::wallet_context::WalletContext;
use mgo_sdk::{MgoClient, MgoClientBuilder};
use mgo_types::base_types::MgoAddress;
use mgo_types::crypto::{KeypairTraits, Signature};
use mgo_types::transaction::TransactionData;
use tracing::{info, info_span, Instrument};

pub struct WalletClient {
    wallet_context: WalletContext,
    address: MgoAddress,
    fullnode_client: MgoClient,
}

#[allow(clippy::borrowed_box)]
impl WalletClient {
    pub async fn new_from_cluster(cluster: &(dyn Cluster + Sync + Send)) -> Self {
        let key = cluster.user_key();
        let address: MgoAddress = key.public().into();
        let wallet_context = new_wallet_context_from_cluster(cluster, key)
            .instrument(info_span!("init_wallet_context_for_test_user"))
            .await;

        let rpc_url = String::from(cluster.fullnode_url());
        info!("Use fullnode rpc: {}", &rpc_url);
        let fullnode_client = MgoClientBuilder::default().build(rpc_url).await.unwrap();

        Self {
            wallet_context,
            address,
            fullnode_client,
        }
    }

    pub fn get_wallet(&self) -> &WalletContext {
        &self.wallet_context
    }

    pub fn get_wallet_mut(&mut self) -> &mut WalletContext {
        &mut self.wallet_context
    }

    pub fn get_wallet_address(&self) -> MgoAddress {
        self.address
    }

    pub fn get_fullnode_client(&self) -> &MgoClient {
        &self.fullnode_client
    }

    pub fn sign(&self, txn_data: &TransactionData, desc: &str) -> Signature {
        self.get_wallet()
            .config
            .keystore
            .sign_secure(&self.address, txn_data, Intent::mgo_transaction())
            .unwrap_or_else(|e| panic!("Failed to sign transaction for {}. {}", desc, e))
    }
}
