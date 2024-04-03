// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use super::{
    balance::{self, Balance},
    base64::Base64,
    big_int::BigInt,
    coin::Coin,
    cursor::Page,
    display::DisplayEntry,
    dynamic_field::{DynamicField, DynamicFieldName},
    move_object::{MoveObject, MoveObjectImpl},
    move_value::MoveValue,
    object::{self, Object, ObjectFilter, ObjectImpl, ObjectLookupKey, ObjectOwner, ObjectStatus},
    owner::OwnerImpl,
    stake::StakedMgo,
    string_input::impl_string_input,
    mgo_address::MgoAddress,
    transaction_block::{self, TransactionBlock, TransactionBlockFilter},
    type_filter::ExactTypeFilter,
};
use crate::{data::Db, error::Error};
use async_graphql::{connection::Connection, *};
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use serde::{Deserialize, Serialize};
use mgo_json_rpc::name_service::{Domain as NativeDomain, NameRecord, NameServiceConfig};
use mgo_types::{base_types::MgoAddress as NativeMgoAddress, dynamic_field::Field, id::UID};

const MOD_REGISTRATION: &IdentStr = ident_str!("mgons_registration");
const TYP_REGISTRATION: &IdentStr = ident_str!("MgonsRegistration");

/// Represents the "core" of the name service (e.g. the on-chain registry and reverse registry). It
/// doesn't contain any fields because we look them up based on the `NameServiceConfig`.
pub(crate) struct NameService;

/// Wrap MgoNS Domain type to expose as a string scalar in GraphQL.
#[derive(Debug)]
pub(crate) struct Domain(NativeDomain);

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct NativeMgonsRegistration {
    pub id: UID,
    pub domain: NativeDomain,
    pub domain_name: String,
    pub expiration_timestamp_ms: u64,
    pub image_url: String,
}

#[derive(Clone)]
pub(crate) struct MgonsRegistration {
    /// Representation of this MgonsRegistration as a generic Move object.
    pub super_: MoveObject,

    /// The deserialized representation of the Move object's contents.
    pub native: NativeMgonsRegistration,
}

pub(crate) enum MgonsRegistrationDowncastError {
    NotAMgonsRegistration,
    Bcs(bcs::Error),
}

#[Object]
impl MgonsRegistration {
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

    /// Domain name of the MgonsRegistration object
    async fn domain(&self) -> &str {
        &self.native.domain_name
    }
}

impl NameService {
    /// Lookup the MgoNS NameRecord for the given `domain` name. `config` specifies where to find
    /// the domain name registry, and its type.
    pub(crate) async fn resolve_to_record(
        db: &Db,
        config: &NameServiceConfig,
        domain: &Domain,
    ) -> Result<Option<NameRecord>, Error> {
        let record_id = config.record_field_id(&domain.0);

        let Some(object) = MoveObject::query(db, record_id.into(), ObjectLookupKey::Latest).await?
        else {
            return Ok(None);
        };

        let field: Field<NativeDomain, NameRecord> = object
            .native
            .to_rust()
            .ok_or_else(|| Error::Internal("Malformed Mgons NameRecord".to_string()))?;

        Ok(Some(field.value))
    }

    /// Lookup the MgoNS Domain for the given `address`. `config` specifies where to find the domain
    /// name registry, and its type.
    pub(crate) async fn reverse_resolve_to_name(
        db: &Db,
        config: &NameServiceConfig,
        address: MgoAddress,
    ) -> Result<Option<NativeDomain>, Error> {
        let reverse_record_id = config.reverse_record_field_id(address.as_slice());

        let Some(object) =
            MoveObject::query(db, reverse_record_id.into(), ObjectLookupKey::Latest).await?
        else {
            return Ok(None);
        };

        let field: Field<NativeMgoAddress, NativeDomain> = object
            .native
            .to_rust()
            .ok_or_else(|| Error::Internal("Malformed Mgons Domain".to_string()))?;

        Ok(Some(field.value))
    }
}

impl MgonsRegistration {
    /// Query the database for a `page` of MgoNS registrations. The page uses the same cursor type
    /// as is used for `Object`, and is further filtered to a particular `owner`. `config` specifies
    /// where to find the domain name registry and its type.
    ///
    /// `checkpoint_viewed_at` represents the checkpoint sequence number at which this page was
    /// queried for, or `None` if the data was requested at the latest checkpoint. Each entity
    /// returned in the connection will inherit this checkpoint, so that when viewing that entity's
    /// state, it will be as if it was read at the same checkpoint.
    pub(crate) async fn paginate(
        db: &Db,
        config: &NameServiceConfig,
        page: Page<object::Cursor>,
        owner: MgoAddress,
        checkpoint_viewed_at: Option<u64>,
    ) -> Result<Connection<String, MgonsRegistration>, Error> {
        let type_ = MgonsRegistration::type_(config.package_address.into());

        let filter = ObjectFilter {
            type_: Some(type_.clone().into()),
            owner: Some(owner),
            ..Default::default()
        };

        Object::paginate_subtype(db, page, filter, checkpoint_viewed_at, |object| {
            let address = object.address;
            let move_object = MoveObject::try_from(&object).map_err(|_| {
                Error::Internal(format!(
                    "Expected {address} to be a MgonsRegistration, but it's not a Move Object.",
                ))
            })?;

            MgonsRegistration::try_from(&move_object, &type_).map_err(|_| {
                Error::Internal(format!(
                    "Expected {address} to be a MgonsRegistration, but it is not."
                ))
            })
        })
        .await
    }

    /// Return the type representing a `MgonsRegistration` on chain. This can change from chain to
    /// chain (mainnet, testnet, devnet etc).
    pub(crate) fn type_(package: MgoAddress) -> StructTag {
        StructTag {
            address: package.into(),
            module: MOD_REGISTRATION.to_owned(),
            name: TYP_REGISTRATION.to_owned(),
            type_params: vec![],
        }
    }

    // Because the type of the MgonsRegistration object is not constant,
    // we need to take it in as a param.
    pub(crate) fn try_from(
        move_object: &MoveObject,
        tag: &StructTag,
    ) -> Result<Self, MgonsRegistrationDowncastError> {
        if !move_object.native.is_type(tag) {
            return Err(MgonsRegistrationDowncastError::NotAMgonsRegistration);
        }

        Ok(Self {
            super_: move_object.clone(),
            native: bcs::from_bytes(move_object.native.contents())
                .map_err(MgonsRegistrationDowncastError::Bcs)?,
        })
    }
}

impl_string_input!(Domain);

impl FromStr for Domain {
    type Err = <NativeDomain as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Domain(NativeDomain::from_str(s)?))
    }
}
