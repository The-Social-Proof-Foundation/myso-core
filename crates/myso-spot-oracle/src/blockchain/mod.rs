// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod create_market;
pub mod settle;
pub mod worker;

use anyhow::Context;
use myso_types::base_types::ObjectID;

pub fn parse_object_id(id: &str) -> anyhow::Result<ObjectID> {
    ObjectID::from_hex_literal(id.trim()).context("invalid object id")
}

pub fn chain_configured(args: &crate::config::OracleArgs) -> bool {
    args.private_key_hex.is_some()
        && args.spot_config_object_id.is_some()
        && args.admin_cap_object_id.is_some()
}
