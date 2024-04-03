// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::{context_data::db_data_provider::PgManager, data::Db};

use super::balance::{self, Balance};
use super::base64::Base64;
use super::coin::Coin;
use super::cursor::Page;
use super::display::DisplayEntry;
use super::dynamic_field::{DynamicField, DynamicFieldName};
use super::move_object::MoveObjectImpl;
use super::move_value::MoveValue;
use super::object::{Object, ObjectFilter, ObjectImpl, ObjectOwner, ObjectStatus};
use super::owner::OwnerImpl;
use super::mgons_registration::MgonsRegistration;
use super::transaction_block::{self, TransactionBlock, TransactionBlockFilter};
use super::type_filter::ExactTypeFilter;
use super::{
    big_int::BigInt, epoch::Epoch, move_object::MoveObject, object, mgo_address::MgoAddress,
};
use async_graphql::connection::Connection;
use async_graphql::*;
use move_core_types::language_storage::StructTag;
use mgo_json_rpc_types::{Stake as RpcStakedMgo, StakeStatus as RpcStakeStatus};
use mgo_types::base_types::MoveObjectType;
use mgo_types::governance::StakedMgo as NativeStakedMgo;

#[derive(Copy, Clone, Enum, PartialEq, Eq)]
/// The stake's possible status: active, pending, or unstaked.
pub(crate) enum StakeStatus {
    /// The stake object is active in a staking pool and it is generating rewards.
    Active,
    /// The stake awaits to join a staking pool in the next epoch.
    Pending,
    /// The stake is no longer active in any staking pool.
    Unstaked,
}

pub(crate) enum StakedMgoDowncastError {
    NotAStakedMgo,
    Bcs(bcs::Error),
}

#[derive(Clone)]
pub(crate) struct StakedMgo {
    /// Representation of this StakedMgo as a generic Move Object.
    pub super_: MoveObject,

    /// Deserialized representation of the Move Object's contents as a
    /// `0x3::staking_pool::StakedMgo`.
    pub native: NativeStakedMgo,
}

/// Represents a `0x3::staking_pool::StakedMgo` Move object on-chain.
#[Object]
impl StakedMgo {
    pub(crate) async fn address(&self) -> MgoAddress {
        OwnerImpl::from(&self.super_.super_).address().await
    }

    /// Objects owned by this object, optionally `filter`-ed.
    pub(crate) async fn objects(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
        filter: Option<ObjectFilter>,
    ) -> Result<Connection<String, MoveObject>> {
        OwnerImpl::from(&self.super_.super_)
            .objects(ctx, first, after, last, before, filter)
            .await
    }

    /// Total balance of all coins with marker type owned by this object. If type is not supplied,
    /// it defaults to `0x2::mgo::MGO`.
    pub(crate) async fn balance(
        &self,
        ctx: &Context<'_>,
        type_: Option<ExactTypeFilter>,
    ) -> Result<Option<Balance>> {
        OwnerImpl::from(&self.super_.super_)
            .balance(ctx, type_)
            .await
    }

    /// The balances of all coin types owned by this object.
    pub(crate) async fn balances(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<balance::Cursor>,
        last: Option<u64>,
        before: Option<balance::Cursor>,
    ) -> Result<Connection<String, Balance>> {
        OwnerImpl::from(&self.super_.super_)
            .balances(ctx, first, after, last, before)
            .await
    }

    /// The coin objects for this object.
    ///
    ///`type` is a filter on the coin's type parameter, defaulting to `0x2::mgo::MGO`.
    pub(crate) async fn coins(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
        type_: Option<ExactTypeFilter>,
    ) -> Result<Connection<String, Coin>> {
        OwnerImpl::from(&self.super_.super_)
            .coins(ctx, first, after, last, before, type_)
            .await
    }

    /// The `0x3::staking_pool::StakedMgo` objects owned by this object.
    pub(crate) async fn staked_mgos(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
    ) -> Result<Connection<String, StakedMgo>> {
        OwnerImpl::from(&self.super_.super_)
            .staked_mgos(ctx, first, after, last, before)
            .await
    }

    /// The domain explicitly configured as the default domain pointing to this object.
    pub(crate) async fn default_mgons_name(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        OwnerImpl::from(&self.super_.super_)
            .default_mgons_name(ctx)
            .await
    }

    /// The MgonsRegistration NFTs owned by this object. These grant the owner the capability to
    /// manage the associated domain.
    pub(crate) async fn mgons_registrations(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
    ) -> Result<Connection<String, MgonsRegistration>> {
        OwnerImpl::from(&self.super_.super_)
            .mgons_registrations(ctx, first, after, last, before)
            .await
    }

    pub(crate) async fn version(&self) -> u64 {
        ObjectImpl(&self.super_.super_).version().await
    }

    /// The current status of the object as read from the off-chain store. The possible states are:
    /// NOT_INDEXED, the object is loaded from serialized data, such as the contents of a genesis or
    /// system package upgrade transaction. LIVE, the version returned is the most recent for the
    /// object, and it is not deleted or wrapped at that version. HISTORICAL, the object was
    /// referenced at a specific version or checkpoint, so is fetched from historical tables and may
    /// not be the latest version of the object. WRAPPED_OR_DELETED, the object is deleted or
    /// wrapped and only partial information can be loaded."
    pub(crate) async fn status(&self) -> ObjectStatus {
        ObjectImpl(&self.super_.super_).status().await
    }

    /// 32-byte hash that identifies the object's contents, encoded as a Base58 string.
    pub(crate) async fn digest(&self) -> Option<String> {
        ObjectImpl(&self.super_.super_).digest().await
    }

    /// The owner type of this object: Immutable, Shared, Parent, Address
    pub(crate) async fn owner(&self, ctx: &Context<'_>) -> Option<ObjectOwner> {
        ObjectImpl(&self.super_.super_).owner(ctx).await
    }

    /// The transaction block that created this version of the object.
    pub(crate) async fn previous_transaction_block(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<TransactionBlock>> {
        ObjectImpl(&self.super_.super_)
            .previous_transaction_block(ctx)
            .await
    }

    /// The amount of MGO we would rebate if this object gets deleted or mutated. This number is
    /// recalculated based on the present storage gas price.
    pub(crate) async fn storage_rebate(&self) -> Option<BigInt> {
        ObjectImpl(&self.super_.super_).storage_rebate().await
    }

    /// The transaction blocks that sent objects to this object.
    pub(crate) async fn received_transaction_blocks(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<transaction_block::Cursor>,
        last: Option<u64>,
        before: Option<transaction_block::Cursor>,
        filter: Option<TransactionBlockFilter>,
    ) -> Result<Connection<String, TransactionBlock>> {
        ObjectImpl(&self.super_.super_)
            .received_transaction_blocks(ctx, first, after, last, before, filter)
            .await
    }

    /// The Base64-encoded BCS serialization of the object's content.
    pub(crate) async fn bcs(&self) -> Result<Option<Base64>> {
        ObjectImpl(&self.super_.super_).bcs().await
    }

    /// Displays the contents of the Move object in a JSON string and through GraphQL types. Also
    /// provides the flat representation of the type signature, and the BCS of the corresponding
    /// data.
    pub(crate) async fn contents(&self) -> Option<MoveValue> {
        MoveObjectImpl(&self.super_).contents().await
    }

    /// Determines whether a transaction can transfer this object, using the TransferObjects
    /// transaction command or `mgo::transfer::public_transfer`, both of which require the object to
    /// have the `key` and `store` abilities.
    pub(crate) async fn has_public_transfer(&self, ctx: &Context<'_>) -> Result<bool> {
        MoveObjectImpl(&self.super_).has_public_transfer(ctx).await
    }

    /// The set of named templates defined on-chain for the type of this object, to be handled
    /// off-chain. The server substitutes data from the object into these templates to generate a
    /// display string per template.
    pub(crate) async fn display(&self, ctx: &Context<'_>) -> Result<Option<Vec<DisplayEntry>>> {
        ObjectImpl(&self.super_.super_).display(ctx).await
    }

    /// Access a dynamic field on an object using its name. Names are arbitrary Move values whose
    /// type have `copy`, `drop`, and `store`, and are specified using their type, and their BCS
    /// contents, Base64 encoded.
    ///
    /// Dynamic fields on wrapped objects can be accessed by using the same API under the Owner
    /// type.
    pub(crate) async fn dynamic_field(
        &self,
        ctx: &Context<'_>,
        name: DynamicFieldName,
    ) -> Result<Option<DynamicField>> {
        OwnerImpl::from(&self.super_.super_)
            .dynamic_field(ctx, name, Some(self.super_.super_.version_impl()))
            .await
    }

    /// Access a dynamic object field on an object using its name. Names are arbitrary Move values
    /// whose type have `copy`, `drop`, and `store`, and are specified using their type, and their
    /// BCS contents, Base64 encoded. The value of a dynamic object field can also be accessed
    /// off-chain directly via its address (e.g. using `Query.object`).
    ///
    /// Dynamic fields on wrapped objects can be accessed by using the same API under the Owner
    /// type.
    pub(crate) async fn dynamic_object_field(
        &self,
        ctx: &Context<'_>,
        name: DynamicFieldName,
    ) -> Result<Option<DynamicField>> {
        OwnerImpl::from(&self.super_.super_)
            .dynamic_object_field(ctx, name, Some(self.super_.super_.version_impl()))
            .await
    }

    /// The dynamic fields and dynamic object fields on an object.
    ///
    /// Dynamic fields on wrapped objects can be accessed by using the same API under the Owner
    /// type.
    pub(crate) async fn dynamic_fields(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
    ) -> Result<Connection<String, DynamicField>> {
        OwnerImpl::from(&self.super_.super_)
            .dynamic_fields(
                ctx,
                first,
                after,
                last,
                before,
                Some(self.super_.super_.version_impl()),
            )
            .await
    }

    /// A stake can be pending, active, or unstaked
    async fn stake_status(&self, ctx: &Context<'_>) -> Result<StakeStatus> {
        Ok(match self.rpc_stake(ctx).await.extend()?.status {
            RpcStakeStatus::Pending => StakeStatus::Pending,
            RpcStakeStatus::Active { .. } => StakeStatus::Active,
            RpcStakeStatus::Unstaked => StakeStatus::Unstaked,
        })
    }

    /// The epoch at which this stake became active.
    async fn activated_epoch(&self, ctx: &Context<'_>) -> Result<Option<Epoch>> {
        Epoch::query(
            ctx.data_unchecked(),
            Some(self.native.activation_epoch()),
            self.super_.super_.checkpoint_viewed_at,
        )
        .await
        .extend()
    }

    /// The epoch at which this object was requested to join a stake pool.
    async fn requested_epoch(&self, ctx: &Context<'_>) -> Result<Option<Epoch>> {
        Epoch::query(
            ctx.data_unchecked(),
            Some(self.native.request_epoch()),
            self.super_.super_.checkpoint_viewed_at,
        )
        .await
        .extend()
    }

    /// The object id of the validator staking pool this stake belongs to.
    async fn pool_id(&self) -> Option<MgoAddress> {
        Some(self.native.pool_id().into())
    }

    /// The MGO that was initially staked.
    async fn principal(&self) -> Option<BigInt> {
        Some(BigInt::from(self.native.principal()))
    }

    /// The estimated reward for this stake object, calculated as:
    ///
    ///  principal * (initial_stake_rate / current_stake_rate - 1.0)
    ///
    /// Or 0, if this value is negative, where:
    ///
    /// - `initial_stake_rate` is the stake rate at the epoch this stake was activated at.
    /// - `current_stake_rate` is the stake rate in the current epoch.
    ///
    /// This value is only available if the stake is active.
    async fn estimated_reward(&self, ctx: &Context<'_>) -> Result<Option<BigInt>, Error> {
        let RpcStakeStatus::Active { estimated_reward } = self.rpc_stake(ctx).await?.status else {
            return Ok(None);
        };

        Ok(Some(BigInt::from(estimated_reward)))
    }
}

impl StakedMgo {
    /// Query the database for a `page` of Staked MGO. The page uses the same cursor type as is used
    /// for `Object`, and is further filtered to a particular `owner`.
    ///
    /// `checkpoint_viewed_at` represents the checkpoint sequence number at which this page was
    /// queried for, or `None` if the data was requested at the latest checkpoint. Each entity
    /// returned in the connection will inherit this checkpoint, so that when viewing that entity's
    /// state, it will be as if it was read at the same checkpoint.
    pub(crate) async fn paginate(
        db: &Db,
        page: Page<object::Cursor>,
        owner: MgoAddress,
        checkpoint_viewed_at: Option<u64>,
    ) -> Result<Connection<String, StakedMgo>, Error> {
        let type_: StructTag = MoveObjectType::staked_mgo().into();

        let filter = ObjectFilter {
            type_: Some(type_.into()),
            owner: Some(owner),
            ..Default::default()
        };

        Object::paginate_subtype(db, page, filter, checkpoint_viewed_at, |object| {
            let address = object.address;
            let move_object = MoveObject::try_from(&object).map_err(|_| {
                Error::Internal(format!(
                    "Expected {address} to be a StakedMgo, but it's not a Move Object.",
                ))
            })?;

            StakedMgo::try_from(&move_object).map_err(|_| {
                Error::Internal(format!(
                    "Expected {address} to be a StakedMgo, but it is not."
                ))
            })
        })
        .await
    }

    /// The JSON-RPC representation of a StakedMgo so that we can "cheat" to implement fields that
    /// are not yet implemented directly for GraphQL.
    ///
    /// TODO: Make this obsolete
    async fn rpc_stake(&self, ctx: &Context<'_>) -> Result<RpcStakedMgo, Error> {
        ctx.data_unchecked::<PgManager>()
            .fetch_rpc_staked_mgo(self.native.clone())
            .await
    }
}

impl TryFrom<&MoveObject> for StakedMgo {
    type Error = StakedMgoDowncastError;

    fn try_from(move_object: &MoveObject) -> Result<Self, Self::Error> {
        if !move_object.native.is_staked_mgo() {
            return Err(StakedMgoDowncastError::NotAStakedMgo);
        }

        Ok(Self {
            super_: move_object.clone(),
            native: bcs::from_bytes(move_object.native.contents())
                .map_err(StakedMgoDowncastError::Bcs)?,
        })
    }
}
