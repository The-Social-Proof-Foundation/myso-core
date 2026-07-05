// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Typed BCS layout for `social_contracts::memory::SubAgent` identity fields.

use anyhow::{Context, Result};
use move_core_types::account_address::AccountAddress;
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::ObjectID;
use myso_types::id::{ID, UID};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BcsSubAgentConstraints {
    _approval_required_caps: u64,
    _max_action_spend: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BcsSubAgent {
    _id: UID,
    _memory_account_id: ID,
    _organization_id: ID,
    _principal_owner: AccountAddress,
    _profile_id: AccountAddress,
    pub derived_address: AccountAddress,
    pub public_key: Vec<u8>,
    _label: String,
    _identity_class: u8,
    _role_tags: u64,
    _capabilities: u64,
    _delegatable_caps: u64,
    _register_scope: u8,
    _constraints: BcsSubAgentConstraints,
    _platform_scope: Option<AccountAddress>,
    _parent_object_id: Option<ID>,
    _depth: u8,
    _registered_by: AccountAddress,
    _created_at: u64,
    _expires_at: Option<u64>,
    _active: bool,
}

pub fn parse_sub_agent(data: &[u8]) -> Result<BcsSubAgent, bcs::Error> {
    bcs::from_bytes(data)
}

pub async fn fetch_on_chain_sub_agent(rpc_url: &str, agent_object_id: &str) -> Result<BcsSubAgent> {
    let object_id = ObjectID::from_hex_literal(agent_object_id)?;
    let client = MySoClientBuilder::default().build(rpc_url).await?;
    let data = client
        .read_api()
        .get_move_object_bcs(object_id)
        .await
        .context("fetch SubAgent BCS")?;
    parse_sub_agent(&data).context("parse SubAgent BCS")
}

pub fn address_to_hex(addr: &AccountAddress) -> String {
    format!("0x{}", hex::encode(addr))
}
