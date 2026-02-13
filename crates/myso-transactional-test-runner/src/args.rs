// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use crate::test_adapter::{FakeID, MySoTestAdapter};
use anyhow::{bail, ensure};
use clap;
use clap::{Args, Parser};
use move_compiler::editions::Flavor;
use move_core_types::parsing::{
    parser::Parser as MoveCLParser,
    parser::{Token, parse_u64, parse_u256},
    types::{ParsedType, TypeToken},
    values::ValueToken,
    values::{ParsableValue, ParsedValue},
};
use move_core_types::runtime_value::{MoveStruct, MoveValue};
use move_core_types::u256::U256;
use move_symbol_pool::Symbol;
use move_transactional_test_runner::tasks::{RunCommand, SyntaxChoice};
use myso_types::balance::Balance;
use myso_types::base_types::{MySoAddress, SequenceNumber};
use myso_types::move_package::UpgradePolicy;
use myso_types::object::{Object, Owner};
use myso_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_types::transaction::{
    Argument, CallArg, FundsWithdrawalArg, ObjectArg, SharedObjectMutability,
};

pub const MYSO_ARGS_LONG: &str = "myso-args";
const DEFAULT_CONSISTENT_RANGE: usize = 300;

#[derive(Clone, Debug, clap::Parser)]
pub struct MySoRunArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
    #[clap(long = "summarize")]
    pub summarize: bool,
}

#[derive(Debug, clap::Parser, Default)]
pub struct MySoPublishArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "upgradeable", action = clap::ArgAction::SetTrue)]
    pub upgradeable: bool,
    #[clap(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
    #[clap(long = "dry-run")]
    pub dry_run: bool,
}

#[derive(Debug, clap::Parser)]
pub struct MySoInitArgs {
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
    #[clap(long = "num-custom-validator-accounts")]
    pub num_custom_validator_accounts: Option<u64>,
    #[clap(long = "reference-gas-price")]
    pub reference_gas_price: Option<u64>,
    #[clap(long = "default-gas-price")]
    pub default_gas_price: Option<u64>,
    #[clap(long = "flavor")]
    pub flavor: Option<Flavor>,
    #[clap(long = "consistent-range", default_value_t = DEFAULT_CONSISTENT_RANGE)]
    pub consistent_range: usize,
    /// Dir for simulacrum to write checkpoint files to. To be passed to the offchain indexer and
    /// reader.
    #[clap(long)]
    pub data_ingestion_path: Option<PathBuf>,
    /// URL for the MySo REST API. To be passed to the offchain indexer and reader.
    #[clap(long)]
    pub rest_api_url: Option<String>,
    /// Enable accumulator features for testing (e.g., authenticated event streams)
    #[clap(long = "enable-accumulators")]
    pub enable_accumulators: bool,
    /// Enable authenticated event streams for testing
    #[clap(long = "enable-authenticated-event-streams")]
    pub enable_authenticated_event_streams: bool,
    /// Enable references in PTBs
    #[clap(long = "allow-references-in-ptbs")]
    pub allow_references_in_ptbs: bool,
    /// Enable non-exclusive write objects for testing
    #[clap(long = "enable-non-exclusive-write-objects")]
    pub enable_non_exclusive_writes: bool,
    /// Enable using address balance as gas payments feature for testing
    #[clap(long = "enable-address-balance-gas-payments")]
    pub enable_address_balance_gas_payments: bool,
}

#[derive(Debug, clap::Parser)]
pub struct ViewObjectCommand {
    #[clap(value_parser = parse_fake_id)]
    pub id: FakeID,
    #[clap(long = "hide-contents")]
    pub hide_contents: bool,
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
    #[clap(long = "gas-budget-from-address-balance")]
    pub gas_budget_from_address_balance: Option<u64>,
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
    #[clap(long = "sponsor")]
    pub sponsor: Option<String>,
    #[clap(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[clap(long = "gas-budget-from-address-balance")]
    pub gas_budget_from_address_balance: Option<u64>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
    #[clap(long = "gas-payment", value_parser = parse_fake_id)]
    pub gas_payment: Option<Vec<FakeID>>,
    #[clap(long = "dev-inspect")]
    pub dev_inspect: bool,
    #[clap(long = "dry-run")]
    pub dry_run: bool,
    #[clap(long = "expiration")]
    pub expiration: Option<u64>,
    #[clap(
        long = "inputs",
        value_parser = ParsedValue::<MySoExtraValueArgs>::parse,
        num_args(1..),
        action = clap::ArgAction::Append,
    )]
    pub inputs: Vec<ParsedValue<MySoExtraValueArgs>>,
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
    #[clap(long = "gas-budget-from-address-balance")]
    pub gas_budget_from_address_balance: Option<u64>,
    #[clap(long = "dry-run")]
    pub dry_run: bool,
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
    #[clap(value_parser = ParsedValue::<MySoExtraValueArgs>::parse)]
    pub input: ParsedValue<MySoExtraValueArgs>,
}

#[derive(Debug, clap::Parser)]
pub struct AdvanceClockCommand {
    #[clap(long = "duration")]
    pub duration: Option<String>,
    #[clap(long = "duration-ns")]
    pub duration_ns: Option<u64>,
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
    #[clap(long)]
    pub wait_for_checkpoint_pruned: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct RunJsonRpcCommand {
    #[clap(long = "show-headers")]
    pub show_headers: bool,
    #[clap(long, num_args(1..))]
    pub cursors: Vec<String>,
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
    #[clap(long = "create-authenticator-state")]
    pub create_authenticator_state: bool,
    #[clap(long = "create-authenticator-state-expire")]
    pub create_authenticator_state_expire: bool,
    #[clap(long = "create-deny-list-state")]
    pub create_deny_list_state: bool,
    #[clap(long = "create-bridge-state")]
    pub create_bridge_state: bool,
    #[clap(long = "create-bridge-committee")]
    pub create_bridge_committee: bool,
    #[clap(long = "system-packages-snapshot")]
    pub system_packages_snapshot: Option<u64>,
}

impl From<&AdvanceEpochCommand> for simulacrum::AdvanceEpochConfig {
    fn from(cmd: &AdvanceEpochCommand) -> Self {
        Self {
            create_random_state: cmd.create_random_state,
            create_authenticator_state: cmd.create_authenticator_state,
            create_authenticator_state_expire: cmd.create_authenticator_state_expire,
            create_deny_list_state: cmd.create_deny_list_state,
            create_bridge_state: cmd.create_bridge_state,
            create_bridge_committee: cmd.create_bridge_committee,
            system_packages_snapshot: cmd.system_packages_snapshot,
        }
    }
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
pub struct AuthenticatorStateUpdateCommand {
    #[clap(long = "round")]
    pub round: u64,
    /// List of JWK issuers (e.g., "google.com,microsoft.com").
    /// Key IDs will be automatically generated as "key1", "key2", etc.
    #[clap(long = "jwk-iss", action = clap::ArgAction::Append)]
    pub jwk_iss: Vec<String>,
    #[clap(long = "authenticator-obj-initial-shared-version")]
    pub authenticator_obj_initial_shared_version: Option<u64>,
}

#[derive(Debug)]
pub enum MySoSubcommand<ExtraValueArgs: ParsableValue, ExtraRunArgs: Parser> {
    ViewObject(ViewObjectCommand),
    TransferObject(TransferObjectCommand),
    ConsensusCommitPrologue(ConsensusCommitPrologueCommand),
    ProgrammableTransaction(ProgrammableTransactionCommand),
    UpgradePackage(UpgradePackageCommand),
    StagePackage(StagePackageCommand),
    SetAddress(SetAddressCommand),
    CreateCheckpoint(CreateCheckpointCommand),
    AdvanceEpoch(AdvanceEpochCommand),
    AdvanceClock(AdvanceClockCommand),
    SetRandomState(SetRandomStateCommand),
    AuthenticatorStateUpdate(AuthenticatorStateUpdateCommand),
    ViewCheckpoint,
    RunGraphql(RunGraphqlCommand),
    RunJsonRpc(RunJsonRpcCommand),
    Bench(RunCommand<ExtraValueArgs>, ExtraRunArgs),
}

impl<ExtraValueArgs: ParsableValue, ExtraRunArgs: Parser> clap::FromArgMatches
    for MySoSubcommand<ExtraValueArgs, ExtraRunArgs>
{
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(match matches.subcommand() {
            Some(("view-object", matches)) => {
                MySoSubcommand::ViewObject(ViewObjectCommand::from_arg_matches(matches)?)
            }
            Some(("transfer-object", matches)) => {
                MySoSubcommand::TransferObject(TransferObjectCommand::from_arg_matches(matches)?)
            }
            Some(("consensus-commit-prologue", matches)) => {
                MySoSubcommand::ConsensusCommitPrologue(
                    ConsensusCommitPrologueCommand::from_arg_matches(matches)?,
                )
            }
            Some(("programmable", matches)) => MySoSubcommand::ProgrammableTransaction(
                ProgrammableTransactionCommand::from_arg_matches(matches)?,
            ),
            Some(("upgrade", matches)) => {
                MySoSubcommand::UpgradePackage(UpgradePackageCommand::from_arg_matches(matches)?)
            }
            Some(("stage-package", matches)) => {
                MySoSubcommand::StagePackage(StagePackageCommand::from_arg_matches(matches)?)
            }
            Some(("set-address", matches)) => {
                MySoSubcommand::SetAddress(SetAddressCommand::from_arg_matches(matches)?)
            }
            Some(("create-checkpoint", matches)) => MySoSubcommand::CreateCheckpoint(
                CreateCheckpointCommand::from_arg_matches(matches)?,
            ),
            Some(("advance-epoch", matches)) => {
                MySoSubcommand::AdvanceEpoch(AdvanceEpochCommand::from_arg_matches(matches)?)
            }
            Some(("advance-clock", matches)) => {
                MySoSubcommand::AdvanceClock(AdvanceClockCommand::from_arg_matches(matches)?)
            }
            Some(("set-random-state", matches)) => {
                MySoSubcommand::SetRandomState(SetRandomStateCommand::from_arg_matches(matches)?)
            }
            Some(("authenticator-state-update", matches)) => {
                MySoSubcommand::AuthenticatorStateUpdate(
                    AuthenticatorStateUpdateCommand::from_arg_matches(matches)?,
                )
            }
            Some(("view-checkpoint", _)) => MySoSubcommand::ViewCheckpoint,
            Some(("run-graphql", matches)) => {
                MySoSubcommand::RunGraphql(RunGraphqlCommand::from_arg_matches(matches)?)
            }
            Some(("run-jsonrpc", matches)) => {
                MySoSubcommand::RunJsonRpc(RunJsonRpcCommand::from_arg_matches(matches)?)
            }
            Some(("bench", matches)) => MySoSubcommand::Bench(
                RunCommand::from_arg_matches(matches)?,
                ExtraRunArgs::from_arg_matches(matches)?,
            ),
            _ => {
                return Err(clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    "Invalid submcommand",
                ));
            }
        })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

impl<ExtraValueArgs: ParsableValue, ExtraRunArgs: Parser> clap::CommandFactory
    for MySoSubcommand<ExtraValueArgs, ExtraRunArgs>
{
    fn command() -> clap::Command {
        clap::Command::new("myso_sub_command")
            .subcommand(ViewObjectCommand::command().name("view-object"))
            .subcommand(TransferObjectCommand::command().name("transfer-object"))
            .subcommand(ConsensusCommitPrologueCommand::command().name("consensus-commit-prologue"))
            .subcommand(ProgrammableTransactionCommand::command().name("programmable"))
            .subcommand(UpgradePackageCommand::command().name("upgrade"))
            .subcommand(StagePackageCommand::command().name("stage-package"))
            .subcommand(SetAddressCommand::command().name("set-address"))
            .subcommand(CreateCheckpointCommand::command().name("create-checkpoint"))
            .subcommand(AdvanceEpochCommand::command().name("advance-epoch"))
            .subcommand(AdvanceClockCommand::command().name("advance-clock"))
            .subcommand(SetRandomStateCommand::command().name("set-random-state"))
            .subcommand(
                AuthenticatorStateUpdateCommand::command().name("authenticator-state-update"),
            )
            .subcommand(clap::Command::new("view-checkpoint"))
            .subcommand(RunGraphqlCommand::command().name("run-graphql"))
            .subcommand(RunJsonRpcCommand::command().name("run-jsonrpc"))
            .subcommand(
                RunCommand::<ExtraValueArgs>::augment_args(ExtraRunArgs::command()).name("bench"),
            )
    }

    fn command_for_update() -> clap::Command {
        todo!()
    }
}

impl<ExtraValueArgs: ParsableValue, ExtraRunArgs: Parser> clap::Parser
    for MySoSubcommand<ExtraValueArgs, ExtraRunArgs>
{
}

#[derive(Clone, Debug)]
pub enum MySoExtraValueArgs {
    Object(FakeID, Option<SequenceNumber>),
    Digest(String),
    Receiving(FakeID, Option<SequenceNumber>),
    Owned(FakeID, Option<SequenceNumber>),
    Shared(SharedObjectMutability, FakeID, Option<SequenceNumber>),
    Withdraw(u64, ParsedType),
}

#[derive(Clone)]
pub enum MySoValue {
    MoveValue(MoveValue),
    Object(FakeID, Option<SequenceNumber>),
    ObjVec(Vec<(FakeID, Option<SequenceNumber>)>),
    Digest(String),
    Receiving(FakeID, Option<SequenceNumber>),
    Owned(FakeID, Option<SequenceNumber>),
    Shared(SharedObjectMutability, FakeID, Option<SequenceNumber>),
    Withdraw(u64, move_core_types::language_storage::TypeTag),
}

impl MySoExtraValueArgs {
    fn parse_object_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "object")?;
        Ok(MySoExtraValueArgs::Object(fake_id, version))
    }

    fn parse_receiving_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "receiving")?;
        Ok(MySoExtraValueArgs::Receiving(fake_id, version))
    }

    fn parse_owned_object_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "owned")?;
        Ok(MySoExtraValueArgs::Owned(fake_id, version))
    }

    fn parse_read_shared_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "immshared")?;
        Ok(MySoExtraValueArgs::Shared(
            SharedObjectMutability::Immutable,
            fake_id,
            version,
        ))
    }

    fn parse_mut_shared_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "mutshared")?;
        Ok(MySoExtraValueArgs::Shared(
            SharedObjectMutability::Mutable,
            fake_id,
            version,
        ))
    }

    fn parse_non_exlucsive_write_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "nonexclusive")?;
        Ok(MySoExtraValueArgs::Shared(
            SharedObjectMutability::NonExclusiveWrite,
            fake_id,
            version,
        ))
    }

    fn parse_digest_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let contents = parser.advance(ValueToken::Ident)?;
        ensure!(contents == "digest");
        parser.advance(ValueToken::LParen)?;
        let package = parser.advance(ValueToken::Ident)?;
        parser.advance(ValueToken::RParen)?;
        Ok(MySoExtraValueArgs::Digest(package.to_owned()))
    }

    fn parse_withdraw_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let contents = parser.advance(ValueToken::Ident)?;
        ensure!(contents == "withdraw");

        // Format: withdraw<Type>(amount)
        parser.advance(ValueToken::LAngle)?;

        // Parse type - collect all tokens until we hit the matching RAngle
        // Need to track nesting level for types like Balance<Coin<MYSO>>
        let mut type_parts = Vec::new();
        let mut angle_bracket_depth = 1; // We already consumed the opening <
        loop {
            let (tok, s) = match parser.peek() {
                Some(v) => v,
                None => bail!("Unexpected end of input while parsing withdraw type"),
            };
            match tok {
                ValueToken::Whitespace => {
                    parser.advance(ValueToken::Whitespace)?;
                    // Skip whitespace
                }
                ValueToken::Ident => {
                    parser.advance(ValueToken::Ident)?;
                    type_parts.push(s.to_string());
                }
                ValueToken::ColonColon => {
                    parser.advance(ValueToken::ColonColon)?;
                    type_parts.push("::".to_string());
                }
                ValueToken::LAngle => {
                    parser.advance(ValueToken::LAngle)?;
                    type_parts.push("<".to_string());
                    angle_bracket_depth += 1;
                }
                ValueToken::RAngle => {
                    parser.advance(ValueToken::RAngle)?;
                    angle_bracket_depth -= 1;
                    if angle_bracket_depth == 0 {
                        // This is the closing > for withdraw<Type>
                        break;
                    }
                    type_parts.push(">".to_string());
                }
                ValueToken::Comma => {
                    parser.advance(ValueToken::Comma)?;
                    type_parts.push(",".to_string());
                }
                _ => bail!("Unexpected token {:?} while parsing withdraw type", tok),
            }
        }

        let type_str = type_parts.join("");

        // Parse the type from the type string
        let type_tokens: Vec<_> = TypeToken::tokenize(&type_str)?.into_iter().collect();
        let mut type_parser = move_core_types::parsing::parser::Parser::new(type_tokens);
        let parsed_type = type_parser.parse_type()?;

        // Now parse (amount)
        parser.advance(ValueToken::LParen)?;
        let amount_str = parser.advance(ValueToken::Number)?;
        let (amount, _) = parse_u64(amount_str)?;
        parser.advance(ValueToken::RParen)?;

        Ok(MySoExtraValueArgs::Withdraw(amount, parsed_type))
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
            let address: MySoAddress = MySoAddress::from_bytes(&u256_bytes).unwrap();
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

impl MySoValue {
    fn assert_move_value(self) -> MoveValue {
        match self {
            MySoValue::MoveValue(v) => v,
            MySoValue::Object(_, _) => panic!("unexpected nested MySo object in args"),
            MySoValue::ObjVec(_) => panic!("unexpected nested MySo object vector in args"),
            MySoValue::Digest(_) => panic!("unexpected nested MySo package digest in args"),
            MySoValue::Receiving(_, _) => panic!("unexpected nested MySo receiving object in args"),
            MySoValue::Owned(_, _) => panic!("unexpected nested MySo owned object in args"),
            MySoValue::Shared(_, _, _) => panic!("unexpected nested MySo shared object in args"),
            MySoValue::Withdraw(_, _) => {
                panic!("unexpected nested MySo withdraw reservation in args")
            }
        }
    }

    fn assert_object(self) -> (FakeID, Option<SequenceNumber>) {
        match self {
            MySoValue::MoveValue(_) => panic!("unexpected nested non-object value in args"),
            MySoValue::Object(id, version) => (id, version),
            MySoValue::ObjVec(_) => panic!("unexpected nested MySo object vector in args"),
            MySoValue::Digest(_) => panic!("unexpected nested MySo package digest in args"),
            MySoValue::Receiving(_, _) => panic!("unexpected nested MySo receiving object in args"),
            MySoValue::Owned(_, _) => panic!("unexpected nested MySo owned object in args"),
            MySoValue::Shared(_, _, _) => panic!("unexpected nested MySo shared object in args"),
            MySoValue::Withdraw(_, _) => {
                panic!("unexpected nested MySo withdraw reservation in args")
            }
        }
    }

    fn resolve_object(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &MySoTestAdapter,
    ) -> anyhow::Result<Object> {
        let id = match test_adapter.fake_to_real_object_id(fake_id) {
            Some(id) => id,
            None => bail!("INVALID TEST. Unknown object, object({})", fake_id),
        };
        let obj_res = if let Some(v) = version {
            myso_types::storage::ObjectStore::get_object_by_key(&*test_adapter.executor, &id, v)
        } else {
            myso_types::storage::ObjectStore::get_object(&*test_adapter.executor, &id)
        };
        let obj = match obj_res {
            Some(obj) => obj,
            None => bail!("INVALID TEST. Could not load object argument {}", id),
        };
        Ok(obj)
    }

    fn receiving_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &MySoTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        Ok(ObjectArg::Receiving(obj.compute_object_reference()))
    }

    fn owned_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &MySoTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        let obj_ref = obj.compute_object_reference();
        Ok(ObjectArg::ImmOrOwnedObject(obj_ref))
    }

    fn shared_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &MySoTestAdapter,
        mutability: SharedObjectMutability,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        let id = obj.id();
        let initial_shared_version = match obj.owner {
            Owner::AddressOwner(_) | Owner::ObjectOwner(_) | Owner::Immutable => {
                SequenceNumber::from_u64(0)
            }
            Owner::Shared {
                initial_shared_version,
            } => initial_shared_version,
            Owner::ConsensusAddressOwner { start_version, .. } => start_version,
        };
        Ok(ObjectArg::SharedObject {
            id,
            initial_shared_version,
            mutability,
        })
    }

    fn object_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &MySoTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        let id = obj.id();
        match obj.owner {
            Owner::Shared {
                initial_shared_version,
            }
            | Owner::ConsensusAddressOwner {
                start_version: initial_shared_version,
                ..
            } => Ok(ObjectArg::SharedObject {
                id,
                initial_shared_version,
                mutability: SharedObjectMutability::Mutable,
            }),
            Owner::AddressOwner(_) | Owner::ObjectOwner(_) | Owner::Immutable => {
                let obj_ref = obj.compute_object_reference();
                Ok(ObjectArg::ImmOrOwnedObject(obj_ref))
            }
        }
    }

    pub(crate) fn into_call_arg(self, test_adapter: &MySoTestAdapter) -> anyhow::Result<CallArg> {
        Ok(match self {
            MySoValue::Object(fake_id, version) => {
                CallArg::Object(Self::object_arg(fake_id, version, test_adapter)?)
            }
            MySoValue::MoveValue(v) => CallArg::Pure(v.simple_serialize().unwrap()),
            MySoValue::Receiving(fake_id, version) => {
                CallArg::Object(Self::receiving_arg(fake_id, version, test_adapter)?)
            }
            MySoValue::Owned(fake_id, version) => {
                CallArg::Object(Self::owned_arg(fake_id, version, test_adapter)?)
            }
            MySoValue::Shared(mutability, fake_id, version) => CallArg::Object(Self::shared_arg(
                fake_id,
                version,
                test_adapter,
                mutability,
            )?),
            MySoValue::ObjVec(_) => bail!("obj vec is not supported as an input"),
            MySoValue::Digest(pkg) => {
                let pkg = Symbol::from(pkg);
                let Some(staged) = test_adapter.staged_modules.get(&pkg) else {
                    bail!("Unbound staged package '{pkg}'")
                };
                CallArg::Pure(bcs::to_bytes(&staged.digest).unwrap())
            }
            MySoValue::Withdraw(amount, type_tag) => {
                // Check if the type is Balance<T> and extract the inner type T
                // For now, we only support Balance type for withdraw
                let inner_type =
                    Balance::maybe_get_balance_type_param(&type_tag).ok_or_else(|| {
                        anyhow::anyhow!(
                            "Withdraw only supports Balance<T> types, got: {}",
                            type_tag
                        )
                    })?;
                CallArg::FundsWithdrawal(FundsWithdrawalArg::balance_from_sender(
                    amount, inner_type,
                ))
            }
        })
    }

    pub(crate) fn into_argument(
        self,
        builder: &mut ProgrammableTransactionBuilder,
        test_adapter: &MySoTestAdapter,
    ) -> anyhow::Result<Argument> {
        match self {
            MySoValue::ObjVec(vec) => builder.make_obj_vec(
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

impl ParsableValue for MySoExtraValueArgs {
    type ConcreteValue = MySoValue;

    fn parse_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> Option<anyhow::Result<Self>> {
        match parser.peek()? {
            (ValueToken::Ident, "object") => Some(Self::parse_object_value(parser)),
            (ValueToken::Ident, "digest") => Some(Self::parse_digest_value(parser)),
            (ValueToken::Ident, "receiving") => Some(Self::parse_receiving_value(parser)),
            (ValueToken::Ident, "owned") => Some(Self::parse_owned_object_value(parser)),
            (ValueToken::Ident, "mutshared") => Some(Self::parse_mut_shared_value(parser)),
            (ValueToken::Ident, "immshared") => Some(Self::parse_read_shared_value(parser)),
            (ValueToken::Ident, "nonexclusive") => {
                Some(Self::parse_non_exlucsive_write_value(parser))
            }
            (ValueToken::Ident, "withdraw") => Some(Self::parse_withdraw_value(parser)),
            _ => None,
        }
    }

    fn move_value_into_concrete(v: MoveValue) -> anyhow::Result<Self::ConcreteValue> {
        Ok(MySoValue::MoveValue(v))
    }

    fn concrete_vector(elems: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        if !elems.is_empty() && matches!(elems[0], MySoValue::Object(_, _)) {
            Ok(MySoValue::ObjVec(
                elems.into_iter().map(MySoValue::assert_object).collect(),
            ))
        } else {
            Ok(MySoValue::MoveValue(MoveValue::Vector(
                elems
                    .into_iter()
                    .map(MySoValue::assert_move_value)
                    .collect(),
            )))
        }
    }

    fn concrete_struct(values: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        Ok(MySoValue::MoveValue(MoveValue::Struct(MoveStruct(
            values.into_iter().map(|v| v.assert_move_value()).collect(),
        ))))
    }

    fn into_concrete_value(
        self,
        mapping: &impl Fn(&str) -> Option<move_core_types::account_address::AccountAddress>,
    ) -> anyhow::Result<Self::ConcreteValue> {
        match self {
            MySoExtraValueArgs::Object(id, version) => Ok(MySoValue::Object(id, version)),
            MySoExtraValueArgs::Digest(pkg) => Ok(MySoValue::Digest(pkg)),
            MySoExtraValueArgs::Receiving(id, version) => Ok(MySoValue::Receiving(id, version)),
            MySoExtraValueArgs::Owned(id, version) => Ok(MySoValue::Owned(id, version)),
            MySoExtraValueArgs::Shared(mutability, id, version) => {
                Ok(MySoValue::Shared(mutability, id, version))
            }
            MySoExtraValueArgs::Withdraw(amount, parsed_type) => {
                let type_tag = parsed_type.into_type_tag(mapping)?;
                Ok(MySoValue::Withdraw(amount, type_tag))
            }
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
        let address: MySoAddress = MySoAddress::from_bytes(&u256_bytes).unwrap();
        FakeID::Known(address.into())
    })
}

fn parse_policy(x: &str) -> anyhow::Result<u8> {
    Ok(match x {
        "compatible" => UpgradePolicy::COMPATIBLE,
        "additive" => UpgradePolicy::ADDITIVE,
        "dep_only" => UpgradePolicy::DEP_ONLY,
        _ => bail!(
            "Invalid upgrade policy {x}. Policy must be one of 'compatible', 'additive', or 'dep_only'"
        ),
    })
}
