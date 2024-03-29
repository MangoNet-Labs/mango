// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{
    balance::{self, Balance},
    coin::Coin,
    cursor::Page,
    move_object::MoveObject,
    object::{self, ObjectFilter},
    owner::OwnerImpl,
    stake::StakedMgo,
    mgo_address::MgoAddress,
    mgons_registration::MgonsRegistration,
    transaction_block::{self, TransactionBlock, TransactionBlockFilter},
    type_filter::ExactTypeFilter,
};
use async_graphql::{connection::Connection, *};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub(crate) struct Address {
    pub address: MgoAddress,
    /// The checkpoint sequence number at which this was viewed at, or None if the data was
    /// requested at the latest checkpoint.
    pub checkpoint_viewed_at: Option<u64>,
}

/// The possible relationship types for a transaction block: sign, sent, received, or paid.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub(crate) enum AddressTransactionBlockRelationship {
    /// Transactions this address has signed either as a sender or as a sponsor.
    Sign,
    /// Transactions that sent objects to this address.
    Recv,
}

/// The 32-byte address that is an account address (corresponding to a public key).
#[Object]
impl Address {
    pub(crate) async fn address(&self) -> MgoAddress {
        OwnerImpl::from(self).address().await
    }

    /// Objects owned by this address, optionally `filter`-ed.
    pub(crate) async fn objects(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
        filter: Option<ObjectFilter>,
    ) -> Result<Connection<String, MoveObject>> {
        OwnerImpl::from(self)
            .objects(ctx, first, after, last, before, filter)
            .await
    }

    /// Total balance of all coins with marker type owned by this address. If type is not supplied,
    /// it defaults to `0x2::mgo::MGO`.
    pub(crate) async fn balance(
        &self,
        ctx: &Context<'_>,
        type_: Option<ExactTypeFilter>,
    ) -> Result<Option<Balance>> {
        OwnerImpl::from(self).balance(ctx, type_).await
    }

    /// The balances of all coin types owned by this address.
    pub(crate) async fn balances(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<balance::Cursor>,
        last: Option<u64>,
        before: Option<balance::Cursor>,
    ) -> Result<Connection<String, Balance>> {
        OwnerImpl::from(self)
            .balances(ctx, first, after, last, before)
            .await
    }

    /// The coin objects for this address.
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
        OwnerImpl::from(self)
            .coins(ctx, first, after, last, before, type_)
            .await
    }

    /// The `0x3::staking_pool::StakedMgo` objects owned by this address.
    pub(crate) async fn staked_mgos(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
    ) -> Result<Connection<String, StakedMgo>> {
        OwnerImpl::from(self)
            .staked_mgos(ctx, first, after, last, before)
            .await
    }

    /// The domain explicitly configured as the default domain pointing to this address.
    pub(crate) async fn default_mgons_name(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        OwnerImpl::from(self).default_mgons_name(ctx).await
    }

    /// The MgonsRegistration NFTs owned by this address. These grant the owner the capability to
    /// manage the associated domain.
    pub(crate) async fn mgons_registrations(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
    ) -> Result<Connection<String, MgonsRegistration>> {
        OwnerImpl::from(self)
            .mgons_registrations(ctx, first, after, last, before)
            .await
    }

    /// Similar behavior to the `transactionBlocks` in Query but supporting the additional
    /// `AddressTransactionBlockRelationship` filter, which defaults to `SIGN`.
    async fn transaction_blocks(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<transaction_block::Cursor>,
        last: Option<u64>,
        before: Option<transaction_block::Cursor>,
        relation: Option<AddressTransactionBlockRelationship>,
        filter: Option<TransactionBlockFilter>,
    ) -> Result<Connection<String, TransactionBlock>> {
        use AddressTransactionBlockRelationship as R;
        let page = Page::from_params(ctx.data_unchecked(), first, after, last, before)?;

        let Some(filter) = filter.unwrap_or_default().intersect(match relation {
            // Relationship defaults to "signer" if none is supplied.
            Some(R::Sign) | None => TransactionBlockFilter {
                sign_address: Some(self.address),
                ..Default::default()
            },

            Some(R::Recv) => TransactionBlockFilter {
                recv_address: Some(self.address),
                ..Default::default()
            },
        }) else {
            return Ok(Connection::new(false, false));
        };

        TransactionBlock::paginate(
            ctx.data_unchecked(),
            page,
            filter,
            self.checkpoint_viewed_at,
        )
        .await
        .extend()
    }
}

impl From<&Address> for OwnerImpl {
    fn from(address: &Address) -> Self {
        OwnerImpl {
            address: address.address,
            checkpoint_viewed_at: address.checkpoint_viewed_at,
        }
    }
}
