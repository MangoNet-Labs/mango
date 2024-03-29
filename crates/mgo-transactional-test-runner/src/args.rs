// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{bail, ensure};
use clap;
use move_command_line_common::parser::{parse_u256, parse_u64};
use move_command_line_common::values::{ParsableValue, ParsedValue};
use move_command_line_common::{parser::Parser as MoveCLParser, values::ValueToken};
use move_core_types::runtime_value::{MoveStruct, MoveValue};
use move_core_types::u256::U256;
use move_symbol_pool::Symbol;
use move_transactional_test_runner::tasks::SyntaxChoice;
use mgo_types::base_types::{SequenceNumber, MgoAddress};
use mgo_types::move_package::UpgradePolicy;
use mgo_types::object::{Object, Owner};
use mgo_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use mgo_types::transaction::{Argument, CallArg, ObjectArg};

use crate::test_adapter::{FakeID, MgoTestAdapter};

pub const MGO_ARGS_LONG: &str = "mgo-args";

#[derive(Debug, clap::Parser)]
pub struct MgoRunArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
    #[clap(long = "summarize")]
    pub summarize: bool,
}

#[derive(Debug, clap::Parser, Default)]
pub struct MgoPublishArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "upgradeable", action = clap::ArgAction::SetTrue)]
    pub upgradeable: bool,
    #[clap(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct MgoInitArgs {
    #[clap(long = "accounts", num_args(1..))]
    pub accounts: Option<Vec<String>>,
    #[clap(long = "protocol-version")]
    pub protocol_version: Option<u64>,
    #[clap(long = "max-gas")]
    pub max_gas: Option<u64>,
    #[clap(long = "shared-object-deletion")]
    pub shared_object_deletion: Option<bool>,
    #[clap(long = "simulator")]
    pub simulator: bool,
    #[clap(long = "custom-validator-account")]
    pub custom_validator_account: bool,
    #[clap(long = "reference-gas-price")]
    pub reference_gas_price: Option<u64>,
    #[clap(long = "default-gas-price")]
    pub default_gas_price: Option<u64>,
    #[clap(long = "object-snapshot-min-checkpoint-lag")]
    pub object_snapshot_min_checkpoint_lag: Option<usize>,
    #[clap(long = "object-snapshot-max-checkpoint-lag")]
    pub object_snapshot_max_checkpoint_lag: Option<usize>,
}

#[derive(Debug, clap::Parser)]
pub struct ViewObjectCommand {
    #[clap(value_parser = parse_fake_id)]
    pub id: FakeID,
}

#[derive(Debug, clap::Parser)]
pub struct TransferObjectCommand {
    #[clap(value_parser = parse_fake_id)]
    pub id: FakeID,
    #[clap(long = "recipient")]
    pub recipient: String,
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct ConsensusCommitPrologueCommand {
    #[clap(long = "timestamp-ms")]
    pub timestamp_ms: u64,
}

#[derive(Debug, clap::Parser)]
pub struct ProgrammableTransactionCommand {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
    #[clap(long = "dev-inspect")]
    pub dev_inspect: bool,
    #[clap(
        long = "inputs",
        value_parser = ParsedValue::<MgoExtraValueArgs>::parse,
        num_args(1..),
        action = clap::ArgAction::Append,
    )]
    pub inputs: Vec<ParsedValue<MgoExtraValueArgs>>,
}

#[derive(Debug, clap::Parser)]
pub struct UpgradePackageCommand {
    #[clap(long = "package")]
    pub package: String,
    #[clap(long = "upgrade-capability", value_parser = parse_fake_id)]
    pub upgrade_capability: FakeID,
    #[clap(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
    #[clap(long = "sender")]
    pub sender: String,
    #[clap(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[clap(long = "syntax")]
    pub syntax: Option<SyntaxChoice>,
    #[clap(long = "policy", default_value="compatible", value_parser = parse_policy)]
    pub policy: u8,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct StagePackageCommand {
    #[clap(long = "syntax")]
    pub syntax: Option<SyntaxChoice>,
    #[clap(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
}

#[derive(Debug, clap::Parser)]
pub struct SetAddressCommand {
    pub address: String,
    #[clap(value_parser = ParsedValue::<MgoExtraValueArgs>::parse)]
    pub input: ParsedValue<MgoExtraValueArgs>,
}

#[derive(Debug, clap::Parser)]
pub struct AdvanceClockCommand {
    #[clap(long = "duration-ns")]
    pub duration_ns: u64,
}

#[derive(Debug, clap::Parser)]
pub struct RunGraphqlCommand {
    #[clap(long = "show-usage")]
    pub show_usage: bool,
    #[clap(long = "show-headers")]
    pub show_headers: bool,
    #[clap(long = "show-service-version")]
    pub show_service_version: bool,
    #[clap(long, num_args(1..))]
    pub cursors: Vec<String>,
}

#[derive(Debug, clap::Parser)]
pub struct ForceObjectSnapshotCatchup {
    #[clap(long = "start-cp")]
    pub start_cp: u64,
    #[clap(long = "end-cp")]
    pub end_cp: u64,
}

#[derive(Debug, clap::Parser)]
pub struct CreateCheckpointCommand {
    pub count: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct AdvanceEpochCommand {
    pub count: Option<u64>,
    #[clap(long = "create-random-state")]
    pub create_random_state: bool,
}

#[derive(Debug, clap::Parser)]
pub struct SetRandomStateCommand {
    #[clap(long = "randomness-round")]
    pub randomness_round: u64,
    #[clap(long = "random-bytes")]
    pub random_bytes: String,
    #[clap(long = "randomness-initial-version")]
    pub randomness_initial_version: u64,
}

#[derive(Debug, clap::Parser)]
pub enum MgoSubcommand {
    #[clap(name = "view-object")]
    ViewObject(ViewObjectCommand),
    #[clap(name = "transfer-object")]
    TransferObject(TransferObjectCommand),
    #[clap(name = "consensus-commit-prologue")]
    ConsensusCommitPrologue(ConsensusCommitPrologueCommand),
    #[clap(name = "programmable")]
    ProgrammableTransaction(ProgrammableTransactionCommand),
    #[clap(name = "upgrade")]
    UpgradePackage(UpgradePackageCommand),
    #[clap(name = "stage-package")]
    StagePackage(StagePackageCommand),
    #[clap(name = "set-address")]
    SetAddress(SetAddressCommand),
    #[clap(name = "create-checkpoint")]
    CreateCheckpoint(CreateCheckpointCommand),
    #[clap(name = "advance-epoch")]
    AdvanceEpoch(AdvanceEpochCommand),
    #[clap(name = "advance-clock")]
    AdvanceClock(AdvanceClockCommand),
    #[clap(name = "set-random-state")]
    SetRandomState(SetRandomStateCommand),
    #[clap(name = "view-checkpoint")]
    ViewCheckpoint,
    #[clap(name = "run-graphql")]
    RunGraphql(RunGraphqlCommand),
    #[clap(name = "force-object-snapshot-catchup")]
    ForceObjectSnapshotCatchup(ForceObjectSnapshotCatchup),
}

#[derive(Clone, Debug)]
pub enum MgoExtraValueArgs {
    Object(FakeID, Option<SequenceNumber>),
    Digest(String),
    Receiving(FakeID, Option<SequenceNumber>),
    ImmShared(FakeID, Option<SequenceNumber>),
}

pub enum MgoValue {
    MoveValue(MoveValue),
    Object(FakeID, Option<SequenceNumber>),
    ObjVec(Vec<(FakeID, Option<SequenceNumber>)>),
    Digest(String),
    Receiving(FakeID, Option<SequenceNumber>),
    ImmShared(FakeID, Option<SequenceNumber>),
}

impl MgoExtraValueArgs {
    fn parse_object_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "object")?;
        Ok(MgoExtraValueArgs::Object(fake_id, version))
    }

    fn parse_receiving_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "receiving")?;
        Ok(MgoExtraValueArgs::Receiving(fake_id, version))
    }

    fn parse_read_shared_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "immshared")?;
        Ok(MgoExtraValueArgs::ImmShared(fake_id, version))
    }

    fn parse_digest_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let contents = parser.advance(ValueToken::Ident)?;
        ensure!(contents == "digest");
        parser.advance(ValueToken::LParen)?;
        let package = parser.advance(ValueToken::Ident)?;
        parser.advance(ValueToken::RParen)?;
        Ok(MgoExtraValueArgs::Digest(package.to_owned()))
    }

    fn parse_receiving_or_object_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
        ident_name: &str,
    ) -> anyhow::Result<(FakeID, Option<SequenceNumber>)> {
        let contents = parser.advance(ValueToken::Ident)?;
        ensure!(contents == ident_name);
        parser.advance(ValueToken::LParen)?;
        let i_str = parser.advance(ValueToken::Number)?;
        let (i, _) = parse_u256(i_str)?;
        let fake_id = if let Some(ValueToken::Comma) = parser.peek_tok() {
            parser.advance(ValueToken::Comma)?;
            let j_str = parser.advance(ValueToken::Number)?;
            let (j, _) = parse_u64(j_str)?;
            if i > U256::from(u64::MAX) {
                bail!("Object ID too large")
            }
            FakeID::Enumerated(i.unchecked_as_u64(), j)
        } else {
            let mut u256_bytes = i.to_le_bytes().to_vec();
            u256_bytes.reverse();
            let address: MgoAddress = MgoAddress::from_bytes(&u256_bytes).unwrap();
            FakeID::Known(address.into())
        };
        parser.advance(ValueToken::RParen)?;
        let version = if let Some(ValueToken::AtSign) = parser.peek_tok() {
            parser.advance(ValueToken::AtSign)?;
            let v_str = parser.advance(ValueToken::Number)?;
            let (v, _) = parse_u64(v_str)?;
            Some(SequenceNumber::from_u64(v))
        } else {
            None
        };
        Ok((fake_id, version))
    }
}

impl MgoValue {
    fn assert_move_value(self) -> MoveValue {
        match self {
            MgoValue::MoveValue(v) => v,
            MgoValue::Object(_, _) => panic!("unexpected nested Mgo object in args"),
            MgoValue::ObjVec(_) => panic!("unexpected nested Mgo object vector in args"),
            MgoValue::Digest(_) => panic!("unexpected nested Mgo package digest in args"),
            MgoValue::Receiving(_, _) => panic!("unexpected nested Mgo receiving object in args"),
            MgoValue::ImmShared(_, _) => panic!("unexpected nested Mgo shared object in args"),
        }
    }

    fn assert_object(self) -> (FakeID, Option<SequenceNumber>) {
        match self {
            MgoValue::MoveValue(_) => panic!("unexpected nested non-object value in args"),
            MgoValue::Object(id, version) => (id, version),
            MgoValue::ObjVec(_) => panic!("unexpected nested Mgo object vector in args"),
            MgoValue::Digest(_) => panic!("unexpected nested Mgo package digest in args"),
            MgoValue::Receiving(_, _) => panic!("unexpected nested Mgo receiving object in args"),
            MgoValue::ImmShared(_, _) => panic!("unexpected nested Mgo shared object in args"),
        }
    }

    fn resolve_object(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &MgoTestAdapter,
    ) -> anyhow::Result<Object> {
        let id = match test_adapter.fake_to_real_object_id(fake_id) {
            Some(id) => id,
            None => bail!("INVALID TEST. Unknown object, object({})", fake_id),
        };
        let obj_res = if let Some(v) = version {
            mgo_types::storage::ObjectStore::get_object_by_key(&*test_adapter.executor, &id, v)
        } else {
            mgo_types::storage::ObjectStore::get_object(&*test_adapter.executor, &id)
        };
        let obj = match obj_res {
            Ok(Some(obj)) => obj,
            Err(_) | Ok(None) => bail!("INVALID TEST. Could not load object argument {}", id),
        };
        Ok(obj)
    }

    fn receiving_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &MgoTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        Ok(ObjectArg::Receiving(obj.compute_object_reference()))
    }

    fn read_shared_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &MgoTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        let id = obj.id();
        if let Owner::Shared {
            initial_shared_version,
        } = obj.owner
        {
            Ok(ObjectArg::SharedObject {
                id,
                initial_shared_version,
                mutable: false,
            })
        } else {
            bail!("{fake_id} is not a shared object.")
        }
    }

    fn object_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &MgoTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        let id = obj.id();
        match obj.owner {
            Owner::Shared {
                initial_shared_version,
            } => Ok(ObjectArg::SharedObject {
                id,
                initial_shared_version,
                mutable: true,
            }),
            Owner::AddressOwner(_) | Owner::ObjectOwner(_) | Owner::Immutable => {
                let obj_ref = obj.compute_object_reference();
                Ok(ObjectArg::ImmOrOwnedObject(obj_ref))
            }
        }
    }

    pub(crate) fn into_call_arg(self, test_adapter: &MgoTestAdapter) -> anyhow::Result<CallArg> {
        Ok(match self {
            MgoValue::Object(fake_id, version) => {
                CallArg::Object(Self::object_arg(fake_id, version, test_adapter)?)
            }
            MgoValue::MoveValue(v) => CallArg::Pure(v.simple_serialize().unwrap()),
            MgoValue::Receiving(fake_id, version) => {
                CallArg::Object(Self::receiving_arg(fake_id, version, test_adapter)?)
            }
            MgoValue::ImmShared(fake_id, version) => {
                CallArg::Object(Self::read_shared_arg(fake_id, version, test_adapter)?)
            }
            MgoValue::ObjVec(_) => bail!("obj vec is not supported as an input"),
            MgoValue::Digest(pkg) => {
                let pkg = Symbol::from(pkg);
                let Some(staged) = test_adapter.staged_modules.get(&pkg) else {
                    bail!("Unbound staged package '{pkg}'")
                };
                CallArg::Pure(bcs::to_bytes(&staged.digest).unwrap())
            }
        })
    }

    pub(crate) fn into_argument(
        self,
        builder: &mut ProgrammableTransactionBuilder,
        test_adapter: &MgoTestAdapter,
    ) -> anyhow::Result<Argument> {
        match self {
            MgoValue::ObjVec(vec) => builder.make_obj_vec(
                vec.iter()
                    .map(|(fake_id, version)| Self::object_arg(*fake_id, *version, test_adapter))
                    .collect::<Result<Vec<ObjectArg>, _>>()?,
            ),
            value => {
                let call_arg = value.into_call_arg(test_adapter)?;
                builder.input(call_arg)
            }
        }
    }
}

impl ParsableValue for MgoExtraValueArgs {
    type ConcreteValue = MgoValue;

    fn parse_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> Option<anyhow::Result<Self>> {
        match parser.peek()? {
            (ValueToken::Ident, "object") => Some(Self::parse_object_value(parser)),
            (ValueToken::Ident, "digest") => Some(Self::parse_digest_value(parser)),
            (ValueToken::Ident, "receiving") => Some(Self::parse_receiving_value(parser)),
            (ValueToken::Ident, "immshared") => Some(Self::parse_read_shared_value(parser)),
            _ => None,
        }
    }

    fn move_value_into_concrete(v: MoveValue) -> anyhow::Result<Self::ConcreteValue> {
        Ok(MgoValue::MoveValue(v))
    }

    fn concrete_vector(elems: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        if !elems.is_empty() && matches!(elems[0], MgoValue::Object(_, _)) {
            Ok(MgoValue::ObjVec(
                elems.into_iter().map(MgoValue::assert_object).collect(),
            ))
        } else {
            Ok(MgoValue::MoveValue(MoveValue::Vector(
                elems.into_iter().map(MgoValue::assert_move_value).collect(),
            )))
        }
    }

    fn concrete_struct(values: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        Ok(MgoValue::MoveValue(MoveValue::Struct(MoveStruct(
            values.into_iter().map(|v| v.assert_move_value()).collect(),
        ))))
    }

    fn into_concrete_value(
        self,
        _mapping: &impl Fn(&str) -> Option<move_core_types::account_address::AccountAddress>,
    ) -> anyhow::Result<Self::ConcreteValue> {
        match self {
            MgoExtraValueArgs::Object(id, version) => Ok(MgoValue::Object(id, version)),
            MgoExtraValueArgs::Digest(pkg) => Ok(MgoValue::Digest(pkg)),
            MgoExtraValueArgs::Receiving(id, version) => Ok(MgoValue::Receiving(id, version)),
            MgoExtraValueArgs::ImmShared(id, version) => Ok(MgoValue::ImmShared(id, version)),
        }
    }
}

fn parse_fake_id(s: &str) -> anyhow::Result<FakeID> {
    Ok(if let Some((s1, s2)) = s.split_once(',') {
        let (i, _) = parse_u64(s1)?;
        let (j, _) = parse_u64(s2)?;
        FakeID::Enumerated(i, j)
    } else {
        let (i, _) = parse_u256(s)?;
        let mut u256_bytes = i.to_le_bytes().to_vec();
        u256_bytes.reverse();
        let address: MgoAddress = MgoAddress::from_bytes(&u256_bytes).unwrap();
        FakeID::Known(address.into())
    })
}

fn parse_policy(x: &str) -> anyhow::Result<u8> {
    Ok(match x {
            "compatible" => UpgradePolicy::COMPATIBLE,
            "additive" => UpgradePolicy::ADDITIVE,
            "dep_only" => UpgradePolicy::DEP_ONLY,
        _ => bail!("Invalid upgrade policy {x}. Policy must be one of 'compatible', 'additive', or 'dep_only'")
    })
}
