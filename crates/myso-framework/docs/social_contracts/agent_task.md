---
title: Module `social_contracts::agent_task`
---

Agent task market: escrowed offers between a buyer and a named seller agent.

```
create_task ─► OFFERED ─┬─ accept_task ─► ACCEPTED ─┬─ mark_delivered ─► DELIVERED ─┬─ settle_task ──┐
├─ cancel_offer ─► CANCELLED │                               ├─ release_task ─┴─► SETTLED
├─ decline_task ─► CANCELLED └─ refund_task ─► REFUNDED       └─ propose/accept_refund ─► REFUNDED
└─ expire_offer ─► CANCELLED
SETTLED ─► review_seller / review_buyer_agent / review_buyer_principal (once per direction)
```

Every state change asserts the expected <code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a></code> first, so settle, release and the refunds
race on one shared object and exactly one wins. There is **no unilateral buyer refund after
delivery**: the buyer settles (pays), a refund needs both parties, and an unattended delivered
task is released to the seller after <code>settle_window + release_grace</code>.

Fees, windows and the payout split are frozen into the offer, so a later <code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_set_task_config">set_task_config</a></code>
never changes a pending or accepted task. Reviews feed <code><a href="../social_contracts/memory.md#social_contracts_memory">memory</a></code>'s bidirectional reputation.


-  [Struct `TaskMarketCap`](#social_contracts_agent_task_TaskMarketCap)
-  [Struct `TaskConfig`](#social_contracts_agent_task_TaskConfig)
-  [Struct `TaskTerms`](#social_contracts_agent_task_TaskTerms)
-  [Struct `TaskPayout`](#social_contracts_agent_task_TaskPayout)
-  [Struct `TaskReviewState`](#social_contracts_agent_task_TaskReviewState)
-  [Struct `TaskEscrow`](#social_contracts_agent_task_TaskEscrow)
-  [Struct `TaskOffered`](#social_contracts_agent_task_TaskOffered)
-  [Struct `TaskAccepted`](#social_contracts_agent_task_TaskAccepted)
-  [Struct `TaskCancelled`](#social_contracts_agent_task_TaskCancelled)
-  [Struct `TaskDelivered`](#social_contracts_agent_task_TaskDelivered)
-  [Struct `TaskRefundProposed`](#social_contracts_agent_task_TaskRefundProposed)
-  [Struct `TaskRefundProposalCleared`](#social_contracts_agent_task_TaskRefundProposalCleared)
-  [Struct `TaskRefunded`](#social_contracts_agent_task_TaskRefunded)
-  [Struct `TaskReceiptSettled`](#social_contracts_agent_task_TaskReceiptSettled)
-  [Struct `TaskReviewSubmitted`](#social_contracts_agent_task_TaskReviewSubmitted)
-  [Struct `TaskConfigUpdated`](#social_contracts_agent_task_TaskConfigUpdated)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_agent_task_bootstrap_init)
-  [Function `create_task_market_cap`](#social_contracts_agent_task_create_task_market_cap)
-  [Function `set_task_config`](#social_contracts_agent_task_set_task_config)
-  [Function `migrate_task_config`](#social_contracts_agent_task_migrate_task_config)
-  [Function `validate_task_config`](#social_contracts_agent_task_validate_task_config)
-  [Function `emit_config_updated`](#social_contracts_agent_task_emit_config_updated)
-  [Function `create_task`](#social_contracts_agent_task_create_task)
-  [Function `accept_task`](#social_contracts_agent_task_accept_task)
-  [Function `decline_task`](#social_contracts_agent_task_decline_task)
-  [Function `cancel_offer`](#social_contracts_agent_task_cancel_offer)
-  [Function `expire_offer`](#social_contracts_agent_task_expire_offer)
-  [Function `close_offer`](#social_contracts_agent_task_close_offer)
-  [Function `mark_delivered`](#social_contracts_agent_task_mark_delivered)
-  [Function `settle_task`](#social_contracts_agent_task_settle_task)
-  [Function `release_task`](#social_contracts_agent_task_release_task)
-  [Function `finalize_settlement`](#social_contracts_agent_task_finalize_settlement)
-  [Function `pay_seller`](#social_contracts_agent_task_pay_seller)
-  [Function `refund_task`](#social_contracts_agent_task_refund_task)
-  [Function `propose_refund`](#social_contracts_agent_task_propose_refund)
-  [Function `accept_refund`](#social_contracts_agent_task_accept_refund)
-  [Function `reject_refund`](#social_contracts_agent_task_reject_refund)
-  [Function `withdraw_refund`](#social_contracts_agent_task_withdraw_refund)
-  [Function `clear_proposal`](#social_contracts_agent_task_clear_proposal)
-  [Function `close_as_refunded`](#social_contracts_agent_task_close_as_refunded)
-  [Function `pay_buyer_back`](#social_contracts_agent_task_pay_buyer_back)
-  [Function `review_seller`](#social_contracts_agent_task_review_seller)
-  [Function `review_buyer_agent`](#social_contracts_agent_task_review_buyer_agent)
-  [Function `review_buyer_principal`](#social_contracts_agent_task_review_buyer_principal)
-  [Function `record_seller_review`](#social_contracts_agent_task_record_seller_review)
-  [Function `assert_reviewable`](#social_contracts_agent_task_assert_reviewable)
-  [Function `resolve_buyer_actor`](#social_contracts_agent_task_resolve_buyer_actor)
-  [Function `assert_seller_with_cap`](#social_contracts_agent_task_assert_seller_with_cap)
-  [Function `assert_seller_sender`](#social_contracts_agent_task_assert_seller_sender)
-  [Function `resolve_party`](#social_contracts_agent_task_resolve_party)
-  [Function `compute_terms_digest`](#social_contracts_agent_task_compute_terms_digest)
-  [Function `status_offered`](#social_contracts_agent_task_status_offered)
-  [Function `status_accepted`](#social_contracts_agent_task_status_accepted)
-  [Function `status_delivered`](#social_contracts_agent_task_status_delivered)
-  [Function `status_settled`](#social_contracts_agent_task_status_settled)
-  [Function `status_refunded`](#social_contracts_agent_task_status_refunded)
-  [Function `status_cancelled`](#social_contracts_agent_task_status_cancelled)
-  [Function `status`](#social_contracts_agent_task_status)
-  [Function `close_kind`](#social_contracts_agent_task_close_kind)
-  [Function `settlement_kind`](#social_contracts_agent_task_settlement_kind)
-  [Function `amount_mist`](#social_contracts_agent_task_amount_mist)
-  [Function `escrowed_mist`](#social_contracts_agent_task_escrowed_mist)
-  [Function `buyer_principal`](#social_contracts_agent_task_buyer_principal)
-  [Function `seller_principal`](#social_contracts_agent_task_seller_principal)
-  [Function `buyer_agent_id`](#social_contracts_agent_task_buyer_agent_id)
-  [Function `seller_agent_id`](#social_contracts_agent_task_seller_agent_id)
-  [Function `terms_digest`](#social_contracts_agent_task_terms_digest)
-  [Function `platform_fee_bps`](#social_contracts_agent_task_platform_fee_bps)
-  [Function `ecosystem_fee_bps`](#social_contracts_agent_task_ecosystem_fee_bps)
-  [Function `accept_deadline_ms`](#social_contracts_agent_task_accept_deadline_ms)
-  [Function `delivery_deadline_ms`](#social_contracts_agent_task_delivery_deadline_ms)
-  [Function `settle_deadline_ms`](#social_contracts_agent_task_settle_deadline_ms)
-  [Function `review_deadline_ms`](#social_contracts_agent_task_review_deadline_ms)
-  [Function `settled_at_ms`](#social_contracts_agent_task_settled_at_ms)
-  [Function `platform_fee_mist`](#social_contracts_agent_task_platform_fee_mist)
-  [Function `ecosystem_fee_mist`](#social_contracts_agent_task_ecosystem_fee_mist)
-  [Function `net_amount_mist`](#social_contracts_agent_task_net_amount_mist)
-  [Function `refund_proposer`](#social_contracts_agent_task_refund_proposer)
-  [Function `buyer_credibility_bps`](#social_contracts_agent_task_buyer_credibility_bps)
-  [Function `seller_credibility_bps`](#social_contracts_agent_task_seller_credibility_bps)
-  [Function `buyer_review_stars`](#social_contracts_agent_task_buyer_review_stars)
-  [Function `seller_review_stars`](#social_contracts_agent_task_seller_review_stars)
-  [Function `config_active`](#social_contracts_agent_task_config_active)
-  [Function `config_release_grace_ms`](#social_contracts_agent_task_config_release_grace_ms)
-  [Function `config_review_window_ms`](#social_contracts_agent_task_config_review_window_ms)
-  [Function `config_min_escrow_mist`](#social_contracts_agent_task_config_min_escrow_mist)
-  [Function `config_reputation`](#social_contracts_agent_task_config_reputation)


<pre><code><b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
<b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
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
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
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



<a name="social_contracts_agent_task_TaskMarketCap"></a>

## Struct `TaskMarketCap`

Admin capability for [<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a></code>].


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskMarketCap">TaskMarketCap</a> <b>has</b> key, store
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

<a name="social_contracts_agent_task_TaskConfig"></a>

## Struct `TaskConfig`

Shared market configuration. Pausing (<code>active = <b>false</b></code>) blocks only new offers.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a> <b>has</b> key
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
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee_recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>min_escrow_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accept_window_min_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accept_window_default_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accept_window_max_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delivery_window_min_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delivery_window_max_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>settle_window_min_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>settle_window_max_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>release_grace_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>review_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_title_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_uri_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reputation: <a href="../social_contracts/memory.md#social_contracts_memory_ReputationParams">social_contracts::memory::ReputationParams</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskTerms"></a>

## Struct `TaskTerms`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskTerms">TaskTerms</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee_recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>accept_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delivery_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>settle_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskPayout"></a>

## Struct `TaskPayout`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskPayout">TaskPayout</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_mist">platform_fee_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_mist">ecosystem_fee_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_net_amount_mist">net_amount_mist</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskReviewState"></a>

## Struct `TaskReviewState`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskReviewState">TaskReviewState</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_review_stars">buyer_review_stars</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_review_stars">seller_review_stars</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskEscrow"></a>

## Struct `TaskEscrow`

One escrowed task. Shared; the rarely-touched groups live in the nested structs above.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a> <b>has</b> key
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
<code>buyer_memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seller_memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
 Acting organization of the buyer, for reporting only.
</dd>
<dt>
<code>room_group_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Platform whose treasury receives the platform fee; <code>none</code> folds it into the ecosystem fee.
</dd>
<dt>
<code>title: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>scope_digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>escrowed: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_settlement_kind">settlement_kind</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>terms: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskTerms">social_contracts::agent_task::TaskTerms</a></code>
</dt>
<dd>
 Terms frozen at offer time; a later config change never touches them.
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accepted_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delivered_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_settled_at_ms">settled_at_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>deliverable_digest: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>deliverable_uri: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>payout: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskPayout">social_contracts::agent_task::TaskPayout</a></code>
</dt>
<dd>
 Amounts paid out at settlement.
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>refund_proposed_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reviews: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskReviewState">social_contracts::agent_task::TaskReviewState</a></code>
</dt>
<dd>
 Credibility snapshots and the stars each side gave.
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskOffered"></a>

## Struct `TaskOffered`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskOffered">TaskOffered</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>task_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>buyer_memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>seller_memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>room_group_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>title: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>scope_digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accept_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delivery_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>settle_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a>: u64</code>
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

<a name="social_contracts_agent_task_TaskAccepted"></a>

## Struct `TaskAccepted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskAccepted">TaskAccepted</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>task_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>accepted_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskCancelled"></a>

## Struct `TaskCancelled`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskCancelled">TaskCancelled</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>task_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>cancelled_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskDelivered"></a>

## Struct `TaskDelivered`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskDelivered">TaskDelivered</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>task_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>deliverable_digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>deliverable_uri: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>delivered_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskRefundProposed"></a>

## Struct `TaskRefundProposed`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskRefundProposed">TaskRefundProposed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>task_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>proposer_role: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>proposed_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskRefundProposalCleared"></a>

## Struct `TaskRefundProposalCleared`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskRefundProposalCleared">TaskRefundProposalCleared</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>task_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>cleared_by_role: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>reason: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>cleared_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskRefunded"></a>

## Struct `TaskRefunded`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskRefunded">TaskRefunded</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>task_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>refund_kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>refunded_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskReceiptSettled"></a>

## Struct `TaskReceiptSettled`

<code>receipt_id</code> equals <code>task_id</code>: one receipt per settled task.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskReceiptSettled">TaskReceiptSettled</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>task_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>receipt_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>room_group_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_settlement_kind">settlement_kind</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_mist">platform_fee_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_mist">ecosystem_fee_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_net_amount_mist">net_amount_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>deliverable_digest: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>deliverable_uri: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_settled_at_ms">settled_at_ms</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskReviewSubmitted"></a>

## Struct `TaskReviewSubmitted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskReviewSubmitted">TaskReviewSubmitted</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>task_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>receipt_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>rater_role: u8</code>
</dt>
<dd>
 1 buyer reviewed the seller, 2 seller reviewed the buyer.
</dd>
<dt>
<code>rater_principal: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>rater_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>reviewee_principal: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reviewee_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>stars: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>rater_credibility_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>value_weight_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base_weight_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>prior_pair_reviews: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>weight_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reviewed_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_agent_task_TaskConfigUpdated"></a>

## Struct `TaskConfigUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfigUpdated">TaskConfigUpdated</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee_recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>min_escrow_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accept_window_min_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accept_window_default_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accept_window_max_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delivery_window_min_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delivery_window_max_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>settle_window_min_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>settle_window_max_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>release_grace_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>review_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_title_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_uri_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>prior_mean_milli: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>prior_weight_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>value_ref_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_value_weight_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_value_weight_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>cred_base_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>cred_step_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>cred_cap_n: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>cred_max_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>overall_seller_weight_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>overall_buyer_weight_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_agent_task_STATUS_OFFERED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_OFFERED">STATUS_OFFERED</a>: u8 = 0;
</code></pre>



<a name="social_contracts_agent_task_STATUS_ACCEPTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_ACCEPTED">STATUS_ACCEPTED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_agent_task_STATUS_DELIVERED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_DELIVERED">STATUS_DELIVERED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_agent_task_STATUS_SETTLED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_SETTLED">STATUS_SETTLED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_agent_task_STATUS_REFUNDED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_REFUNDED">STATUS_REFUNDED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_agent_task_STATUS_CANCELLED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_CANCELLED">STATUS_CANCELLED</a>: u8 = 5;
</code></pre>



<a name="social_contracts_agent_task_CLOSE_BUYER_CANCELLED"></a>

<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a></code> of a cancelled offer.


<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLOSE_BUYER_CANCELLED">CLOSE_BUYER_CANCELLED</a>: u8 = 0;
</code></pre>



<a name="social_contracts_agent_task_CLOSE_SELLER_DECLINED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLOSE_SELLER_DECLINED">CLOSE_SELLER_DECLINED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_agent_task_CLOSE_EXPIRED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLOSE_EXPIRED">CLOSE_EXPIRED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_agent_task_SETTLED_BY_BUYER"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_SETTLED_BY_BUYER">SETTLED_BY_BUYER</a>: u8 = 1;
</code></pre>



<a name="social_contracts_agent_task_SETTLED_AUTO_RELEASED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_SETTLED_AUTO_RELEASED">SETTLED_AUTO_RELEASED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_agent_task_REFUND_DELIVERY_MISSED"></a>

<code>refund_kind</code>: the seller never delivered / both parties agreed after delivery.


<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_REFUND_DELIVERY_MISSED">REFUND_DELIVERY_MISSED</a>: u8 = 0;
</code></pre>



<a name="social_contracts_agent_task_REFUND_MUTUAL"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_REFUND_MUTUAL">REFUND_MUTUAL</a>: u8 = 1;
</code></pre>



<a name="social_contracts_agent_task_PARTY_BUYER"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_PARTY_BUYER">PARTY_BUYER</a>: u8 = 1;
</code></pre>



<a name="social_contracts_agent_task_PARTY_SELLER"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_PARTY_SELLER">PARTY_SELLER</a>: u8 = 2;
</code></pre>



<a name="social_contracts_agent_task_CLEARED_WITHDRAWN"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLEARED_WITHDRAWN">CLEARED_WITHDRAWN</a>: u8 = 0;
</code></pre>



<a name="social_contracts_agent_task_CLEARED_REJECTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLEARED_REJECTED">CLEARED_REJECTED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_agent_task_DIGEST_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DIGEST_LENGTH">DIGEST_LENGTH</a>: u64 = 32;
</code></pre>



<a name="social_contracts_agent_task_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>: u64 = 1;
</code></pre>



<a name="social_contracts_agent_task_EMarketPaused"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EMarketPaused">EMarketPaused</a>: u64 = 2;
</code></pre>



<a name="social_contracts_agent_task_EWrongStatus"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>: u64 = 3;
</code></pre>



<a name="social_contracts_agent_task_ETaskSelfDealing"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETaskSelfDealing">ETaskSelfDealing</a>: u64 = 4;
</code></pre>



<a name="social_contracts_agent_task_ENotBuyer"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotBuyer">ENotBuyer</a>: u64 = 5;
</code></pre>



<a name="social_contracts_agent_task_ENotSeller"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotSeller">ENotSeller</a>: u64 = 6;
</code></pre>



<a name="social_contracts_agent_task_EAcceptExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EAcceptExpired">EAcceptExpired</a>: u64 = 7;
</code></pre>



<a name="social_contracts_agent_task_ETermsMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETermsMismatch">ETermsMismatch</a>: u64 = 8;
</code></pre>



<a name="social_contracts_agent_task_EWrongTaskObject"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>: u64 = 9;
</code></pre>



<a name="social_contracts_agent_task_EDeliveryDeadlinePassed"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EDeliveryDeadlinePassed">EDeliveryDeadlinePassed</a>: u64 = 10;
</code></pre>



<a name="social_contracts_agent_task_ETooEarly"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETooEarly">ETooEarly</a>: u64 = 11;
</code></pre>



<a name="social_contracts_agent_task_EInvalidDigest"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidDigest">EInvalidDigest</a>: u64 = 12;
</code></pre>



<a name="social_contracts_agent_task_ETitleTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETitleTooLong">ETitleTooLong</a>: u64 = 13;
</code></pre>



<a name="social_contracts_agent_task_EUriTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EUriTooLong">EUriTooLong</a>: u64 = 14;
</code></pre>



<a name="social_contracts_agent_task_EEscrowTooSmall"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EEscrowTooSmall">EEscrowTooSmall</a>: u64 = 15;
</code></pre>



<a name="social_contracts_agent_task_EInvalidWindow"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidWindow">EInvalidWindow</a>: u64 = 16;
</code></pre>



<a name="social_contracts_agent_task_EInsufficientPayment"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInsufficientPayment">EInsufficientPayment</a>: u64 = 17;
</code></pre>



<a name="social_contracts_agent_task_EInvalidStars"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidStars">EInvalidStars</a>: u64 = 18;
</code></pre>



<a name="social_contracts_agent_task_EReviewWindowClosed"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EReviewWindowClosed">EReviewWindowClosed</a>: u64 = 19;
</code></pre>



<a name="social_contracts_agent_task_EAlreadyReviewed"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EAlreadyReviewed">EAlreadyReviewed</a>: u64 = 20;
</code></pre>



<a name="social_contracts_agent_task_EWrongReviewTarget"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongReviewTarget">EWrongReviewTarget</a>: u64 = 21;
</code></pre>



<a name="social_contracts_agent_task_EProposalPending"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EProposalPending">EProposalPending</a>: u64 = 22;
</code></pre>



<a name="social_contracts_agent_task_ENoProposal"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENoProposal">ENoProposal</a>: u64 = 23;
</code></pre>



<a name="social_contracts_agent_task_ENotCounterparty"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotCounterparty">ENotCounterparty</a>: u64 = 24;
</code></pre>



<a name="social_contracts_agent_task_ENotProposer"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotProposer">ENotProposer</a>: u64 = 25;
</code></pre>



<a name="social_contracts_agent_task_EInvalidTaskConfig"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>: u64 = 26;
</code></pre>



<a name="social_contracts_agent_task_EWrongPlatform"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongPlatform">EWrongPlatform</a>: u64 = 27;
</code></pre>



<a name="social_contracts_agent_task_EAccountInactive"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EAccountInactive">EAccountInactive</a>: u64 = 28;
</code></pre>



<a name="social_contracts_agent_task_DEFAULT_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>: u64 = 200;
</code></pre>



<a name="social_contracts_agent_task_DEFAULT_ECOSYSTEM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>: u64 = 50;
</code></pre>



<a name="social_contracts_agent_task_DEFAULT_MIN_ESCROW_MIST"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_MIN_ESCROW_MIST">DEFAULT_MIN_ESCROW_MIST</a>: u64 = 10000000;
</code></pre>



<a name="social_contracts_agent_task_MS_MINUTE"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_MINUTE">MS_MINUTE</a>: u64 = 60000;
</code></pre>



<a name="social_contracts_agent_task_MS_HOUR"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_HOUR">MS_HOUR</a>: u64 = 3600000;
</code></pre>



<a name="social_contracts_agent_task_MS_DAY"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_DAY">MS_DAY</a>: u64 = 86400000;
</code></pre>



<a name="social_contracts_agent_task_DEFAULT_RELEASE_GRACE_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_RELEASE_GRACE_MS">DEFAULT_RELEASE_GRACE_MS</a>: u64 = 259200000;
</code></pre>



<a name="social_contracts_agent_task_DEFAULT_REVIEW_WINDOW_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_REVIEW_WINDOW_MS">DEFAULT_REVIEW_WINDOW_MS</a>: u64 = 2592000000;
</code></pre>



<a name="social_contracts_agent_task_DEFAULT_MAX_TITLE_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_MAX_TITLE_BYTES">DEFAULT_MAX_TITLE_BYTES</a>: u64 = 128;
</code></pre>



<a name="social_contracts_agent_task_DEFAULT_MAX_URI_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_MAX_URI_BYTES">DEFAULT_MAX_URI_BYTES</a>: u64 = 2048;
</code></pre>



<a name="social_contracts_agent_task_MAX_RELEASE_GRACE_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MAX_RELEASE_GRACE_MS">MAX_RELEASE_GRACE_MS</a>: u64 = 2592000000;
</code></pre>



<a name="social_contracts_agent_task_MAX_REVIEW_WINDOW_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MAX_REVIEW_WINDOW_MS">MAX_REVIEW_WINDOW_MS</a>: u64 = 31536000000;
</code></pre>



<a name="social_contracts_agent_task_BPS_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_BPS_DENOMINATOR">BPS_DENOMINATOR</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_agent_task_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_bootstrap_init">bootstrap_init</a>(admin: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_bootstrap_init">bootstrap_init</a>(admin: <b>address</b>, clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> config = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a> {
        id: object::new(ctx),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_PLATFORM_FEE_BPS">DEFAULT_PLATFORM_FEE_BPS</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_ECOSYSTEM_FEE_BPS">DEFAULT_ECOSYSTEM_FEE_BPS</a>,
        ecosystem_fee_recipient: admin,
        min_escrow_mist: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_MIN_ESCROW_MIST">DEFAULT_MIN_ESCROW_MIST</a>,
        accept_window_min_ms: 5 * <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_MINUTE">MS_MINUTE</a>,
        accept_window_default_ms: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_DAY">MS_DAY</a>,
        accept_window_max_ms: 14 * <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_DAY">MS_DAY</a>,
        delivery_window_min_ms: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_HOUR">MS_HOUR</a>,
        delivery_window_max_ms: 90 * <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_DAY">MS_DAY</a>,
        settle_window_min_ms: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_HOUR">MS_HOUR</a>,
        settle_window_max_ms: 60 * <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MS_DAY">MS_DAY</a>,
        release_grace_ms: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_RELEASE_GRACE_MS">DEFAULT_RELEASE_GRACE_MS</a>,
        review_window_ms: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_REVIEW_WINDOW_MS">DEFAULT_REVIEW_WINDOW_MS</a>,
        max_title_bytes: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_MAX_TITLE_BYTES">DEFAULT_MAX_TITLE_BYTES</a>,
        max_uri_bytes: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DEFAULT_MAX_URI_BYTES">DEFAULT_MAX_URI_BYTES</a>,
        active: <b>true</b>,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        reputation: <a href="../social_contracts/memory.md#social_contracts_memory_new_reputation_params">memory::new_reputation_params</a>(
            3000,
            50000,
            1_000_000_000,
            500,
            20000,
            2000,
            500,
            16,
            10000,
            5000,
            5000,
        ),
    };
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_validate_task_config">validate_task_config</a>(&config);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_emit_config_updated">emit_config_updated</a>(&config, admin, clock);
    transfer::share_object(config);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_create_task_market_cap"></a>

## Function `create_task_market_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_create_task_market_cap">create_task_market_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskMarketCap">social_contracts::agent_task::TaskMarketCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_create_task_market_cap">create_task_market_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskMarketCap">TaskMarketCap</a> {
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskMarketCap">TaskMarketCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="social_contracts_agent_task_set_task_config"></a>

## Function `set_task_config`

Replaces the whole configuration. Every bound is re-validated; a violation aborts.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_set_task_config">set_task_config</a>(_: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskMarketCap">social_contracts::agent_task::TaskMarketCap</a>, config: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: u64, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: u64, ecosystem_fee_recipient: <b>address</b>, min_escrow_mist: u64, accept_window_min_ms: u64, accept_window_default_ms: u64, accept_window_max_ms: u64, delivery_window_min_ms: u64, delivery_window_max_ms: u64, settle_window_min_ms: u64, settle_window_max_ms: u64, release_grace_ms: u64, review_window_ms: u64, max_title_bytes: u64, max_uri_bytes: u64, active: bool, prior_mean_milli: u64, prior_weight_bps: u64, value_ref_mist: u64, min_value_weight_bps: u64, max_value_weight_bps: u64, cred_base_bps: u64, cred_step_bps: u64, cred_cap_n: u64, cred_max_bps: u64, overall_seller_weight_bps: u64, overall_buyer_weight_bps: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_set_task_config">set_task_config</a>(
    _: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskMarketCap">TaskMarketCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: u64,
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: u64,
    ecosystem_fee_recipient: <b>address</b>,
    min_escrow_mist: u64,
    accept_window_min_ms: u64,
    accept_window_default_ms: u64,
    accept_window_max_ms: u64,
    delivery_window_min_ms: u64,
    delivery_window_max_ms: u64,
    settle_window_min_ms: u64,
    settle_window_max_ms: u64,
    release_grace_ms: u64,
    review_window_ms: u64,
    max_title_bytes: u64,
    max_uri_bytes: u64,
    active: bool,
    prior_mean_milli: u64,
    prior_weight_bps: u64,
    value_ref_mist: u64,
    min_value_weight_bps: u64,
    max_value_weight_bps: u64,
    cred_base_bps: u64,
    cred_step_bps: u64,
    cred_cap_n: u64,
    cred_max_bps: u64,
    overall_seller_weight_bps: u64,
    overall_buyer_weight_bps: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    config.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a> = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>;
    config.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a> = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>;
    config.ecosystem_fee_recipient = ecosystem_fee_recipient;
    config.min_escrow_mist = min_escrow_mist;
    config.accept_window_min_ms = accept_window_min_ms;
    config.accept_window_default_ms = accept_window_default_ms;
    config.accept_window_max_ms = accept_window_max_ms;
    config.delivery_window_min_ms = delivery_window_min_ms;
    config.delivery_window_max_ms = delivery_window_max_ms;
    config.settle_window_min_ms = settle_window_min_ms;
    config.settle_window_max_ms = settle_window_max_ms;
    config.release_grace_ms = release_grace_ms;
    config.review_window_ms = review_window_ms;
    config.max_title_bytes = max_title_bytes;
    config.max_uri_bytes = max_uri_bytes;
    config.active = active;
    // Validates the reputation bounds (aborts `EInvalidReputationParams`).
    config.reputation = <a href="../social_contracts/memory.md#social_contracts_memory_new_reputation_params">memory::new_reputation_params</a>(
        prior_mean_milli,
        prior_weight_bps,
        value_ref_mist,
        min_value_weight_bps,
        max_value_weight_bps,
        cred_base_bps,
        cred_step_bps,
        cred_cap_n,
        cred_max_bps,
        overall_seller_weight_bps,
        overall_buyer_weight_bps,
    );
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_validate_task_config">validate_task_config</a>(config);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_emit_config_updated">emit_config_updated</a>(config, tx_context::sender(ctx), clock);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_migrate_task_config"></a>

## Function `migrate_task_config`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_migrate_task_config">migrate_task_config</a>(config: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_migrate_task_config">migrate_task_config</a>(
    config: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(config.version &lt; current_version, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = config.version;
    config.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(config),
        string::utf8(b"<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>"),
        old_version,
        tx_context::sender(ctx),
    );
}
</code></pre>



</details>

<a name="social_contracts_agent_task_validate_task_config"></a>

## Function `validate_task_config`

Non-reputation bounds; reputation bounds live in <code><a href="../social_contracts/memory.md#social_contracts_memory_assert_reputation_params">memory::assert_reputation_params</a></code>.


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_validate_task_config">validate_task_config</a>(c: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_validate_task_config">validate_task_config</a>(c: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>) {
    <b>assert</b>!(c.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a> &lt;= <a href="../social_contracts/agent_task.md#social_contracts_agent_task_BPS_DENOMINATOR">BPS_DENOMINATOR</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>);
    <b>assert</b>!(c.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a> &lt;= <a href="../social_contracts/agent_task.md#social_contracts_agent_task_BPS_DENOMINATOR">BPS_DENOMINATOR</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>);
    <b>assert</b>!(c.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a> + c.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a> &lt;= <a href="../social_contracts/agent_task.md#social_contracts_agent_task_BPS_DENOMINATOR">BPS_DENOMINATOR</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>);
    <b>assert</b>!(c.min_escrow_mist &gt;= 1, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>);
    <b>assert</b>!(
        c.accept_window_min_ms &gt; 0
            && c.accept_window_min_ms &lt;= c.accept_window_default_ms
            && c.accept_window_default_ms &lt;= c.accept_window_max_ms,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>,
    );
    <b>assert</b>!(
        c.delivery_window_min_ms &gt; 0 && c.delivery_window_min_ms &lt;= c.delivery_window_max_ms,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>,
    );
    <b>assert</b>!(
        c.settle_window_min_ms &gt; 0 && c.settle_window_min_ms &lt;= c.settle_window_max_ms,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>,
    );
    <b>assert</b>!(c.release_grace_ms &lt;= <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MAX_RELEASE_GRACE_MS">MAX_RELEASE_GRACE_MS</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>);
    <b>assert</b>!(c.review_window_ms &gt; 0 && c.review_window_ms &lt;= <a href="../social_contracts/agent_task.md#social_contracts_agent_task_MAX_REVIEW_WINDOW_MS">MAX_REVIEW_WINDOW_MS</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>);
    <b>assert</b>!(c.max_title_bytes &gt;= 1 && c.max_uri_bytes &gt;= 1, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_reputation_params">memory::assert_reputation_params</a>(&c.reputation);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_emit_config_updated"></a>

## Function `emit_config_updated`



<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_emit_config_updated">emit_config_updated</a>(c: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, updated_by: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_emit_config_updated">emit_config_updated</a>(c: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>, updated_by: <b>address</b>, clock: &Clock) {
    <b>let</b> p = &c.reputation;
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfigUpdated">TaskConfigUpdated</a> {
        updated_by,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: c.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: c.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>,
        ecosystem_fee_recipient: c.ecosystem_fee_recipient,
        min_escrow_mist: c.min_escrow_mist,
        accept_window_min_ms: c.accept_window_min_ms,
        accept_window_default_ms: c.accept_window_default_ms,
        accept_window_max_ms: c.accept_window_max_ms,
        delivery_window_min_ms: c.delivery_window_min_ms,
        delivery_window_max_ms: c.delivery_window_max_ms,
        settle_window_min_ms: c.settle_window_min_ms,
        settle_window_max_ms: c.settle_window_max_ms,
        release_grace_ms: c.release_grace_ms,
        review_window_ms: c.review_window_ms,
        max_title_bytes: c.max_title_bytes,
        max_uri_bytes: c.max_uri_bytes,
        active: c.active,
        prior_mean_milli: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_prior_mean_milli">memory::reputation_prior_mean_milli</a>(p),
        prior_weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_prior_weight_bps">memory::reputation_prior_weight_bps</a>(p),
        value_ref_mist: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_value_ref_mist">memory::reputation_value_ref_mist</a>(p),
        min_value_weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_min_value_weight_bps">memory::reputation_min_value_weight_bps</a>(p),
        max_value_weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_max_value_weight_bps">memory::reputation_max_value_weight_bps</a>(p),
        cred_base_bps: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_cred_base_bps">memory::reputation_cred_base_bps</a>(p),
        cred_step_bps: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_cred_step_bps">memory::reputation_cred_step_bps</a>(p),
        cred_cap_n: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_cred_cap_n">memory::reputation_cred_cap_n</a>(p),
        cred_max_bps: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_cred_max_bps">memory::reputation_cred_max_bps</a>(p),
        overall_seller_weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_overall_seller_weight_bps">memory::reputation_overall_seller_weight_bps</a>(p),
        overall_buyer_weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_reputation_overall_buyer_weight_bps">memory::reputation_overall_buyer_weight_bps</a>(p),
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_agent_task_create_task"></a>

## Function `create_task`

Escrows <code>escrow_amount</code> from <code>payment</code> as an offer to <code>seller_agent</code>.

The buyer is resolved with <code>CAP_TASK_TRADE</code> and the spend cap (<code>escrow_amount</code>), so an
agent's per-action limit and approval-required caps apply to funding an offer.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_create_task">create_task</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, escrow_amount: u64, scope_digest: vector&lt;u8&gt;, title: <a href="../std/string.md#std_string_String">std::string::String</a>, accept_window_ms: u64, delivery_window_ms: u64, settle_window_ms: u64, room_group_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_create_task">create_task</a>(
    config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    memory_config: &MemoryConfig,
    buyer_account: &MemoryAccount,
    seller_account: &MemoryAccount,
    seller_agent: &SubAgent,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    escrow_amount: u64,
    scope_digest: vector&lt;u8&gt;,
    title: String,
    accept_window_ms: u64,
    delivery_window_ms: u64,
    settle_window_ms: u64,
    room_group_id: Option&lt;ID&gt;,
    platform_id: Option&lt;<b>address</b>&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(config.active, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EMarketPaused">EMarketPaused</a>);
    <b>assert</b>!(escrow_amount &gt;= config.min_escrow_mist, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EEscrowTooSmall">EEscrowTooSmall</a>);
    <b>assert</b>!(coin::value(payment) &gt;= escrow_amount, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInsufficientPayment">EInsufficientPayment</a>);
    <b>assert</b>!(vector::length(&scope_digest) == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DIGEST_LENGTH">DIGEST_LENGTH</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidDigest">EInvalidDigest</a>);
    <b>assert</b>!(string::length(&title) &lt;= config.max_title_bytes, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETitleTooLong">ETitleTooLong</a>);
    <b>assert</b>!(
        accept_window_ms &gt;= config.accept_window_min_ms
            && accept_window_ms &lt;= config.accept_window_max_ms,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidWindow">EInvalidWindow</a>,
    );
    <b>assert</b>!(
        delivery_window_ms &gt;= config.delivery_window_min_ms
            && delivery_window_ms &lt;= config.delivery_window_max_ms,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidWindow">EInvalidWindow</a>,
    );
    <b>assert</b>!(
        settle_window_ms &gt;= config.settle_window_min_ms
            && settle_window_ms &lt;= config.settle_window_max_ms,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidWindow">EInvalidWindow</a>,
    );
    // Buyer: principal or an agent with the cap, within its spend limit and approval rules.
    <b>let</b> cap = <a href="../social_contracts/memory.md#social_contracts_memory_cap_task_trade">memory::cap_task_trade</a>();
    <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">memory::resolve_actor_with_cap</a>(
        memory_config,
        buyer_account,
        cap,
        option::none(),
        escrow_amount,
        clock,
        ctx,
    );
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_direct_execution_allowed">memory::assert_direct_execution_allowed</a>(buyer_account, cap, ctx);
    <b>let</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting);
    // Seller: a distinct principal's registered, active agent.
    <b>let</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a> = object::id(seller_agent);
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_memory_account_id">memory::sub_agent_memory_account_id</a>(seller_agent) == object::id(seller_account),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>,
    );
    <b>assert</b>!(object::id(buyer_account) != object::id(seller_account), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETaskSelfDealing">ETaskSelfDealing</a>);
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">memory::owner</a>(buyer_account) != <a href="../social_contracts/memory.md#social_contracts_memory_owner">memory::owner</a>(seller_account),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETaskSelfDealing">ETaskSelfDealing</a>,
    );
    <b>assert</b>!(
        !option::contains(&<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>, &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETaskSelfDealing">ETaskSelfDealing</a>,
    );
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_is_active">memory::is_active</a>(seller_account), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EAccountInactive">EAccountInactive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(seller_agent, clock);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a> = <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting);
    <b>let</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a> = <a href="../social_contracts/memory.md#social_contracts_memory_owner">memory::owner</a>(seller_account);
    <b>let</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a> = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_compute_terms_digest">compute_terms_digest</a>(
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>,
        escrow_amount,
        &scope_digest,
        accept_window_ms,
        delivery_window_ms,
        settle_window_ms,
        config.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>,
        config.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>,
        config.ecosystem_fee_recipient,
        &platform_id,
    );
    <b>let</b> escrowed = coin::into_balance(coin::split(payment, escrow_amount, ctx));
    <b>let</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a> = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, accept_window_ms);
    <b>let</b> task = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a> {
        id: object::new(ctx),
        buyer_memory_account_id: object::id(buyer_account),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>,
        seller_memory_account_id: object::id(seller_account),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>,
        organization_id: <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">memory::acting_organization_id</a>(&acting),
        room_group_id,
        platform_id,
        title,
        scope_digest,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: escrow_amount,
        escrowed,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a>: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_OFFERED">STATUS_OFFERED</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a>: 0,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settlement_kind">settlement_kind</a>: 0,
        terms: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskTerms">TaskTerms</a> {
            <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: config.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>,
            <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: config.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>,
            ecosystem_fee_recipient: config.ecosystem_fee_recipient,
            accept_window_ms,
            delivery_window_ms,
            settle_window_ms,
            <a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a>,
        },
        created_at_ms: now,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a>,
        accepted_at_ms: 0,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a>: 0,
        delivered_at_ms: 0,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>: 0,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settled_at_ms">settled_at_ms</a>: 0,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a>: 0,
        deliverable_digest: option::none(),
        deliverable_uri: option::none(),
        payout: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskPayout">TaskPayout</a> { <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_mist">platform_fee_mist</a>: 0, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_mist">ecosystem_fee_mist</a>: 0, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_net_amount_mist">net_amount_mist</a>: 0 },
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a>: 0,
        refund_proposed_at_ms: 0,
        reviews: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskReviewState">TaskReviewState</a> {
            <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a>: 0,
            <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>: 0,
            <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_review_stars">buyer_review_stars</a>: 0,
            <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_review_stars">seller_review_stars</a>: 0,
        },
    };
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskOffered">TaskOffered</a> {
        task_id: object::id(&task),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>,
        buyer_memory_account_id: task.buyer_memory_account_id,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>,
        seller_memory_account_id: task.seller_memory_account_id,
        organization_id: task.organization_id,
        room_group_id,
        platform_id,
        title: task.title,
        scope_digest: task.scope_digest,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: escrow_amount,
        accept_window_ms,
        delivery_window_ms,
        settle_window_ms,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: task.terms.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: task.terms.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a>: task.terms.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a>,
        created_at_ms: now,
    });
    transfer::share_object(task);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_accept_task"></a>

## Function `accept_task`

The seller approves the exact terms. <code>expected_terms_digest</code> must equal the offer's
<code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a></code>, so a seller cannot accept different scope, payment or windows by mistake.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_task">accept_task</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, seller_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, expected_terms_digest: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_task">accept_task</a>(
    config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    seller_account: &MemoryAccount,
    seller_agent: &SubAgent,
    expected_terms_digest: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_OFFERED">STATUS_OFFERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt;= task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EAcceptExpired">EAcceptExpired</a>);
    <b>assert</b>!(expected_terms_digest == task.terms.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETermsMismatch">ETermsMismatch</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_seller_with_cap">assert_seller_with_cap</a>(task, memory_config, seller_account, seller_agent, clock, ctx);
    // Accepting is taking on new work; deactivated or expired agents cannot.
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_is_active">memory::is_active</a>(seller_account), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EAccountInactive">EAccountInactive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(seller_agent, clock);
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_ACCEPTED">STATUS_ACCEPTED</a>;
    task.accepted_at_ms = now;
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a> = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, task.terms.delivery_window_ms);
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskAccepted">TaskAccepted</a> {
        task_id: object::id(task),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>,
        accepted_at_ms: now,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_agent_task_decline_task"></a>

## Function `decline_task`

The seller turns an offer down. No <code>active</code> check: a deactivated seller can still decline.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_decline_task">decline_task</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, seller_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_decline_task">decline_task</a>(
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    seller_account: &MemoryAccount,
    seller_agent: &SubAgent,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_OFFERED">STATUS_OFFERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_seller_with_cap">assert_seller_with_cap</a>(task, memory_config, seller_account, seller_agent, clock, ctx);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_offer">close_offer</a>(task, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLOSE_SELLER_DECLINED">CLOSE_SELLER_DECLINED</a>, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_cancel_offer"></a>

## Function `cancel_offer`

The buyer withdraws an offer the seller has not accepted.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_cancel_offer">cancel_offer</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_cancel_offer">cancel_offer</a>(
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    buyer_account: &MemoryAccount,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_OFFERED">STATUS_OFFERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_buyer_actor">resolve_buyer_actor</a>(task, memory_config, buyer_account, 0, clock, ctx);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_offer">close_offer</a>(task, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLOSE_BUYER_CANCELLED">CLOSE_BUYER_CANCELLED</a>, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_expire_offer"></a>

## Function `expire_offer`

Anyone may return an unanswered offer's escrow once the accept deadline passed.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_expire_offer">expire_offer</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_expire_offer">expire_offer</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>, clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_OFFERED">STATUS_OFFERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(clock::timestamp_ms(clock) &gt; task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETooEarly">ETooEarly</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_offer">close_offer</a>(task, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLOSE_EXPIRED">CLOSE_EXPIRED</a>, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_close_offer"></a>

## Function `close_offer`



<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_offer">close_offer</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a>: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_offer">close_offer</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a>: u8, clock: &Clock, ctx: &<b>mut</b> TxContext) {
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_CANCELLED">STATUS_CANCELLED</a>;
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a> = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a>;
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_pay_buyer_back">pay_buyer_back</a>(task, ctx);
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskCancelled">TaskCancelled</a> {
        task_id: object::id(task),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>,
        cancelled_at_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_agent_task_mark_delivered"></a>

## Function `mark_delivered`

The accepted seller agent records the deliverable. No cap and no <code>active</code> check: an
accepted task can still be delivered after the agent is deactivated.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_mark_delivered">mark_delivered</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, digest: vector&lt;u8&gt;, uri: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_mark_delivered">mark_delivered</a>(
    config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    seller_agent: &SubAgent,
    digest: vector&lt;u8&gt;,
    uri: String,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_ACCEPTED">STATUS_ACCEPTED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(object::id(seller_agent) == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>assert</b>!(
        tx_context::sender(ctx) == <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_derived_address">memory::sub_agent_derived_address</a>(seller_agent),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotSeller">ENotSeller</a>,
    );
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt;= task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EDeliveryDeadlinePassed">EDeliveryDeadlinePassed</a>);
    <b>assert</b>!(vector::length(&digest) == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_DIGEST_LENGTH">DIGEST_LENGTH</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidDigest">EInvalidDigest</a>);
    <b>assert</b>!(string::length(&uri) &lt;= config.max_uri_bytes, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EUriTooLong">EUriTooLong</a>);
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_DELIVERED">STATUS_DELIVERED</a>;
    task.delivered_at_ms = now;
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a> = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, task.terms.settle_window_ms);
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskDelivered">TaskDelivered</a> {
        task_id: object::id(task),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>,
        deliverable_digest: digest,
        deliverable_uri: uri,
        delivered_at_ms: now,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>,
    });
    task.deliverable_digest = option::some(digest);
    task.deliverable_uri = option::some(uri);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_settle_task"></a>

## Function `settle_task`

The buyer pays for delivered work. No clock check: the buyer can settle until a release
or mutual refund lands. Payment carries no rating; reviews are separate calls.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_task">settle_task</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_task">settle_task</a>(
    config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    buyer_account: &<b>mut</b> MemoryAccount,
    seller_account: &<b>mut</b> MemoryAccount,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_DELIVERED">STATUS_DELIVERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_buyer_actor">resolve_buyer_actor</a>(task, memory_config, buyer_account, 0, clock, ctx);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_finalize_settlement">finalize_settlement</a>(config, task, buyer_account, seller_account, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_SETTLED_BY_BUYER">SETTLED_BY_BUYER</a>, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_release_task"></a>

## Function `release_task`

Anyone may pay the seller once <code>settle_deadline + release_grace</code> has passed, whatever
refund proposals are pending. Produces a receipt both parties may review.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_release_task">release_task</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, buyer_account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_release_task">release_task</a>(
    config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    buyer_account: &<b>mut</b> MemoryAccount,
    seller_account: &<b>mut</b> MemoryAccount,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_DELIVERED">STATUS_DELIVERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>let</b> releasable_at = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>, config.release_grace_ms);
    <b>assert</b>!(clock::timestamp_ms(clock) &gt; releasable_at, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETooEarly">ETooEarly</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_finalize_settlement">finalize_settlement</a>(config, task, buyer_account, seller_account, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_SETTLED_AUTO_RELEASED">SETTLED_AUTO_RELEASED</a>, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_finalize_settlement"></a>

## Function `finalize_settlement`



<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_finalize_settlement">finalize_settlement</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, buyer_account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, kind: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_finalize_settlement">finalize_settlement</a>(
    config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    buyer_account: &<b>mut</b> MemoryAccount,
    seller_account: &<b>mut</b> MemoryAccount,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    kind: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(object::id(buyer_account) == task.buyer_memory_account_id, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>assert</b>!(object::id(seller_account) == task.seller_memory_account_id, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>if</b> (option::is_some(&task.platform_id)) {
        <b>assert</b>!(
            object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)) == *option::borrow(&task.platform_id),
            <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongPlatform">EWrongPlatform</a>,
        );
    };
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_pay_seller">pay_seller</a>(task, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, clock, ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a> =
        <a href="../social_contracts/memory.md#social_contracts_memory_record_settlement">memory::record_settlement</a>(buyer_account, task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>, &config.reputation, clock);
    task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a> =
        <a href="../social_contracts/memory.md#social_contracts_memory_record_settlement">memory::record_settlement</a>(seller_account, task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>, &config.reputation, clock);
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_SETTLED">STATUS_SETTLED</a>;
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_settlement_kind">settlement_kind</a> = kind;
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_settled_at_ms">settled_at_ms</a> = now;
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a> = <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(now, config.review_window_ms);
    // A pending refund proposal is void once the task is paid.
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> = 0;
    task.refund_proposed_at_ms = 0;
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskReceiptSettled">TaskReceiptSettled</a> {
        task_id: object::id(task),
        receipt_id: object::id(task),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>,
        organization_id: task.organization_id,
        room_group_id: task.room_group_id,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settlement_kind">settlement_kind</a>: kind,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_mist">platform_fee_mist</a>: task.payout.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_mist">platform_fee_mist</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_mist">ecosystem_fee_mist</a>: task.payout.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_mist">ecosystem_fee_mist</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_net_amount_mist">net_amount_mist</a>: task.payout.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_net_amount_mist">net_amount_mist</a>,
        deliverable_digest: task.deliverable_digest,
        deliverable_uri: task.deliverable_uri,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a>: task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>: task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settled_at_ms">settled_at_ms</a>: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_agent_task_pay_seller"></a>

## Function `pay_seller`

Splits the escrow with the frozen fee bps: platform treasury, ecosystem recipient, seller.
Without a platform the platform share folds into the ecosystem fee.


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_pay_seller">pay_seller</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_pay_seller">pay_seller</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform, clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> amount = task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>;
    <b>let</b> platform_share = <a href="../social_contracts/memory.md#social_contracts_memory_checked_bps_of">memory::checked_bps_of</a>(amount, task.terms.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>);
    <b>let</b> ecosystem_share = <a href="../social_contracts/memory.md#social_contracts_memory_checked_bps_of">memory::checked_bps_of</a>(amount, task.terms.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(platform_share, ecosystem_share) &lt;= amount, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidTaskConfig">EInvalidTaskConfig</a>);
    <b>let</b> has_platform = option::is_some(&task.platform_id);
    <b>let</b> platform_fee = <b>if</b> (has_platform) { platform_share } <b>else</b> { 0 };
    <b>let</b> ecosystem_fee = <b>if</b> (has_platform) {
        ecosystem_share
    } <b>else</b> {
        <a href="../social_contracts/memory.md#social_contracts_memory_checked_add_u64">memory::checked_add_u64</a>(platform_share, ecosystem_share)
    };
    <b>let</b> <b>mut</b> payout = coin::from_balance(balance::withdraw_all(&<b>mut</b> task.escrowed), ctx);
    <b>if</b> (platform_fee &gt; 0) {
        <b>let</b> <b>mut</b> platform_coin = coin::split(&<b>mut</b> payout, platform_fee, ctx);
        <a href="../social_contracts/platform.md#social_contracts_platform_fund_platform_treasury_from_coin">platform::fund_platform_treasury_from_coin</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_coin, platform_fee, clock, ctx);
        <b>if</b> (coin::value(&platform_coin) &gt; 0) {
            // Anything the treasury did not take goes to the seller with the net.
            coin::join(&<b>mut</b> payout, platform_coin);
        } <b>else</b> {
            coin::destroy_zero(platform_coin);
        };
    };
    <b>if</b> (ecosystem_fee &gt; 0) {
        transfer::public_transfer(
            coin::split(&<b>mut</b> payout, ecosystem_fee, ctx),
            task.terms.ecosystem_fee_recipient,
        );
    };
    task.payout.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_mist">platform_fee_mist</a> = platform_fee;
    task.payout.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_mist">ecosystem_fee_mist</a> = ecosystem_fee;
    task.payout.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_net_amount_mist">net_amount_mist</a> = coin::value(&payout);
    <b>if</b> (coin::value(&payout) &gt; 0) {
        transfer::public_transfer(payout, task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>);
    } <b>else</b> {
        coin::destroy_zero(payout);
    };
}
</code></pre>



</details>

<a name="social_contracts_agent_task_refund_task"></a>

## Function `refund_task`

Anyone may refund an **accepted** task whose seller missed the delivery deadline.
It aborts on OFFERED (expire/cancel/decline) and on DELIVERED (settle, release, or a
mutual refund), so a delivered task can never be refunded unilaterally.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_task">refund_task</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_task">refund_task</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>, clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_ACCEPTED">STATUS_ACCEPTED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(clock::timestamp_ms(clock) &gt; task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ETooEarly">ETooEarly</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_as_refunded">close_as_refunded</a>(task, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_REFUND_DELIVERY_MISSED">REFUND_DELIVERY_MISSED</a>, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_propose_refund"></a>

## Function `propose_refund`

Either party proposes a full refund of a delivered task. A party can refresh its own
proposal but cannot override the counterparty's pending one.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_propose_refund">propose_refund</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_propose_refund">propose_refund</a>(
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    buyer_account: &MemoryAccount,
    seller_agent: &SubAgent,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_DELIVERED">STATUS_DELIVERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>let</b> role = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_party">resolve_party</a>(task, memory_config, buyer_account, seller_agent, clock, ctx);
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> == 0 || task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> == role, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EProposalPending">EProposalPending</a>);
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> = role;
    task.refund_proposed_at_ms = clock::timestamp_ms(clock);
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskRefundProposed">TaskRefundProposed</a> {
        task_id: object::id(task),
        proposer_role: role,
        proposed_at_ms: task.refund_proposed_at_ms,
    });
}
</code></pre>



</details>

<a name="social_contracts_agent_task_accept_refund"></a>

## Function `accept_refund`

The counterparty accepts: full escrow back to the buyer, no fees.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_refund">accept_refund</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_refund">accept_refund</a>(
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    buyer_account: &MemoryAccount,
    seller_agent: &SubAgent,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_DELIVERED">STATUS_DELIVERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> != 0, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENoProposal">ENoProposal</a>);
    <b>let</b> role = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_party">resolve_party</a>(task, memory_config, buyer_account, seller_agent, clock, ctx);
    <b>assert</b>!(role != task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotCounterparty">ENotCounterparty</a>);
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> = 0;
    task.refund_proposed_at_ms = 0;
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_as_refunded">close_as_refunded</a>(task, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_REFUND_MUTUAL">REFUND_MUTUAL</a>, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_reject_refund"></a>

## Function `reject_refund`

The counterparty turns a proposal down.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_reject_refund">reject_refund</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_reject_refund">reject_refund</a>(
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    buyer_account: &MemoryAccount,
    seller_agent: &SubAgent,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_DELIVERED">STATUS_DELIVERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> != 0, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENoProposal">ENoProposal</a>);
    <b>let</b> role = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_party">resolve_party</a>(task, memory_config, buyer_account, seller_agent, clock, ctx);
    <b>assert</b>!(role != task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotCounterparty">ENotCounterparty</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_clear_proposal">clear_proposal</a>(task, role, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLEARED_REJECTED">CLEARED_REJECTED</a>, clock);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_withdraw_refund"></a>

## Function `withdraw_refund`

The proposer takes its proposal back.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_withdraw_refund">withdraw_refund</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_withdraw_refund">withdraw_refund</a>(
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    buyer_account: &MemoryAccount,
    seller_agent: &SubAgent,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_DELIVERED">STATUS_DELIVERED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> != 0, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENoProposal">ENoProposal</a>);
    <b>let</b> role = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_party">resolve_party</a>(task, memory_config, buyer_account, seller_agent, clock, ctx);
    <b>assert</b>!(role == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotProposer">ENotProposer</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_clear_proposal">clear_proposal</a>(task, role, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_CLEARED_WITHDRAWN">CLEARED_WITHDRAWN</a>, clock);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_clear_proposal"></a>

## Function `clear_proposal`



<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_clear_proposal">clear_proposal</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, by_role: u8, reason: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_clear_proposal">clear_proposal</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>, by_role: u8, reason: u8, clock: &Clock) {
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> = 0;
    task.refund_proposed_at_ms = 0;
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskRefundProposalCleared">TaskRefundProposalCleared</a> {
        task_id: object::id(task),
        cleared_by_role: by_role,
        reason,
        cleared_at_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_agent_task_close_as_refunded"></a>

## Function `close_as_refunded`



<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_as_refunded">close_as_refunded</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, refund_kind: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_as_refunded">close_as_refunded</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>, refund_kind: u8, clock: &Clock, ctx: &<b>mut</b> TxContext) {
    task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> = <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_REFUNDED">STATUS_REFUNDED</a>;
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_pay_buyer_back">pay_buyer_back</a>(task, ctx);
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskRefunded">TaskRefunded</a> {
        task_id: object::id(task),
        refund_kind,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>,
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>,
        refunded_at_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_agent_task_pay_buyer_back"></a>

## Function `pay_buyer_back`



<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_pay_buyer_back">pay_buyer_back</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_pay_buyer_back">pay_buyer_back</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> refund = coin::from_balance(balance::withdraw_all(&<b>mut</b> task.escrowed), ctx);
    transfer::public_transfer(refund, task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_review_seller"></a>

## Function `review_seller`

The buyer rates the seller agent 1-5 stars. Updates the agent's seller-role aggregate.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_seller">review_seller</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, stars: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_seller">review_seller</a>(
    config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    buyer_account: &MemoryAccount,
    seller_account: &<b>mut</b> MemoryAccount,
    seller_agent: &<b>mut</b> SubAgent,
    stars: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_reviewable">assert_reviewable</a>(task, stars, clock);
    <b>assert</b>!(task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_review_stars">buyer_review_stars</a> == 0, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EAlreadyReviewed">EAlreadyReviewed</a>);
    <b>assert</b>!(object::id(seller_agent) == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>assert</b>!(object::id(seller_account) == task.seller_memory_account_id, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_buyer_actor">resolve_buyer_actor</a>(task, memory_config, buyer_account, 0, clock, ctx);
    <b>let</b> outcome = <a href="../social_contracts/memory.md#social_contracts_memory_apply_seller_review">memory::apply_seller_review</a>(
        seller_agent,
        seller_account,
        task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>,
        stars,
        task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>,
        task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a>,
        &config.reputation,
        object::id(task),
        clock,
    );
    task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_review_stars">buyer_review_stars</a> = stars;
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskReviewSubmitted">TaskReviewSubmitted</a> {
        task_id: object::id(task),
        receipt_id: object::id(task),
        rater_role: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_PARTY_BUYER">PARTY_BUYER</a>,
        rater_principal: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>,
        rater_agent_id: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>,
        reviewee_principal: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>,
        reviewee_agent_id: option::some(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>),
        stars,
        rater_credibility_bps: task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a>,
        value_weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_outcome_value_weight_bps">memory::outcome_value_weight_bps</a>(&outcome),
        base_weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_outcome_base_weight_bps">memory::outcome_base_weight_bps</a>(&outcome),
        prior_pair_reviews: <a href="../social_contracts/memory.md#social_contracts_memory_outcome_prior_pair_reviews">memory::outcome_prior_pair_reviews</a>(&outcome),
        weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_outcome_weight_bps">memory::outcome_weight_bps</a>(&outcome),
        reviewed_at_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_agent_task_review_buyer_agent"></a>

## Function `review_buyer_agent`

The seller rates a buyer **agent**. Aborts <code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongReviewTarget">EWrongReviewTarget</a></code> for a direct-buyer task.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_buyer_agent">review_buyer_agent</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, buyer_account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, buyer_agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, stars: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_buyer_agent">review_buyer_agent</a>(
    config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    buyer_account: &<b>mut</b> MemoryAccount,
    buyer_agent: &<b>mut</b> SubAgent,
    seller_agent: &SubAgent,
    stars: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_reviewable">assert_reviewable</a>(task, stars, clock);
    <b>assert</b>!(task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_review_stars">seller_review_stars</a> == 0, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EAlreadyReviewed">EAlreadyReviewed</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_seller_sender">assert_seller_sender</a>(task, seller_agent, ctx);
    <b>assert</b>!(
        option::contains(&task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>, &object::id(buyer_agent)),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongReviewTarget">EWrongReviewTarget</a>,
    );
    <b>assert</b>!(object::id(buyer_account) == task.buyer_memory_account_id, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>let</b> outcome = <a href="../social_contracts/memory.md#social_contracts_memory_apply_buyer_agent_review">memory::apply_buyer_agent_review</a>(
        buyer_agent,
        buyer_account,
        task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>,
        stars,
        task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>,
        task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>,
        &config.reputation,
        object::id(task),
        clock,
    );
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_record_seller_review">record_seller_review</a>(task, stars, &outcome, clock);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_review_buyer_principal"></a>

## Function `review_buyer_principal`

The seller rates a principal that bought directly. Aborts <code><a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongReviewTarget">EWrongReviewTarget</a></code> when the
task had a buyer agent.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_buyer_principal">review_buyer_principal</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>, task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, buyer_account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, stars: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_buyer_principal">review_buyer_principal</a>(
    config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>,
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    buyer_account: &<b>mut</b> MemoryAccount,
    seller_agent: &SubAgent,
    stars: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongVersion">EWrongVersion</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_reviewable">assert_reviewable</a>(task, stars, clock);
    <b>assert</b>!(task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_review_stars">seller_review_stars</a> == 0, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EAlreadyReviewed">EAlreadyReviewed</a>);
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_seller_sender">assert_seller_sender</a>(task, seller_agent, ctx);
    <b>assert</b>!(option::is_none(&task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongReviewTarget">EWrongReviewTarget</a>);
    <b>assert</b>!(object::id(buyer_account) == task.buyer_memory_account_id, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>let</b> outcome = <a href="../social_contracts/memory.md#social_contracts_memory_apply_human_buyer_review">memory::apply_human_buyer_review</a>(
        buyer_account,
        task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>,
        stars,
        task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>,
        task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>,
        &config.reputation,
        object::id(task),
        clock,
    );
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_record_seller_review">record_seller_review</a>(task, stars, &outcome, clock);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_record_seller_review"></a>

## Function `record_seller_review`



<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_record_seller_review">record_seller_review</a>(task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, stars: u8, outcome: &<a href="../social_contracts/memory.md#social_contracts_memory_ReviewOutcome">social_contracts::memory::ReviewOutcome</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_record_seller_review">record_seller_review</a>(
    task: &<b>mut</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    stars: u8,
    outcome: &<a href="../social_contracts/memory.md#social_contracts_memory_ReviewOutcome">memory::ReviewOutcome</a>,
    clock: &Clock,
) {
    task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_review_stars">seller_review_stars</a> = stars;
    event::emit(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskReviewSubmitted">TaskReviewSubmitted</a> {
        task_id: object::id(task),
        receipt_id: object::id(task),
        rater_role: <a href="../social_contracts/agent_task.md#social_contracts_agent_task_PARTY_SELLER">PARTY_SELLER</a>,
        rater_principal: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>,
        rater_agent_id: option::some(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>),
        reviewee_principal: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>,
        reviewee_agent_id: task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>,
        stars,
        rater_credibility_bps: task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>,
        value_weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_outcome_value_weight_bps">memory::outcome_value_weight_bps</a>(outcome),
        base_weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_outcome_base_weight_bps">memory::outcome_base_weight_bps</a>(outcome),
        prior_pair_reviews: <a href="../social_contracts/memory.md#social_contracts_memory_outcome_prior_pair_reviews">memory::outcome_prior_pair_reviews</a>(outcome),
        weight_bps: <a href="../social_contracts/memory.md#social_contracts_memory_outcome_weight_bps">memory::outcome_weight_bps</a>(outcome),
        reviewed_at_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_agent_task_assert_reviewable"></a>

## Function `assert_reviewable`



<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_reviewable">assert_reviewable</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, stars: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_reviewable">assert_reviewable</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>, stars: u8, clock: &Clock) {
    <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> == <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_SETTLED">STATUS_SETTLED</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongStatus">EWrongStatus</a>);
    <b>assert</b>!(clock::timestamp_ms(clock) &lt;= task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EReviewWindowClosed">EReviewWindowClosed</a>);
    <b>assert</b>!(stars &gt;= 1 && stars &lt;= 5, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EInvalidStars">EInvalidStars</a>);
}
</code></pre>



</details>

<a name="social_contracts_agent_task_resolve_buyer_actor"></a>

## Function `resolve_buyer_actor`

The task's buyer: its principal, or the buyer agent that created the offer, holding the
cap and within its spend limit. A different agent of the same principal is rejected.


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_buyer_actor">resolve_buyer_actor</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, spend_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_buyer_actor">resolve_buyer_actor</a>(
    task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    buyer_account: &MemoryAccount,
    spend_amount: u64,
    clock: &Clock,
    ctx: &TxContext,
): ActingContext {
    <b>assert</b>!(object::id(buyer_account) == task.buyer_memory_account_id, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>let</b> cap = <a href="../social_contracts/memory.md#social_contracts_memory_cap_task_trade">memory::cap_task_trade</a>();
    <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">memory::resolve_actor_with_cap</a>(
        memory_config,
        buyer_account,
        cap,
        option::none(),
        spend_amount,
        clock,
        ctx,
    );
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_direct_execution_allowed">memory::assert_direct_execution_allowed</a>(buyer_account, cap, ctx);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">memory::acting_principal_owner</a>(&acting) == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotBuyer">ENotBuyer</a>);
    <b>let</b> actor_agent = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting);
    <b>if</b> (option::is_some(&actor_agent)) {
        <b>assert</b>!(task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a> == actor_agent, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotBuyer">ENotBuyer</a>);
    };
    acting
}
</code></pre>



</details>

<a name="social_contracts_agent_task_assert_seller_with_cap"></a>

## Function `assert_seller_with_cap`

Seller principal, or the accepted seller agent holding the cap (for accept and decline).


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_seller_with_cap">assert_seller_with_cap</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, seller_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_seller_with_cap">assert_seller_with_cap</a>(
    task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    seller_account: &MemoryAccount,
    seller_agent: &SubAgent,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>assert</b>!(object::id(seller_agent) == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>assert</b>!(object::id(seller_account) == task.seller_memory_account_id, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (sender == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>) {
        <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">memory::resolve_human_actor</a>(seller_account, ctx);
    } <b>else</b> {
        <b>assert</b>!(sender == <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_derived_address">memory::sub_agent_derived_address</a>(seller_agent), <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotSeller">ENotSeller</a>);
        <b>let</b> cap = <a href="../social_contracts/memory.md#social_contracts_memory_cap_task_trade">memory::cap_task_trade</a>();
        <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">memory::resolve_actor_with_cap</a>(
            memory_config,
            seller_account,
            cap,
            option::none(),
            0,
            clock,
            ctx,
        );
        <a href="../social_contracts/memory.md#social_contracts_memory_assert_direct_execution_allowed">memory::assert_direct_execution_allowed</a>(seller_account, cap, ctx);
        <b>assert</b>!(
            option::contains(&<a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">memory::acting_sub_agent_id</a>(&acting), &task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>),
            <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotSeller">ENotSeller</a>,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_agent_task_assert_seller_sender"></a>

## Function `assert_seller_sender`

Seller principal or the seller agent itself, with no cap: used where no funds move to
the seller (review, refund proposals).


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_seller_sender">assert_seller_sender</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_seller_sender">assert_seller_sender</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>, seller_agent: &SubAgent, ctx: &TxContext) {
    <b>assert</b>!(object::id(seller_agent) == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_EWrongTaskObject">EWrongTaskObject</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(
        sender == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a> || sender == <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_derived_address">memory::sub_agent_derived_address</a>(seller_agent),
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ENotSeller">ENotSeller</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_agent_task_resolve_party"></a>

## Function `resolve_party`

Which side the sender is on: seller (principal or agent) or buyer (with the cap).


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_party">resolve_party</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, buyer_account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, seller_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_party">resolve_party</a>(
    task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>,
    memory_config: &MemoryConfig,
    buyer_account: &MemoryAccount,
    seller_agent: &SubAgent,
    clock: &Clock,
    ctx: &TxContext,
): u8 {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (sender == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>
        || (object::id(seller_agent) == task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>
            && sender == <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_derived_address">memory::sub_agent_derived_address</a>(seller_agent))) {
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_assert_seller_sender">assert_seller_sender</a>(task, seller_agent, ctx);
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_PARTY_SELLER">PARTY_SELLER</a>
    } <b>else</b> {
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_resolve_buyer_actor">resolve_buyer_actor</a>(task, memory_config, buyer_account, 0, clock, ctx);
        <a href="../social_contracts/agent_task.md#social_contracts_agent_task_PARTY_BUYER">PARTY_BUYER</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_agent_task_compute_terms_digest"></a>

## Function `compute_terms_digest`

sha3-256 over the BCS of the fields the seller must approve, in this order:
buyer principal, seller agent id, amount, scope digest, accept/delivery/settle windows,
platform fee bps, ecosystem fee bps, ecosystem recipient, platform id.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_compute_terms_digest">compute_terms_digest</a>(<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: <b>address</b>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: u64, scope_digest: &vector&lt;u8&gt;, accept_window_ms: u64, delivery_window_ms: u64, settle_window_ms: u64, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: u64, <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: u64, ecosystem_fee_recipient: <b>address</b>, platform_id: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_compute_terms_digest">compute_terms_digest</a>(
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>: <b>address</b>,
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>: ID,
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>: u64,
    scope_digest: &vector&lt;u8&gt;,
    accept_window_ms: u64,
    delivery_window_ms: u64,
    settle_window_ms: u64,
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>: u64,
    <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>: u64,
    ecosystem_fee_recipient: <b>address</b>,
    platform_id: &Option&lt;<b>address</b>&gt;,
): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> bytes = vector::empty&lt;u8&gt;();
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(scope_digest));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&accept_window_ms));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&delivery_window_ms));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&settle_window_ms));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(&ecosystem_fee_recipient));
    vector::append(&<b>mut</b> bytes, bcs::to_bytes(platform_id));
    hash::sha3_256(bytes)
}
</code></pre>



</details>

<a name="social_contracts_agent_task_status_offered"></a>

## Function `status_offered`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_offered">status_offered</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_offered">status_offered</a>(): u8 { <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_OFFERED">STATUS_OFFERED</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_status_accepted"></a>

## Function `status_accepted`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_accepted">status_accepted</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_accepted">status_accepted</a>(): u8 { <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_ACCEPTED">STATUS_ACCEPTED</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_status_delivered"></a>

## Function `status_delivered`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_delivered">status_delivered</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_delivered">status_delivered</a>(): u8 { <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_DELIVERED">STATUS_DELIVERED</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_status_settled"></a>

## Function `status_settled`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_settled">status_settled</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_settled">status_settled</a>(): u8 { <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_SETTLED">STATUS_SETTLED</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_status_refunded"></a>

## Function `status_refunded`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_refunded">status_refunded</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_refunded">status_refunded</a>(): u8 { <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_REFUNDED">STATUS_REFUNDED</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_status_cancelled"></a>

## Function `status_cancelled`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_cancelled">status_cancelled</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status_cancelled">status_cancelled</a>(): u8 { <a href="../social_contracts/agent_task.md#social_contracts_agent_task_STATUS_CANCELLED">STATUS_CANCELLED</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_status"></a>

## Function `status`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u8 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_status">status</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_close_kind"></a>

## Function `close_kind`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u8 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_close_kind">close_kind</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_settlement_kind"></a>

## Function `settlement_kind`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settlement_kind">settlement_kind</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settlement_kind">settlement_kind</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u8 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_settlement_kind">settlement_kind</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_amount_mist"></a>

## Function `amount_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_amount_mist">amount_mist</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_escrowed_mist"></a>

## Function `escrowed_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_escrowed_mist">escrowed_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_escrowed_mist">escrowed_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { balance::value(&task.escrowed) }
</code></pre>



</details>

<a name="social_contracts_agent_task_buyer_principal"></a>

## Function `buyer_principal`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): <b>address</b> { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_principal">buyer_principal</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_seller_principal"></a>

## Function `seller_principal`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): <b>address</b> { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_principal">seller_principal</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_buyer_agent_id"></a>

## Function `buyer_agent_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): Option&lt;ID&gt; { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_agent_id">buyer_agent_id</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_seller_agent_id"></a>

## Function `seller_agent_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): ID { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_agent_id">seller_agent_id</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_terms_digest"></a>

## Function `terms_digest`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): vector&lt;u8&gt; { task.terms.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_terms_digest">terms_digest</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_platform_fee_bps"></a>

## Function `platform_fee_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.terms.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_bps">platform_fee_bps</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_ecosystem_fee_bps"></a>

## Function `ecosystem_fee_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.terms.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_bps">ecosystem_fee_bps</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_accept_deadline_ms"></a>

## Function `accept_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_accept_deadline_ms">accept_deadline_ms</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_delivery_deadline_ms"></a>

## Function `delivery_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_delivery_deadline_ms">delivery_deadline_ms</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_settle_deadline_ms"></a>

## Function `settle_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_settle_deadline_ms">settle_deadline_ms</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_review_deadline_ms"></a>

## Function `review_deadline_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_review_deadline_ms">review_deadline_ms</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_settled_at_ms"></a>

## Function `settled_at_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settled_at_ms">settled_at_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_settled_at_ms">settled_at_ms</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_settled_at_ms">settled_at_ms</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_platform_fee_mist"></a>

## Function `platform_fee_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_mist">platform_fee_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_mist">platform_fee_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.payout.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_platform_fee_mist">platform_fee_mist</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_ecosystem_fee_mist"></a>

## Function `ecosystem_fee_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_mist">ecosystem_fee_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_mist">ecosystem_fee_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.payout.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_ecosystem_fee_mist">ecosystem_fee_mist</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_net_amount_mist"></a>

## Function `net_amount_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_net_amount_mist">net_amount_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_net_amount_mist">net_amount_mist</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.payout.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_net_amount_mist">net_amount_mist</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_refund_proposer"></a>

## Function `refund_proposer`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u8 { task.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_refund_proposer">refund_proposer</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_buyer_credibility_bps"></a>

## Function `buyer_credibility_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_credibility_bps">buyer_credibility_bps</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_seller_credibility_bps"></a>

## Function `seller_credibility_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u64 { task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_credibility_bps">seller_credibility_bps</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_buyer_review_stars"></a>

## Function `buyer_review_stars`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_review_stars">buyer_review_stars</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_review_stars">buyer_review_stars</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u8 { task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_buyer_review_stars">buyer_review_stars</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_seller_review_stars"></a>

## Function `seller_review_stars`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_review_stars">seller_review_stars</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">social_contracts::agent_task::TaskEscrow</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_review_stars">seller_review_stars</a>(task: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskEscrow">TaskEscrow</a>): u8 { task.reviews.<a href="../social_contracts/agent_task.md#social_contracts_agent_task_seller_review_stars">seller_review_stars</a> }
</code></pre>



</details>

<a name="social_contracts_agent_task_config_active"></a>

## Function `config_active`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_active">config_active</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_active">config_active</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>): bool { config.active }
</code></pre>



</details>

<a name="social_contracts_agent_task_config_release_grace_ms"></a>

## Function `config_release_grace_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_release_grace_ms">config_release_grace_ms</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_release_grace_ms">config_release_grace_ms</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>): u64 { config.release_grace_ms }
</code></pre>



</details>

<a name="social_contracts_agent_task_config_review_window_ms"></a>

## Function `config_review_window_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_review_window_ms">config_review_window_ms</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_review_window_ms">config_review_window_ms</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>): u64 { config.review_window_ms }
</code></pre>



</details>

<a name="social_contracts_agent_task_config_min_escrow_mist"></a>

## Function `config_min_escrow_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_min_escrow_mist">config_min_escrow_mist</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_min_escrow_mist">config_min_escrow_mist</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>): u64 { config.min_escrow_mist }
</code></pre>



</details>

<a name="social_contracts_agent_task_config_reputation"></a>

## Function `config_reputation`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_reputation">config_reputation</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">social_contracts::agent_task::TaskConfig</a>): <a href="../social_contracts/memory.md#social_contracts_memory_ReputationParams">social_contracts::memory::ReputationParams</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/agent_task.md#social_contracts_agent_task_config_reputation">config_reputation</a>(config: &<a href="../social_contracts/agent_task.md#social_contracts_agent_task_TaskConfig">TaskConfig</a>): ReputationParams { config.reputation }
</code></pre>



</details>
