// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use colored::Colorize;
use itertools::Itertools;
use move_binary_format::file_format::{Ability, AbilitySet, DatatypeTyParameter, Visibility};
use move_binary_format::normalized::{
    self, Enum as NormalizedEnum, Field as NormalizedField, Function as NormalizedFunction,
    Module as NormalizedModule, Struct as NormalizedStruct, Type as NormalizedType,
};
use move_command_line_common::error_bitset::ErrorBitset;
use move_core_types::annotated_value::{MoveStruct, MoveValue, MoveVariant};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::hash::Hash;
use myso_macros::EnumVariantOrder;
use tracing::warn;

use myso_types::base_types::{ObjectID, MySoAddress};
use myso_types::execution_status::MoveLocation;
use myso_types::myso_serde::MySoStructTag;

pub type MySoMoveTypeParameterIndex = u16;

#[cfg(test)]
#[path = "unit_tests/myso_move_tests.rs"]
mod myso_move_tests;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum MySoMoveAbility {
    Copy,
    Drop,
    Store,
    Key,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct MySoMoveAbilitySet {
    pub abilities: Vec<MySoMoveAbility>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum MySoMoveVisibility {
    Private,
    Public,
    Friend,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MySoMoveStructTypeParameter {
    pub constraints: MySoMoveAbilitySet,
    pub is_phantom: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct MySoMoveNormalizedField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: MySoMoveNormalizedType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MySoMoveNormalizedStruct {
    pub abilities: MySoMoveAbilitySet,
    pub type_parameters: Vec<MySoMoveStructTypeParameter>,
    pub fields: Vec<MySoMoveNormalizedField>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MySoMoveNormalizedEnum {
    pub abilities: MySoMoveAbilitySet,
    pub type_parameters: Vec<MySoMoveStructTypeParameter>,
    pub variants: BTreeMap<String, Vec<MySoMoveNormalizedField>>,
    #[serde(default)]
    pub variant_declaration_order: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum MySoMoveNormalizedType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Address,
    Signer,
    Struct {
        #[serde(flatten)]
        inner: Box<MySoMoveNormalizedStructType>,
    },
    Vector(Box<MySoMoveNormalizedType>),
    TypeParameter(MySoMoveTypeParameterIndex),
    Reference(Box<MySoMoveNormalizedType>),
    MutableReference(Box<MySoMoveNormalizedType>),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MySoMoveNormalizedStructType {
    pub address: String,
    pub module: String,
    pub name: String,
    pub type_arguments: Vec<MySoMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MySoMoveNormalizedFunction {
    pub visibility: MySoMoveVisibility,
    pub is_entry: bool,
    pub type_parameters: Vec<MySoMoveAbilitySet>,
    pub parameters: Vec<MySoMoveNormalizedType>,
    pub return_: Vec<MySoMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct MySoMoveModuleId {
    address: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MySoMoveNormalizedModule {
    pub file_format_version: u32,
    pub address: String,
    pub name: String,
    pub friends: Vec<MySoMoveModuleId>,
    pub structs: BTreeMap<String, MySoMoveNormalizedStruct>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub enums: BTreeMap<String, MySoMoveNormalizedEnum>,
    pub exposed_functions: BTreeMap<String, MySoMoveNormalizedFunction>,
}

impl PartialEq for MySoMoveNormalizedModule {
    fn eq(&self, other: &Self) -> bool {
        self.file_format_version == other.file_format_version
            && self.address == other.address
            && self.name == other.name
    }
}

impl<S: std::hash::Hash + Eq + ToString> From<&NormalizedModule<S>> for MySoMoveNormalizedModule {
    fn from(module: &NormalizedModule<S>) -> Self {
        Self {
            file_format_version: module.file_format_version,
            address: module.address().to_hex_literal(),
            name: module.name().to_string(),
            friends: module
                .friends
                .iter()
                .map(|module_id| MySoMoveModuleId {
                    address: module_id.address.to_hex_literal(),
                    name: module_id.name.to_string(),
                })
                .collect::<Vec<MySoMoveModuleId>>(),
            structs: module
                .structs
                .iter()
                .map(|(name, struct_)| {
                    (name.to_string(), MySoMoveNormalizedStruct::from(&**struct_))
                })
                .collect::<BTreeMap<String, MySoMoveNormalizedStruct>>(),
            enums: module
                .enums
                .iter()
                .map(|(name, enum_)| (name.to_string(), MySoMoveNormalizedEnum::from(&**enum_)))
                .collect(),
            exposed_functions: module
                .functions
                .iter()
                .filter(|(_name, function)| {
                    function.is_entry || function.visibility != Visibility::Private
                })
                .map(|(name, function)| {
                    // TODO: Do we want to expose the private functions as well?

                    (
                        name.to_string(),
                        MySoMoveNormalizedFunction::from(&**function),
                    )
                })
                .collect::<BTreeMap<String, MySoMoveNormalizedFunction>>(),
        }
    }
}

impl<S: Hash + Eq + ToString> From<&NormalizedFunction<S>> for MySoMoveNormalizedFunction {
    fn from(function: &NormalizedFunction<S>) -> Self {
        Self {
            visibility: match function.visibility {
                Visibility::Private => MySoMoveVisibility::Private,
                Visibility::Public => MySoMoveVisibility::Public,
                Visibility::Friend => MySoMoveVisibility::Friend,
            },
            is_entry: function.is_entry,
            type_parameters: function
                .type_parameters
                .iter()
                .copied()
                .map(|a| a.into())
                .collect::<Vec<MySoMoveAbilitySet>>(),
            parameters: function
                .parameters
                .iter()
                .map(|t| MySoMoveNormalizedType::from(&**t))
                .collect::<Vec<MySoMoveNormalizedType>>(),
            return_: function
                .return_
                .iter()
                .map(|t| MySoMoveNormalizedType::from(&**t))
                .collect::<Vec<MySoMoveNormalizedType>>(),
        }
    }
}

impl<S: Hash + Eq + ToString> From<&NormalizedStruct<S>> for MySoMoveNormalizedStruct {
    fn from(struct_: &NormalizedStruct<S>) -> Self {
        Self {
            abilities: struct_.abilities.into(),
            type_parameters: struct_
                .type_parameters
                .iter()
                .copied()
                .map(MySoMoveStructTypeParameter::from)
                .collect::<Vec<MySoMoveStructTypeParameter>>(),
            fields: struct_
                .fields
                .0
                .values()
                .map(|f| MySoMoveNormalizedField::from(&**f))
                .collect::<Vec<MySoMoveNormalizedField>>(),
        }
    }
}

impl<S: Hash + Eq + ToString> From<&NormalizedEnum<S>> for MySoMoveNormalizedEnum {
    fn from(value: &NormalizedEnum<S>) -> Self {
        let variants = value
            .variants
            .values()
            .map(|variant| {
                (
                    variant.name.to_string(),
                    variant
                        .fields
                        .0
                        .values()
                        .map(|f| MySoMoveNormalizedField::from(&**f))
                        .collect::<Vec<MySoMoveNormalizedField>>(),
                )
            })
            .collect::<Vec<(String, Vec<MySoMoveNormalizedField>)>>();
        let variant_declaration_order = variants
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<String>>();
        let variants = variants.into_iter().collect();
        Self {
            abilities: value.abilities.into(),
            type_parameters: value
                .type_parameters
                .iter()
                .copied()
                .map(MySoMoveStructTypeParameter::from)
                .collect::<Vec<MySoMoveStructTypeParameter>>(),
            variants,
            variant_declaration_order: Some(variant_declaration_order),
        }
    }
}

impl From<DatatypeTyParameter> for MySoMoveStructTypeParameter {
    fn from(type_parameter: DatatypeTyParameter) -> Self {
        Self {
            constraints: type_parameter.constraints.into(),
            is_phantom: type_parameter.is_phantom,
        }
    }
}

impl<S: ToString> From<&NormalizedField<S>> for MySoMoveNormalizedField {
    fn from(normalized_field: &NormalizedField<S>) -> Self {
        Self {
            name: normalized_field.name.to_string(),
            type_: MySoMoveNormalizedType::from(&normalized_field.type_),
        }
    }
}

impl<S: ToString> From<&NormalizedType<S>> for MySoMoveNormalizedType {
    fn from(type_: &NormalizedType<S>) -> Self {
        match type_ {
            NormalizedType::Bool => MySoMoveNormalizedType::Bool,
            NormalizedType::U8 => MySoMoveNormalizedType::U8,
            NormalizedType::U16 => MySoMoveNormalizedType::U16,
            NormalizedType::U32 => MySoMoveNormalizedType::U32,
            NormalizedType::U64 => MySoMoveNormalizedType::U64,
            NormalizedType::U128 => MySoMoveNormalizedType::U128,
            NormalizedType::U256 => MySoMoveNormalizedType::U256,
            NormalizedType::Address => MySoMoveNormalizedType::Address,
            NormalizedType::Signer => MySoMoveNormalizedType::Signer,
            NormalizedType::Datatype(dt) => {
                let normalized::Datatype {
                    module,
                    name,
                    type_arguments,
                } = &**dt;
                MySoMoveNormalizedType::new_struct(
                    module.address.to_hex_literal(),
                    module.name.to_string(),
                    name.to_string(),
                    type_arguments
                        .iter()
                        .map(MySoMoveNormalizedType::from)
                        .collect::<Vec<MySoMoveNormalizedType>>(),
                )
            }
            NormalizedType::Vector(v) => {
                MySoMoveNormalizedType::Vector(Box::new(MySoMoveNormalizedType::from(&**v)))
            }
            NormalizedType::TypeParameter(t) => MySoMoveNormalizedType::TypeParameter(*t),
            NormalizedType::Reference(false, r) => {
                MySoMoveNormalizedType::Reference(Box::new(MySoMoveNormalizedType::from(&**r)))
            }
            NormalizedType::Reference(true, mr) => MySoMoveNormalizedType::MutableReference(
                Box::new(MySoMoveNormalizedType::from(&**mr)),
            ),
        }
    }
}

impl From<AbilitySet> for MySoMoveAbilitySet {
    fn from(set: AbilitySet) -> MySoMoveAbilitySet {
        Self {
            abilities: set
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => MySoMoveAbility::Copy,
                    Ability::Drop => MySoMoveAbility::Drop,
                    Ability::Key => MySoMoveAbility::Key,
                    Ability::Store => MySoMoveAbility::Store,
                })
                .collect::<Vec<MySoMoveAbility>>(),
        }
    }
}

impl MySoMoveNormalizedType {
    pub fn new_struct(
        address: String,
        module: String,
        name: String,
        type_arguments: Vec<MySoMoveNormalizedType>,
    ) -> Self {
        MySoMoveNormalizedType::Struct {
            inner: Box::new(MySoMoveNormalizedStructType {
                address,
                module,
                name,
                type_arguments,
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum ObjectValueKind {
    ByImmutableReference,
    ByMutableReference,
    ByValue,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum MoveFunctionArgType {
    Pure,
    Object(ObjectValueKind),
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveValue")]
pub enum MySoMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(MySoAddress),
    Vector(Vec<MySoMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(MySoMoveStruct),
    Option(Box<Option<MySoMoveValue>>),
    Variant(MySoMoveVariant),
}

impl MySoMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            MySoMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            MySoMoveValue::Vector(values) => MySoMoveStruct::Runtime(values).to_json_value(),
            MySoMoveValue::Number(v) => json!(v),
            MySoMoveValue::Bool(v) => json!(v),
            MySoMoveValue::Address(v) => json!(v),
            MySoMoveValue::String(v) => json!(v),
            MySoMoveValue::UID { id } => json!({ "id": id }),
            MySoMoveValue::Option(v) => json!(v),
            MySoMoveValue::Variant(v) => v.to_json_value(),
        }
    }
}

impl Display for MySoMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            MySoMoveValue::Number(value) => write!(writer, "{}", value)?,
            MySoMoveValue::Bool(value) => write!(writer, "{}", value)?,
            MySoMoveValue::Address(value) => write!(writer, "{}", value)?,
            MySoMoveValue::String(value) => write!(writer, "{}", value)?,
            MySoMoveValue::UID { id } => write!(writer, "{id}")?,
            MySoMoveValue::Struct(value) => write!(writer, "{}", value)?,
            MySoMoveValue::Option(value) => write!(writer, "{:?}", value)?,
            MySoMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
            MySoMoveValue::Variant(value) => write!(writer, "{}", value)?,
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl From<MoveValue> for MySoMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => MySoMoveValue::Number(value.into()),
            MoveValue::U16(value) => MySoMoveValue::Number(value.into()),
            MoveValue::U32(value) => MySoMoveValue::Number(value),
            MoveValue::U64(value) => MySoMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => MySoMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => MySoMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => MySoMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                MySoMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort MySo core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                MySoMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                MySoMoveValue::Address(MySoAddress::from(ObjectID::from(value)))
            }
            MoveValue::Variant(MoveVariant {
                type_,
                variant_name,
                tag: _,
                fields,
            }) => MySoMoveValue::Variant(MySoMoveVariant {
                type_: type_.clone(),
                variant: variant_name.to_string(),
                fields: fields
                    .into_iter()
                    .map(|(id, value)| (id.into_string(), value.into()))
                    .collect::<BTreeMap<_, _>>(),
            }),
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
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "MoveVariant")]
pub struct MySoMoveVariant {
    #[schemars(with = "String")]
    #[serde(rename = "type")]
    #[serde_as(as = "MySoStructTag")]
    pub type_: StructTag,
    pub variant: String,
    pub fields: BTreeMap<String, MySoMoveValue>,
}

impl MySoMoveVariant {
    pub fn to_json_value(self) -> Value {
        // We only care about values here, assuming type information is known at the client side.
        let fields = self
            .fields
            .into_iter()
            .map(|(key, value)| (key, value.to_json_value()))
            .collect::<BTreeMap<_, _>>();
        json!({
            "variant": self.variant,
            "fields": fields,
        })
    }
}

impl Display for MySoMoveVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        let MySoMoveVariant {
            type_,
            variant,
            fields,
        } = self;
        writeln!(writer)?;
        writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
        writeln!(writer, "  {}: {variant}", "variant".bold().bright_black())?;
        for (name, value) in fields {
            let value = format!("{}", value);
            let value = if value.starts_with('\n') {
                indent(&value, 2)
            } else {
                value
            };
            writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
        }

        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveStruct")]
pub enum MySoMoveStruct {
    Runtime(Vec<MySoMoveValue>),
    WithTypes {
        #[schemars(with = "String")]
        #[serde(rename = "type")]
        #[serde_as(as = "MySoStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, MySoMoveValue>,
    },
    WithFields(BTreeMap<String, MySoMoveValue>),
}

impl MySoMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            MySoMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the client side.
            MySoMoveStruct::WithTypes { type_: _, fields } | MySoMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }

    pub fn field_value(&self, field_name: &str) -> Option<MySoMoveValue> {
        match self {
            MySoMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            MySoMoveStruct::WithTypes { type_: _, fields } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

impl Display for MySoMoveStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            MySoMoveStruct::Runtime(_) => {}
            MySoMoveStruct::WithFields(fields) => {
                for (name, value) in fields {
                    writeln!(writer, "{}: {value}", name.bold().bright_black())?;
                }
            }
            MySoMoveStruct::WithTypes { type_, fields } => {
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

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct MySoMoveAbort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<u64>,
}

impl MySoMoveAbort {
    pub fn new(move_location: MoveLocation, code: u64) -> Self {
        let module = move_location.module.to_canonical_string(true);
        let (error_code, line) = match ErrorBitset::from_u64(code) {
            Some(c) => (c.error_code().map(|c| c as u64), c.line_number()),
            None => (Some(code), None),
        };
        Self {
            module_id: Some(module),
            function: move_location.function_name.clone(),
            line,
            error_code,
        }
    }
}

impl Display for MySoMoveAbort {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        if let Some(module_id) = &self.module_id {
            writeln!(writer, "Module ID: {module_id}")?;
        }
        if let Some(function) = &self.function {
            writeln!(writer, "Function: {function}")?;
        }
        if let Some(line) = &self.line {
            writeln!(writer, "Line: {line}")?;
        }
        if let Some(error_code) = &self.error_code {
            writeln!(writer, "Error code: {error_code}")?;
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

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<MySoMoveValue> {
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
                    .map(MySoMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(MySoMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(MySoMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(MySoMoveValue::from);
            if let Some(MySoMoveValue::Address(address)) = id {
                return Some(MySoMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(MySoMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(MySoMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(MySoMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to MySoMoveValue"
    );
    None
}

impl From<MoveStruct> for MySoMoveStruct {
    fn from(move_struct: MoveStruct) -> Self {
        MySoMoveStruct::WithTypes {
            type_: move_struct.type_,
            fields: move_struct
                .fields
                .into_iter()
                .map(|(id, value)| (id.into_string(), value.into()))
                .collect(),
        }
    }
}

#[test]
fn enum_size() {
    assert_eq!(std::mem::size_of::<MySoMoveNormalizedType>(), 16);
}
