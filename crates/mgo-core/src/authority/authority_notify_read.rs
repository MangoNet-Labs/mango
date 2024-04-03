// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use mgo_types::base_types::{TransactionDigest, TransactionEffectsDigest};
use mgo_types::effects::TransactionEffects;
use mgo_types::error::MgoResult;

#[async_trait]
pub trait EffectsNotifyRead: Send + Sync + 'static {
    /// This method reads executed transaction effects from database.
    /// If effects are not available immediately (i.e. haven't been executed yet),
    /// the method blocks until they are persisted in the database.
    ///
    /// This method **does not** schedule transactions for execution - it is responsibility of the caller
    /// to schedule transactions for execution before calling this method.
    async fn notify_read_executed_effects(
        &self,
        digests: Vec<TransactionDigest>,
    ) -> MgoResult<Vec<TransactionEffects>>;

    async fn notify_read_executed_effects_digests(
        &self,
        digests: Vec<TransactionDigest>,
    ) -> MgoResult<Vec<TransactionEffectsDigest>>;

    fn multi_get_executed_effects(
        &self,
        digests: &[TransactionDigest],
    ) -> MgoResult<Vec<Option<TransactionEffects>>>;
}
