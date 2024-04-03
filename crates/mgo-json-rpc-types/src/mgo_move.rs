// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use colored::Colorize;
use itertools::Itertools;
use move_binary_format::file_format::{Ability, AbilitySet, StructTypeParameter, Visibility};
use move_binary_format::normalized::{
    Field as NormalizedField, Function as MgoNormalizedFunction, Module as NormalizedModule,
    Struct as NormalizedStruct, Type as NormalizedType,
};
use move_core_types::annotated_value::{MoveStruct, MoveValue};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use mgo_macros::EnumVariantOrder;
use tracing::warn;

use mgo_types::base_types::{ObjectID, MgoAddress};
use mgo_types::mgo_serde::MgoStructTag;

pub type MgoMoveTypeParameterIndex = u16;

#[cfg(test)]
#[path = "unit_tests/mgo_move_tests.rs"]
mod mgo_move_tests;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum MgoMoveAbility {
    Copy,
    Drop,
    Store,
    Key,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct MgoMoveAbilitySet {
    pub abilities: Vec<MgoMoveAbility>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum MgoMoveVisibility {
    Private,
    Public,
    Friend,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MgoMoveStructTypeParameter {
    pub constraints: MgoMoveAbilitySet,
    pub is_phantom: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct MgoMoveNormalizedField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: MgoMoveNormalizedType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MgoMoveNormalizedStruct {
    pub abilities: MgoMoveAbilitySet,
    pub type_parameters: Vec<MgoMoveStructTypeParameter>,
    pub fields: Vec<MgoMoveNormalizedField>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum MgoMoveNormalizedType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Address,
    Signer,
    #[serde(rename_all = "camelCase")]
    Struct {
        address: String,
        module: String,
        name: String,
        type_arguments: Vec<MgoMoveNormalizedType>,
    },
    Vector(Box<MgoMoveNormalizedType>),
    TypeParameter(MgoMoveTypeParameterIndex),
    Reference(Box<MgoMoveNormalizedType>),
    MutableReference(Box<MgoMoveNormalizedType>),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MgoMoveNormalizedFunction {
    pub visibility: MgoMoveVisibility,
    pub is_entry: bool,
    pub type_parameters: Vec<MgoMoveAbilitySet>,
    pub parameters: Vec<MgoMoveNormalizedType>,
    pub return_: Vec<MgoMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct MgoMoveModuleId {
    address: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MgoMoveNormalizedModule {
    pub file_format_version: u32,
    pub address: String,
    pub name: String,
    pub friends: Vec<MgoMoveModuleId>,
    pub structs: BTreeMap<String, MgoMoveNormalizedStruct>,
    pub exposed_functions: BTreeMap<String, MgoMoveNormalizedFunction>,
}

impl PartialEq for MgoMoveNormalizedModule {
    fn eq(&self, other: &Self) -> bool {
        self.file_format_version == other.file_format_version
            && self.address == other.address
            && self.name == other.name
    }
}

impl From<NormalizedModule> for MgoMoveNormalizedModule {
    fn from(module: NormalizedModule) -> Self {
        Self {
            file_format_version: module.file_format_version,
            address: module.address.to_hex_literal(),
            name: module.name.to_string(),
            friends: module
                .friends
                .into_iter()
                .map(|module_id| MgoMoveModuleId {
                    address: module_id.address().to_hex_literal(),
                    name: module_id.name().to_string(),
                })
                .collect::<Vec<MgoMoveModuleId>>(),
            structs: module
                .structs
                .into_iter()
                .map(|(name, struct_)| (name.to_string(), MgoMoveNormalizedStruct::from(struct_)))
                .collect::<BTreeMap<String, MgoMoveNormalizedStruct>>(),
            exposed_functions: module
                .functions
                .into_iter()
                .filter_map(|(name, function)| {
                    // TODO: Do we want to expose the private functions as well?
                    (function.is_entry || function.visibility != Visibility::Private)
                        .then(|| (name.to_string(), MgoMoveNormalizedFunction::from(function)))
                })
                .collect::<BTreeMap<String, MgoMoveNormalizedFunction>>(),
        }
    }
}

impl From<MgoNormalizedFunction> for MgoMoveNormalizedFunction {
    fn from(function: MgoNormalizedFunction) -> Self {
        Self {
            visibility: match function.visibility {
                Visibility::Private => MgoMoveVisibility::Private,
                Visibility::Public => MgoMoveVisibility::Public,
                Visibility::Friend => MgoMoveVisibility::Friend,
            },
            is_entry: function.is_entry,
            type_parameters: function
                .type_parameters
                .into_iter()
                .map(|a| a.into())
                .collect::<Vec<MgoMoveAbilitySet>>(),
            parameters: function
                .parameters
                .into_iter()
                .map(MgoMoveNormalizedType::from)
                .collect::<Vec<MgoMoveNormalizedType>>(),
            return_: function
                .return_
                .into_iter()
                .map(MgoMoveNormalizedType::from)
                .collect::<Vec<MgoMoveNormalizedType>>(),
        }
    }
}

impl From<NormalizedStruct> for MgoMoveNormalizedStruct {
    fn from(struct_: NormalizedStruct) -> Self {
        Self {
            abilities: struct_.abilities.into(),
            type_parameters: struct_
                .type_parameters
                .into_iter()
                .map(MgoMoveStructTypeParameter::from)
                .collect::<Vec<MgoMoveStructTypeParameter>>(),
            fields: struct_
                .fields
                .into_iter()
                .map(MgoMoveNormalizedField::from)
                .collect::<Vec<MgoMoveNormalizedField>>(),
        }
    }
}

impl From<StructTypeParameter> for MgoMoveStructTypeParameter {
    fn from(type_parameter: StructTypeParameter) -> Self {
        Self {
            constraints: type_parameter.constraints.into(),
            is_phantom: type_parameter.is_phantom,
        }
    }
}

impl From<NormalizedField> for MgoMoveNormalizedField {
    fn from(normalized_field: NormalizedField) -> Self {
        Self {
            name: normalized_field.name.to_string(),
            type_: MgoMoveNormalizedType::from(normalized_field.type_),
        }
    }
}

impl From<NormalizedType> for MgoMoveNormalizedType {
    fn from(type_: NormalizedType) -> Self {
        match type_ {
            NormalizedType::Bool => MgoMoveNormalizedType::Bool,
            NormalizedType::U8 => MgoMoveNormalizedType::U8,
            NormalizedType::U16 => MgoMoveNormalizedType::U16,
            NormalizedType::U32 => MgoMoveNormalizedType::U32,
            NormalizedType::U64 => MgoMoveNormalizedType::U64,
            NormalizedType::U128 => MgoMoveNormalizedType::U128,
            NormalizedType::U256 => MgoMoveNormalizedType::U256,
            NormalizedType::Address => MgoMoveNormalizedType::Address,
            NormalizedType::Signer => MgoMoveNormalizedType::Signer,
            NormalizedType::Struct {
                address,
                module,
                name,
                type_arguments,
            } => MgoMoveNormalizedType::Struct {
                address: address.to_hex_literal(),
                module: module.to_string(),
                name: name.to_string(),
                type_arguments: type_arguments
                    .into_iter()
                    .map(MgoMoveNormalizedType::from)
                    .collect::<Vec<MgoMoveNormalizedType>>(),
            },
            NormalizedType::Vector(v) => {
                MgoMoveNormalizedType::Vector(Box::new(MgoMoveNormalizedType::from(*v)))
            }
            NormalizedType::TypeParameter(t) => MgoMoveNormalizedType::TypeParameter(t),
            NormalizedType::Reference(r) => {
                MgoMoveNormalizedType::Reference(Box::new(MgoMoveNormalizedType::from(*r)))
            }
            NormalizedType::MutableReference(mr) => {
                MgoMoveNormalizedType::MutableReference(Box::new(MgoMoveNormalizedType::from(*mr)))
            }
        }
    }
}

impl From<AbilitySet> for MgoMoveAbilitySet {
    fn from(set: AbilitySet) -> MgoMoveAbilitySet {
        Self {
            abilities: set
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => MgoMoveAbility::Copy,
                    Ability::Drop => MgoMoveAbility::Drop,
                    Ability::Key => MgoMoveAbility::Key,
                    Ability::Store => MgoMoveAbility::Store,
                })
                .collect::<Vec<MgoMoveAbility>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ObjectValueKind {
    ByImmutableReference,
    ByMutableReference,
    ByValue,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum MoveFunctionArgType {
    Pure,
    Object(ObjectValueKind),
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveValue")]
pub enum MgoMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(MgoAddress),
    Vector(Vec<MgoMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(MgoMoveStruct),
    Option(Box<Option<MgoMoveValue>>),
}

impl MgoMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            MgoMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            MgoMoveValue::Vector(values) => MgoMoveStruct::Runtime(values).to_json_value(),
            MgoMoveValue::Number(v) => json!(v),
            MgoMoveValue::Bool(v) => json!(v),
            MgoMoveValue::Address(v) => json!(v),
            MgoMoveValue::String(v) => json!(v),
            MgoMoveValue::UID { id } => json!({ "id": id }),
            MgoMoveValue::Option(v) => json!(v),
        }
    }
}

impl Display for MgoMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            MgoMoveValue::Number(value) => write!(writer, "{}", value)?,
            MgoMoveValue::Bool(value) => write!(writer, "{}", value)?,
            MgoMoveValue::Address(value) => write!(writer, "{}", value)?,
            MgoMoveValue::String(value) => write!(writer, "{}", value)?,
            MgoMoveValue::UID { id } => write!(writer, "{id}")?,
            MgoMoveValue::Struct(value) => write!(writer, "{}", value)?,
            MgoMoveValue::Option(value) => write!(writer, "{:?}", value)?,
            MgoMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl From<MoveValue> for MgoMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => MgoMoveValue::Number(value.into()),
            MoveValue::U16(value) => MgoMoveValue::Number(value.into()),
            MoveValue::U32(value) => MgoMoveValue::Number(value),
            MoveValue::U64(value) => MgoMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => MgoMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => MgoMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => MgoMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                MgoMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort Mgo core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                MgoMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                MgoMoveValue::Address(MgoAddress::from(ObjectID::from(value)))
            }
        }
    }
}

fn to_bytearray(value: &[MoveValue]) -> Option<Vec<u8>> {
    if value.iter().all(|value| matches!(value, MoveValue::U8(_))) {
        let bytearray = value
            .iter()
            .flat_map(|value| {
                if let MoveValue::U8(u8) = value {
                    Some(*u8)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Some(bytearray)
    } else {
        None
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveStruct")]
pub enum MgoMoveStruct {
    Runtime(Vec<MgoMoveValue>),
    WithTypes {
        #[schemars(with = "String")]
        #[serde(rename = "type")]
        #[serde_as(as = "MgoStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, MgoMoveValue>,
    },
    WithFields(BTreeMap<String, MgoMoveValue>),
}

impl MgoMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            MgoMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the client side.
            MgoMoveStruct::WithTypes { type_: _, fields } | MgoMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }

    pub fn read_dynamic_field_value(&self, field_name: &str) -> Option<MgoMoveValue> {
        match self {
            MgoMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            MgoMoveStruct::WithTypes { type_: _, fields } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

impl Display for MgoMoveStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            MgoMoveStruct::Runtime(_) => {}
            MgoMoveStruct::WithFields(fields) => {
                for (name, value) in fields {
                    writeln!(writer, "{}: {value}", name.bold().bright_black())?;
                }
            }
            MgoMoveStruct::WithTypes { type_, fields } => {
                writeln!(writer)?;
                writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
                for (name, value) in fields {
                    let value = format!("{}", value);
                    let value = if value.starts_with('\n') {
                        indent(&value, 2)
                    } else {
                        value
                    };
                    writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
                }
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

fn indent<T: Display>(d: &T, indent: usize) -> String {
    d.to_string()
        .lines()
        .map(|line| format!("{:indent$}{}", "", line))
        .join("\n")
}

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<MgoMoveValue> {
    let struct_name = format!(
        "0x{}::{}::{}",
        type_.address.short_str_lossless(),
        type_.module,
        type_.name
    );
    let mut values = fields
        .iter()
        .map(|(id, value)| (id.to_string(), value))
        .collect::<BTreeMap<_, _>>();
    match struct_name.as_str() {
        "0x1::string::String" | "0x1::ascii::String" => {
            if let Some(MoveValue::Vector(bytes)) = values.remove("bytes") {
                return to_bytearray(bytes)
                    .and_then(|bytes| String::from_utf8(bytes).ok())
                    .map(MgoMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(MgoMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(MgoMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(MgoMoveValue::from);
            if let Some(MgoMoveValue::Address(address)) = id {
                return Some(MgoMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(MgoMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(MgoMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(MgoMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to MgoMoveValue"
    );
    None
}

impl From<MoveStruct> for MgoMoveStruct {
    fn from(move_struct: MoveStruct) -> Self {
        MgoMoveStruct::WithTypes {
            type_: move_struct.type_,
            fields: move_struct
                .fields
                .into_iter()
                .map(|(id, value)| (id.into_string(), value.into()))
                .collect(),
        }
    }
}
