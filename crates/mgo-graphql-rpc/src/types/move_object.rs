// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use super::balance::{self, Balance};
use super::base64::Base64;
use super::big_int::BigInt;
use super::coin::CoinDowncastError;
use super::coin_metadata::{CoinMetadata, CoinMetadataDowncastError};
use super::cursor::Page;
use super::display::DisplayEntry;
use super::dynamic_field::{DynamicField, DynamicFieldName};
use super::move_type::MoveType;
use super::move_value::MoveValue;
use super::object::{self, ObjectFilter, ObjectImpl, ObjectLookupKey, ObjectOwner, ObjectStatus};
use super::owner::OwnerImpl;
use super::stake::StakedMgoDowncastError;
use super::mgo_address::MgoAddress;
use super::mgons_registration::{MgonsRegistration, MgonsRegistrationDowncastError};
use super::transaction_block::{self, TransactionBlock, TransactionBlockFilter};
use super::type_filter::ExactTypeFilter;
use super::{coin::Coin, object::Object};
use crate::data::Db;
use crate::error::Error;
use crate::types::stake::StakedMgo;
use async_graphql::connection::Connection;
use async_graphql::*;
use mgo_json_rpc::name_service::NameServiceConfig;
use mgo_types::object::{Data, MoveObject as NativeMoveObject};
use mgo_types::TypeTag;

#[derive(Clone)]
pub(crate) struct MoveObject {
    /// Representation of this Move Object as a generic Object.
    pub super_: Object,

    /// Move-object-specific data, extracted from the native representation at
    /// `graphql_object.native_object.data`.
    pub native: NativeMoveObject,
}

/// Type to implement GraphQL fields that are shared by all MoveObjects.
pub(crate) struct MoveObjectImpl<'o>(pub &'o MoveObject);

pub(crate) enum MoveObjectDowncastError {
    WrappedOrDeleted,
    NotAMoveObject,
}

/// This interface is implemented by types that represent a Move object on-chain (A Move value whose
/// type has `key`).
#[derive(Interface)]
#[graphql(
    name = "IMoveObject",
    field(
        name = "contents",
        ty = "Option<MoveValue>",
        desc = "Displays the contents of the Move object in a JSON string and through GraphQL \
                types. Also provides the flat representation of the type signature, and the BCS of \
                the corresponding data."
    ),
    field(
        name = "has_public_transfer",
        ty = "bool",
        desc = "Determines whether a transaction can transfer this object, using the \
                TransferObjects transaction command or `mgo::transfer::public_transfer`, both of \
                which require the object to have the `key` and `store` abilities."
    ),
    field(
        name = "display",
        ty = "Option<Vec<DisplayEntry>>",
        desc = "The set of named templates defined on-chain for the type of this object, to be \
                handled off-chain. The server substitutes data from the object into these \
                templates to generate a display string per template."
    ),
    field(
        name = "dynamic_field",
        arg(name = "name", ty = "DynamicFieldName"),
        ty = "Option<DynamicField>",
        desc = "Access a dynamic field on an object using its name. Names are arbitrary Move \
                values whose type have `copy`, `drop`, and `store`, and are specified using their \
                type, and their BCS contents, Base64 encoded.\n\n\
                Dynamic fields on wrapped objects can be accessed by using the same API under the \
                Ownertype."
    ),
    field(
        name = "dynamic_object_field",
        arg(name = "name", ty = "DynamicFieldName"),
        ty = "Option<DynamicField>",
        desc = "Access a dynamic object field on an object using its name. Names are arbitrary \
                Move values whose type have `copy`, `drop`, and `store`, and are specified using \
                their type, and their BCS contents, Base64 encoded. The value of a dynamic object \
                field can also be accessed off-chain directly via its address (e.g. using \
                `Query.object`).\n\n\
                Dynamic fields on wrapped objects can be accessed by using the same API under the \
                Owner type."
    ),
    field(
        name = "dynamic_fields",
        arg(name = "first", ty = "Option<u64>"),
        arg(name = "after", ty = "Option<object::Cursor>"),
        arg(name = "last", ty = "Option<u64>"),
        arg(name = "before", ty = "Option<object::Cursor>"),
        ty = "Connection<String, DynamicField>",
        desc = "The dynamic fields and dynamic object fields on an object.\n\n\
                Dynamic fields on wrapped objects can be accessed by using the same API under the \
                Owner type."
    )
)]
pub(crate) enum IMoveObject {
    MoveObject(MoveObject),
    Coin(Coin),
    CoinMetadata(CoinMetadata),
    StakedMgo(StakedMgo),
    MgonsRegistration(MgonsRegistration),
}

/// The representation of an object as a Move Object, which exposes additional information
/// (content, module that governs it, version, is transferrable, etc.) about this object.
#[Object]
impl MoveObject {
    pub(crate) async fn address(&self) -> MgoAddress {
        OwnerImpl::from(&self.super_).address().await
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
        OwnerImpl::from(&self.super_)
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
        OwnerImpl::from(&self.super_).balance(ctx, type_).await
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
        OwnerImpl::from(&self.super_)
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
        OwnerImpl::from(&self.super_)
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
        OwnerImpl::from(&self.super_)
            .staked_mgos(ctx, first, after, last, before)
            .await
    }

    /// The domain explicitly configured as the default domain pointing to this object.
    pub(crate) async fn default_mgons_name(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        OwnerImpl::from(&self.super_).default_mgons_name(ctx).await
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
        OwnerImpl::from(&self.super_)
            .mgons_registrations(ctx, first, after, last, before)
            .await
    }

    pub(crate) async fn version(&self) -> u64 {
        ObjectImpl(&self.super_).version().await
    }

    /// The current status of the object as read from the off-chain store. The possible states are:
    /// NOT_INDEXED, the object is loaded from serialized data, such as the contents of a genesis or
    /// system package upgrade transaction. LIVE, the version returned is the most recent for the
    /// object, and it is not deleted or wrapped at that version. HISTORICAL, the object was
    /// referenced at a specific version or checkpoint, so is fetched from historical tables and may
    /// not be the latest version of the object. WRAPPED_OR_DELETED, the object is deleted or
    /// wrapped and only partial information can be loaded."
    pub(crate) async fn status(&self) -> ObjectStatus {
        ObjectImpl(&self.super_).status().await
    }

    /// 32-byte hash that identifies the object's contents, encoded as a Base58 string.
    pub(crate) async fn digest(&self) -> Option<String> {
        ObjectImpl(&self.super_).digest().await
    }

    /// The owner type of this object: Immutable, Shared, Parent, Address
    pub(crate) async fn owner(&self, ctx: &Context<'_>) -> Option<ObjectOwner> {
        ObjectImpl(&self.super_).owner(ctx).await
    }

    /// The transaction block that created this version of the object.
    pub(crate) async fn previous_transaction_block(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<TransactionBlock>> {
        ObjectImpl(&self.super_)
            .previous_transaction_block(ctx)
            .await
    }

    /// The amount of MGO we would rebate if this object gets deleted or mutated. This number is
    /// recalculated based on the present storage gas price.
    pub(crate) async fn storage_rebate(&self) -> Option<BigInt> {
        ObjectImpl(&self.super_).storage_rebate().await
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
        ObjectImpl(&self.super_)
            .received_transaction_blocks(ctx, first, after, last, before, filter)
            .await
    }

    /// The Base64-encoded BCS serialization of the object's content.
    pub(crate) async fn bcs(&self) -> Result<Option<Base64>> {
        ObjectImpl(&self.super_).bcs().await
    }

    /// Displays the contents of the Move object in a JSON string and through GraphQL types. Also
    /// provides the flat representation of the type signature, and the BCS of the corresponding
    /// data.
    pub(crate) async fn contents(&self) -> Option<MoveValue> {
        MoveObjectImpl(self).contents().await
    }

    /// Determines whether a transaction can transfer this object, using the TransferObjects
    /// transaction command or `mgo::transfer::public_transfer`, both of which require the object to
    /// have the `key` and `store` abilities.
    pub(crate) async fn has_public_transfer(&self, ctx: &Context<'_>) -> Result<bool> {
        MoveObjectImpl(self).has_public_transfer(ctx).await
    }

    /// The set of named templates defined on-chain for the type of this object, to be handled
    /// off-chain. The server substitutes data from the object into these templates to generate a
    /// display string per template.
    pub(crate) async fn display(&self, ctx: &Context<'_>) -> Result<Option<Vec<DisplayEntry>>> {
        ObjectImpl(&self.super_).display(ctx).await
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
        OwnerImpl::from(&self.super_)
            .dynamic_field(ctx, name, Some(self.super_.version_impl()))
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
        OwnerImpl::from(&self.super_)
            .dynamic_object_field(ctx, name, Some(self.super_.version_impl()))
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
        OwnerImpl::from(&self.super_)
            .dynamic_fields(
                ctx,
                first,
                after,
                last,
                before,
                Some(self.super_.version_impl()),
            )
            .await
    }

    /// Attempts to convert the Move object into a `0x2::coin::Coin`.
    async fn as_coin(&self) -> Result<Option<Coin>> {
        match Coin::try_from(self) {
            Ok(coin) => Ok(Some(coin)),
            Err(CoinDowncastError::NotACoin) => Ok(None),
            Err(CoinDowncastError::Bcs(e)) => {
                Err(Error::Internal(format!("Failed to deserialize Coin: {e}"))).extend()
            }
        }
    }

    /// Attempts to convert the Move object into a `0x3::staking_pool::StakedMgo`.
    async fn as_staked_mgo(&self) -> Result<Option<StakedMgo>> {
        match StakedMgo::try_from(self) {
            Ok(coin) => Ok(Some(coin)),
            Err(StakedMgoDowncastError::NotAStakedMgo) => Ok(None),
            Err(StakedMgoDowncastError::Bcs(e)) => Err(Error::Internal(format!(
                "Failed to deserialize StakedMgo: {e}"
            )))
            .extend(),
        }
    }

    /// Attempts to convert the Move object into a `0x2::coin::CoinMetadata`.
    async fn as_coin_metadata(&self) -> Result<Option<CoinMetadata>> {
        match CoinMetadata::try_from(self) {
            Ok(metadata) => Ok(Some(metadata)),
            Err(CoinMetadataDowncastError::NotCoinMetadata) => Ok(None),
            Err(CoinMetadataDowncastError::Bcs(e)) => Err(Error::Internal(format!(
                "Failed to deserialize CoinMetadata: {e}"
            )))
            .extend(),
        }
    }

    /// Attempts to convert the Move object into a `MgonsRegistration` object.
    async fn as_mgons_registration(&self, ctx: &Context<'_>) -> Result<Option<MgonsRegistration>> {
        let cfg: &NameServiceConfig = ctx.data_unchecked();
        let tag = MgonsRegistration::type_(cfg.package_address.into());

        match MgonsRegistration::try_from(self, &tag) {
            Ok(registration) => Ok(Some(registration)),
            Err(MgonsRegistrationDowncastError::NotAMgonsRegistration) => Ok(None),
            Err(MgonsRegistrationDowncastError::Bcs(e)) => Err(Error::Internal(format!(
                "Failed to deserialize MgonsRegistration: {e}",
            )))
            .extend(),
        }
    }
}

impl MoveObjectImpl<'_> {
    pub(crate) async fn contents(&self) -> Option<MoveValue> {
        let type_ = TypeTag::from(self.0.native.type_().clone());
        Some(MoveValue::new(type_, self.0.native.contents().into()))
    }

    pub(crate) async fn has_public_transfer(&self, ctx: &Context<'_>) -> Result<bool> {
        let type_ = MoveType::new(TypeTag::from(self.0.native.type_().clone()));
        let set = type_.abilities_impl(ctx.data_unchecked()).await.extend()?;
        Ok(set.has_key() && set.has_store())
    }
}

impl MoveObject {
    pub(crate) async fn query(
        db: &Db,
        address: MgoAddress,
        key: ObjectLookupKey,
    ) -> Result<Option<Self>, Error> {
        let Some(object) = Object::query(db, address, key).await? else {
            return Ok(None);
        };

        match MoveObject::try_from(&object) {
            Ok(object) => Ok(Some(object)),
            Err(MoveObjectDowncastError::WrappedOrDeleted) => Ok(None),
            Err(MoveObjectDowncastError::NotAMoveObject) => {
                Err(Error::Internal(format!("{address} is not a Move object")))?
            }
        }
    }

    /// Query the database for a `page` of Move objects, optionally `filter`-ed.
    ///
    /// `checkpoint_viewed_at` represents the checkpoint sequence number at which this page was
    /// queried for, or `None` if the data was requested at the latest checkpoint. Each entity
    /// returned in the connection will inherit this checkpoint, so that when viewing that entity's
    /// state, it will be as if it was read at the same checkpoint.
    pub(crate) async fn paginate(
        db: &Db,
        page: Page<object::Cursor>,
        filter: ObjectFilter,
        checkpoint_viewed_at: Option<u64>,
    ) -> Result<Connection<String, MoveObject>, Error> {
        Object::paginate_subtype(db, page, filter, checkpoint_viewed_at, |object| {
            let address = object.address;
            MoveObject::try_from(&object).map_err(|_| {
                Error::Internal(format!(
                    "Expected {address} to be a Move object, but it's not."
                ))
            })
        })
        .await
    }
}

impl TryFrom<&Object> for MoveObject {
    type Error = MoveObjectDowncastError;

    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        let Some(native) = object.native_impl() else {
            return Err(MoveObjectDowncastError::WrappedOrDeleted);
        };

        if let Data::Move(move_object) = &native.data {
            Ok(Self {
                super_: object.clone(),
                native: move_object.clone(),
            })
        } else {
            Err(MoveObjectDowncastError::NotAMoveObject)
        }
    }
}
