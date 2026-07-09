// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod create_market;
pub mod refund;
pub mod settle;
pub mod worker;

use anyhow::Context;
use myso_json_rpc_types::MySoObjectDataOptions;
use myso_types::base_types::ObjectID;
use myso_types::transaction::{ObjectArg, SharedObjectMutability};

pub const CLOCK_OBJECT_ID: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000006";

pub fn parse_object_id(id: &str) -> anyhow::Result<ObjectID> {
    ObjectID::from_hex_literal(id.trim()).context("invalid object id")
}

pub fn chain_configured(args: &crate::config::OracleArgs) -> bool {
    args.private_key_hex.is_some()
        && args.spot_config_object_id.is_some()
        && args.admin_cap_object_id.is_some()
        && args.spot_registry_object_id.is_some()
}

pub async fn shared_object_arg(
    client: &myso_sdk::MySoClient,
    object_id: ObjectID,
    mutability: SharedObjectMutability,
) -> anyhow::Result<ObjectArg> {
    let object = client
        .read_api()
        .get_object_with_options(object_id, MySoObjectDataOptions::new().with_owner())
        .await?;
    let data = object.data.as_ref().context("object missing data")?;
    let initial_shared_version = match data.owner.as_ref() {
        Some(myso_types::object::Owner::Shared {
            initial_shared_version,
        }) => *initial_shared_version,
        other => anyhow::bail!("object {:?} is not shared: {:?}", object_id, other),
    };
    Ok(ObjectArg::SharedObject {
        id: object_id,
        initial_shared_version,
        mutability,
    })
}
