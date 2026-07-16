// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Chain tx: `finalize_spot_claims_for_post`. Commits a post's multi-claim analysis so its
//! future-linked markets become bettable (the Move bet-lock requires `completed` status).

use anyhow::Context;
use myso_json_rpc_types::{MySoObjectDataOptions, MySoTransactionBlockResponseOptions};
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::{MySoAddress, ObjectID};
use myso_types::crypto::MySoKeyPair;
use myso_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_types::transaction::{
    Command, ObjectArg, ProgrammableMoveCall, SharedObjectMutability, Transaction, TransactionData,
};
use myso_types::transaction_driver_types::ExecuteTransactionRequestType;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

use crate::api::AppState;
use crate::blockchain::{chain_configured, parse_object_id, shared_object_arg, CLOCK_OBJECT_ID};
use crate::config::SOCIAL_PACKAGE_ID;
use crate::store::jobs::SpotJob;

/// Counts committed at finalize.
#[derive(Debug, Clone, Copy)]
pub struct FinalizeCounts {
    pub detected: u64,
    pub rejected: u64,
    pub truncated: u64,
    pub past_verified: u64,
}

/// Past-verdict payload (parallel vectors) committed on-chain in the finalize manifest.
#[derive(Debug, Clone, Default)]
pub struct PastVerdicts {
    pub veracity_manifest_hash: Option<Vec<u8>>,
    pub claim_indexes: Vec<u64>,
    pub verdicts: Vec<u8>,
    pub related_markets: Vec<MySoAddress>,
    pub evidence_hashes: Vec<Vec<u8>>,
}

pub async fn submit_finalize_post(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let post_id = job
        .payload
        .get("post_id")
        .and_then(|v| v.as_str())
        .context("finalize_post missing post_id")?
        .to_string();
    let counts = FinalizeCounts {
        detected: json_u64(job, "detected_claim_count").unwrap_or(0),
        rejected: json_u64(job, "rejected_claim_count").unwrap_or(0),
        truncated: json_u64(job, "truncated_claim_count").unwrap_or(0),
        past_verified: json_u64(job, "past_verified_count").unwrap_or(0),
    };
    let past = parse_past_verdicts(job)?;

    if !chain_configured(&state.args) {
        tracing::warn!(post_id, "chain not configured — finalize_post skipped");
        return Ok(());
    }

    let nonce = format!("finalize-post-{post_id}");
    let tx_id = state
        .store
        .insert_transaction(job.market_id, "finalize_post", &nonce)
        .await?;

    match build_and_submit_finalize(&state.args, &post_id, counts, &past).await {
        Ok(digest) => {
            state
                .store
                .update_transaction_status(tx_id, "confirmed", Some(&digest), None)
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["finalize_post", "confirmed"])
                .inc();
            info!(post_id, digest, "finalize_spot_claims_for_post submitted");
            Ok(())
        }
        Err(err) => {
            state
                .store
                .update_transaction_status(tx_id, "failed", None, Some(&err.to_string()))
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["finalize_post", "failed"])
                .inc();
            Err(err)
        }
    }
}

fn json_u64(job: &SpotJob, key: &str) -> Option<u64> {
    job.payload.get(key).and_then(|v| v.as_u64())
}

fn decode_hex(s: &str) -> Vec<u8> {
    hex::decode(s.trim_start_matches("0x")).unwrap_or_default()
}

/// Parse the parallel past-verdict vectors from the finalize job payload.
fn parse_past_verdicts(job: &SpotJob) -> anyhow::Result<PastVerdicts> {
    let arr = |k: &str| -> Vec<serde_json::Value> {
        job.payload
            .get(k)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default()
    };
    let related_markets = arr("past_related_market_ids")
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| {
            MySoAddress::from_str(s)
                .map_err(|e| anyhow::anyhow!("bad related_market_id {s}: {e:?}"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(PastVerdicts {
        veracity_manifest_hash: job
            .payload
            .get("veracity_manifest_hash")
            .and_then(|v| v.as_str())
            .map(decode_hex),
        claim_indexes: arr("past_claim_indexes")
            .iter()
            .filter_map(|v| v.as_u64())
            .collect(),
        verdicts: arr("past_verdicts")
            .iter()
            .filter_map(|v| v.as_u64().map(|n| n as u8))
            .collect(),
        related_markets,
        evidence_hashes: arr("past_evidence_hashes")
            .iter()
            .filter_map(|v| v.as_str().map(decode_hex))
            .collect(),
    })
}

async fn build_and_submit_finalize(
    args: &crate::config::OracleArgs,
    post_id: &str,
    counts: FinalizeCounts,
    past: &PastVerdicts,
) -> anyhow::Result<String> {
    let key_hex = args
        .private_key_hex
        .as_ref()
        .context("missing private key")?;
    let key_bytes = hex::decode(key_hex.trim())?;
    let key_pair =
        MySoKeyPair::from_bytes(&key_bytes).map_err(|e| anyhow::anyhow!("invalid key: {:?}", e))?;
    let sender = MySoAddress::from(&key_pair.public());

    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;

    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let gas_obj = gas_coins.data.first().context("sender has no gas coins")?;

    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let admin_cap = parse_object_id(args.admin_cap_object_id.as_ref().unwrap())?;
    let spot_config = parse_object_id(args.spot_config_object_id.as_ref().unwrap())?;
    let post_obj = parse_object_id(post_id)?;
    let clock = ObjectID::from_hex_literal(CLOCK_OBJECT_ID)?;

    let admin_obj = client
        .read_api()
        .get_object_with_options(admin_cap, MySoObjectDataOptions::default())
        .await?
        .into_object()?;
    let admin_arg = ObjectArg::ImmOrOwnedObject(admin_obj.object_ref());
    let config_arg =
        shared_object_arg(&client, spot_config, SharedObjectMutability::Immutable).await?;
    let post_arg = shared_object_arg(&client, post_obj, SharedObjectMutability::Mutable).await?;
    let clock_arg = shared_object_arg(&client, clock, SharedObjectMutability::Immutable).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let admin_input = ptb.obj(admin_arg)?;
    let config_input = ptb.obj(config_arg)?;
    let post_input = ptb.obj(post_arg)?;
    let detected_input = ptb.pure(counts.detected)?;
    let rejected_input = ptb.pure(counts.rejected)?;
    let truncated_input = ptb.pure(counts.truncated)?;
    let past_verified_input = ptb.pure(counts.past_verified)?;
    // Future claim manifest is not carried on-chain (links already have it); past verdicts are.
    let claim_manifest_input = ptb.pure(Option::<Vec<u8>>::None)?;
    let veracity_manifest_input = ptb.pure(past.veracity_manifest_hash.clone())?;
    let past_indexes_input = ptb.pure(past.claim_indexes.clone())?;
    let past_verdicts_input = ptb.pure(past.verdicts.clone())?;
    let past_related_markets_input = ptb.pure(past.related_markets.clone())?;
    let past_evidence_hashes_input = ptb.pure(past.evidence_hashes.clone())?;
    let clock_input = ptb.obj(clock_arg)?;

    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "social_proof_of_truth".to_string(),
        function: "finalize_spot_claims_for_post".to_string(),
        type_arguments: vec![],
        arguments: vec![
            admin_input,
            config_input,
            post_input,
            detected_input,
            rejected_input,
            truncated_input,
            past_verified_input,
            claim_manifest_input,
            veracity_manifest_input,
            past_indexes_input,
            past_verdicts_input,
            past_related_markets_input,
            past_evidence_hashes_input,
            clock_input,
        ],
    })));

    let pt = ptb.finish();
    let rgp = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_obj.object_ref()], pt, 30_000_000, rgp);
    let signed = Transaction::from_data_and_signer(tx_data, vec![&key_pair]);
    let response = client
        .quorum_driver_api()
        .execute_transaction_block(
            signed,
            MySoTransactionBlockResponseOptions::new().with_effects(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    if response.status_ok() == Some(false) {
        anyhow::bail!(
            "finalize_spot_claims_for_post failed: {:?}",
            response.effects
        );
    }
    Ok(response.digest.to_string())
}
