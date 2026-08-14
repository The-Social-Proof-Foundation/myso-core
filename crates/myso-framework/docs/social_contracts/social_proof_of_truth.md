---
title: Module `social_contracts::social_proof_of_truth`
---

Social Proof of Truth (SPoT)
Claim → Market → Post architecture. Semantic claims dedupe off-chain content; markets
hold escrow and resolution state; posts link to claims for attribution and creator fees.
Oracle/DAO resolves outcomes; winners and creators claim payouts after resolution.


-  [Struct `SpotAdminCap`](#social_contracts_social_proof_of_truth_SpotAdminCap)
-  [Struct `SpotOracleAdminCap`](#social_contracts_social_proof_of_truth_SpotOracleAdminCap)
-  [Struct `SpotConfig`](#social_contracts_social_proof_of_truth_SpotConfig)
-  [Struct `SpotPostLink`](#social_contracts_social_proof_of_truth_SpotPostLink)
-  [Struct `PostClaimIndexKey`](#social_contracts_social_proof_of_truth_PostClaimIndexKey)
-  [Struct `PostMarketKey`](#social_contracts_social_proof_of_truth_PostMarketKey)
-  [Struct `SpotClaim`](#social_contracts_social_proof_of_truth_SpotClaim)
-  [Struct `SpotClaimRegistry`](#social_contracts_social_proof_of_truth_SpotClaimRegistry)
-  [Struct `SpotCreatorPayout`](#social_contracts_social_proof_of_truth_SpotCreatorPayout)
-  [Struct `SpotMarket`](#social_contracts_social_proof_of_truth_SpotMarket)
-  [Struct `SpotBet`](#social_contracts_social_proof_of_truth_SpotBet)
-  [Struct `SpotBetPlacedEvent`](#social_contracts_social_proof_of_truth_SpotBetPlacedEvent)
-  [Struct `SpotResolvedEvent`](#social_contracts_social_proof_of_truth_SpotResolvedEvent)
-  [Struct `SpotDaoRequiredEvent`](#social_contracts_social_proof_of_truth_SpotDaoRequiredEvent)
-  [Struct `SpotGovernanceProposalLinkedEvent`](#social_contracts_social_proof_of_truth_SpotGovernanceProposalLinkedEvent)
-  [Struct `SpotGovernanceProposalClearedEvent`](#social_contracts_social_proof_of_truth_SpotGovernanceProposalClearedEvent)
-  [Struct `SpotPayoutEvent`](#social_contracts_social_proof_of_truth_SpotPayoutEvent)
-  [Struct `SpotCreatorPayoutAccruedEvent`](#social_contracts_social_proof_of_truth_SpotCreatorPayoutAccruedEvent)
-  [Struct `SpotCreatorPayoutClaimedEvent`](#social_contracts_social_proof_of_truth_SpotCreatorPayoutClaimedEvent)
-  [Struct `SpotCreatorPayoutReclaimedEvent`](#social_contracts_social_proof_of_truth_SpotCreatorPayoutReclaimedEvent)
-  [Struct `SpotRefundEvent`](#social_contracts_social_proof_of_truth_SpotRefundEvent)
-  [Struct `SpotConfigUpdatedEvent`](#social_contracts_social_proof_of_truth_SpotConfigUpdatedEvent)
-  [Struct `SpotBetWithdrawnEvent`](#social_contracts_social_proof_of_truth_SpotBetWithdrawnEvent)
-  [Struct `SpotClaimCreatedEvent`](#social_contracts_social_proof_of_truth_SpotClaimCreatedEvent)
-  [Struct `SpotMarketCreatedEvent`](#social_contracts_social_proof_of_truth_SpotMarketCreatedEvent)
-  [Struct `SpotPostLinkedEvent`](#social_contracts_social_proof_of_truth_SpotPostLinkedEvent)
-  [Struct `SpotClaimsFinalizedForPost`](#social_contracts_social_proof_of_truth_SpotClaimsFinalizedForPost)
-  [Constants](#@Constants_0)
-  [Function `get_status`](#social_contracts_social_proof_of_truth_get_status)
-  [Function `get_bets_len`](#social_contracts_social_proof_of_truth_get_bets_len)
-  [Function `get_betting_options`](#social_contracts_social_proof_of_truth_get_betting_options)
-  [Function `get_option_escrow`](#social_contracts_social_proof_of_truth_get_option_escrow)
-  [Function `get_id_address`](#social_contracts_social_proof_of_truth_get_id_address)
-  [Function `get_outcome`](#social_contracts_social_proof_of_truth_get_outcome)
-  [Function `is_open`](#social_contracts_social_proof_of_truth_is_open)
-  [Function `is_resolved`](#social_contracts_social_proof_of_truth_is_resolved)
-  [Function `outcome_draw`](#social_contracts_social_proof_of_truth_outcome_draw)
-  [Function `outcome_unapplicable`](#social_contracts_social_proof_of_truth_outcome_unapplicable)
-  [Function `get_user_option_amount`](#social_contracts_social_proof_of_truth_get_user_option_amount)
-  [Function `num_betting_options`](#social_contracts_social_proof_of_truth_num_betting_options)
-  [Function `total_option_escrow`](#social_contracts_social_proof_of_truth_total_option_escrow)
-  [Function `assert_valid_option_id`](#social_contracts_social_proof_of_truth_assert_valid_option_id)
-  [Function `claim_id`](#social_contracts_social_proof_of_truth_claim_id)
-  [Function `market_key_hash`](#social_contracts_social_proof_of_truth_market_key_hash)
-  [Function `primary_post_id`](#social_contracts_social_proof_of_truth_primary_post_id)
-  [Function `is_enabled`](#social_contracts_social_proof_of_truth_is_enabled)
-  [Function `max_claim_per_post`](#social_contracts_social_proof_of_truth_max_claim_per_post)
-  [Function `spot_governance_registry_id`](#social_contracts_social_proof_of_truth_spot_governance_registry_id)
-  [Function `active_proposal_id`](#social_contracts_social_proof_of_truth_active_proposal_id)
-  [Function `proposed_outcome`](#social_contracts_social_proof_of_truth_proposed_outcome)
-  [Function `oracle_proposed_outcome`](#social_contracts_social_proof_of_truth_oracle_proposed_outcome)
-  [Function `dao_escalated_at_ms`](#social_contracts_social_proof_of_truth_dao_escalated_at_ms)
-  [Function `semantic_claim_hash`](#social_contracts_social_proof_of_truth_semantic_claim_hash)
-  [Function `assert_valid_hash`](#social_contracts_social_proof_of_truth_assert_valid_hash)
-  [Function `new_spot_config`](#social_contracts_social_proof_of_truth_new_spot_config)
-  [Function `new_spot_claim_registry`](#social_contracts_social_proof_of_truth_new_spot_claim_registry)
-  [Function `emit_config_updated`](#social_contracts_social_proof_of_truth_emit_config_updated)
-  [Function `bootstrap_init`](#social_contracts_social_proof_of_truth_bootstrap_init)
-  [Function `create_spot_admin_cap`](#social_contracts_social_proof_of_truth_create_spot_admin_cap)
-  [Function `create_spot_oracle_admin_cap`](#social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap)
-  [Function `assert_config_version`](#social_contracts_social_proof_of_truth_assert_config_version)
-  [Function `assert_registry_version`](#social_contracts_social_proof_of_truth_assert_registry_version)
-  [Function `assert_claim_version`](#social_contracts_social_proof_of_truth_assert_claim_version)
-  [Function `assert_market_version`](#social_contracts_social_proof_of_truth_assert_market_version)
-  [Function `update_spot_config`](#social_contracts_social_proof_of_truth_update_spot_config)
-  [Function `rescale_spot_config_windows_from_epoch_counts`](#social_contracts_social_proof_of_truth_rescale_spot_config_windows_from_epoch_counts)
-  [Function `register_spot_claim`](#social_contracts_social_proof_of_truth_register_spot_claim)
-  [Function `create_spot_claim`](#social_contracts_social_proof_of_truth_create_spot_claim)
-  [Function `register_future_link`](#social_contracts_social_proof_of_truth_register_future_link)
-  [Function `create_spot_market_for_claim`](#social_contracts_social_proof_of_truth_create_spot_market_for_claim)
-  [Function `link_post_to_spot_claim`](#social_contracts_social_proof_of_truth_link_post_to_spot_claim)
-  [Function `finalize_spot_claims_for_post`](#social_contracts_social_proof_of_truth_finalize_spot_claims_for_post)
-  [Function `create_and_finalize_spot_market_for_post`](#social_contracts_social_proof_of_truth_create_and_finalize_spot_market_for_post)
-  [Function `assert_market_open_for_post`](#social_contracts_social_proof_of_truth_assert_market_open_for_post)
-  [Function `place_spot_bet_for_post`](#social_contracts_social_proof_of_truth_place_spot_bet_for_post)
-  [Function `place_spot_bet_internal`](#social_contracts_social_proof_of_truth_place_spot_bet_internal)
-  [Function `withdraw_spot_bet`](#social_contracts_social_proof_of_truth_withdraw_spot_bet)
-  [Function `oracle_resolve`](#social_contracts_social_proof_of_truth_oracle_resolve)
-  [Function `submit_spot_resolution_proposal_to_governance`](#social_contracts_social_proof_of_truth_submit_spot_resolution_proposal_to_governance)
-  [Function `implement_spot_resolution_from_governance`](#social_contracts_social_proof_of_truth_implement_spot_resolution_from_governance)
-  [Function `clear_spot_proposal_link_on_reject`](#social_contracts_social_proof_of_truth_clear_spot_proposal_link_on_reject)
-  [Function `finalize_spot_governance_proposal`](#social_contracts_social_proof_of_truth_finalize_spot_governance_proposal)
-  [Function `finalize_via_dao`](#social_contracts_social_proof_of_truth_finalize_via_dao)
-  [Function `assert_spot_governance_registry`](#social_contracts_social_proof_of_truth_assert_spot_governance_registry)
-  [Function `validate_proposed_outcome`](#social_contracts_social_proof_of_truth_validate_proposed_outcome)
-  [Function `refund_unresolved`](#social_contracts_social_proof_of_truth_refund_unresolved)
-  [Function `referrer_post_id_for_bet`](#social_contracts_social_proof_of_truth_referrer_post_id_for_bet)
-  [Function `track_creator_payout_index`](#social_contracts_social_proof_of_truth_track_creator_payout_index)
-  [Function `untrack_creator_payout_index`](#social_contracts_social_proof_of_truth_untrack_creator_payout_index)
-  [Function `creator_for_referrer_post`](#social_contracts_social_proof_of_truth_creator_for_referrer_post)
-  [Function `vector_contains_address`](#social_contracts_social_proof_of_truth_vector_contains_address)
-  [Function `referred_volume_for_post`](#social_contracts_social_proof_of_truth_referred_volume_for_post)
-  [Function `total_referred_volume`](#social_contracts_social_proof_of_truth_total_referred_volume)
-  [Function `accrue_creator_payouts`](#social_contracts_social_proof_of_truth_accrue_creator_payouts)
-  [Function `finalize_resolution_and_payout`](#social_contracts_social_proof_of_truth_finalize_resolution_and_payout)
-  [Function `claim_payout`](#social_contracts_social_proof_of_truth_claim_payout)
-  [Function `claim_creator_payout`](#social_contracts_social_proof_of_truth_claim_creator_payout)
-  [Function `reclaim_expired_creator_rewards`](#social_contracts_social_proof_of_truth_reclaim_expired_creator_rewards)
-  [Function `migrate_config`](#social_contracts_social_proof_of_truth_migrate_config)
-  [Function `migrate_claim_registry`](#social_contracts_social_proof_of_truth_migrate_claim_registry)
-  [Function `migrate_claim`](#social_contracts_social_proof_of_truth_migrate_claim)
-  [Function `migrate_market`](#social_contracts_social_proof_of_truth_migrate_market)
-  [Function `migrate_record`](#social_contracts_social_proof_of_truth_migrate_record)


<pre><code><b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
<b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
<b>use</b> <a href="../mydata/merkle.md#mydata_merkle">mydata::merkle</a>;
<b>use</b> <a href="../mydata/polynomial.md#mydata_polynomial">mydata::polynomial</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bls12381.md#myso_bls12381">myso::bls12381</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/ed25519.md#myso_ed25519">myso::ed25519</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/hmac.md#myso_hmac">myso::hmac</a>;
<b>use</b> <a href="../myso/myso.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/permissioned_group.md#myso_permissioned_group">myso::permissioned_group</a>;
<b>use</b> <a href="../myso/permissions_table.md#myso_permissions_table">myso::permissions_table</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/unpause_cap.md#myso_unpause_cap">myso::unpause_cap</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit">social_contracts::ai_credit</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph">social_contracts::derivative_graph</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_license_template">social_contracts::license_template</a>;
<b>use</b> <a href="../social_contracts/media_asset.md#social_contracts_media_asset">social_contracts::media_asset</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">social_contracts::mydata</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault">social_contracts::poc_vault</a>;
<b>use</b> <a href="../social_contracts/post.md#social_contracts_post">social_contracts::post</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">social_contracts::subscription</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_social_proof_of_truth_SpotAdminCap"></a>

## Struct `SpotAdminCap`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotOracleAdminCap"></a>

## Struct `SpotOracleAdminCap`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotConfig"></a>

## Struct `SpotConfig`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>truth_enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>confidence_threshold_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_resolution_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>payout_delay_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_claim_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expired_creator_ecosystem_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_betting_options: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_betting_options: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_reasoning_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_reasoning_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_evidence_urls: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>oracle_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>max_single_bet: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_bets_per_record: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotPostLink"></a>

## Struct `SpotPostLink`

Post linked to a semantic claim at a specific claim index (creator stored for fee routing).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPostLink">SpotPostLink</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>claim_index: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_PostClaimIndexKey"></a>

## Struct `PostClaimIndexKey`

Registry key: a post's future-claim link at a given claim index.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_PostClaimIndexKey">PostClaimIndexKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>claim_index: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_PostMarketKey"></a>

## Struct `PostMarketKey`

Registry key: a (post, market) future-link — authoritative bet-eligibility check.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_PostMarketKey">PostMarketKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotClaim"></a>

## Struct `SpotClaim`

Semantic claim object — deduped by <code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>linked_posts: vector&lt;<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPostLink">social_contracts::social_proof_of_truth::SpotPostLink</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotClaimRegistry"></a>

## Struct `SpotClaimRegistry`

Shared registry mapping hashes and open markets. Multi-claim: a post may hold
several future-claim links keyed by claim index / market.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>claims_by_semantic_hash: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;vector&lt;u8&gt;, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>markets_by_key_hash: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;vector&lt;u8&gt;, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>open_market_by_claim: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>post_claim_index_to_market: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_PostClaimIndexKey">social_contracts::social_proof_of_truth::PostClaimIndexKey</a>, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>post_market_to_claim: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_PostMarketKey">social_contracts::social_proof_of_truth::PostMarketKey</a>, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotCreatorPayout"></a>

## Struct `SpotCreatorPayout`

Pending creator payout (O(1) claim by <code>payout_id</code>).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotCreatorPayout">SpotCreatorPayout</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>source_post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotMarket"></a>

## Struct `SpotMarket`

Prediction market for a claim (evolved from per-post SpotRecord).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>primary_creator: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>outcome: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>escrow: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>betting_options: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>option_escrow: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u8, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>user_option_amounts: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, vector&lt;u64&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>bets: vector&lt;<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">social_contracts::social_proof_of_truth::SpotBet</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_window_ms: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>max_resolution_window_ms: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>last_resolution_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>pending_payouts: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>pending_creator_payouts: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u64, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotCreatorPayout">social_contracts::social_proof_of_truth::SpotCreatorPayout</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>next_creator_payout_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_payout_index: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, vector&lt;u64&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_proposed_outcome">oracle_proposed_outcome</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_dao_escalated_at_ms">dao_escalated_at_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotBet"></a>

## Struct `SpotBet`

A single bet


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">SpotBet</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>referrer_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotBetPlacedEvent"></a>

## Struct `SpotBetPlacedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBetPlacedEvent">SpotBetPlacedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>referrer_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotResolvedEvent"></a>

## Struct `SpotResolvedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotResolvedEvent">SpotResolvedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>outcome: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>total_escrow: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_taken: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_fee_total: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reasoning: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_urls: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotDaoRequiredEvent"></a>

## Struct `SpotDaoRequiredEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotDaoRequiredEvent">SpotDaoRequiredEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>spot_record_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>confidence_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_proposed_outcome">oracle_proposed_outcome</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_dao_escalated_at_ms">dao_escalated_at_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reasoning: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotGovernanceProposalLinkedEvent"></a>

## Struct `SpotGovernanceProposalLinkedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotGovernanceProposalLinkedEvent">SpotGovernanceProposalLinkedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>spot_record_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotGovernanceProposalClearedEvent"></a>

## Struct `SpotGovernanceProposalClearedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotGovernanceProposalClearedEvent">SpotGovernanceProposalClearedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>spot_record_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotPayoutEvent"></a>

## Struct `SpotPayoutEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPayoutEvent">SpotPayoutEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotCreatorPayoutAccruedEvent"></a>

## Struct `SpotCreatorPayoutAccruedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotCreatorPayoutAccruedEvent">SpotCreatorPayoutAccruedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payout_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>referrer_post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotCreatorPayoutClaimedEvent"></a>

## Struct `SpotCreatorPayoutClaimedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotCreatorPayoutClaimedEvent">SpotCreatorPayoutClaimedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payout_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotCreatorPayoutReclaimedEvent"></a>

## Struct `SpotCreatorPayoutReclaimedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotCreatorPayoutReclaimedEvent">SpotCreatorPayoutReclaimedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payout_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotRefundEvent"></a>

## Struct `SpotRefundEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRefundEvent">SpotRefundEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotConfigUpdatedEvent"></a>

## Struct `SpotConfigUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfigUpdatedEvent">SpotConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>truth_enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>confidence_threshold_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_resolution_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>payout_delay_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_claim_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expired_creator_ecosystem_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_betting_options: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_betting_options: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_reasoning_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_reasoning_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_evidence_urls: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>oracle_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>max_single_bet: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_bets_per_record: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotBetWithdrawnEvent"></a>

## Struct `SpotBetWithdrawnEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBetWithdrawnEvent">SpotBetWithdrawnEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_taken: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotClaimCreatedEvent"></a>

## Struct `SpotClaimCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimCreatedEvent">SpotClaimCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotMarketCreatedEvent"></a>

## Struct `SpotMarketCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarketCreatedEvent">SpotMarketCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>market_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>claim_index: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_policy_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>betting_options: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>resolution_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_resolution_window_ms: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotPostLinkedEvent"></a>

## Struct `SpotPostLinkedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPostLinkedEvent">SpotPostLinkedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>market_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_index: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>policy_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_social_proof_of_truth_SpotClaimsFinalizedForPost"></a>

## Struct `SpotClaimsFinalizedForPost`

Batch finalize projection for a post's multi-claim analysis. Carries future-link
arrays (claim_index order) plus parallel past-verdict vectors for the indexer.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimsFinalizedForPost">SpotClaimsFinalizedForPost</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>detected_claim_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>rejected_claim_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>truncated_claim_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>future_accepted_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>past_verified_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_claim_per_post_applied: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_manifest_hash: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>veracity_manifest_hash: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>future_claim_indexes: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>future_claim_ids: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>future_market_ids: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>past_claim_indexes: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>past_verdicts: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>past_related_market_ids: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>past_evidence_hashes: vector&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>finalized_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_social_proof_of_truth_EDisabled"></a>

Errors


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>: u64 = 1;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EInvalidAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>: u64 = 2;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EAlreadyResolved"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EAlreadyResolved">EAlreadyResolved</a>: u64 = 3;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ETooEarly"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>: u64 = 4;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ETooClose"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooClose">ETooClose</a>: u64 = 5;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EWrongStatus"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>: u64 = 6;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENotOracle"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotOracle">ENotOracle</a>: u64 = 7;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENoBets"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoBets">ENoBets</a>: u64 = 8;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EOverflow"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EOverflow">EOverflow</a>: u64 = 9;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EInvalidReasoning"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EInvalidOptionId"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidOptionId">EInvalidOptionId</a>: u64 = 11;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EWithdrawalNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWithdrawalNotAllowed">EWithdrawalNotAllowed</a>: u64 = 12;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EBetNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EBetNotFound">EBetNotFound</a>: u64 = 13;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EAlreadyInitialized"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EAlreadyInitialized">EAlreadyInitialized</a>: u64 = 14;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EDuplicateOption"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDuplicateOption">EDuplicateOption</a>: u64 = 15;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ETooManyBets"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooManyBets">ETooManyBets</a>: u64 = 16;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongVersion">EWrongVersion</a>: u64 = 17;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EActiveProposalExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EActiveProposalExists">EActiveProposalExists</a>: u64 = 18;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENoActiveProposal"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoActiveProposal">ENoActiveProposal</a>: u64 = 19;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EWrongProposal"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongProposal">EWrongProposal</a>: u64 = 20;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENotDaoRequired"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotDaoRequired">ENotDaoRequired</a>: u64 = 21;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EDaoDebateFrozen"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDaoDebateFrozen">EDaoDebateFrozen</a>: u64 = 22;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EInvalidGovernanceRegistry"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidGovernanceRegistry">EInvalidGovernanceRegistry</a>: u64 = 23;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EProposalNotApproved"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EProposalNotApproved">EProposalNotApproved</a>: u64 = 24;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EClaimExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EClaimExists">EClaimExists</a>: u64 = 25;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EClaimNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EClaimNotFound">EClaimNotFound</a>: u64 = 26;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EMarketExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EMarketExists">EMarketExists</a>: u64 = 27;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EMarketNotOpen"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EMarketNotOpen">EMarketNotOpen</a>: u64 = 28;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EPostNotLinked"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPostNotLinked">EPostNotLinked</a>: u64 = 29;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EPayoutNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPayoutNotFound">EPayoutNotFound</a>: u64 = 30;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENotCreator"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotCreator">ENotCreator</a>: u64 = 31;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ECreatorPayoutExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ECreatorPayoutExpired">ECreatorPayoutExpired</a>: u64 = 32;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EInvalidHash"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidHash">EInvalidHash</a>: u64 = 33;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENotFinalized"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotFinalized">ENotFinalized</a>: u64 = 34;
</code></pre>



<a name="social_contracts_social_proof_of_truth_EPastVerdictMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPastVerdictMismatch">EPastVerdictMismatch</a>: u64 = 35;
</code></pre>



<a name="social_contracts_social_proof_of_truth_ENoOpenMarket"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoOpenMarket">ENoOpenMarket</a>: u64 = 36;
</code></pre>



<a name="social_contracts_social_proof_of_truth_STATUS_OPEN"></a>

Status


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>: u8 = 1;
</code></pre>



<a name="social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_social_proof_of_truth_STATUS_RESOLVED"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_social_proof_of_truth_STATUS_REFUNDABLE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_REFUNDABLE">STATUS_REFUNDABLE</a>: u8 = 4;
</code></pre>



<a name="social_contracts_social_proof_of_truth_OUTCOME_DRAW"></a>

Outcomes


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_DRAW">OUTCOME_DRAW</a>: u8 = 255;
</code></pre>



<a name="social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE">OUTCOME_UNAPPLICABLE</a>: u8 = 254;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MS_PER_DAY"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MS_PER_DAY">MS_PER_DAY</a>: u64 = 86400000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_CONFIDENCE_THRESHOLD_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CONFIDENCE_THRESHOLD_BPS">DEFAULT_CONFIDENCE_THRESHOLD_BPS</a>: u64 = 7000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_ENABLE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_ENABLE">DEFAULT_ENABLE</a>: bool = <b>false</b>;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_RESOLUTION_WINDOW_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_RESOLUTION_WINDOW_MS">DEFAULT_RESOLUTION_WINDOW_MS</a>: u64 = 6220800000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_MAX_RESOLUTION_WINDOW_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_RESOLUTION_WINDOW_MS">DEFAULT_MAX_RESOLUTION_WINDOW_MS</a>: u64 = 12441600000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_PAYOUT_DELAY_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_PAYOUT_DELAY_MS">DEFAULT_PAYOUT_DELAY_MS</a>: u64 = 0;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>: u64 = 50;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_ECOSYSTEM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>: u64 = 50;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_CREATOR_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CREATOR_FEE_BPS">DEFAULT_CREATOR_FEE_BPS</a>: u64 = 100;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_CREATOR_CLAIM_WINDOW_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CREATOR_CLAIM_WINDOW_MS">DEFAULT_CREATOR_CLAIM_WINDOW_MS</a>: u64 = 2592000000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS">DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_MIN_BETTING_OPTIONS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MIN_BETTING_OPTIONS">DEFAULT_MIN_BETTING_OPTIONS</a>: u64 = 2;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_MAX_BETTING_OPTIONS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_BETTING_OPTIONS">DEFAULT_MAX_BETTING_OPTIONS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_MIN_REASONING_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MIN_REASONING_LENGTH">DEFAULT_MIN_REASONING_LENGTH</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_MAX_REASONING_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_REASONING_LENGTH">DEFAULT_MAX_REASONING_LENGTH</a>: u64 = 5000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_MAX_EVIDENCE_URLS"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_EVIDENCE_URLS">DEFAULT_MAX_EVIDENCE_URLS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_MAX_BETS_PER_RECORD"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_BETS_PER_RECORD">DEFAULT_MAX_BETS_PER_RECORD</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_social_proof_of_truth_DEFAULT_MAX_CLAIM_PER_POST"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_CLAIM_PER_POST">DEFAULT_MAX_CLAIM_PER_POST</a>: u64 = 10;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MIN_MAX_CLAIM_PER_POST"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MIN_MAX_CLAIM_PER_POST">MIN_MAX_CLAIM_PER_POST</a>: u64 = 1;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MAX_MAX_CLAIM_PER_POST"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_MAX_CLAIM_PER_POST">MAX_MAX_CLAIM_PER_POST</a>: u64 = 20;
</code></pre>



<a name="social_contracts_social_proof_of_truth_VERDICT_TRUE"></a>

Past-claim verdict values (mirror indexer/GraphQL): 1=true, 2=false, 3=unverifiable.


<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_VERDICT_TRUE">VERDICT_TRUE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_social_proof_of_truth_VERDICT_FALSE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_VERDICT_FALSE">VERDICT_FALSE</a>: u8 = 2;
</code></pre>



<a name="social_contracts_social_proof_of_truth_VERDICT_UNVERIFIABLE"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_VERDICT_UNVERIFIABLE">VERDICT_UNVERIFIABLE</a>: u8 = 3;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MAX_U64"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_social_proof_of_truth_MIN_HASH_LEN"></a>



<pre><code><b>const</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MIN_HASH_LEN">MIN_HASH_LEN</a>: u64 = 8;
</code></pre>



<a name="social_contracts_social_proof_of_truth_get_status"></a>

## Function `get_status`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_status">get_status</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_status">get_status</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): u8 { market.status }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_bets_len"></a>

## Function `get_bets_len`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_bets_len">get_bets_len</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_bets_len">get_bets_len</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): u64 { vector::length(&market.bets) }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_betting_options"></a>

## Function `get_betting_options`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_betting_options">get_betting_options</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_betting_options">get_betting_options</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): vector&lt;String&gt; { market.betting_options }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_option_escrow"></a>

## Function `get_option_escrow`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_option_escrow">get_option_escrow</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, option_id: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_option_escrow">get_option_escrow</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>, option_id: u8): u64 {
    <b>if</b> (table::contains(&market.option_escrow, option_id)) {
        *table::borrow(&market.option_escrow, option_id)
    } <b>else</b> { 0 }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_id_address"></a>

## Function `get_id_address`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_id_address">get_id_address</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_id_address">get_id_address</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): <b>address</b> {
    object::uid_to_address(&market.id)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_outcome"></a>

## Function `get_outcome`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_outcome">get_outcome</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_outcome">get_outcome</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): &Option&lt;u8&gt; { &market.outcome }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_is_open"></a>

## Function `is_open`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_open">is_open</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_open">is_open</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): bool { market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_is_resolved"></a>

## Function `is_resolved`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_resolved">is_resolved</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_resolved">is_resolved</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): bool { market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_outcome_draw"></a>

## Function `outcome_draw`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_outcome_draw">outcome_draw</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_outcome_draw">outcome_draw</a>(): u8 { <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_DRAW">OUTCOME_DRAW</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_outcome_unapplicable"></a>

## Function `outcome_unapplicable`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_outcome_unapplicable">outcome_unapplicable</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_outcome_unapplicable">outcome_unapplicable</a>(): u8 { <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE">OUTCOME_UNAPPLICABLE</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_get_user_option_amount"></a>

## Function `get_user_option_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_user_option_amount">get_user_option_amount</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, user: <b>address</b>, option_id: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_user_option_amount">get_user_option_amount</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>, user: <b>address</b>, option_id: u8): u64 {
    <b>if</b> (!table::contains(&market.user_option_amounts, user)) {
        0
    } <b>else</b> {
        <b>let</b> amounts = table::borrow(&market.user_option_amounts, user);
        <b>let</b> idx = option_id <b>as</b> u64;
        <b>if</b> (idx &gt;= vector::length(amounts)) { 0 } <b>else</b> { *vector::borrow(amounts, idx) }
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_num_betting_options"></a>

## Function `num_betting_options`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_num_betting_options">num_betting_options</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_num_betting_options">num_betting_options</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): u64 {
    vector::length(&market.betting_options)
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_total_option_escrow"></a>

## Function `total_option_escrow`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_total_option_escrow">total_option_escrow</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_total_option_escrow">total_option_escrow</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): u64 {
    <b>let</b> <b>mut</b> total = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> n = vector::length(&market.betting_options);
    <b>while</b> (i &lt; n) {
        <b>let</b> option_id = (i <b>as</b> u8);
        <b>let</b> amt = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_option_escrow">get_option_escrow</a>(market, option_id);
        <b>assert</b>!(total &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_U64">MAX_U64</a> - amt, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EOverflow">EOverflow</a>);
        total = total + amt;
        i = i + 1;
    };
    total
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_assert_valid_option_id"></a>

## Function `assert_valid_option_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_valid_option_id">assert_valid_option_id</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, option_id: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_valid_option_id">assert_valid_option_id</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>, option_id: u8) {
    <b>assert</b>!((option_id <b>as</b> u64) &lt; vector::length(&market.betting_options), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidOptionId">EInvalidOptionId</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_claim_id"></a>

## Function `claim_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): <b>address</b> { market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_market_key_hash"></a>

## Function `market_key_hash`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): vector&lt;u8&gt; { market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_primary_post_id"></a>

## Function `primary_post_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): <b>address</b> { market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_is_enabled"></a>

## Function `is_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_enabled">is_enabled</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_is_enabled">is_enabled</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>): bool { config.truth_enabled }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_max_claim_per_post"></a>

## Function `max_claim_per_post`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>): u64 { config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a> }
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_spot_governance_registry_id"></a>

## Function `spot_governance_registry_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>): ID {
    config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_active_proposal_id"></a>

## Function `active_proposal_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): &Option&lt;ID&gt; {
    &market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_proposed_outcome"></a>

## Function `proposed_outcome`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): &Option&lt;u8&gt; {
    &market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_oracle_proposed_outcome"></a>

## Function `oracle_proposed_outcome`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_proposed_outcome">oracle_proposed_outcome</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_proposed_outcome">oracle_proposed_outcome</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): &Option&lt;u8&gt; {
    &market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_proposed_outcome">oracle_proposed_outcome</a>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_dao_escalated_at_ms"></a>

## Function `dao_escalated_at_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_dao_escalated_at_ms">dao_escalated_at_ms</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_dao_escalated_at_ms">dao_escalated_at_ms</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): u64 {
    market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_dao_escalated_at_ms">dao_escalated_at_ms</a>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_semantic_claim_hash"></a>

## Function `semantic_claim_hash`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>(claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>(claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>): vector&lt;u8&gt; {
    claim.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_assert_valid_hash"></a>

## Function `assert_valid_hash`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_valid_hash">assert_valid_hash</a>(hash: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_valid_hash">assert_valid_hash</a>(hash: &vector&lt;u8&gt;) {
    <b>assert</b>!(vector::length(hash) &gt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MIN_HASH_LEN">MIN_HASH_LEN</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidHash">EInvalidHash</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_new_spot_config"></a>

## Function `new_spot_config`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_new_spot_config">new_spot_config</a>(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_new_spot_config">new_spot_config</a>(
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>: ID,
    ctx: &<b>mut</b> TxContext,
): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a> {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a> {
        id: object::new(ctx),
        truth_enabled: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_ENABLE">DEFAULT_ENABLE</a>,
        confidence_threshold_bps: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CONFIDENCE_THRESHOLD_BPS">DEFAULT_CONFIDENCE_THRESHOLD_BPS</a>,
        resolution_window_ms: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_RESOLUTION_WINDOW_MS">DEFAULT_RESOLUTION_WINDOW_MS</a>,
        max_resolution_window_ms: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_RESOLUTION_WINDOW_MS">DEFAULT_MAX_RESOLUTION_WINDOW_MS</a>,
        payout_delay_ms: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_PAYOUT_DELAY_MS">DEFAULT_PAYOUT_DELAY_MS</a>,
        platform_fee_bps: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>,
        ecosystem_fee_bps: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>,
        creator_fee_bps: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CREATOR_FEE_BPS">DEFAULT_CREATOR_FEE_BPS</a>,
        creator_claim_window_ms: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CREATOR_CLAIM_WINDOW_MS">DEFAULT_CREATOR_CLAIM_WINDOW_MS</a>,
        expired_creator_ecosystem_bps: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS">DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS</a>,
        min_betting_options: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MIN_BETTING_OPTIONS">DEFAULT_MIN_BETTING_OPTIONS</a>,
        max_betting_options: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_BETTING_OPTIONS">DEFAULT_MAX_BETTING_OPTIONS</a>,
        min_reasoning_length: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MIN_REASONING_LENGTH">DEFAULT_MIN_REASONING_LENGTH</a>,
        max_reasoning_length: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_REASONING_LENGTH">DEFAULT_MAX_REASONING_LENGTH</a>,
        max_evidence_urls: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_EVIDENCE_URLS">DEFAULT_MAX_EVIDENCE_URLS</a>,
        oracle_address: tx_context::sender(ctx),
        max_single_bet: 0,
        max_bets_per_record: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_BETS_PER_RECORD">DEFAULT_MAX_BETS_PER_RECORD</a>,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_CLAIM_PER_POST">DEFAULT_MAX_CLAIM_PER_POST</a>,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_new_spot_claim_registry"></a>

## Function `new_spot_claim_registry`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_new_spot_claim_registry">new_spot_claim_registry</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_new_spot_claim_registry">new_spot_claim_registry</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a> {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a> {
        id: object::new(ctx),
        claims_by_semantic_hash: table::new(ctx),
        markets_by_key_hash: table::new(ctx),
        open_market_by_claim: table::new(ctx),
        post_claim_index_to_market: table::new(ctx),
        post_market_to_claim: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_emit_config_updated"></a>

## Function `emit_config_updated`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_emit_config_updated">emit_config_updated</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_emit_config_updated">emit_config_updated</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>, clock: &Clock, ctx: &TxContext) {
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfigUpdatedEvent">SpotConfigUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        truth_enabled: config.truth_enabled,
        confidence_threshold_bps: config.confidence_threshold_bps,
        resolution_window_ms: config.resolution_window_ms,
        max_resolution_window_ms: config.max_resolution_window_ms,
        payout_delay_ms: config.payout_delay_ms,
        platform_fee_bps: config.platform_fee_bps,
        ecosystem_fee_bps: config.ecosystem_fee_bps,
        creator_fee_bps: config.creator_fee_bps,
        creator_claim_window_ms: config.creator_claim_window_ms,
        expired_creator_ecosystem_bps: config.expired_creator_ecosystem_bps,
        min_betting_options: config.min_betting_options,
        max_betting_options: config.max_betting_options,
        min_reasoning_length: config.min_reasoning_length,
        max_reasoning_length: config.max_reasoning_length,
        max_evidence_urls: config.max_evidence_urls,
        oracle_address: config.oracle_address,
        max_single_bet: config.max_single_bet,
        max_bets_per_record: config.max_bets_per_record,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>: config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>: config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>,
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_bootstrap_init">bootstrap_init</a>(clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_bootstrap_init">bootstrap_init</a>(
    clock: &Clock,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> config = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_new_spot_config">new_spot_config</a>(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>, ctx);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_emit_config_updated">emit_config_updated</a>(&config, clock, ctx);
    transfer::share_object(config);
    transfer::share_object(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_new_spot_claim_registry">new_spot_claim_registry</a>(ctx));
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_create_spot_admin_cap"></a>

## Function `create_spot_admin_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_admin_cap">create_spot_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">social_contracts::social_proof_of_truth::SpotAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_admin_cap">create_spot_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a> {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap"></a>

## Function `create_spot_oracle_admin_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap">create_spot_oracle_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap">create_spot_oracle_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a> {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_assert_config_version"></a>

## Function `assert_config_version`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_assert_registry_version"></a>

## Function `assert_registry_version`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(registry: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(registry: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>) {
    <b>assert</b>!(registry.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_assert_claim_version"></a>

## Function `assert_claim_version`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_claim_version">assert_claim_version</a>(claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_claim_version">assert_claim_version</a>(claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>) {
    <b>assert</b>!(claim.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_assert_market_version"></a>

## Function `assert_market_version`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>) {
    <b>assert</b>!(market.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_update_spot_config"></a>

## Function `update_spot_config`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_update_spot_config">update_spot_config</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">social_contracts::social_proof_of_truth::SpotAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, truth_enabled: bool, confidence_threshold_bps: u64, resolution_window_ms: u64, max_resolution_window_ms: u64, payout_delay_ms: u64, platform_fee_bps: u64, ecosystem_fee_bps: u64, creator_fee_bps: u64, creator_claim_window_ms: u64, expired_creator_ecosystem_bps: u64, min_betting_options: u64, max_betting_options: u64, min_reasoning_length: u64, max_reasoning_length: u64, max_evidence_urls: u64, oracle_address: <b>address</b>, max_single_bet: u64, max_bets_per_record: u64, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>: u64, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_update_spot_config">update_spot_config</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    truth_enabled: bool,
    confidence_threshold_bps: u64,
    resolution_window_ms: u64,
    max_resolution_window_ms: u64,
    payout_delay_ms: u64,
    platform_fee_bps: u64,
    ecosystem_fee_bps: u64,
    creator_fee_bps: u64,
    creator_claim_window_ms: u64,
    expired_creator_ecosystem_bps: u64,
    min_betting_options: u64,
    max_betting_options: u64,
    min_reasoning_length: u64,
    max_reasoning_length: u64,
    max_evidence_urls: u64,
    oracle_address: <b>address</b>,
    max_single_bet: u64,
    max_bets_per_record: u64,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>: u64,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>: ID,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(config);
    <b>assert</b>!(confidence_threshold_bps &lt;= 10000, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a> &gt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MIN_MAX_CLAIM_PER_POST">MIN_MAX_CLAIM_PER_POST</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a> &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_MAX_CLAIM_PER_POST">MAX_MAX_CLAIM_PER_POST</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(platform_fee_bps &lt;= 10000, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(ecosystem_fee_bps &lt;= 10000, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(creator_fee_bps &lt;= 10000, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(expired_creator_ecosystem_bps &lt;= 10000, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(platform_fee_bps + ecosystem_fee_bps + creator_fee_bps &lt;= 10000, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(min_betting_options &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(min_betting_options &lt;= max_betting_options, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(min_reasoning_length &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>);
    <b>assert</b>!(min_reasoning_length &lt;= max_reasoning_length, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>);
    <b>assert</b>!(max_evidence_urls &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    config.truth_enabled = truth_enabled;
    config.confidence_threshold_bps = confidence_threshold_bps;
    config.resolution_window_ms = resolution_window_ms;
    config.max_resolution_window_ms = max_resolution_window_ms;
    config.payout_delay_ms = payout_delay_ms;
    config.platform_fee_bps = platform_fee_bps;
    config.ecosystem_fee_bps = ecosystem_fee_bps;
    config.creator_fee_bps = creator_fee_bps;
    config.creator_claim_window_ms = creator_claim_window_ms;
    config.expired_creator_ecosystem_bps = expired_creator_ecosystem_bps;
    config.min_betting_options = min_betting_options;
    config.max_betting_options = max_betting_options;
    config.min_reasoning_length = min_reasoning_length;
    config.max_reasoning_length = max_reasoning_length;
    config.max_evidence_urls = max_evidence_urls;
    config.oracle_address = oracle_address;
    config.max_single_bet = max_single_bet;
    config.max_bets_per_record = max_bets_per_record;
    config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a> = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>;
    config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a> = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>;
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_emit_config_updated">emit_config_updated</a>(config, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_rescale_spot_config_windows_from_epoch_counts"></a>

## Function `rescale_spot_config_windows_from_epoch_counts`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_rescale_spot_config_windows_from_epoch_counts">rescale_spot_config_windows_from_epoch_counts</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">social_contracts::social_proof_of_truth::SpotAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, epoch_duration_ms: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_rescale_spot_config_windows_from_epoch_counts">rescale_spot_config_windows_from_epoch_counts</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotAdminCap">SpotAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    epoch_duration_ms: u64,
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(config);
    <b>assert</b>!(epoch_duration_ms &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    config.resolution_window_ms = config.resolution_window_ms * epoch_duration_ms;
    config.max_resolution_window_ms = config.max_resolution_window_ms * epoch_duration_ms;
    config.creator_claim_window_ms = config.creator_claim_window_ms * epoch_duration_ms;
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_register_spot_claim"></a>

## Function `register_spot_claim`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_register_spot_claim">register_spot_claim</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>: vector&lt;u8&gt;, created_at_ms: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_register_spot_claim">register_spot_claim</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>: vector&lt;u8&gt;,
    created_at_ms: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a> {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_valid_hash">assert_valid_hash</a>(&<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>);
    <b>assert</b>!(!table::contains(&registry.claims_by_semantic_hash, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EClaimExists">EClaimExists</a>);
    <b>let</b> claim = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a> {
        id: object::new(ctx),
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>,
        created_at_ms,
        linked_posts: vector::empty(),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a> = object::uid_to_address(&claim.id);
    table::add(&<b>mut</b> registry.claims_by_semantic_hash, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>);
    claim
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_create_spot_claim"></a>

## Function `create_spot_claim`

Oracle-only: register a semantic claim (deduped by hash).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_claim">create_spot_claim</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_claim">create_spot_claim</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(registry);
    <b>assert</b>!(config.truth_enabled, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>let</b> created_at_ms = clock::timestamp_ms(clock);
    <b>let</b> claim = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_register_spot_claim">register_spot_claim</a>(registry, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>, created_at_ms, ctx);
    <b>let</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a> = object::uid_to_address(&claim.id);
    <b>let</b> hash_copy = claim.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>;
    transfer::share_object(claim);
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimCreatedEvent">SpotClaimCreatedEvent</a> {
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>: hash_copy,
        created_at_ms,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_register_future_link"></a>

## Function `register_future_link`

Record a future-claim link: registers <code>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, claim_index)</code> and <code>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, market)</code>,
pushes a <code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPostLink">SpotPostLink</a></code>, and appends to the post's pending analysis vectors.


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_register_future_link">register_future_link</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, claim: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, market_id: <b>address</b>, claim_index: u64, resolution_policy_hash: vector&lt;u8&gt;, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_register_future_link">register_future_link</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    claim: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> Post,
    market_id: <b>address</b>,
    claim_index: u64,
    resolution_policy_hash: vector&lt;u8&gt;,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>: u64,
) {
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a> = object::uid_to_address(&claim.id);
    <b>let</b> idx_key = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_PostClaimIndexKey">PostClaimIndexKey</a> { post_id, claim_index };
    <b>assert</b>!(!table::contains(&registry.post_claim_index_to_market, idx_key), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EClaimExists">EClaimExists</a>);
    <b>let</b> creator = <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">post::get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    vector::push_back(&<b>mut</b> claim.linked_posts, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPostLink">SpotPostLink</a> { post_id, creator, claim_index });
    table::add(&<b>mut</b> registry.post_claim_index_to_market, idx_key, market_id);
    table::add(&<b>mut</b> registry.post_market_to_claim, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_PostMarketKey">PostMarketKey</a> { post_id, market_id }, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>);
    <a href="../social_contracts/post.md#social_contracts_post_ensure_spot_analysis_pending">post::ensure_spot_analysis_pending</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>);
    <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_append_future">post::spot_analysis_append_future</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>, claim_index, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>, market_id, resolution_policy_hash);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_create_spot_market_for_claim"></a>

## Function `create_spot_market_for_claim`

Oracle-only: open a market for an existing claim, linking <code>primary_post</code> at <code>claim_index</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_market_for_claim">create_spot_market_for_claim</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, claim: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, primary_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, claim_index: u64, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>: vector&lt;u8&gt;, resolution_policy_hash: vector&lt;u8&gt;, betting_options: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, resolution_at_ms: u64, max_resolution_window_ms: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_market_for_claim">create_spot_market_for_claim</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    claim: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    primary_post: &<b>mut</b> Post,
    claim_index: u64,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>: vector&lt;u8&gt;,
    resolution_policy_hash: vector&lt;u8&gt;,
    betting_options: vector&lt;String&gt;,
    resolution_at_ms: u64,
    max_resolution_window_ms: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(registry);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_claim_version">assert_claim_version</a>(claim);
    <b>assert</b>!(config.truth_enabled, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_valid_hash">assert_valid_hash</a>(&<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>);
    <b>assert</b>!(!table::contains(&registry.markets_by_key_hash, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EMarketExists">EMarketExists</a>);
    <b>let</b> now_ms = clock::timestamp_ms(clock);
    <b>assert</b>!(resolution_at_ms &gt;= now_ms, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
    <b>let</b> options_len = vector::length(&betting_options);
    <b>assert</b>!(options_len &gt;= config.min_betting_options, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(options_len &lt;= config.max_betting_options, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; options_len) {
        <b>let</b> option_i = vector::borrow(&betting_options, i);
        <b>let</b> <b>mut</b> j = i + 1;
        <b>while</b> (j &lt; options_len) {
            <b>assert</b>!(*option_i != *vector::borrow(&betting_options, j), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDuplicateOption">EDuplicateOption</a>);
            j = j + 1;
        };
        i = i + 1;
    };
    <b>let</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a> = object::uid_to_address(&claim.id);
    <b>let</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a> = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(primary_post);
    <b>let</b> primary_creator = <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">post::get_post_owner</a>(primary_post);
    <b>let</b> market = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a> {
        id: object::new(ctx),
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a>,
        primary_creator,
        created_at_ms: clock::timestamp_ms(clock),
        status: <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>,
        outcome: option::none(),
        escrow: balance::zero(),
        betting_options,
        option_escrow: table::new(ctx),
        user_option_amounts: table::new(ctx),
        bets: vector::empty(),
        resolution_window_ms: option::none(),
        max_resolution_window_ms,
        resolution_at_ms,
        last_resolution_at_ms: 0,
        resolution_timestamp_ms: 0,
        pending_payouts: table::new(ctx),
        pending_creator_payouts: table::new(ctx),
        next_creator_payout_id: 0,
        creator_payout_index: table::new(ctx),
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>: option::none(),
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_proposed_outcome">oracle_proposed_outcome</a>: option::none(),
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>: option::none(),
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_dao_escalated_at_ms">dao_escalated_at_ms</a>: 0,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> market_id = object::uid_to_address(&market.id);
    <b>let</b> hash_copy = market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>;
    <b>let</b> betting_options_copy = market.betting_options;
    <b>let</b> max_resolution_window = market.max_resolution_window_ms;
    <b>let</b> resolution_at = market.resolution_at_ms;
    <b>let</b> created_at_ms = market.created_at_ms;
    table::add(&<b>mut</b> registry.markets_by_key_hash, hash_copy, market_id);
    <b>if</b> (table::contains(&registry.open_market_by_claim, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>)) {
        table::remove(&<b>mut</b> registry.open_market_by_claim, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>);
    };
    table::add(&<b>mut</b> registry.open_market_by_claim, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>, market_id);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_register_future_link">register_future_link</a>(
        registry,
        claim,
        primary_post,
        market_id,
        claim_index,
        resolution_policy_hash,
        config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>,
    );
    transfer::share_object(market);
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarketCreatedEvent">SpotMarketCreatedEvent</a> {
        market_id,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>: hash_copy,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a>,
        claim_index,
        resolution_policy_hash,
        created_at_ms,
        betting_options: betting_options_copy,
        resolution_at_ms: resolution_at,
        max_resolution_window_ms: max_resolution_window,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_link_post_to_spot_claim"></a>

## Function `link_post_to_spot_claim`

Link an additional post as a future-claim referrer into an existing open market
(hybrid liquidity reuse). Requires the claim to have a live open market.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_link_post_to_spot_claim">link_post_to_spot_claim</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, claim: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, claim_index: u64, resolution_policy_hash: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_link_post_to_spot_claim">link_post_to_spot_claim</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    claim: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> Post,
    claim_index: u64,
    resolution_policy_hash: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(registry);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_claim_version">assert_claim_version</a>(claim);
    <b>assert</b>!(config.truth_enabled, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>let</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a> = object::uid_to_address(&claim.id);
    <b>assert</b>!(table::contains(&registry.open_market_by_claim, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoOpenMarket">ENoOpenMarket</a>);
    <b>let</b> market_id = *table::borrow(&registry.open_market_by_claim, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>);
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_register_future_link">register_future_link</a>(
        registry,
        claim,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        market_id,
        claim_index,
        resolution_policy_hash,
        config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>,
    );
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPostLinkedEvent">SpotPostLinkedEvent</a> {
        post_id,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>,
        market_id: option::some(market_id),
        claim_index,
        policy_hash: resolution_policy_hash,
    });
    <b>let</b> _ = clock;
    <b>let</b> _ = ctx;
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_finalize_spot_claims_for_post"></a>

## Function `finalize_spot_claims_for_post`

Oracle-only: commit a post's multi-claim analysis. Sets terminal status, counts and
manifests, and emits the batch projection (future arrays + parallel past verdicts).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_spot_claims_for_post">finalize_spot_claims_for_post</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, detected_claim_count: u64, rejected_claim_count: u64, truncated_claim_count: u64, past_verified_count: u64, claim_manifest_hash: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, veracity_manifest_hash: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, past_claim_indexes: vector&lt;u64&gt;, past_verdicts: vector&lt;u8&gt;, past_related_market_ids: vector&lt;<b>address</b>&gt;, past_evidence_hashes: vector&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_spot_claims_for_post">finalize_spot_claims_for_post</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> Post,
    detected_claim_count: u64,
    rejected_claim_count: u64,
    truncated_claim_count: u64,
    past_verified_count: u64,
    claim_manifest_hash: Option&lt;vector&lt;u8&gt;&gt;,
    veracity_manifest_hash: Option&lt;vector&lt;u8&gt;&gt;,
    past_claim_indexes: vector&lt;u64&gt;,
    past_verdicts: vector&lt;u8&gt;,
    past_related_market_ids: vector&lt;<b>address</b>&gt;,
    past_evidence_hashes: vector&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(config);
    <b>assert</b>!(config.truth_enabled, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>let</b> past_len = vector::length(&past_claim_indexes);
    <b>assert</b>!(past_len == past_verified_count, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPastVerdictMismatch">EPastVerdictMismatch</a>);
    <b>assert</b>!(vector::length(&past_verdicts) == past_len, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPastVerdictMismatch">EPastVerdictMismatch</a>);
    <b>assert</b>!(vector::length(&past_related_market_ids) == past_len, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPastVerdictMismatch">EPastVerdictMismatch</a>);
    <b>assert</b>!(vector::length(&past_evidence_hashes) == past_len, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPastVerdictMismatch">EPastVerdictMismatch</a>);
    <b>let</b> <b>mut</b> vi = 0;
    <b>while</b> (vi &lt; past_len) {
        <b>let</b> v = *vector::borrow(&past_verdicts, vi);
        <b>assert</b>!(v == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_VERDICT_TRUE">VERDICT_TRUE</a> || v == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_VERDICT_FALSE">VERDICT_FALSE</a> || v == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_VERDICT_UNVERIFIABLE">VERDICT_UNVERIFIABLE</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPastVerdictMismatch">EPastVerdictMismatch</a>);
        vi = vi + 1;
    };
    <b>let</b> future_accepted = <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_future_accepted_count">post::spot_analysis_future_accepted_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> status = <b>if</b> (future_accepted &gt; 0 || past_verified_count &gt; 0) {
        <a href="../social_contracts/post.md#social_contracts_post_spot_status_completed">post::spot_status_completed</a>()
    } <b>else</b> {
        <a href="../social_contracts/post.md#social_contracts_post_spot_status_completed_no_actionable">post::spot_status_completed_no_actionable</a>()
    };
    <a href="../social_contracts/post.md#social_contracts_post_finalize_spot_analysis">post::finalize_spot_analysis</a>(
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        status,
        detected_claim_count,
        rejected_claim_count,
        truncated_claim_count,
        past_verified_count,
        config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>,
        claim_manifest_hash,
        veracity_manifest_hash,
    );
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimsFinalizedForPost">SpotClaimsFinalizedForPost</a> {
        post_id,
        status,
        detected_claim_count,
        rejected_claim_count,
        truncated_claim_count,
        future_accepted_count: future_accepted,
        past_verified_count,
        max_claim_per_post_applied: config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>,
        claim_manifest_hash,
        veracity_manifest_hash,
        future_claim_indexes: <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_claim_indexes">post::spot_analysis_claim_indexes</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        future_claim_ids: <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_claim_ids">post::spot_analysis_claim_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        future_market_ids: <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_market_ids">post::spot_analysis_market_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        past_claim_indexes,
        past_verdicts,
        past_related_market_ids,
        past_evidence_hashes,
        finalized_at_ms: clock::timestamp_ms(clock),
    });
    <b>let</b> _ = ctx;
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_create_and_finalize_spot_market_for_post"></a>

## Function `create_and_finalize_spot_market_for_post`

Convenience one-shot: register claim + open one future market + finalize (single-claim
posts and test setup). Emits <code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarketCreatedEvent">SpotMarketCreatedEvent</a></code> + <code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimsFinalizedForPost">SpotClaimsFinalizedForPost</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_and_finalize_spot_market_for_post">create_and_finalize_spot_market_for_post</a>(oracle_cap: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>: vector&lt;u8&gt;, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>: vector&lt;u8&gt;, resolution_policy_hash: vector&lt;u8&gt;, betting_options: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, resolution_at_ms: u64, max_resolution_window_ms: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_and_finalize_spot_market_for_post">create_and_finalize_spot_market_for_post</a>(
    oracle_cap: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> Post,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>: vector&lt;u8&gt;,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>: vector&lt;u8&gt;,
    resolution_policy_hash: vector&lt;u8&gt;,
    betting_options: vector&lt;String&gt;,
    resolution_at_ms: u64,
    max_resolution_window_ms: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(registry);
    <b>assert</b>!(config.truth_enabled, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>let</b> created_at_ms = clock::timestamp_ms(clock);
    <b>let</b> <b>mut</b> claim = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_register_spot_claim">register_spot_claim</a>(registry, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_semantic_claim_hash">semantic_claim_hash</a>, created_at_ms, ctx);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_market_for_claim">create_spot_market_for_claim</a>(
        oracle_cap,
        config,
        registry,
        &<b>mut</b> claim,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        0,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_market_key_hash">market_key_hash</a>,
        resolution_policy_hash,
        betting_options,
        resolution_at_ms,
        max_resolution_window_ms,
        clock,
        ctx,
    );
    transfer::share_object(claim);
    <b>let</b> status = <a href="../social_contracts/post.md#social_contracts_post_spot_status_completed">post::spot_status_completed</a>();
    <a href="../social_contracts/post.md#social_contracts_post_finalize_spot_analysis">post::finalize_spot_analysis</a>(
        <a href="../social_contracts/post.md#social_contracts_post">post</a>, status, 1, 0, 0, 0, config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>, option::none(), option::none(),
    );
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimsFinalizedForPost">SpotClaimsFinalizedForPost</a> {
        post_id,
        status,
        detected_claim_count: 1,
        rejected_claim_count: 0,
        truncated_claim_count: 0,
        future_accepted_count: 1,
        past_verified_count: 0,
        max_claim_per_post_applied: config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a>,
        claim_manifest_hash: option::none(),
        veracity_manifest_hash: option::none(),
        future_claim_indexes: <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_claim_indexes">post::spot_analysis_claim_indexes</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        future_claim_ids: <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_claim_ids">post::spot_analysis_claim_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        future_market_ids: <a href="../social_contracts/post.md#social_contracts_post_spot_analysis_market_ids">post::spot_analysis_market_ids</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        past_claim_indexes: vector::empty(),
        past_verdicts: vector::empty(),
        past_related_market_ids: vector::empty(),
        past_evidence_hashes: vector::empty(),
        finalized_at_ms: created_at_ms,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_assert_market_open_for_post"></a>

## Function `assert_market_open_for_post`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_open_for_post">assert_market_open_for_post</a>(registry: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_open_for_post">assert_market_open_for_post</a>(
    registry: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
) {
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(market.status != <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDaoDebateFrozen">EDaoDebateFrozen</a>);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EMarketNotOpen">EMarketNotOpen</a>);
    <b>let</b> market_id = object::uid_to_address(&market.id);
    <b>let</b> key = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_PostMarketKey">PostMarketKey</a> { post_id, market_id };
    <b>assert</b>!(table::contains(&registry.post_market_to_claim, key), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPostNotLinked">EPostNotLinked</a>);
    <b>let</b> linked_claim = *table::borrow(&registry.post_market_to_claim, key);
    <b>assert</b>!(linked_claim == market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPostNotLinked">EPostNotLinked</a>);
    <b>let</b> open_id = *table::borrow(&registry.open_market_by_claim, market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>);
    <b>assert</b>!(open_id == market_id, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EMarketNotOpen">EMarketNotOpen</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_place_spot_bet_for_post"></a>

## Function `place_spot_bet_for_post`

Sole public betting entry — registry validates the market is open for this claim.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_place_spot_bet_for_post">place_spot_bet_for_post</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, option_id: u8, amount: u64, referrer_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_place_spot_bet_for_post">place_spot_bet_for_post</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    option_id: u8,
    amount: u64,
    referrer_post_id: Option&lt;<b>address</b>&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(registry);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <b>assert</b>!(spot_config.truth_enabled, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_spot_analysis_status">post::spot_analysis_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>) == <a href="../social_contracts/post.md#social_contracts_post_spot_status_completed">post::spot_status_completed</a>(), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotFinalized">ENotFinalized</a>);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_open_for_post">assert_market_open_for_post</a>(registry, market, <a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>let</b> ref_id = <b>if</b> (option::is_some(&referrer_post_id)) {
        *option::borrow(&referrer_post_id)
    } <b>else</b> {
        post_id
    };
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_place_spot_bet_internal">place_spot_bet_internal</a>(
        spot_config,
        market,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        &<b>mut</b> payment,
        option_id,
        amount,
        option::some(ref_id),
        clock,
        ctx,
    );
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, tx_context::sender(ctx));
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_place_spot_bet_internal"></a>

## Function `place_spot_bet_internal`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_place_spot_bet_internal">place_spot_bet_internal</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, option_id: u8, amount: u64, referrer_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_place_spot_bet_internal">place_spot_bet_internal</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    option_id: u8,
    amount: u64,
    referrer_post_id: Option&lt;<b>address</b>&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>if</b> (spot_config.max_single_bet &gt; 0) {
        <b>assert</b>!(amount &lt;= spot_config.max_single_bet, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    };
    <b>assert</b>!(coin::value(payment) &gt;= amount, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>if</b> (spot_config.max_bets_per_record &gt; 0) {
        <b>assert</b>!(vector::length(&market.bets) &lt; spot_config.max_bets_per_record, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooManyBets">ETooManyBets</a>);
    };
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_valid_option_id">assert_valid_option_id</a>(market, option_id);
    <b>let</b> bet_coin = coin::split(payment, amount, ctx);
    balance::join(&<b>mut</b> market.escrow, coin::into_balance(bet_coin));
    <b>let</b> current_escrow = <b>if</b> (table::contains(&market.option_escrow, option_id)) {
        *table::borrow(&market.option_escrow, option_id)
    } <b>else</b> { 0 };
    <b>assert</b>!(current_escrow &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EOverflow">EOverflow</a>);
    <b>if</b> (table::contains(&market.option_escrow, option_id)) {
        *table::borrow_mut(&<b>mut</b> market.option_escrow, option_id) = current_escrow + amount;
    } <b>else</b> {
        table::add(&<b>mut</b> market.option_escrow, option_id, amount);
    };
    <b>let</b> ts = clock::timestamp_ms(clock);
    <b>let</b> user = tx_context::sender(ctx);
    vector::push_back(&<b>mut</b> market.bets, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">SpotBet</a> {
        user,
        option_id,
        amount,
        timestamp_ms: ts,
        referrer_post_id,
    });
    <b>let</b> options_len = vector::length(&market.betting_options);
    <b>if</b> (!table::contains(&market.user_option_amounts, user)) {
        <b>let</b> <b>mut</b> amounts = vector::empty&lt;u64&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>while</b> (i &lt; options_len) {
            vector::push_back(&<b>mut</b> amounts, 0);
            i = i + 1;
        };
        table::add(&<b>mut</b> market.user_option_amounts, user, amounts);
    };
    <b>let</b> user_amounts = table::borrow_mut(&<b>mut</b> market.user_option_amounts, user);
    <b>let</b> idx = option_id <b>as</b> u64;
    <b>let</b> current_user_amount = *vector::borrow(user_amounts, idx);
    <b>assert</b>!(current_user_amount &lt;= <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EOverflow">EOverflow</a>);
    *vector::borrow_mut(user_amounts, idx) = current_user_amount + amount;
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBetPlacedEvent">SpotBetPlacedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        market_id: object::uid_to_address(&market.id),
        user,
        option_id,
        amount,
        timestamp_ms: ts,
        referrer_post_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_withdraw_spot_bet"></a>

## Function `withdraw_spot_bet`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_withdraw_spot_bet">withdraw_spot_bet</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, referrer_post: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, bet_index: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_withdraw_spot_bet">withdraw_spot_bet</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    referrer_post: &Post,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    treasury: &EcosystemTreasury,
    bet_index: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_claim_version">assert_claim_version</a>(claim);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <b>assert</b>!(spot_config.truth_enabled, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWithdrawalNotAllowed">EWithdrawalNotAllowed</a>);
    <b>let</b> bets_len = vector::length(&market.bets);
    <b>assert</b>!(bet_index &lt; bets_len, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EBetNotFound">EBetNotFound</a>);
    <b>let</b> bet = *vector::borrow(&market.bets, bet_index);
    <b>assert</b>!(bet.user == tx_context::sender(ctx), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(bet.amount &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> ref_post_id = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_referrer_post_id_for_bet">referrer_post_id_for_bet</a>(&bet, market);
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(referrer_post) == ref_post_id, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPostNotLinked">EPostNotLinked</a>);
    <b>let</b> platform_fee = (bet.amount * spot_config.platform_fee_bps) / 10000;
    <b>let</b> ecosystem_fee = (bet.amount * spot_config.ecosystem_fee_bps) / 10000;
    <b>let</b> creator_fee = (bet.amount * spot_config.creator_fee_bps) / 10000;
    <b>let</b> fee = platform_fee + ecosystem_fee + creator_fee;
    <b>let</b> refund_amount = bet.amount - fee;
    <b>if</b> (platform_fee + ecosystem_fee &gt; 0) {
        <b>let</b> protocol_fee = platform_fee + ecosystem_fee;
        <b>let</b> <b>mut</b> fee_coin = coin::from_balance(balance::split(&<b>mut</b> market.escrow, protocol_fee), ctx);
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_coin = coin::split(&<b>mut</b> fee_coin, platform_fee, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_coin);
        };
        <b>if</b> (ecosystem_fee &gt; 0) {
            transfer::public_transfer(fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        } <b>else</b> {
            coin::destroy_zero(fee_coin);
        };
    };
    <b>if</b> (creator_fee &gt; 0) {
        <b>let</b> creator = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_creator_for_referrer_post">creator_for_referrer_post</a>(claim, ref_post_id, market.primary_creator);
        <b>if</b> (creator != @0x0) {
            transfer::public_transfer(
                coin::from_balance(balance::split(&<b>mut</b> market.escrow, creator_fee), ctx),
                creator,
            );
        };
    };
    <b>if</b> (refund_amount &gt; 0) {
        transfer::public_transfer(
            coin::from_balance(balance::split(&<b>mut</b> market.escrow, refund_amount), ctx),
            bet.user,
        );
    };
    <b>let</b> option_id = bet.option_id;
    <b>if</b> (table::contains(&market.option_escrow, option_id)) {
        <b>let</b> current_escrow = *table::borrow(&market.option_escrow, option_id);
        <b>if</b> (current_escrow &gt;= bet.amount) {
            *table::borrow_mut(&<b>mut</b> market.option_escrow, option_id) = current_escrow - bet.amount;
        };
    };
    <b>if</b> (table::contains(&market.user_option_amounts, bet.user)) {
        <b>let</b> user_amounts = table::borrow_mut(&<b>mut</b> market.user_option_amounts, bet.user);
        <b>let</b> idx = bet.option_id <b>as</b> u64;
        <b>if</b> (idx &lt; vector::length(user_amounts)) {
            <b>let</b> current_user_amount = *vector::borrow(user_amounts, idx);
            <b>if</b> (current_user_amount &gt;= bet.amount) {
                *vector::borrow_mut(user_amounts, idx) = current_user_amount - bet.amount;
            };
        };
    };
    <b>let</b> last_index = bets_len - 1;
    <b>if</b> (bet_index != last_index) {
        *vector::borrow_mut(&<b>mut</b> market.bets, bet_index) = *vector::borrow(&market.bets, last_index);
    };
    vector::pop_back(&<b>mut</b> market.bets);
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBetWithdrawnEvent">SpotBetWithdrawnEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        user: bet.user,
        option_id: bet.option_id,
        amount: bet.amount,
        fee_taken: fee,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_oracle_resolve"></a>

## Function `oracle_resolve`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_resolve">oracle_resolve</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, outcome_option_id: u8, confidence_bps: u64, reasoning: <a href="../std/string.md#std_string_String">std::string::String</a>, evidence_urls: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_resolve">oracle_resolve</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    treasury: &EcosystemTreasury,
    outcome_option_id: u8,
    confidence_bps: u64,
    reasoning: String,
    evidence_urls: vector&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(registry);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_claim_version">assert_claim_version</a>(claim);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(option::is_none(&market.outcome), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EAlreadyResolved">EAlreadyResolved</a>);
    <b>let</b> now_ms = clock::timestamp_ms(clock);
    <b>assert</b>!(now_ms &gt;= market.resolution_at_ms, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_valid_option_id">assert_valid_option_id</a>(market, outcome_option_id);
    <b>let</b> reasoning_len = string::length(&reasoning);
    <b>assert</b>!(reasoning_len &gt;= spot_config.min_reasoning_length, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>);
    <b>assert</b>!(reasoning_len &lt;= spot_config.max_reasoning_length, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>);
    <b>assert</b>!(vector::length(&evidence_urls) &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(vector::length(&evidence_urls) &lt;= spot_config.max_evidence_urls, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>if</b> (confidence_bps &lt; spot_config.confidence_threshold_bps) {
        <b>assert</b>!(option::is_none(&market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EActiveProposalExists">EActiveProposalExists</a>);
        market.status = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>;
        market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_proposed_outcome">oracle_proposed_outcome</a> = option::some(outcome_option_id);
        market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_dao_escalated_at_ms">dao_escalated_at_ms</a> = now_ms;
        event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotDaoRequiredEvent">SpotDaoRequiredEvent</a> {
            post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
            spot_record_id: object::uid_to_address(&market.id),
            confidence_bps,
            <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_proposed_outcome">oracle_proposed_outcome</a>: outcome_option_id,
            <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_dao_escalated_at_ms">dao_escalated_at_ms</a>: now_ms,
            reasoning,
        });
        <b>return</b>
    };
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(
        spot_config,
        registry,
        claim,
        market,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        treasury,
        outcome_option_id,
        reasoning,
        option::some(evidence_urls),
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_submit_spot_resolution_proposal_to_governance"></a>

## Function `submit_spot_resolution_proposal_to_governance`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_submit_spot_resolution_proposal_to_governance">submit_spot_resolution_proposal_to_governance</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>: u8, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_submit_spot_resolution_proposal_to_governance">submit_spot_resolution_proposal_to_governance</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> GovernanceDAO,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    title: String,
    description: String,
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>: u8,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_spot_governance_registry">assert_spot_governance_registry</a>(spot_config, registry);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotDaoRequired">ENotDaoRequired</a>);
    <b>assert</b>!(option::is_none(&market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EActiveProposalExists">EActiveProposalExists</a>);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_validate_proposed_outcome">validate_proposed_outcome</a>(market, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>);
    <b>let</b> spot_record_id = object::id(market);
    <b>let</b> proposal_id = <a href="../social_contracts/governance.md#social_contracts_governance_submit_spot_proposal_and_return_id">governance::submit_spot_proposal_and_return_id</a>(
        registry,
        title,
        description,
        spot_record_id,
        metadata_json,
        coin,
        clock,
        ctx,
    );
    market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a> = option::some(proposal_id);
    market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a> = option::some(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>);
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotGovernanceProposalLinkedEvent">SpotGovernanceProposalLinkedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        spot_record_id: object::uid_to_address(&market.id),
        proposal_id,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_implement_spot_resolution_from_governance"></a>

## Function `implement_spot_resolution_from_governance`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_implement_spot_resolution_from_governance">implement_spot_resolution_from_governance</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry_gov: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, spot_registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, reasoning: <a href="../std/string.md#std_string_String">std::string::String</a>, evidence_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_implement_spot_resolution_from_governance">implement_spot_resolution_from_governance</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry_gov: &<b>mut</b> GovernanceDAO,
    proposal: &<b>mut</b> Proposal,
    spot_registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    treasury: &EcosystemTreasury,
    reasoning: String,
    evidence_urls: Option&lt;vector&lt;String&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(spot_registry);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_claim_version">assert_claim_version</a>(claim);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_spot_governance_registry">assert_spot_governance_registry</a>(spot_config, registry_gov);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotDaoRequired">ENotDaoRequired</a>);
    <b>assert</b>!(option::is_some(&market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoActiveProposal">ENoActiveProposal</a>);
    <b>assert</b>!(option::is_some(&market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongProposal">EWrongProposal</a>);
    <b>let</b> active_id = *option::borrow(&market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>);
    <b>assert</b>!(active_id == object::id(proposal), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongProposal">EWrongProposal</a>);
    <b>assert</b>!(
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_status">governance::proposal_status</a>(proposal) == <a href="../social_contracts/governance.md#social_contracts_governance_status_approved_value">governance::status_approved_value</a>(),
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EProposalNotApproved">EProposalNotApproved</a>
    );
    <b>let</b> outcome = *option::borrow(&market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a>);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_validate_proposed_outcome">validate_proposed_outcome</a>(market, outcome);
    <b>let</b> reasoning_len = string::length(&reasoning);
    <b>assert</b>!(reasoning_len &gt;= spot_config.min_reasoning_length, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>);
    <b>assert</b>!(reasoning_len &lt;= spot_config.max_reasoning_length, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidReasoning">EInvalidReasoning</a>);
    <b>if</b> (option::is_some(&evidence_urls)) {
        <b>assert</b>!(vector::length(option::borrow(&evidence_urls)) &lt;= spot_config.max_evidence_urls, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    };
    <b>let</b> submitter = <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submitter">governance::proposal_submitter</a>(proposal);
    <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_take_pool">governance::mark_proposal_implemented_take_pool</a>(
        registry_gov,
        proposal,
        option::none(),
        clock,
        ctx,
    );
    <b>let</b> amount = balance::value(&bal);
    <b>if</b> (amount &gt; 0) {
        transfer::public_transfer(coin::from_balance(bal, ctx), submitter);
    } <b>else</b> {
        balance::destroy_zero(bal);
    };
    <b>let</b> proposal_id = active_id;
    market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a> = option::none();
    market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a> = option::none();
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(
        spot_config,
        spot_registry,
        claim,
        market,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        treasury,
        outcome,
        reasoning,
        evidence_urls,
        clock,
        ctx,
    );
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotGovernanceProposalClearedEvent">SpotGovernanceProposalClearedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        spot_record_id: object::uid_to_address(&market.id),
        proposal_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_clear_spot_proposal_link_on_reject"></a>

## Function `clear_spot_proposal_link_on_reject`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_clear_spot_proposal_link_on_reject">clear_spot_proposal_link_on_reject</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_clear_spot_proposal_link_on_reject">clear_spot_proposal_link_on_reject</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &GovernanceDAO,
    proposal: &Proposal,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_spot_governance_registry">assert_spot_governance_registry</a>(spot_config, registry);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotDaoRequired">ENotDaoRequired</a>);
    <b>assert</b>!(option::is_some(&market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoActiveProposal">ENoActiveProposal</a>);
    <b>let</b> active_id = *option::borrow(&market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a>);
    <b>assert</b>!(active_id == object::id(proposal), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongProposal">EWrongProposal</a>);
    <b>assert</b>!(
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_status">governance::proposal_status</a>(proposal) == <a href="../social_contracts/governance.md#social_contracts_governance_status_rejected_value">governance::status_rejected_value</a>(),
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EProposalNotApproved">EProposalNotApproved</a>
    );
    <b>let</b> proposal_id = active_id;
    market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a> = option::none();
    market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a> = option::none();
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotGovernanceProposalClearedEvent">SpotGovernanceProposalClearedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        spot_record_id: object::uid_to_address(&market.id),
        proposal_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_finalize_spot_governance_proposal"></a>

## Function `finalize_spot_governance_proposal`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_spot_governance_proposal">finalize_spot_governance_proposal</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, ecosystem_treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_spot_governance_proposal">finalize_spot_governance_proposal</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> GovernanceDAO,
    proposal: &<b>mut</b> Proposal,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    ecosystem_treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_spot_governance_registry">assert_spot_governance_registry</a>(spot_config, registry);
    <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal">governance::finalize_proposal</a>(registry, proposal, ecosystem_treasury, clock, ctx);
    <b>if</b> (<a href="../social_contracts/governance.md#social_contracts_governance_proposal_status">governance::proposal_status</a>(proposal) == <a href="../social_contracts/governance.md#social_contracts_governance_status_rejected_value">governance::status_rejected_value</a>()) {
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_clear_spot_proposal_link_on_reject">clear_spot_proposal_link_on_reject</a>(spot_config, registry, proposal, market, <a href="../social_contracts/post.md#social_contracts_post">post</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_finalize_via_dao"></a>

## Function `finalize_via_dao`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_via_dao">finalize_via_dao</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, spot_registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, evidence_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_via_dao">finalize_via_dao</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> GovernanceDAO,
    proposal: &<b>mut</b> Proposal,
    spot_registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    treasury: &EcosystemTreasury,
    <b>mut</b> reasoning: Option&lt;String&gt;,
    evidence_urls: Option&lt;vector&lt;String&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> final_reasoning = <b>if</b> (option::is_some(&reasoning)) {
        option::extract(&<b>mut</b> reasoning)
    } <b>else</b> {
        string::utf8(b"DAO resolution based on community discussion")
    };
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_implement_spot_resolution_from_governance">implement_spot_resolution_from_governance</a>(
        spot_config,
        registry,
        proposal,
        spot_registry,
        claim,
        market,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        treasury,
        final_reasoning,
        evidence_urls,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_assert_spot_governance_registry"></a>

## Function `assert_spot_governance_registry`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_spot_governance_registry">assert_spot_governance_registry</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_spot_governance_registry">assert_spot_governance_registry</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>, registry: &GovernanceDAO) {
    <b>assert</b>!(
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">governance::registry_type</a>(registry) == <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_spot_value">governance::proposal_type_spot_value</a>(),
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidGovernanceRegistry">EInvalidGovernanceRegistry</a>
    );
    <b>assert</b>!(
        object::id(registry) == spot_config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a>,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidGovernanceRegistry">EInvalidGovernanceRegistry</a>
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_validate_proposed_outcome"></a>

## Function `validate_proposed_outcome`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_validate_proposed_outcome">validate_proposed_outcome</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, outcome: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_validate_proposed_outcome">validate_proposed_outcome</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>, outcome: u8) {
    <b>if</b> (outcome == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_DRAW">OUTCOME_DRAW</a> || outcome == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE">OUTCOME_UNAPPLICABLE</a>) {
        <b>return</b>
    };
    <b>assert</b>!((outcome <b>as</b> u64) &lt; vector::length(&market.betting_options), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidOptionId">EInvalidOptionId</a>);
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_refund_unresolved"></a>

## Function `refund_unresolved`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_refund_unresolved">refund_unresolved</a>(_: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">social_contracts::social_proof_of_truth::SpotOracleAdminCap</a>, spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_refund_unresolved">refund_unresolved</a>(
    _: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotOracleAdminCap">SpotOracleAdminCap</a>,
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_registry_version">assert_registry_version</a>(registry);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <b>assert</b>!(option::is_some(&market.max_resolution_window_ms), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> now_ms = clock::timestamp_ms(clock);
    <b>let</b> max_window = *option::borrow(&market.max_resolution_window_ms);
    <b>assert</b>!(now_ms &gt;= market.resolution_at_ms + max_window, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a> || market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(vector::length(&market.bets) &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoBets">ENoBets</a>);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&market.bets);
    <b>while</b> (i &lt; len) {
        <b>let</b> bet = vector::borrow(&market.bets, i);
        <b>if</b> (bet.amount &gt; 0) {
            <b>let</b> c = coin::from_balance(balance::split(&<b>mut</b> market.escrow, bet.amount), ctx);
            transfer::public_transfer(c, bet.user);
            event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRefundEvent">SpotRefundEvent</a> {
                post_id: market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a>,
                user: bet.user,
                amount: bet.amount,
            });
        };
        i = i + 1;
    };
    <b>if</b> (table::contains(&registry.open_market_by_claim, market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>)) {
        table::remove(&<b>mut</b> registry.open_market_by_claim, market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>);
    };
    market.status = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_REFUNDABLE">STATUS_REFUNDABLE</a>;
    market.outcome = option::none();
    market.last_resolution_at_ms = now_ms;
    <b>let</b> _ = <a href="../social_contracts/post.md#social_contracts_post">post</a>;
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_referrer_post_id_for_bet"></a>

## Function `referrer_post_id_for_bet`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_referrer_post_id_for_bet">referrer_post_id_for_bet</a>(bet: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">social_contracts::social_proof_of_truth::SpotBet</a>, market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_referrer_post_id_for_bet">referrer_post_id_for_bet</a>(bet: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotBet">SpotBet</a>, market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): <b>address</b> {
    <b>if</b> (option::is_some(&bet.referrer_post_id)) {
        *option::borrow(&bet.referrer_post_id)
    } <b>else</b> {
        market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_track_creator_payout_index"></a>

## Function `track_creator_payout_index`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_track_creator_payout_index">track_creator_payout_index</a>(market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, creator: <b>address</b>, payout_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_track_creator_payout_index">track_creator_payout_index</a>(market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>, creator: <b>address</b>, payout_id: u64) {
    <b>if</b> (table::contains(&market.creator_payout_index, creator)) {
        <b>let</b> ids = table::borrow_mut(&<b>mut</b> market.creator_payout_index, creator);
        vector::push_back(ids, payout_id);
    } <b>else</b> {
        <b>let</b> <b>mut</b> ids = vector::empty&lt;u64&gt;();
        vector::push_back(&<b>mut</b> ids, payout_id);
        table::add(&<b>mut</b> market.creator_payout_index, creator, ids);
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_untrack_creator_payout_index"></a>

## Function `untrack_creator_payout_index`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_untrack_creator_payout_index">untrack_creator_payout_index</a>(market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, creator: <b>address</b>, payout_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_untrack_creator_payout_index">untrack_creator_payout_index</a>(market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>, creator: <b>address</b>, payout_id: u64) {
    <b>if</b> (!table::contains(&market.creator_payout_index, creator)) {
        <b>return</b>
    };
    <b>let</b> ids = table::borrow_mut(&<b>mut</b> market.creator_payout_index, creator);
    <b>let</b> <b>mut</b> k = 0;
    <b>let</b> len = vector::length(ids);
    <b>while</b> (k &lt; len) {
        <b>if</b> (*vector::borrow(ids, k) == payout_id) {
            vector::remove(ids, k);
            <b>break</b>
        };
        k = k + 1;
    };
    <b>if</b> (vector::is_empty(ids)) {
        table::remove(&<b>mut</b> market.creator_payout_index, creator);
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_creator_for_referrer_post"></a>

## Function `creator_for_referrer_post`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_creator_for_referrer_post">creator_for_referrer_post</a>(claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, referrer_post_id: <b>address</b>, fallback_creator: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_creator_for_referrer_post">creator_for_referrer_post</a>(
    claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    referrer_post_id: <b>address</b>,
    fallback_creator: <b>address</b>,
): <b>address</b> {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&claim.linked_posts);
    <b>while</b> (i &lt; len) {
        <b>let</b> link = vector::borrow(&claim.linked_posts, i);
        <b>if</b> (link.post_id == referrer_post_id) {
            <b>return</b> link.creator
        };
        i = i + 1;
    };
    fallback_creator
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_vector_contains_address"></a>

## Function `vector_contains_address`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_vector_contains_address">vector_contains_address</a>(addrs: &vector&lt;<b>address</b>&gt;, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_vector_contains_address">vector_contains_address</a>(addrs: &vector&lt;<b>address</b>&gt;, addr: <b>address</b>): bool {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(addrs);
    <b>while</b> (i &lt; len) {
        <b>if</b> (*vector::borrow(addrs, i) == addr) {
            <b>return</b> <b>true</b>
        };
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_referred_volume_for_post"></a>

## Function `referred_volume_for_post`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_referred_volume_for_post">referred_volume_for_post</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, referrer_post_id: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_referred_volume_for_post">referred_volume_for_post</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>, referrer_post_id: <b>address</b>): u64 {
    <b>let</b> <b>mut</b> total = 0u64;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&market.bets);
    <b>while</b> (i &lt; len) {
        <b>let</b> bet = vector::borrow(&market.bets, i);
        <b>if</b> (<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_referrer_post_id_for_bet">referrer_post_id_for_bet</a>(bet, market) == referrer_post_id) {
            total = total + bet.amount;
        };
        i = i + 1;
    };
    total
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_total_referred_volume"></a>

## Function `total_referred_volume`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_total_referred_volume">total_referred_volume</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_total_referred_volume">total_referred_volume</a>(market: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>): u64 {
    <b>let</b> <b>mut</b> total = 0u64;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&market.bets);
    <b>while</b> (i &lt; len) {
        <b>let</b> bet = vector::borrow(&market.bets, i);
        total = total + bet.amount;
        i = i + 1;
    };
    total
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_accrue_creator_payouts"></a>

## Function `accrue_creator_payouts`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_accrue_creator_payouts">accrue_creator_payouts</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, creator_fee_total: u64, resolution_timestamp_ms: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_accrue_creator_payouts">accrue_creator_payouts</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    creator_fee_total: u64,
    resolution_timestamp_ms: u64,
) {
    <b>if</b> (creator_fee_total == 0) {
        <b>return</b>
    };
    <b>let</b> total_volume = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_total_referred_volume">total_referred_volume</a>(market);
    <b>if</b> (total_volume == 0) {
        <b>return</b>
    };
    <b>let</b> expires_at_ms = resolution_timestamp_ms + spot_config.creator_claim_window_ms;
    <b>let</b> market_id = object::uid_to_address(&market.id);
    <b>let</b> <b>mut</b> unique_refs = vector::empty&lt;<b>address</b>&gt;();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> bets_len = vector::length(&market.bets);
    <b>while</b> (i &lt; bets_len) {
        <b>let</b> bet = vector::borrow(&market.bets, i);
        <b>let</b> ref_post = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_referrer_post_id_for_bet">referrer_post_id_for_bet</a>(bet, market);
        <b>if</b> (!<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_vector_contains_address">vector_contains_address</a>(&unique_refs, ref_post)) {
            vector::push_back(&<b>mut</b> unique_refs, ref_post);
        };
        i = i + 1;
    };
    <b>let</b> <b>mut</b> j = 0;
    <b>let</b> refs_len = vector::length(&unique_refs);
    <b>while</b> (j &lt; refs_len) {
        <b>let</b> ref_post = *vector::borrow(&unique_refs, j);
        <b>let</b> volume = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_referred_volume_for_post">referred_volume_for_post</a>(market, ref_post);
        <b>if</b> (volume &gt; 0) {
            <b>let</b> amount = ((volume <b>as</b> u128) * (creator_fee_total <b>as</b> u128) / (total_volume <b>as</b> u128)) <b>as</b> u64;
            <b>if</b> (amount &gt; 0) {
                <b>let</b> creator = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_creator_for_referrer_post">creator_for_referrer_post</a>(claim, ref_post, market.primary_creator);
                <b>if</b> (creator != @0x0) {
                    <b>let</b> payout_id = market.next_creator_payout_id;
                    market.next_creator_payout_id = payout_id + 1;
                    table::add(&<b>mut</b> market.pending_creator_payouts, payout_id, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotCreatorPayout">SpotCreatorPayout</a> {
                        creator,
                        source_post_id: ref_post,
                        amount,
                        expires_at_ms,
                    });
                    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotCreatorPayoutAccruedEvent">SpotCreatorPayoutAccruedEvent</a> {
                        market_id,
                        payout_id,
                        creator,
                        referrer_post_id: ref_post,
                        amount,
                        expires_at_ms,
                    });
                    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_track_creator_payout_index">track_creator_payout_index</a>(market, creator, payout_id);
                };
            };
        };
        j = j + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_finalize_resolution_and_payout"></a>

## Function `finalize_resolution_and_payout`



<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, outcome: u8, reasoning: <a href="../std/string.md#std_string_String">std::string::String</a>, evidence_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_finalize_resolution_and_payout">finalize_resolution_and_payout</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    claim: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    treasury: &EcosystemTreasury,
    outcome: u8,
    reasoning: String,
    evidence_urls: Option&lt;vector&lt;String&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_OPEN">STATUS_OPEN</a> || market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_DAO_REQUIRED">STATUS_DAO_REQUIRED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(vector::length(&market.bets) &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENoBets">ENoBets</a>);
    <b>let</b> total_escrow = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_total_option_escrow">total_option_escrow</a>(market);
    <b>let</b> now_ms = clock::timestamp_ms(clock);
    <b>let</b> market_id = object::uid_to_address(&market.id);
    <b>if</b> (outcome == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_DRAW">OUTCOME_DRAW</a> || outcome == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_OUTCOME_UNAPPLICABLE">OUTCOME_UNAPPLICABLE</a>) {
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&market.bets);
        <b>while</b> (i &lt; len) {
            <b>let</b> bet = vector::borrow(&market.bets, i);
            <b>if</b> (bet.amount &gt; 0) {
                <b>let</b> c = coin::from_balance(balance::split(&<b>mut</b> market.escrow, bet.amount), ctx);
                transfer::public_transfer(c, bet.user);
                event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotRefundEvent">SpotRefundEvent</a> {
                    post_id: market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_primary_post_id">primary_post_id</a>,
                    user: bet.user,
                    amount: bet.amount,
                });
            };
            i = i + 1;
        };
        <b>if</b> (table::contains(&registry.open_market_by_claim, market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>)) {
            table::remove(&<b>mut</b> registry.open_market_by_claim, market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>);
        };
        market.status = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a>;
        market.outcome = option::some(outcome);
        market.last_resolution_at_ms = now_ms;
        market.resolution_timestamp_ms = now_ms;
        <b>let</b> evidence_urls_vec = <b>if</b> (option::is_some(&evidence_urls)) {
            *option::borrow(&evidence_urls)
        } <b>else</b> { vector::empty() };
        event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotResolvedEvent">SpotResolvedEvent</a> {
            post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
            market_id,
            <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>: market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>,
            outcome,
            total_escrow,
            fee_taken: 0,
            creator_fee_total: 0,
            reasoning,
            evidence_urls: evidence_urls_vec,
        });
        <b>return</b>
    };
    <b>let</b> winning_total = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_get_option_escrow">get_option_escrow</a>(market, outcome);
    <b>let</b> platform_fee = (total_escrow * spot_config.platform_fee_bps) / 10000;
    <b>let</b> ecosystem_fee = (total_escrow * spot_config.ecosystem_fee_bps) / 10000;
    <b>let</b> creator_fee_total = (total_escrow * spot_config.creator_fee_bps) / 10000;
    <b>let</b> protocol_fee = platform_fee + ecosystem_fee;
    <b>let</b> distributable = total_escrow - protocol_fee - creator_fee_total;
    <b>if</b> (protocol_fee &gt; 0) {
        <b>let</b> <b>mut</b> fee_coin = coin::from_balance(balance::split(&<b>mut</b> market.escrow, protocol_fee), ctx);
        <b>if</b> (platform_fee &gt; 0) {
            <b>let</b> <b>mut</b> platform_coin = coin::split(&<b>mut</b> fee_coin, platform_fee, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_coin);
        };
        <b>if</b> (ecosystem_fee &gt; 0) {
            transfer::public_transfer(fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        } <b>else</b> {
            coin::destroy_zero(fee_coin);
        };
    };
    <b>if</b> (creator_fee_total &gt; 0) {
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_accrue_creator_payouts">accrue_creator_payouts</a>(
            spot_config,
            claim,
            market,
            creator_fee_total,
            now_ms,
        );
    };
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&market.bets);
    <b>while</b> (i &lt; len) {
        <b>let</b> bet = vector::borrow(&market.bets, i);
        <b>if</b> (bet.option_id == outcome && winning_total &gt; 0 && bet.amount &gt; 0) {
            <b>let</b> payout = (((bet.amount <b>as</b> u128) * (distributable <b>as</b> u128)) / (winning_total <b>as</b> u128)) <b>as</b> u64;
            <b>if</b> (payout &gt; 0) {
                <b>if</b> (table::contains(&market.pending_payouts, bet.user)) {
                    *table::borrow_mut(&<b>mut</b> market.pending_payouts, bet.user) =
                        *table::borrow(&market.pending_payouts, bet.user) + payout;
                } <b>else</b> {
                    table::add(&<b>mut</b> market.pending_payouts, bet.user, payout);
                };
            };
        };
        i = i + 1;
    };
    <b>if</b> (table::contains(&registry.open_market_by_claim, market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>)) {
        table::remove(&<b>mut</b> registry.open_market_by_claim, market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>);
    };
    market.status = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a>;
    market.outcome = option::some(outcome);
    market.last_resolution_at_ms = now_ms;
    market.resolution_timestamp_ms = now_ms;
    <b>let</b> evidence_urls_vec = <b>if</b> (option::is_some(&evidence_urls)) {
        *option::borrow(&evidence_urls)
    } <b>else</b> { vector::empty() };
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotResolvedEvent">SpotResolvedEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        market_id,
        <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>: market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_id">claim_id</a>,
        outcome,
        total_escrow,
        fee_taken: protocol_fee,
        creator_fee_total,
        reasoning,
        evidence_urls: evidence_urls_vec,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_claim_payout"></a>

## Function `claim_payout`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_payout">claim_payout</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_payout">claim_payout</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &Post,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <b>assert</b>!(spot_config.truth_enabled, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(option::is_some(&market.outcome), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotOracle">ENotOracle</a>);
    <b>let</b> user = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&market.pending_payouts, user), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EBetNotFound">EBetNotFound</a>);
    <b>let</b> pending_amount = *table::borrow(&market.pending_payouts, user);
    <b>assert</b>!(pending_amount &gt; 0, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>assert</b>!(current_time &gt;= market.resolution_timestamp_ms + spot_config.payout_delay_ms, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
    transfer::public_transfer(
        coin::from_balance(balance::split(&<b>mut</b> market.escrow, pending_amount), ctx),
        user,
    );
    table::remove(&<b>mut</b> market.pending_payouts, user);
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotPayoutEvent">SpotPayoutEvent</a> {
        post_id: <a href="../social_contracts/post.md#social_contracts_post_get_id_address">post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>),
        user,
        amount: pending_amount,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_claim_creator_payout"></a>

## Function `claim_creator_payout`

Single O(1) creator fee claim by <code>payout_id</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_creator_payout">claim_creator_payout</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, payout_id: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_claim_creator_payout">claim_creator_payout</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    payout_id: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <b>assert</b>!(spot_config.truth_enabled, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EDisabled">EDisabled</a>);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(table::contains(&market.pending_creator_payouts, payout_id), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPayoutNotFound">EPayoutNotFound</a>);
    <b>let</b> payout = *table::borrow(&market.pending_creator_payouts, payout_id);
    <b>assert</b>!(tx_context::sender(ctx) == payout.creator, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ENotCreator">ENotCreator</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &gt;= market.resolution_timestamp_ms + spot_config.payout_delay_ms, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
    <b>assert</b>!(now &lt;= payout.expires_at_ms, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ECreatorPayoutExpired">ECreatorPayoutExpired</a>);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_untrack_creator_payout_index">untrack_creator_payout_index</a>(market, payout.creator, payout_id);
    table::remove(&<b>mut</b> market.pending_creator_payouts, payout_id);
    transfer::public_transfer(
        coin::from_balance(balance::split(&<b>mut</b> market.escrow, payout.amount), ctx),
        payout.creator,
    );
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotCreatorPayoutClaimedEvent">SpotCreatorPayoutClaimedEvent</a> {
        market_id: object::uid_to_address(&market.id),
        payout_id,
        creator: payout.creator,
        amount: payout.amount,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_reclaim_expired_creator_rewards"></a>

## Function `reclaim_expired_creator_rewards`

Reclaim expired creator rewards to ecosystem (+ platform remainder).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_reclaim_expired_creator_rewards">reclaim_expired_creator_rewards</a>(spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, payout_id: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_reclaim_expired_creator_rewards">reclaim_expired_creator_rewards</a>(
    spot_config: &<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    treasury: &EcosystemTreasury,
    payout_id: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_config_version">assert_config_version</a>(spot_config);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_assert_market_version">assert_market_version</a>(market);
    <b>assert</b>!(market.status == <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_STATUS_RESOLVED">STATUS_RESOLVED</a>, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(table::contains(&market.pending_creator_payouts, payout_id), <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EPayoutNotFound">EPayoutNotFound</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> payout = *table::borrow(&market.pending_creator_payouts, payout_id);
    <b>assert</b>!(now &gt; payout.expires_at_ms, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_ETooEarly">ETooEarly</a>);
    <b>let</b> payout = table::remove(&<b>mut</b> market.pending_creator_payouts, payout_id);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_untrack_creator_payout_index">untrack_creator_payout_index</a>(market, payout.creator, payout_id);
    <b>let</b> amount = payout.amount;
    <b>let</b> ecosystem_amount = (amount * spot_config.expired_creator_ecosystem_bps) / 10000;
    <b>let</b> platform_amount = amount - ecosystem_amount;
    <b>if</b> (amount &gt; 0) {
        <b>let</b> <b>mut</b> fee_coin = coin::from_balance(balance::split(&<b>mut</b> market.escrow, amount), ctx);
        <b>if</b> (platform_amount &gt; 0) {
            <b>let</b> <b>mut</b> platform_coin = coin::split(&<b>mut</b> fee_coin, platform_amount, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_coin, platform_amount, clock, ctx);
            coin::destroy_zero(platform_coin);
        };
        <b>if</b> (ecosystem_amount &gt; 0) {
            transfer::public_transfer(fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
        } <b>else</b> {
            coin::destroy_zero(fee_coin);
        };
    };
    event::emit(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotCreatorPayoutReclaimedEvent">SpotCreatorPayoutReclaimedEvent</a> {
        market_id: object::uid_to_address(&market.id),
        payout_id,
        ecosystem_amount,
        platform_amount,
    });
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_migrate_config"></a>

## Function `migrate_config`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_config">migrate_config</a>(config: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">social_contracts::social_proof_of_truth::SpotConfig</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_config">migrate_config</a>(
    config: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(config.version &lt; current_version, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = config.version;
    <b>if</b> (old_version == 0) {
        config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_spot_governance_registry_id">spot_governance_registry_id</a> = object::id_from_address(@0x0);
    };
    <b>if</b> (old_version &lt; 2) {
        config.creator_fee_bps = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CREATOR_FEE_BPS">DEFAULT_CREATOR_FEE_BPS</a>;
        config.creator_claim_window_ms = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_CREATOR_CLAIM_WINDOW_MS">DEFAULT_CREATOR_CLAIM_WINDOW_MS</a>;
        config.expired_creator_ecosystem_bps = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS">DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS</a>;
    };
    <b>if</b> (config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a> == 0) {
        config.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_max_claim_per_post">max_claim_per_post</a> = <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_DEFAULT_MAX_CLAIM_PER_POST">DEFAULT_MAX_CLAIM_PER_POST</a>;
    };
    config.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(config),
        string::utf8(b"<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotConfig">SpotConfig</a>"),
        old_version,
        tx_context::sender(ctx),
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_migrate_claim_registry"></a>

## Function `migrate_claim_registry`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_claim_registry">migrate_claim_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">social_contracts::social_proof_of_truth::SpotClaimRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_claim_registry">migrate_claim_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(registry.version &lt; current_version, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = registry.version;
    registry.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(registry),
        string::utf8(b"<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaimRegistry">SpotClaimRegistry</a>"),
        old_version,
        tx_context::sender(ctx),
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_migrate_claim"></a>

## Function `migrate_claim`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_claim">migrate_claim</a>(claim: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">social_contracts::social_proof_of_truth::SpotClaim</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_claim">migrate_claim</a>(
    claim: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(claim.version &lt; current_version, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = claim.version;
    claim.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(claim),
        string::utf8(b"<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotClaim">SpotClaim</a>"),
        old_version,
        tx_context::sender(ctx),
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_migrate_market"></a>

## Function `migrate_market`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_market">migrate_market</a>(market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_market">migrate_market</a>(
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(market.version &lt; current_version, <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = market.version;
    <b>if</b> (old_version == 0) {
        market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_active_proposal_id">active_proposal_id</a> = option::none();
        market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_oracle_proposed_outcome">oracle_proposed_outcome</a> = option::none();
        market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_proposed_outcome">proposed_outcome</a> = option::none();
        market.<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_dao_escalated_at_ms">dao_escalated_at_ms</a> = 0;
    };
    <b>if</b> (old_version &lt; 2) {
        market.next_creator_payout_id = 0;
    };
    market.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(market),
        string::utf8(b"<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>"),
        old_version,
        tx_context::sender(ctx),
    );
}
</code></pre>



</details>

<a name="social_contracts_social_proof_of_truth_migrate_record"></a>

## Function `migrate_record`

Deprecated alias for <code><a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_market">migrate_market</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_record">migrate_record</a>(market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">social_contracts::social_proof_of_truth::SpotMarket</a>, cap: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_record">migrate_record</a>(
    market: &<b>mut</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_SpotMarket">SpotMarket</a>,
    cap: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_migrate_market">migrate_market</a>(market, cap, ctx);
}
</code></pre>



</details>
