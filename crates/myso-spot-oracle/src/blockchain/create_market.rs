// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Context;
use myso_json_rpc_types::{
    MySoObjectDataOptions, MySoTransactionBlockEffectsAPI, MySoTransactionBlockResponseOptions,
    ObjectChange,
};
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::{MySoAddress, ObjectID};
use myso_types::crypto::MySoKeyPair;
use myso_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_types::transaction::{
    Command, ObjectArg, ProgrammableMoveCall, SharedObjectMutability, Transaction, TransactionData,
};
use myso_types::transaction_driver_types::ExecuteTransactionRequestType;
use tracing::info;
use uuid::Uuid;

use crate::api::AppState;
use crate::blockchain::chain_lookup::{
    lookup_claim_object_id_by_semantic_hash, lookup_market_by_key_hash,
};
use crate::blockchain::{chain_configured, parse_object_id, shared_object_arg, CLOCK_OBJECT_ID};
use crate::config::SOCIAL_PACKAGE_ID;
use crate::store::jobs::SpotJob;

pub async fn submit_create_claim_market(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let oracle_market_id = job
        .market_id
        .context("create_claim_market missing market_id")?;
    let spot_claim_id = job
        .payload
        .get("spot_claim_id")
        .and_then(|v| v.as_str())
        .context("create_claim_market missing spot_claim_id")?
        .parse::<Uuid>()?;
    let spot_market_id = job
        .payload
        .get("spot_market_id")
        .and_then(|v| v.as_str())
        .context("create_claim_market missing spot_market_id")?
        .parse::<Uuid>()?;

    let market = state
        .store
        .get_market(oracle_market_id)
        .await?
        .context("oracle market not found")?;
    let spot_claim = state
        .store
        .get_claim_by_id(spot_claim_id)
        .await?
        .context("spot claim row not found")?;
    let spot_market = state
        .store
        .get_spot_market_by_id(spot_market_id)
        .await?
        .context("spot market row not found")?;

    if !chain_configured(&state.args) {
        tracing::warn!(market_id = %oracle_market_id, "chain not configured — activating market off-chain");
        let mut ctx = crate::claim::lifecycle::default_context_for(
            &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
        );
        ctx.job_id = Some(job.id);
        ctx.on_chain_status = Some(1);
        state
            .store
            .apply_market_transition(
                oracle_market_id,
                &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
                &ctx,
            )
            .await?;
        return Ok(());
    }

    let options: Vec<String> = serde_json::from_value(market.betting_options.clone())?;
    let semantic_hash = decode_hash_hex(&spot_claim.semantic_claim_hash)?;
    let market_key_hash = decode_hash_hex(&spot_market.market_key_hash)?;

    let mut claim_object_id = spot_claim.spot_claim_object_id.clone();
    if claim_object_id.is_none() {
        claim_object_id =
            lookup_claim_object_id_by_semantic_hash(&state.args, &semantic_hash).await?;
    }

    let mut existing_market = spot_market.spot_market_object_id.clone();
    let mut existing_market_claim = claim_object_id.clone();
    if existing_market.is_none() {
        if let Some(on_chain) = lookup_market_by_key_hash(&state.args, &market_key_hash).await? {
            existing_market = Some(on_chain.market_object_id);
            if existing_market_claim.is_none() {
                existing_market_claim = Some(on_chain.claim_object_id);
            }
        }
    }

    if let Some(market_object_id) = existing_market {
        let claim_id = existing_market_claim
            .as_deref()
            .context("on-chain market exists but claim object id is unknown")?;
        return submit_link_to_existing_market(
            state,
            job,
            oracle_market_id,
            spot_claim_id,
            spot_market_id,
            claim_id,
            &market_object_id,
            &market.post_id,
        )
        .await;
    }

    let nonce = format!("create-claim-market-{oracle_market_id}");
    let tx_id = state
        .store
        .insert_transaction(Some(oracle_market_id), "create_claim_market", &nonce)
        .await?;

    match build_and_submit_create(
        &state.args,
        &market.post_id,
        &options,
        spot_market.deadline,
        market.max_resolution_window_ms as u64,
        &semantic_hash,
        &market_key_hash,
        claim_object_id.as_deref(),
    )
    .await
    {
        Ok((digest, claim_object_id, market_object_id)) => {
            state
                .store
                .update_transaction_status(tx_id, "confirmed", Some(&digest), None)
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["create_claim_market", "confirmed"])
                .inc();
            if spot_claim.spot_claim_object_id.is_none() {
                state
                    .store
                    .set_spot_claim_object_id(spot_claim_id, &claim_object_id)
                    .await?;
            }
            state
                .store
                .set_spot_market_object_id(spot_market_id, &market_object_id)
                .await?;
            let mut ctx = crate::claim::lifecycle::default_context_for(
                &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
            );
            ctx.job_id = Some(job.id);
            ctx.tx_digest = Some(digest.clone());
            ctx.on_chain_status = Some(1);
            state
                .store
                .set_spot_market_object_id_on_market(oracle_market_id, &market_object_id, ctx)
                .await?;
            info!(
                market_id = %oracle_market_id,
                digest,
                claim_object_id,
                market_object_id,
                "create_spot_claim + create_spot_market_for_claim submitted"
            );
            enqueue_finalize_post(&state, oracle_market_id, &market.post_id, 1).await?;
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
                .with_label_values(&["create_claim_market", "failed"])
                .inc();
            Err(err)
        }
    }
}

pub async fn submit_link_post(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let oracle_market_id = job.market_id.context("link_post missing market_id")?;
    let spot_claim_id = job
        .payload
        .get("spot_claim_id")
        .and_then(|v| v.as_str())
        .context("link_post missing spot_claim_id")?
        .parse::<Uuid>()?;
    let spot_market_id = job
        .payload
        .get("spot_market_id")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<Uuid>().ok());

    let market = state
        .store
        .get_market(oracle_market_id)
        .await?
        .context("oracle market not found")?;
    let spot_claim = state
        .store
        .get_claim_by_id(spot_claim_id)
        .await?
        .context("spot claim row not found")?;

    let mut claim_object_id = spot_claim.spot_claim_object_id.clone();
    if claim_object_id.is_none() {
        let semantic_hash = decode_hash_hex(&spot_claim.semantic_claim_hash)?;
        claim_object_id =
            lookup_claim_object_id_by_semantic_hash(&state.args, &semantic_hash).await?;
    }
    let claim_object_id = claim_object_id
        .as_deref()
        .context("link_post requires on-chain claim object id")?;

    let mut market_object_id = None;
    if let Some(spot_market_id) = spot_market_id {
        if let Some(spot_market) = state.store.get_spot_market_by_id(spot_market_id).await? {
            market_object_id = spot_market.spot_market_object_id;
            if market_object_id.is_none() {
                let market_key_hash = decode_hash_hex(&spot_market.market_key_hash)?;
                if let Some(on_chain) =
                    lookup_market_by_key_hash(&state.args, &market_key_hash).await?
                {
                    market_object_id = Some(on_chain.market_object_id);
                }
            }
        }
    }

    if !chain_configured(&state.args) {
        tracing::warn!(market_id = %oracle_market_id, "chain not configured — link_post off-chain only");
        return Ok(());
    }

    let nonce = format!("link-post-{oracle_market_id}");
    let tx_id = state
        .store
        .insert_transaction(Some(oracle_market_id), "link_post", &nonce)
        .await?;

    match build_and_submit_link_post(&state.args, claim_object_id, &market.post_id).await {
        Ok(digest) => {
            state
                .store
                .update_transaction_status(tx_id, "confirmed", Some(&digest), None)
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["link_post", "confirmed"])
                .inc();
            if spot_claim.spot_claim_object_id.is_none() {
                state
                    .store
                    .set_spot_claim_object_id(spot_claim_id, claim_object_id)
                    .await?;
            }
            if let (Some(spot_market_id), Some(market_object_id)) =
                (spot_market_id, market_object_id.as_deref())
            {
                state
                    .store
                    .set_spot_market_object_id(spot_market_id, market_object_id)
                    .await?;
                let mut ctx = crate::claim::lifecycle::default_context_for(
                    &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
                );
                ctx.job_id = Some(job.id);
                ctx.tx_digest = Some(digest.clone());
                ctx.on_chain_status = Some(1);
                state
                    .store
                    .set_spot_market_object_id_on_market(
                        oracle_market_id,
                        market_object_id,
                        ctx,
                    )
                    .await?;
            }
            info!(market_id = %oracle_market_id, digest, "link_post submitted");
            enqueue_finalize_post(&state, oracle_market_id, &market.post_id, 1).await?;
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
                .with_label_values(&["link_post", "failed"])
                .inc();
            Err(err)
        }
    }
}

pub async fn submit_create_market(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    submit_create_claim_market(state, job).await
}

/// Enqueue an on-chain `finalize_spot_claims_for_post` after a market create/link confirms, so
/// the post's analysis reaches `completed` and its future-linked markets become bettable.
/// Single future-claim baseline: `detected_claim_count = 1`, no past verdicts.
async fn enqueue_finalize_post(
    state: &Arc<AppState>,
    oracle_market_id: Uuid,
    post_id: &str,
    detected: u64,
) -> anyhow::Result<()> {
    crate::store::jobs::enqueue_job(
        state.store.pool(),
        "SubmitChainTx",
        Some(oracle_market_id),
        None,
        80,
        chrono::Utc::now(),
        serde_json::json!({
            "tx_kind": "finalize_post",
            "post_id": post_id,
            "detected_claim_count": detected,
            "rejected_claim_count": 0,
            "truncated_claim_count": 0,
            "past_verified_count": 0,
        }),
    )
    .await?;
    Ok(())
}

async fn submit_link_to_existing_market(
    state: Arc<AppState>,
    job: &SpotJob,
    oracle_market_id: Uuid,
    spot_claim_id: Uuid,
    spot_market_id: Uuid,
    claim_object_id: &str,
    market_object_id: &str,
    post_id: &str,
) -> anyhow::Result<()> {
    let nonce = format!("link-existing-market-{oracle_market_id}");
    let tx_id = state
        .store
        .insert_transaction(Some(oracle_market_id), "link_post", &nonce)
        .await?;

    match build_and_submit_link_post(&state.args, claim_object_id, post_id).await {
        Ok(digest) => {
            state
                .store
                .update_transaction_status(tx_id, "confirmed", Some(&digest), None)
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["link_post", "confirmed"])
                .inc();
            state
                .store
                .set_spot_claim_object_id(spot_claim_id, claim_object_id)
                .await?;
            state
                .store
                .set_spot_market_object_id(spot_market_id, market_object_id)
                .await?;
            let mut ctx = crate::claim::lifecycle::default_context_for(
                &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
            );
            ctx.job_id = Some(job.id);
            ctx.tx_digest = Some(digest.clone());
            ctx.on_chain_status = Some(1);
            state
                .store
                .set_spot_market_object_id_on_market(oracle_market_id, market_object_id, ctx)
                .await?;
            info!(
                market_id = %oracle_market_id,
                digest,
                claim_object_id,
                market_object_id,
                "linked post to existing on-chain claim/market"
            );
            enqueue_finalize_post(&state, oracle_market_id, post_id, 1).await?;
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
                .with_label_values(&["link_post", "failed"])
                .inc();
            Err(err)
        }
    }
}

fn decode_hash_hex(hex_str: &str) -> anyhow::Result<Vec<u8>> {
    let trimmed = hex_str.trim().trim_start_matches("0x");
    let bytes = hex::decode(trimmed).context("invalid claim/market hash hex")?;
    if bytes.len() < 8 {
        anyhow::bail!("hash too short (need >= 8 bytes)");
    }
    Ok(bytes)
}

async fn build_and_submit_create_market(
    args: &crate::config::OracleArgs,
    claim_object_id: &str,
    post_id: &str,
    betting_options: &[String],
    resolution_at_ms: u64,
    max_resolution_buffer_ms: u64,
    market_key_hash: &[u8],
) -> anyhow::Result<(String, String)> {
    let key_hex = args.private_key_hex.as_ref().context("missing private key")?;
    let key_bytes = hex::decode(key_hex.trim())?;
    let key_pair = MySoKeyPair::from_bytes(&key_bytes)
        .map_err(|e| anyhow::anyhow!("invalid key: {:?}", e))?;
    let sender = MySoAddress::from(&key_pair.public());

    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;

    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let gas_obj = gas_coins
        .data
        .first()
        .context("sender has no gas coins")?;

    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let admin_cap = parse_object_id(args.admin_cap_object_id.as_ref().unwrap())?;
    let spot_config = parse_object_id(args.spot_config_object_id.as_ref().unwrap())?;
    let registry_id = parse_object_id(args.spot_registry_object_id.as_ref().unwrap())?;
    let claim_obj = parse_object_id(claim_object_id)?;
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
    let registry_arg =
        shared_object_arg(&client, registry_id, SharedObjectMutability::Mutable).await?;
    // Move entry takes `&mut SpotClaim` (may call link_post_to_claim_internal).
    let claim_arg =
        shared_object_arg(&client, claim_obj, SharedObjectMutability::Mutable).await?;
    let post_arg = shared_object_arg(&client, post_obj, SharedObjectMutability::Mutable).await?;
    let clock_arg = shared_object_arg(&client, clock, SharedObjectMutability::Immutable).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let admin_input = ptb.obj(admin_arg)?;
    let config_input = ptb.obj(config_arg)?;
    let registry_input = ptb.obj(registry_arg)?;
    let claim_input = ptb.obj(claim_arg)?;
    let post_input = ptb.obj(post_arg)?;
    // Single-claim baseline: primary post links this market at claim_index 0. The market key
    // hash doubles as the resolution policy binding (both derived from the resolver spec).
    let claim_index_input = ptb.pure(0u64)?;
    let key_input = ptb.pure(market_key_hash.to_vec())?;
    let policy_input = ptb.pure(market_key_hash.to_vec())?;
    let options_input = ptb.pure(betting_options.to_vec())?;
    let resolution_at_input = ptb.pure(resolution_at_ms)?;
    let max_rw_input = ptb.pure(Some(max_resolution_buffer_ms))?;
    let clock_input = ptb.obj(clock_arg)?;
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "social_proof_of_truth".to_string(),
        function: "create_spot_market_for_claim".to_string(),
        type_arguments: vec![],
        arguments: vec![
            admin_input,
            config_input,
            registry_input,
            claim_input,
            post_input,
            claim_index_input,
            key_input,
            policy_input,
            options_input,
            resolution_at_input,
            max_rw_input,
            clock_input,
        ],
    })));

    let pt = ptb.finish();
    let rgp = client.read_api().get_reference_gas_price().await?;
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![gas_obj.object_ref()],
        pt,
        50_000_000,
        rgp,
    );
    let signed = Transaction::from_data_and_signer(tx_data, vec![&key_pair]);
    let response = client
        .quorum_driver_api()
        .execute_transaction_block(
            signed,
            MySoTransactionBlockResponseOptions::new()
                .with_effects()
                .with_object_changes(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    if response.status_ok() == Some(false) {
        let status = response
            .effects
            .as_ref()
            .map(|e| format!("{:?}", e.status()));
        anyhow::bail!("create_spot_market_for_claim transaction failed: {status:?}");
    }
    let digest = response.digest.to_string();
    let market_object_id = match find_created_type(&response.object_changes, "SpotMarket") {
        Some(id) => id,
        None => lookup_market_by_key_hash(args, market_key_hash)
            .await?
            .map(|m| m.market_object_id)
            .context(
                "create_spot_market_for_claim did not create SpotMarket and no existing market found on-chain",
            )?,
    };
    Ok((digest, market_object_id))
}

async fn build_and_submit_create(
    args: &crate::config::OracleArgs,
    post_id: &str,
    betting_options: &[String],
    deadline: Option<chrono::DateTime<chrono::Utc>>,
    max_resolution_buffer_ms: u64,
    semantic_claim_hash: &[u8],
    market_key_hash: &[u8],
    existing_claim_object_id: Option<&str>,
) -> anyhow::Result<(String, String, String)> {
    let resolution_at_ms = deadline
        .context("spot market missing deadline for on-chain resolution_at_ms")?
        .timestamp_millis()
        .try_into()
        .context("resolution_at_ms overflow")?;
    let key_hex = args.private_key_hex.as_ref().context("missing private key")?;
    let key_bytes = hex::decode(key_hex.trim())?;
    let key_pair = MySoKeyPair::from_bytes(&key_bytes)
        .map_err(|e| anyhow::anyhow!("invalid key: {:?}", e))?;
    let sender = MySoAddress::from(&key_pair.public());

    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;

    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let gas_obj = gas_coins
        .data
        .first()
        .context("sender has no gas coins")?;

    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let admin_cap = parse_object_id(args.admin_cap_object_id.as_ref().unwrap())?;
    let spot_config = parse_object_id(args.spot_config_object_id.as_ref().unwrap())?;
    let registry_id = parse_object_id(args.spot_registry_object_id.as_ref().unwrap())?;
    let clock = ObjectID::from_hex_literal(CLOCK_OBJECT_ID)?;

    let admin_obj = client
        .read_api()
        .get_object_with_options(admin_cap, MySoObjectDataOptions::default())
        .await?
        .into_object()?;
    let admin_arg = ObjectArg::ImmOrOwnedObject(admin_obj.object_ref());
    let config_arg =
        shared_object_arg(&client, spot_config, SharedObjectMutability::Immutable).await?;
    let registry_arg =
        shared_object_arg(&client, registry_id, SharedObjectMutability::Mutable).await?;
    let clock_arg = shared_object_arg(&client, clock, SharedObjectMutability::Immutable).await?;

    let claim_object_id = if let Some(existing) = existing_claim_object_id {
        existing.to_string()
    } else {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let admin_input = ptb.obj(admin_arg)?;
        let config_input = ptb.obj(config_arg)?;
        let registry_input = ptb.obj(registry_arg)?;
        let clock_input = ptb.obj(clock_arg)?;
        let hash_input = ptb.pure(semantic_claim_hash.to_vec())?;
        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package,
            module: "social_proof_of_truth".to_string(),
            function: "create_spot_claim".to_string(),
            type_arguments: vec![],
            arguments: vec![
                admin_input,
                config_input,
                registry_input,
                hash_input,
                clock_input,
            ],
        })));

        let pt = ptb.finish();
        let rgp = client.read_api().get_reference_gas_price().await?;
        let tx_data = TransactionData::new_programmable(
            sender,
            vec![gas_obj.object_ref()],
            pt,
            30_000_000,
            rgp,
        );
        let signed = Transaction::from_data_and_signer(tx_data, vec![&key_pair]);
        let response = client
            .quorum_driver_api()
            .execute_transaction_block(
                signed,
                MySoTransactionBlockResponseOptions::new().with_object_changes(),
                Some(ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await?;
        if response.status_ok() == Some(false) {
            let status = response
                .effects
                .as_ref()
                .map(|e| format!("{:?}", e.status()));
            anyhow::bail!(
                "create_spot_claim transaction failed (existing claim may already be registered): {status:?}"
            );
        }
        match find_created_type(&response.object_changes, "SpotClaim") {
            Some(id) => id,
            None => lookup_claim_object_id_by_semantic_hash(args, semantic_claim_hash)
                .await?
                .context(
                    "create_spot_claim did not create SpotClaim and no existing claim found on-chain",
                )?,
        }
    };

    // place_spot_bet_for_post requires registry.post_to_claim — link before opening market.
    build_and_submit_link_post(args, &claim_object_id, post_id).await?;

    let (digest, market_object_id) = build_and_submit_create_market(
        args,
        &claim_object_id,
        post_id,
        betting_options,
        resolution_at_ms,
        max_resolution_buffer_ms,
        market_key_hash,
    )
    .await?;

    Ok((digest, claim_object_id, market_object_id))
}

async fn build_and_submit_link_post(
    args: &crate::config::OracleArgs,
    claim_object_id: &str,
    post_id: &str,
) -> anyhow::Result<String> {
    let key_hex = args.private_key_hex.as_ref().context("missing private key")?;
    let key_bytes = hex::decode(key_hex.trim())?;
    let key_pair = MySoKeyPair::from_bytes(&key_bytes)
        .map_err(|e| anyhow::anyhow!("invalid key: {:?}", e))?;
    let sender = MySoAddress::from(&key_pair.public());

    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;

    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let gas_obj = gas_coins
        .data
        .first()
        .context("sender has no gas coins")?;

    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let admin_cap = parse_object_id(args.admin_cap_object_id.as_ref().unwrap())?;
    let spot_config = parse_object_id(args.spot_config_object_id.as_ref().unwrap())?;
    let registry_id = parse_object_id(args.spot_registry_object_id.as_ref().unwrap())?;
    let claim_obj = parse_object_id(claim_object_id)?;
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
    let registry_arg =
        shared_object_arg(&client, registry_id, SharedObjectMutability::Mutable).await?;
    let claim_arg =
        shared_object_arg(&client, claim_obj, SharedObjectMutability::Mutable).await?;
    let post_arg = shared_object_arg(&client, post_obj, SharedObjectMutability::Mutable).await?;
    let clock_arg = shared_object_arg(&client, clock, SharedObjectMutability::Immutable).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let admin_input = ptb.obj(admin_arg)?;
    let config_input = ptb.obj(config_arg)?;
    let registry_input = ptb.obj(registry_arg)?;
    let claim_input = ptb.obj(claim_arg)?;
    let post_input = ptb.obj(post_arg)?;
    // Single-claim baseline: hybrid referrers link at claim_index 0; the claim id bytes stand
    // in as the resolution policy binding until per-claim policy threading lands.
    let claim_index_input = ptb.pure(0u64)?;
    let policy_input = ptb.pure(claim_obj.into_bytes().to_vec())?;
    let clock_input = ptb.obj(clock_arg)?;

    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "social_proof_of_truth".to_string(),
        function: "link_post_to_spot_claim".to_string(),
        type_arguments: vec![],
        arguments: vec![
            admin_input,
            config_input,
            registry_input,
            claim_input,
            post_input,
            claim_index_input,
            policy_input,
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
            MySoTransactionBlockResponseOptions::new(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    Ok(response.digest.to_string())
}

fn find_created_type(changes: &Option<Vec<ObjectChange>>, type_name: &str) -> Option<String> {
    changes
        .as_ref()
        .into_iter()
        .flatten()
        .find_map(|change| match change {
            ObjectChange::Created {
                object_type,
                object_id,
                ..
            } if object_type.name.as_str() == type_name => {
                Some(object_id.to_canonical_string(true))
            }
            _ => None,
        })
}
