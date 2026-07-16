// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use move_core_types::identifier::Identifier;
use myso_json_rpc_types::{EventFilter, MySoTransactionBlockEvents};
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::ObjectID;

use crate::config::{OracleArgs, SOCIAL_PACKAGE_ID};

/// Extract `claim_id` from `SpotClaimCreatedEvent` in the transaction response.
pub fn find_claim_id_in_tx_events(
    events: Option<&MySoTransactionBlockEvents>,
    semantic_hash: Option<&[u8]>,
) -> Option<String> {
    let data = &events?.data;
    for event in data {
        if event.type_.name.as_str() != "SpotClaimCreatedEvent" {
            continue;
        }
        if let Some(expected) = semantic_hash {
            let hash = event.parsed_json.get("semantic_claim_hash")?;
            if !hash_json_value_matches(hash, expected) {
                continue;
            }
        }
        return event
            .parsed_json
            .get("claim_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
    }
    None
}

/// Extract `market_id` from `SpotMarketCreatedEvent` in the transaction response.
pub fn find_market_id_in_tx_events(
    events: Option<&MySoTransactionBlockEvents>,
    market_key_hash: Option<&[u8]>,
) -> Option<String> {
    let data = &events?.data;
    for event in data {
        if event.type_.name.as_str() != "SpotMarketCreatedEvent" {
            continue;
        }
        if let Some(expected) = market_key_hash {
            let hash = event.parsed_json.get("market_key_hash")?;
            if !hash_json_value_matches(hash, expected) {
                continue;
            }
        }
        return event
            .parsed_json
            .get("market_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
    }
    None
}

const MAX_EVENT_PAGES: usize = 20;

#[derive(Debug, Clone)]
pub struct OnChainMarketRef {
    pub market_object_id: String,
    pub claim_object_id: String,
}

pub async fn lookup_claim_object_id_by_semantic_hash(
    args: &OracleArgs,
    semantic_hash: &[u8],
) -> anyhow::Result<Option<String>> {
    scan_spot_events(args, "SpotClaimCreatedEvent", |parsed| {
        let hash = parsed.get("semantic_claim_hash")?;
        if !hash_json_value_matches(hash, semantic_hash) {
            return None;
        }
        parsed.get("claim_id")?.as_str().map(|s| s.to_string())
    })
    .await
}

pub async fn lookup_market_by_key_hash(
    args: &OracleArgs,
    market_key_hash: &[u8],
) -> anyhow::Result<Option<OnChainMarketRef>> {
    scan_spot_events(args, "SpotMarketCreatedEvent", |parsed| {
        let hash = parsed.get("market_key_hash")?;
        if !hash_json_value_matches(hash, market_key_hash) {
            return None;
        }
        let market_object_id = parsed.get("market_id")?.as_str()?.to_string();
        let claim_object_id = parsed.get("claim_id")?.as_str()?.to_string();
        Some(OnChainMarketRef {
            market_object_id,
            claim_object_id,
        })
    })
    .await
}

async fn scan_spot_events<T, F>(
    args: &OracleArgs,
    event_name: &str,
    mut matcher: F,
) -> anyhow::Result<Option<T>>
where
    F: FnMut(&serde_json::Value) -> Option<T>,
{
    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;
    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let filter = EventFilter::MoveEventModule {
        package,
        module: Identifier::new("social_proof_of_truth").context("invalid module id")?,
    };

    let mut cursor = None;
    for _ in 0..MAX_EVENT_PAGES {
        let page = client
            .event_api()
            .query_events(filter.clone(), cursor, Some(200), true)
            .await?;
        for event in &page.data {
            if event.type_.name.as_str() != event_name {
                continue;
            }
            if let Some(result) = matcher(&event.parsed_json) {
                return Ok(Some(result));
            }
        }
        if !page.has_next_page {
            break;
        }
        cursor = page.next_cursor;
    }
    Ok(None)
}

fn hash_json_value_matches(value: &serde_json::Value, expected: &[u8]) -> bool {
    hash_from_json_value(value)
        .map(|bytes| bytes == expected)
        .unwrap_or(false)
}

fn hash_from_json_value(value: &serde_json::Value) -> Option<Vec<u8>> {
    match value {
        serde_json::Value::String(s) => {
            let trimmed = s.trim().trim_start_matches("0x");
            hex::decode(trimmed).ok()
        }
        serde_json::Value::Array(arr) => {
            let mut bytes = Vec::with_capacity(arr.len());
            for v in arr {
                let n = v.as_u64()?;
                if n > 255 {
                    return None;
                }
                bytes.push(n as u8);
            }
            Some(bytes)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_field_matches_hex_prefix() {
        let bytes = vec![0xabu8; 32];
        assert!(hash_json_value_matches(
            &serde_json::json!(format!("0x{}", hex::encode(&bytes))),
            &bytes
        ));
    }

    #[test]
    fn hash_field_matches_byte_array() {
        let bytes = vec![62, 181, 214, 190, 125];
        assert!(hash_json_value_matches(
            &serde_json::json!([62, 181, 214, 190, 125]),
            &bytes
        ));
    }

    #[test]
    fn tx_events_yield_claim_id() {
        use move_core_types::account_address::AccountAddress;
        use move_core_types::identifier::Identifier;
        use move_core_types::language_storage::StructTag;
        use myso_json_rpc_types::{BcsEvent, MySoEvent, MySoTransactionBlockEvents};
        use myso_types::base_types::TransactionDigest;
        use myso_types::event::EventID;

        let hash = vec![0xabu8; 32];
        let events = MySoTransactionBlockEvents {
            data: vec![MySoEvent {
                id: EventID {
                    tx_digest: TransactionDigest::random(),
                    event_seq: 0,
                },
                package_id: ObjectID::from_hex_literal("0x50c1").expect("package"),
                transaction_module: Identifier::new("social_proof_of_truth").expect("module"),
                sender: AccountAddress::ZERO.into(),
                type_: StructTag {
                    address: AccountAddress::from_hex_literal("0x50c1").expect("addr"),
                    module: Identifier::new("social_proof_of_truth").expect("module"),
                    name: Identifier::new("SpotClaimCreatedEvent").expect("event"),
                    type_params: vec![],
                },
                parsed_json: serde_json::json!({
                    "claim_id": "0xclaim",
                    "semantic_claim_hash": format!("0x{}", hex::encode(&hash)),
                    "created_at_ms": 1
                }),
                bcs: BcsEvent::Base64 { bcs: vec![] },
                timestamp_ms: None,
            }],
        };
        let claim_id = super::find_claim_id_in_tx_events(Some(&events), Some(&hash));
        assert_eq!(claim_id.as_deref(), Some("0xclaim"));
    }
}
