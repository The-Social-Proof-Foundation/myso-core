---
title: Module `social_contracts::ai_credit`
---

MIST-denominated AI spend escrow. Users deposit MYSO and the platform oracle reserves
the worst-case MIST charge before provider work begins, then captures the actual charge
or releases the reservation. Legacy signed usage receipts remain available during the
reserve-and-capture migration.


-  [Struct `AiCreditOracleAdminCap`](#social_contracts_ai_credit_AiCreditOracleAdminCap)
-  [Struct `AiCreditConfig`](#social_contracts_ai_credit_AiCreditConfig)
-  [Struct `AgentBudgetEntry`](#social_contracts_ai_credit_AgentBudgetEntry)
-  [Struct `SpendApprovalKey`](#social_contracts_ai_credit_SpendApprovalKey)
-  [Struct `SpendApproval`](#social_contracts_ai_credit_SpendApproval)
-  [Struct `ApprovalNonceKey`](#social_contracts_ai_credit_ApprovalNonceKey)
-  [Struct `AiCreditBalance`](#social_contracts_ai_credit_AiCreditBalance)
-  [Struct `IntentMessage`](#social_contracts_ai_credit_IntentMessage)
-  [Struct `UsageReceipt`](#social_contracts_ai_credit_UsageReceipt)
-  [Struct `SpendReservationIntent`](#social_contracts_ai_credit_SpendReservationIntent)
-  [Struct `SpendReservation`](#social_contracts_ai_credit_SpendReservation)
-  [Struct `CaptureSpendIntent`](#social_contracts_ai_credit_CaptureSpendIntent)
-  [Struct `CancelSpendIntent`](#social_contracts_ai_credit_CancelSpendIntent)
-  [Struct `AiCreditBalanceCreated`](#social_contracts_ai_credit_AiCreditBalanceCreated)
-  [Struct `AiCreditDeposited`](#social_contracts_ai_credit_AiCreditDeposited)
-  [Struct `AiCreditWithdrawn`](#social_contracts_ai_credit_AiCreditWithdrawn)
-  [Struct `AiCreditAccountCapsUpdated`](#social_contracts_ai_credit_AiCreditAccountCapsUpdated)
-  [Struct `AiCreditAgentBudgetUpdated`](#social_contracts_ai_credit_AiCreditAgentBudgetUpdated)
-  [Struct `AiCreditAgentBudgetDisabled`](#social_contracts_ai_credit_AiCreditAgentBudgetDisabled)
-  [Struct `AiCreditAgentBudgetChanged`](#social_contracts_ai_credit_AiCreditAgentBudgetChanged)
-  [Struct `AiCreditSpendApproved`](#social_contracts_ai_credit_AiCreditSpendApproved)
-  [Struct `AiCreditSpendApprovalRevoked`](#social_contracts_ai_credit_AiCreditSpendApprovalRevoked)
-  [Struct `AiCreditSpendApprovalConsumed`](#social_contracts_ai_credit_AiCreditSpendApprovalConsumed)
-  [Struct `AiCreditUsageSettled`](#social_contracts_ai_credit_AiCreditUsageSettled)
-  [Struct `AiSpendReserved`](#social_contracts_ai_credit_AiSpendReserved)
-  [Struct `AiSpendCaptured`](#social_contracts_ai_credit_AiSpendCaptured)
-  [Struct `AiSpendCancelled`](#social_contracts_ai_credit_AiSpendCancelled)
-  [Struct `AiSpendExpired`](#social_contracts_ai_credit_AiSpendExpired)
-  [Struct `AiCreditBalanceDepleted`](#social_contracts_ai_credit_AiCreditBalanceDepleted)
-  [Struct `AiCreditBalancePaused`](#social_contracts_ai_credit_AiCreditBalancePaused)
-  [Struct `AiCreditBalanceReactivated`](#social_contracts_ai_credit_AiCreditBalanceReactivated)
-  [Struct `AiCreditOraclePubkeyUpdated`](#social_contracts_ai_credit_AiCreditOraclePubkeyUpdated)
-  [Struct `AiCreditSettlementLimitsUpdated`](#social_contracts_ai_credit_AiCreditSettlementLimitsUpdated)
-  [Struct `AiCreditMarkupUpdated`](#social_contracts_ai_credit_AiCreditMarkupUpdated)
-  [Struct `AiCreditMinDepositUpdated`](#social_contracts_ai_credit_AiCreditMinDepositUpdated)
-  [Struct `AiCreditConfigInitialized`](#social_contracts_ai_credit_AiCreditConfigInitialized)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_ai_credit_bootstrap_init)
-  [Function `create_oracle_admin_cap`](#social_contracts_ai_credit_create_oracle_admin_cap)
-  [Function `create_and_share_balance`](#social_contracts_ai_credit_create_and_share_balance)
-  [Function `new_balance`](#social_contracts_ai_credit_new_balance)
-  [Function `deposit`](#social_contracts_ai_credit_deposit)
-  [Function `withdraw`](#social_contracts_ai_credit_withdraw)
-  [Function `set_account_caps`](#social_contracts_ai_credit_set_account_caps)
-  [Function `set_agent_budget`](#social_contracts_ai_credit_set_agent_budget)
-  [Function `set_agent_budget_as_manager`](#social_contracts_ai_credit_set_agent_budget_as_manager)
-  [Function `disable_agent_budget`](#social_contracts_ai_credit_disable_agent_budget)
-  [Function `approve_agent_spend`](#social_contracts_ai_credit_approve_agent_spend)
-  [Function `approve_agent_spend_as_approver`](#social_contracts_ai_credit_approve_agent_spend_as_approver)
-  [Function `revoke_agent_spend_approval`](#social_contracts_ai_credit_revoke_agent_spend_approval)
-  [Function `set_child_agent_budget`](#social_contracts_ai_credit_set_child_agent_budget)
-  [Function `disable_child_agent_budget`](#social_contracts_ai_credit_disable_child_agent_budget)
-  [Function `approve_child_agent_spend`](#social_contracts_ai_credit_approve_child_agent_spend)
-  [Function `pause_balance`](#social_contracts_ai_credit_pause_balance)
-  [Function `reactivate_balance`](#social_contracts_ai_credit_reactivate_balance)
-  [Function `reserve_signed_spend`](#social_contracts_ai_credit_reserve_signed_spend)
-  [Function `make_spend_reservation_intent_from_objects`](#social_contracts_ai_credit_make_spend_reservation_intent_from_objects)
-  [Function `capture_reserved_spend`](#social_contracts_ai_credit_capture_reserved_spend)
-  [Function `cancel_reserved_spend`](#social_contracts_ai_credit_cancel_reserved_spend)
-  [Function `expire_reservation`](#social_contracts_ai_credit_expire_reservation)
-  [Function `settle_usage`](#social_contracts_ai_credit_settle_usage)
-  [Function `settle_usage_batch`](#social_contracts_ai_credit_settle_usage_batch)
-  [Function `settle_signed_usage`](#social_contracts_ai_credit_settle_signed_usage)
-  [Function `make_usage_receipt_from_objects`](#social_contracts_ai_credit_make_usage_receipt_from_objects)
-  [Function `settle_usage_with_signature`](#social_contracts_ai_credit_settle_usage_with_signature)
-  [Function `execute_settlement`](#social_contracts_ai_credit_execute_settlement)
-  [Function `execute_reservation`](#social_contracts_ai_credit_execute_reservation)
-  [Function `execute_capture`](#social_contracts_ai_credit_execute_capture)
-  [Function `execute_cancellation`](#social_contracts_ai_credit_execute_cancellation)
-  [Function `update_oracle_pubkey`](#social_contracts_ai_credit_update_oracle_pubkey)
-  [Function `update_oracle_markup`](#social_contracts_ai_credit_update_oracle_markup)
-  [Function `update_min_deposit`](#social_contracts_ai_credit_update_min_deposit)
-  [Function `update_settlement_limits`](#social_contracts_ai_credit_update_settlement_limits)
-  [Function `update_treasury`](#social_contracts_ai_credit_update_treasury)
-  [Function `balance_mist`](#social_contracts_ai_credit_balance_mist)
-  [Function `available_mist`](#social_contracts_ai_credit_available_mist)
-  [Function `reserved_mist`](#social_contracts_ai_credit_reserved_mist)
-  [Function `latest_reservation_nonce`](#social_contracts_ai_credit_latest_reservation_nonce)
-  [Function `reservation_for`](#social_contracts_ai_credit_reservation_for)
-  [Function `reservation_max_amount_mist`](#social_contracts_ai_credit_reservation_max_amount_mist)
-  [Function `reservation_agent_object_id`](#social_contracts_ai_credit_reservation_agent_object_id)
-  [Function `reservation_hard_expiry_ms`](#social_contracts_ai_credit_reservation_hard_expiry_ms)
-  [Function `oracle_markup_bps`](#social_contracts_ai_credit_oracle_markup_bps)
-  [Function `min_deposit_mist`](#social_contracts_ai_credit_min_deposit_mist)
-  [Function `spend_approval_for`](#social_contracts_ai_credit_spend_approval_for)
-  [Function `approval_max_amount_mist`](#social_contracts_ai_credit_approval_max_amount_mist)
-  [Function `approval_expires_at_ms`](#social_contracts_ai_credit_approval_expires_at_ms)
-  [Function `approval_approved_by`](#social_contracts_ai_credit_approval_approved_by)
-  [Function `approval_nonce`](#social_contracts_ai_credit_approval_nonce)
-  [Function `agent_approval_threshold`](#social_contracts_ai_credit_agent_approval_threshold)
-  [Function `agent_remaining_mist`](#social_contracts_ai_credit_agent_remaining_mist)
-  [Function `usage_inference`](#social_contracts_ai_credit_usage_inference)
-  [Function `usage_tool`](#social_contracts_ai_credit_usage_tool)
-  [Function `usage_embed`](#social_contracts_ai_credit_usage_embed)
-  [Function `usage_storage`](#social_contracts_ai_credit_usage_storage)
-  [Function `usage_workflow`](#social_contracts_ai_credit_usage_workflow)
-  [Function `mist_per_myso`](#social_contracts_ai_credit_mist_per_myso)
-  [Function `assert_agent_may_spend`](#social_contracts_ai_credit_assert_agent_may_spend)
-  [Function `assert_version`](#social_contracts_ai_credit_assert_version)
-  [Function `assert_owner`](#social_contracts_ai_credit_assert_owner)
-  [Function `assert_active`](#social_contracts_ai_credit_assert_active)
-  [Function `assert_agent_linked`](#social_contracts_ai_credit_assert_agent_linked)
-  [Function `assert_org_gate_for_agent`](#social_contracts_ai_credit_assert_org_gate_for_agent)
-  [Function `assert_parent_manages_child`](#social_contracts_ai_credit_assert_parent_manages_child)
-  [Function `assert_child_budget_within_parent_envelope`](#social_contracts_ai_credit_assert_child_budget_within_parent_envelope)
-  [Function `assert_limit_not_looser`](#social_contracts_ai_credit_assert_limit_not_looser)
-  [Function `assert_within_parent_envelope`](#social_contracts_ai_credit_assert_within_parent_envelope)
-  [Function `upsert_agent_budget`](#social_contracts_ai_credit_upsert_agent_budget)
-  [Function `disable_agent_budget_internal`](#social_contracts_ai_credit_disable_agent_budget_internal)
-  [Function `store_spend_approval`](#social_contracts_ai_credit_store_spend_approval)
-  [Function `next_approval_nonce`](#social_contracts_ai_credit_next_approval_nonce)
-  [Function `maybe_consume_spend_approval`](#social_contracts_ai_credit_maybe_consume_spend_approval)
-  [Function `assert_oracle_admin`](#social_contracts_ai_credit_assert_oracle_admin)
-  [Function `verify_receipt_signature`](#social_contracts_ai_credit_verify_receipt_signature)
-  [Function `verify_reservation_signature`](#social_contracts_ai_credit_verify_reservation_signature)
-  [Function `verify_capture_signature`](#social_contracts_ai_credit_verify_capture_signature)
-  [Function `verify_cancel_signature`](#social_contracts_ai_credit_verify_cancel_signature)
-  [Function `assert_receipt_fresh`](#social_contracts_ai_credit_assert_receipt_fresh)
-  [Function `assert_signed_timestamp_fresh`](#social_contracts_ai_credit_assert_signed_timestamp_fresh)
-  [Function `roll_account_windows`](#social_contracts_ai_credit_roll_account_windows)
-  [Function `roll_agent_windows`](#social_contracts_ai_credit_roll_agent_windows)
-  [Function `assert_account_caps`](#social_contracts_ai_credit_assert_account_caps)
-  [Function `assert_agent_caps`](#social_contracts_ai_credit_assert_agent_caps)
-  [Function `release_reservation_counters`](#social_contracts_ai_credit_release_reservation_counters)
-  [Function `capture_reservation_counters`](#social_contracts_ai_credit_capture_reservation_counters)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
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
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
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
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_ai_credit_AiCreditOracleAdminCap"></a>

## Struct `AiCreditOracleAdminCap`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_ai_credit_AiCreditConfig"></a>

## Struct `AiCreditConfig`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a> <b>has</b> key
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
<code>oracle_pubkey: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_single_settlement_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>receipt_ttl_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>balances_by_memory: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
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

<a name="social_contracts_ai_credit_AgentBudgetEntry"></a>

## Struct `AgentBudgetEntry`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AgentBudgetEntry">AgentBudgetEntry</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>budget_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>spent_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>spent_day_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved_day_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>spent_month_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved_month_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>day_anchor_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>month_anchor_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>require_approval_above_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_SpendApprovalKey"></a>

## Struct `SpendApprovalKey`

Dynamic-field key on <code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>.id</code> for the agent's live spend allowance.
One allowance per agent; re-approving overwrites. Stored as a dynamic field so the
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a></code> struct layout never changes (upgrade-safe).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_SpendApproval"></a>

## Struct `SpendApproval`

One-shot spend allowance consumed by the first over-threshold settlement it covers.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>max_amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>approved_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_ApprovalNonceKey"></a>

## Struct `ApprovalNonceKey`

Dynamic-field key on <code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>.id</code> for the monotonic approval nonce counter.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ApprovalNonceKey">ApprovalNonceKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditBalance"></a>

## Struct `AiCreditBalance`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a> <b>has</b> key
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
<code>memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>spent_total_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>spent_day_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved_day_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>spent_month_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reserved_month_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>day_anchor_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>month_anchor_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>settlement_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reservations: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u64, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">social_contracts::ai_credit::SpendReservation</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_budgets: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AgentBudgetEntry">social_contracts::ai_credit::AgentBudgetEntry</a>&gt;</code>
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
</dl>


</details>

<a name="social_contracts_ai_credit_IntentMessage"></a>

## Struct `IntentMessage`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_IntentMessage">IntentMessage</a>&lt;T: drop&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>intent: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>payload: T</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_UsageReceipt"></a>

## Struct `UsageReceipt`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>receipt_id: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>usage_kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>settlement_nonce: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_SpendReservationIntent"></a>

## Struct `SpendReservationIntent`

Oracle-signed authorization to lock a deterministic worst-case MIST charge.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservationIntent">SpendReservationIntent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>provider_envelope_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>request_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>fx_quote_id: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_usd_e8: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>markup_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>capture_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>hard_expiry_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_SpendReservation"></a>

## Struct `SpendReservation`

Canonical live reservation stored directly in the greenfield account object.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">SpendReservation</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>reservation_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>max_amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>provider_envelope_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>request_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>fx_quote_id: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_usd_e8: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>markup_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>capture_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>hard_expiry_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>account_day_anchor_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>account_month_anchor_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_budget_reserved: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_day_anchor_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_month_anchor_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_CaptureSpendIntent"></a>

## Struct `CaptureSpendIntent`

Oracle-signed final provider charge for an existing reservation.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CaptureSpendIntent">CaptureSpendIntent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>provider_cost_usd_micros: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>provider_generation_hash: vector&lt;u8&gt;</code>
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

<a name="social_contracts_ai_credit_CancelSpendIntent"></a>

## Struct `CancelSpendIntent`

Oracle-signed release when the provider confirms that no billable generation exists.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CancelSpendIntent">CancelSpendIntent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_nonce: u64</code>
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

<a name="social_contracts_ai_credit_AiCreditBalanceCreated"></a>

## Struct `AiCreditBalanceCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalanceCreated">AiCreditBalanceCreated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditDeposited"></a>

## Struct `AiCreditDeposited`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditDeposited">AiCreditDeposited</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance_mist: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditWithdrawn"></a>

## Struct `AiCreditWithdrawn`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditWithdrawn">AiCreditWithdrawn</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance_mist: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditAccountCapsUpdated"></a>

## Struct `AiCreditAccountCapsUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAccountCapsUpdated">AiCreditAccountCapsUpdated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditAgentBudgetUpdated"></a>

## Struct `AiCreditAgentBudgetUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetUpdated">AiCreditAgentBudgetUpdated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>budget_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>require_approval_above_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditAgentBudgetDisabled"></a>

## Struct `AiCreditAgentBudgetDisabled`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetDisabled">AiCreditAgentBudgetDisabled</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditAgentBudgetChanged"></a>

## Struct `AiCreditAgentBudgetChanged`

Audit-grade budget change event carrying previous and new values plus the actor.
Emitted alongside the legacy <code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetUpdated">AiCreditAgentBudgetUpdated</a></code>/<code>Disabled</code> events.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetChanged">AiCreditAgentBudgetChanged</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>had_previous_entry: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>prev_budget_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>prev_daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>prev_monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>prev_require_approval_above_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>prev_enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>budget_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>require_approval_above_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>set_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>set_by_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
 Set when a parent agent changed a descendant's budget.
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
 Set when the change went through an org role gate.
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditSpendApproved"></a>

## Struct `AiCreditSpendApproved`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditSpendApproved">AiCreditSpendApproved</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>approved_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>approved_by_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
 Set when a parent agent approved a descendant's spend.
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
 Set when the approval went through an org role gate.
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditSpendApprovalRevoked"></a>

## Struct `AiCreditSpendApprovalRevoked`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditSpendApprovalRevoked">AiCreditSpendApprovalRevoked</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>revoked_by: <b>address</b></code>
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

<a name="social_contracts_ai_credit_AiCreditSpendApprovalConsumed"></a>

## Struct `AiCreditSpendApprovalConsumed`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditSpendApprovalConsumed">AiCreditSpendApprovalConsumed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>approved_by: <b>address</b></code>
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

<a name="social_contracts_ai_credit_AiCreditUsageSettled"></a>

## Struct `AiCreditUsageSettled`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditUsageSettled">AiCreditUsageSettled</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>receipt_id: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>usage_kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>settlement_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>remaining_mist: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiSpendReserved"></a>

## Struct `AiSpendReserved`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiSpendReserved">AiSpendReserved</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>provider_envelope_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>request_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>fx_quote_id: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_usd_e8: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>markup_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>capture_deadline_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>hard_expiry_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiSpendCaptured"></a>

## Struct `AiSpendCaptured`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiSpendCaptured">AiSpendCaptured</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>captured_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>released_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>provider_cost_usd_micros: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>provider_generation_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>fx_quote_id: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_usd_e8: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>markup_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>captured_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>remaining_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiSpendCancelled"></a>

## Struct `AiSpendCancelled`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiSpendCancelled">AiSpendCancelled</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>released_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>cancelled_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiSpendExpired"></a>

## Struct `AiSpendExpired`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiSpendExpired">AiSpendExpired</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reservation_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>released_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expired_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditBalanceDepleted"></a>

## Struct `AiCreditBalanceDepleted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalanceDepleted">AiCreditBalanceDepleted</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditBalancePaused"></a>

## Struct `AiCreditBalancePaused`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalancePaused">AiCreditBalancePaused</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditBalanceReactivated"></a>

## Struct `AiCreditBalanceReactivated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalanceReactivated">AiCreditBalanceReactivated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditOraclePubkeyUpdated"></a>

## Struct `AiCreditOraclePubkeyUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOraclePubkeyUpdated">AiCreditOraclePubkeyUpdated</a> <b>has</b> <b>copy</b>, drop
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
<code>new_pubkey: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditSettlementLimitsUpdated"></a>

## Struct `AiCreditSettlementLimitsUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditSettlementLimitsUpdated">AiCreditSettlementLimitsUpdated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>max_single_settlement_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>receipt_ttl_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditMarkupUpdated"></a>

## Struct `AiCreditMarkupUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditMarkupUpdated">AiCreditMarkupUpdated</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditMinDepositUpdated"></a>

## Struct `AiCreditMinDepositUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditMinDepositUpdated">AiCreditMinDepositUpdated</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_ai_credit_AiCreditConfigInitialized"></a>

## Struct `AiCreditConfigInitialized`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfigInitialized">AiCreditConfigInitialized</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>oracle_pubkey: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_single_settlement_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>receipt_ttl_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_ai_credit_MIST_PER_MYSO"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MIST_PER_MYSO">MIST_PER_MYSO</a>: u64 = 1000000000;
</code></pre>



<a name="social_contracts_ai_credit_DAY_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_DAY_MS">DAY_MS</a>: u64 = 86400000;
</code></pre>



<a name="social_contracts_ai_credit_MONTH_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MONTH_MS">MONTH_MS</a>: u64 = 2592000000;
</code></pre>



<a name="social_contracts_ai_credit_INTENT_AI_CREDIT_USAGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_INTENT_AI_CREDIT_USAGE">INTENT_AI_CREDIT_USAGE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_ai_credit_INTENT_AI_CREDIT_RESERVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_INTENT_AI_CREDIT_RESERVE">INTENT_AI_CREDIT_RESERVE</a>: u8 = 2;
</code></pre>



<a name="social_contracts_ai_credit_INTENT_AI_CREDIT_CAPTURE"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_INTENT_AI_CREDIT_CAPTURE">INTENT_AI_CREDIT_CAPTURE</a>: u8 = 3;
</code></pre>



<a name="social_contracts_ai_credit_INTENT_AI_CREDIT_CANCEL"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_INTENT_AI_CREDIT_CANCEL">INTENT_AI_CREDIT_CANCEL</a>: u8 = 4;
</code></pre>



<a name="social_contracts_ai_credit_ED25519_PK_LEN"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ED25519_PK_LEN">ED25519_PK_LEN</a>: u64 = 32;
</code></pre>



<a name="social_contracts_ai_credit_HASH_LEN"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_HASH_LEN">HASH_LEN</a>: u64 = 32;
</code></pre>



<a name="social_contracts_ai_credit_DEFAULT_ORACLE_MARKUP_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_DEFAULT_ORACLE_MARKUP_BPS">DEFAULT_ORACLE_MARKUP_BPS</a>: u64 = 1500;
</code></pre>



<a name="social_contracts_ai_credit_MAX_CAPTURE_WINDOW_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MAX_CAPTURE_WINDOW_MS">MAX_CAPTURE_WINDOW_MS</a>: u64 = 900000;
</code></pre>



<a name="social_contracts_ai_credit_MAX_RESERVATION_LIFETIME_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MAX_RESERVATION_LIFETIME_MS">MAX_RESERVATION_LIFETIME_MS</a>: u64 = 1800000;
</code></pre>



<a name="social_contracts_ai_credit_USAGE_INFERENCE"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_INFERENCE">USAGE_INFERENCE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_ai_credit_USAGE_TOOL"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_TOOL">USAGE_TOOL</a>: u8 = 2;
</code></pre>



<a name="social_contracts_ai_credit_USAGE_EMBED"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_EMBED">USAGE_EMBED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_ai_credit_USAGE_STORAGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_STORAGE">USAGE_STORAGE</a>: u8 = 4;
</code></pre>



<a name="social_contracts_ai_credit_USAGE_WORKFLOW"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_WORKFLOW">USAGE_WORKFLOW</a>: u8 = 5;
</code></pre>



<a name="social_contracts_ai_credit_ENotOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ENotOwner">ENotOwner</a>: u64 = 1;
</code></pre>



<a name="social_contracts_ai_credit_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EWrongVersion">EWrongVersion</a>: u64 = 2;
</code></pre>



<a name="social_contracts_ai_credit_EInvalidAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>: u64 = 3;
</code></pre>



<a name="social_contracts_ai_credit_EInsufficientBalance"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInsufficientBalance">EInsufficientBalance</a>: u64 = 4;
</code></pre>



<a name="social_contracts_ai_credit_EInactive"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInactive">EInactive</a>: u64 = 5;
</code></pre>



<a name="social_contracts_ai_credit_EInvalidSignature"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidSignature">EInvalidSignature</a>: u64 = 6;
</code></pre>



<a name="social_contracts_ai_credit_EStaleReceipt"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EStaleReceipt">EStaleReceipt</a>: u64 = 7;
</code></pre>



<a name="social_contracts_ai_credit_EInvalidNonce"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidNonce">EInvalidNonce</a>: u64 = 8;
</code></pre>



<a name="social_contracts_ai_credit_ECapExceeded"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>: u64 = 9;
</code></pre>



<a name="social_contracts_ai_credit_EAgentNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentNotFound">EAgentNotFound</a>: u64 = 10;
</code></pre>



<a name="social_contracts_ai_credit_EAgentDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentDisabled">EAgentDisabled</a>: u64 = 11;
</code></pre>



<a name="social_contracts_ai_credit_EInvalidPubkey"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidPubkey">EInvalidPubkey</a>: u64 = 12;
</code></pre>



<a name="social_contracts_ai_credit_EAccountMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>: u64 = 13;
</code></pre>



<a name="social_contracts_ai_credit_EAgentMissingCap"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentMissingCap">EAgentMissingCap</a>: u64 = 15;
</code></pre>



<a name="social_contracts_ai_credit_EBalanceAlreadyExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EBalanceAlreadyExists">EBalanceAlreadyExists</a>: u64 = 17;
</code></pre>



<a name="social_contracts_ai_credit_EApprovalRequired"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EApprovalRequired">EApprovalRequired</a>: u64 = 18;
</code></pre>



<a name="social_contracts_ai_credit_EApprovalExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EApprovalExpired">EApprovalExpired</a>: u64 = 19;
</code></pre>



<a name="social_contracts_ai_credit_EApprovalInsufficient"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EApprovalInsufficient">EApprovalInsufficient</a>: u64 = 20;
</code></pre>



<a name="social_contracts_ai_credit_EApprovalNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EApprovalNotFound">EApprovalNotFound</a>: u64 = 21;
</code></pre>



<a name="social_contracts_ai_credit_ENotDescendant"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ENotDescendant">ENotDescendant</a>: u64 = 22;
</code></pre>



<a name="social_contracts_ai_credit_ENotParentSigner"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ENotParentSigner">ENotParentSigner</a>: u64 = 23;
</code></pre>



<a name="social_contracts_ai_credit_EParentEnvelopeExceeded"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EParentEnvelopeExceeded">EParentEnvelopeExceeded</a>: u64 = 24;
</code></pre>



<a name="social_contracts_ai_credit_ECannotManageSelf"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECannotManageSelf">ECannotManageSelf</a>: u64 = 25;
</code></pre>



<a name="social_contracts_ai_credit_EAgentNotInOrg"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentNotInOrg">EAgentNotInOrg</a>: u64 = 26;
</code></pre>



<a name="social_contracts_ai_credit_EInvalidExpiry"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidExpiry">EInvalidExpiry</a>: u64 = 27;
</code></pre>



<a name="social_contracts_ai_credit_EReservationNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationNotFound">EReservationNotFound</a>: u64 = 28;
</code></pre>



<a name="social_contracts_ai_credit_EReservationExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationExpired">EReservationExpired</a>: u64 = 29;
</code></pre>



<a name="social_contracts_ai_credit_EReservationNotExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationNotExpired">EReservationNotExpired</a>: u64 = 30;
</code></pre>



<a name="social_contracts_ai_credit_ECaptureWindowClosed"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECaptureWindowClosed">ECaptureWindowClosed</a>: u64 = 31;
</code></pre>



<a name="social_contracts_ai_credit_EReservationMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationMismatch">EReservationMismatch</a>: u64 = 32;
</code></pre>



<a name="social_contracts_ai_credit_EInvalidReservationWindow"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidReservationWindow">EInvalidReservationWindow</a>: u64 = 33;
</code></pre>



<a name="social_contracts_ai_credit_EInvalidHash"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidHash">EInvalidHash</a>: u64 = 34;
</code></pre>



<a name="social_contracts_ai_credit_EMarkupMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EMarkupMismatch">EMarkupMismatch</a>: u64 = 35;
</code></pre>



<a name="social_contracts_ai_credit_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_bootstrap_init">bootstrap_init</a>(treasury: <b>address</b>, oracle_pubkey: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_bootstrap_init">bootstrap_init</a>(treasury: <b>address</b>, oracle_pubkey: vector&lt;u8&gt;, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(vector::length(&oracle_pubkey) == <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ED25519_PK_LEN">ED25519_PK_LEN</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidPubkey">EInvalidPubkey</a>);
    <b>let</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a> = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MIST_PER_MYSO">MIST_PER_MYSO</a>;
    <b>let</b> max_single_settlement_mist = 1000 * <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MIST_PER_MYSO">MIST_PER_MYSO</a>;
    <b>let</b> receipt_ttl_ms = 300_000;
    <b>let</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a> = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_DEFAULT_ORACLE_MARKUP_BPS">DEFAULT_ORACLE_MARKUP_BPS</a>;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfigInitialized">AiCreditConfigInitialized</a> {
        oracle_pubkey,
        treasury,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>,
        max_single_settlement_mist,
        receipt_ttl_ms,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>,
    });
    <b>let</b> config = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a> {
        id: object::new(ctx),
        oracle_pubkey,
        treasury,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>,
        max_single_settlement_mist,
        receipt_ttl_ms,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>,
        balances_by_memory: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    transfer::share_object(config);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_create_oracle_admin_cap"></a>

## Function `create_oracle_admin_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_create_oracle_admin_cap">create_oracle_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">social_contracts::ai_credit::AiCreditOracleAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_create_oracle_admin_cap">create_oracle_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a> {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a> {
        id: object::new(ctx),
    }
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_create_and_share_balance"></a>

## Function `create_and_share_balance`

Called only from [<code><a href="../social_contracts/profile.md#social_contracts_profile_create_profile">profile::create_profile</a></code>] — one balance per memory account.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_create_and_share_balance">create_and_share_balance</a>(config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, principal_owner: <b>address</b>, profile_id: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_create_and_share_balance">create_and_share_balance</a>(
    config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    memory_account_id: ID,
    principal_owner: <b>address</b>,
    profile_id: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): ID {
    <b>assert</b>!(!table::contains(&config.balances_by_memory, memory_account_id), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EBalanceAlreadyExists">EBalanceAlreadyExists</a>);
    <b>let</b> balance = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_new_balance">new_balance</a>(memory_account_id, principal_owner, profile_id, clock, ctx);
    <b>let</b> balance_id = object::id(&balance);
    table::add(&<b>mut</b> config.balances_by_memory, memory_account_id, balance_id);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalanceCreated">AiCreditBalanceCreated</a> {
        balance_id,
        memory_account_id,
        principal_owner,
        profile_id,
    });
    transfer::share_object(balance);
    balance_id
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_new_balance"></a>

## Function `new_balance`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_new_balance">new_balance</a>(memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, principal_owner: <b>address</b>, profile_id: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_new_balance">new_balance</a>(
    memory_account_id: ID,
    principal_owner: <b>address</b>,
    profile_id: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a> {
    <b>let</b> now = clock::timestamp_ms(clock);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a> {
        id: object::new(ctx),
        memory_account_id,
        principal_owner,
        profile_id,
        balance: balance::zero(),
        spent_total_mist: 0,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>: 0,
        spent_day_mist: 0,
        reserved_day_mist: 0,
        spent_month_mist: 0,
        reserved_month_mist: 0,
        day_anchor_ms: now,
        month_anchor_ms: now,
        daily_cap_mist: option::none(),
        monthly_cap_mist: option::none(),
        settlement_nonce: 0,
        reservation_nonce: 0,
        reservations: table::new(ctx),
        agent_budgets: table::new(ctx),
        active: <b>true</b>,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    }
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_deposit"></a>

## Function `deposit`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_deposit">deposit</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_deposit">deposit</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    payment: Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <b>let</b> amount = coin::value(&payment);
    <b>assert</b>!(amount &gt;= config.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    balance::join(&<b>mut</b> balance.balance, coin::into_balance(payment));
    <b>let</b> new_balance_mist = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a>(balance);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditDeposited">AiCreditDeposited</a> {
        balance_id: object::id(balance),
        amount_mist: amount,
        new_balance_mist,
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_withdraw"></a>

## Function `withdraw`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_withdraw">withdraw</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, amount_mist: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_withdraw">withdraw</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    amount_mist: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <b>assert</b>!(amount_mist &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> available = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance);
    <b>assert</b>!(amount_mist &lt;= available, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInsufficientBalance">EInsufficientBalance</a>);
    <b>let</b> payout = balance::split(&<b>mut</b> balance.balance, amount_mist);
    transfer::public_transfer(coin::from_balance(payout, ctx), balance.principal_owner);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditWithdrawn">AiCreditWithdrawn</a> {
        balance_id: object::id(balance),
        amount_mist,
        new_balance_mist: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a>(balance),
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_set_account_caps"></a>

## Function `set_account_caps`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_set_account_caps">set_account_caps</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_set_account_caps">set_account_caps</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    daily_cap_mist: Option&lt;u64&gt;,
    monthly_cap_mist: Option&lt;u64&gt;,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    balance.daily_cap_mist = daily_cap_mist;
    balance.monthly_cap_mist = monthly_cap_mist;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAccountCapsUpdated">AiCreditAccountCapsUpdated</a> {
        balance_id: object::id(balance),
        daily_cap_mist,
        monthly_cap_mist,
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_set_agent_budget"></a>

## Function `set_agent_budget`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_set_agent_budget">set_agent_budget</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, budget_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, require_approval_above_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_set_agent_budget">set_agent_budget</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent: &SubAgent,
    budget_mist: Option&lt;u64&gt;,
    daily_cap_mist: Option&lt;u64&gt;,
    monthly_cap_mist: Option&lt;u64&gt;,
    require_approval_above_mist: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance, agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(agent, clock);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_upsert_agent_budget">upsert_agent_budget</a>(
        balance,
        agent,
        budget_mist,
        daily_cap_mist,
        monthly_cap_mist,
        require_approval_above_mist,
        tx_context::sender(ctx),
        option::none(),
        option::none(),
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_set_agent_budget_as_manager"></a>

## Function `set_agent_budget_as_manager`

Org role-gated budget management: a holder of <code>OrgBudgetManager</code> on the org's memory
share group may manage budgets for agents belonging to that org, without the owner key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_set_agent_budget_as_manager">set_agent_budget_as_manager</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, budget_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, require_approval_above_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_set_agent_budget_as_manager">set_agent_budget_as_manager</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    org: &AgenticOrganization,
    group: &PermissionedGroup&lt;MemorySharePackage&gt;,
    agent: &SubAgent,
    budget_mist: Option&lt;u64&gt;,
    daily_cap_mist: Option&lt;u64&gt;,
    monthly_cap_mist: Option&lt;u64&gt;,
    require_approval_above_mist: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance, agent);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_org_gate_for_agent">assert_org_gate_for_agent</a>(balance, account, org, agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission">memory::assert_org_permission</a>&lt;OrgBudgetManager&gt;(org, group, tx_context::sender(ctx));
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(agent, clock);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_upsert_agent_budget">upsert_agent_budget</a>(
        balance,
        agent,
        budget_mist,
        daily_cap_mist,
        monthly_cap_mist,
        require_approval_above_mist,
        tx_context::sender(ctx),
        option::none(),
        option::some(<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">memory::organization_id</a>(org)),
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_disable_agent_budget"></a>

## Function `disable_agent_budget`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_agent_budget">disable_agent_budget</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_agent_budget">disable_agent_budget</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent_object_id: ID,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_agent_budget_internal">disable_agent_budget_internal</a>(
        balance,
        agent_object_id,
        tx_context::sender(ctx),
        option::none(),
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_approve_agent_spend"></a>

## Function `approve_agent_spend`

Owner grants a one-shot allowance: the agent may settle a single usage receipt up to
<code>max_amount_mist</code> above its approval threshold, until <code>expires_at_ms</code>. Re-approving
overwrites any existing allowance for the agent.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approve_agent_spend">approve_agent_spend</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, max_amount_mist: u64, expires_at_ms: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approve_agent_spend">approve_agent_spend</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent_object_id: ID,
    max_amount_mist: u64,
    expires_at_ms: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_store_spend_approval">store_spend_approval</a>(
        balance,
        agent_object_id,
        max_amount_mist,
        expires_at_ms,
        tx_context::sender(ctx),
        option::none(),
        option::none(),
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_approve_agent_spend_as_approver"></a>

## Function `approve_agent_spend_as_approver`

Org role-gated approval: a holder of <code>OrgSpendApprover</code> on the org's memory share
group may approve spends for agents belonging to that org (Finance Approver flow).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approve_agent_spend_as_approver">approve_agent_spend_as_approver</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, max_amount_mist: u64, expires_at_ms: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approve_agent_spend_as_approver">approve_agent_spend_as_approver</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    org: &AgenticOrganization,
    group: &PermissionedGroup&lt;MemorySharePackage&gt;,
    agent: &SubAgent,
    max_amount_mist: u64,
    expires_at_ms: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance, agent);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_org_gate_for_agent">assert_org_gate_for_agent</a>(balance, account, org, agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission">memory::assert_org_permission</a>&lt;OrgSpendApprover&gt;(org, group, tx_context::sender(ctx));
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_store_spend_approval">store_spend_approval</a>(
        balance,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(agent),
        max_amount_mist,
        expires_at_ms,
        tx_context::sender(ctx),
        option::none(),
        option::some(<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">memory::organization_id</a>(org)),
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_revoke_agent_spend_approval"></a>

## Function `revoke_agent_spend_approval`

Owner revokes an agent's live allowance.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_revoke_agent_spend_approval">revoke_agent_spend_approval</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_revoke_agent_spend_approval">revoke_agent_spend_approval</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent_object_id: ID,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    <b>let</b> key = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a> { agent_object_id };
    <b>assert</b>!(
        df::exists_with_type&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>&gt;(&balance.id, key),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EApprovalNotFound">EApprovalNotFound</a>,
    );
    <b>let</b> approval: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a> = df::remove(&<b>mut</b> balance.id, key);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditSpendApprovalRevoked">AiCreditSpendApprovalRevoked</a> {
        balance_id: object::id(balance),
        agent_object_id,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>: approval.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>,
        revoked_by: tx_context::sender(ctx),
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_set_child_agent_budget"></a>

## Function `set_child_agent_budget`

Parent agent (holding <code>CAP_BUDGET_MANAGE</code>) sets a descendant's budget. Child limits
must be at least as strict as the parent's own envelope; the human owner remains the
unconstrained root.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_set_child_agent_budget">set_child_agent_budget</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, child: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, budget_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, require_approval_above_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_set_child_agent_budget">set_child_agent_budget</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    memory_config: &MemoryConfig,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    parent: &SubAgent,
    child: &SubAgent,
    budget_mist: Option&lt;u64&gt;,
    daily_cap_mist: Option&lt;u64&gt;,
    monthly_cap_mist: Option&lt;u64&gt;,
    require_approval_above_mist: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_parent_manages_child">assert_parent_manages_child</a>(memory_config, balance, account, parent, child, clock, ctx);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_child_budget_within_parent_envelope">assert_child_budget_within_parent_envelope</a>(
        balance,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(parent),
        &budget_mist,
        &daily_cap_mist,
        &monthly_cap_mist,
        &require_approval_above_mist,
    );
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(child, clock);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_upsert_agent_budget">upsert_agent_budget</a>(
        balance,
        child,
        budget_mist,
        daily_cap_mist,
        monthly_cap_mist,
        require_approval_above_mist,
        tx_context::sender(ctx),
        option::some(<a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(parent)),
        option::none(),
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_disable_child_agent_budget"></a>

## Function `disable_child_agent_budget`

Parent kill switch for a descendant's budget.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_child_agent_budget">disable_child_agent_budget</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, child: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_child_agent_budget">disable_child_agent_budget</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    memory_config: &MemoryConfig,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    parent: &SubAgent,
    child: &SubAgent,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_parent_manages_child">assert_parent_manages_child</a>(memory_config, balance, account, parent, child, clock, ctx);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_agent_budget_internal">disable_agent_budget_internal</a>(
        balance,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(child),
        tx_context::sender(ctx),
        option::some(<a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(parent)),
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_approve_child_agent_spend"></a>

## Function `approve_child_agent_spend`

Parent approves a descendant's over-threshold spend, but only within the parent's own
envelope (its threshold and remaining caps). Beyond that, approval escalates up the
tree — ultimately to the human owner via <code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approve_agent_spend">approve_agent_spend</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approve_child_agent_spend">approve_child_agent_spend</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, child: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, max_amount_mist: u64, expires_at_ms: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approve_child_agent_spend">approve_child_agent_spend</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    memory_config: &MemoryConfig,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    parent: &SubAgent,
    child: &SubAgent,
    max_amount_mist: u64,
    expires_at_ms: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_parent_manages_child">assert_parent_manages_child</a>(memory_config, balance, account, parent, child, clock, ctx);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_within_parent_envelope">assert_within_parent_envelope</a>(
        balance,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(parent),
        max_amount_mist,
        clock,
    );
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_store_spend_approval">store_spend_approval</a>(
        balance,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(child),
        max_amount_mist,
        expires_at_ms,
        tx_context::sender(ctx),
        option::some(<a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(parent)),
        option::none(),
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_pause_balance"></a>

## Function `pause_balance`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_pause_balance">pause_balance</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_pause_balance">pause_balance</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    balance.active = <b>false</b>;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalancePaused">AiCreditBalancePaused</a> {
        balance_id: object::id(balance),
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_reactivate_balance"></a>

## Function `reactivate_balance`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reactivate_balance">reactivate_balance</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reactivate_balance">reactivate_balance</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    balance.active = <b>true</b>;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalanceReactivated">AiCreditBalanceReactivated</a> {
        balance_id: object::id(balance),
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_reserve_signed_spend"></a>

## Function `reserve_signed_spend`

Reserve an oracle-authorized, deterministic maximum charge before any provider work.
The signed intent binds the balance, agent, pricing quote, request/envelope hashes,
amount, nonce, and both deadlines.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserve_signed_spend">reserve_signed_spend</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, reservation_nonce: u64, max_amount_mist: u64, provider_envelope_hash: vector&lt;u8&gt;, request_hash: vector&lt;u8&gt;, fx_quote_id: vector&lt;u8&gt;, myso_usd_e8: u64, markup_bps: u64, timestamp_ms: u64, capture_deadline_ms: u64, hard_expiry_ms: u64, signature: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserve_signed_spend">reserve_signed_spend</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    agent: &SubAgent,
    reservation_nonce: u64,
    max_amount_mist: u64,
    provider_envelope_hash: vector&lt;u8&gt;,
    request_hash: vector&lt;u8&gt;,
    fx_quote_id: vector&lt;u8&gt;,
    myso_usd_e8: u64,
    markup_bps: u64,
    timestamp_ms: u64,
    capture_deadline_ms: u64,
    hard_expiry_ms: u64,
    signature: vector&lt;u8&gt;,
    clock: &Clock,
) {
    <b>let</b> intent = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_make_spend_reservation_intent_from_objects">make_spend_reservation_intent_from_objects</a>(
        balance,
        agent,
        reservation_nonce,
        max_amount_mist,
        provider_envelope_hash,
        request_hash,
        fx_quote_id,
        myso_usd_e8,
        markup_bps,
        timestamp_ms,
        capture_deadline_ms,
        hard_expiry_ms,
    );
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_reservation_signature">verify_reservation_signature</a>(config, &intent, &signature);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_reservation">execute_reservation</a>(config, balance, account, agent, intent, clock);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_make_spend_reservation_intent_from_objects"></a>

## Function `make_spend_reservation_intent_from_objects`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_make_spend_reservation_intent_from_objects">make_spend_reservation_intent_from_objects</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, reservation_nonce: u64, max_amount_mist: u64, provider_envelope_hash: vector&lt;u8&gt;, request_hash: vector&lt;u8&gt;, fx_quote_id: vector&lt;u8&gt;, myso_usd_e8: u64, markup_bps: u64, timestamp_ms: u64, capture_deadline_ms: u64, hard_expiry_ms: u64): <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservationIntent">social_contracts::ai_credit::SpendReservationIntent</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_make_spend_reservation_intent_from_objects">make_spend_reservation_intent_from_objects</a>(
    balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent: &SubAgent,
    reservation_nonce: u64,
    max_amount_mist: u64,
    provider_envelope_hash: vector&lt;u8&gt;,
    request_hash: vector&lt;u8&gt;,
    fx_quote_id: vector&lt;u8&gt;,
    myso_usd_e8: u64,
    markup_bps: u64,
    timestamp_ms: u64,
    capture_deadline_ms: u64,
    hard_expiry_ms: u64,
): <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservationIntent">SpendReservationIntent</a> {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservationIntent">SpendReservationIntent</a> {
        balance_id: object::id(balance),
        agent_object_id: <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(agent),
        reservation_nonce,
        max_amount_mist,
        provider_envelope_hash,
        request_hash,
        fx_quote_id,
        myso_usd_e8,
        markup_bps,
        timestamp_ms,
        capture_deadline_ms,
        hard_expiry_ms,
    }
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_capture_reserved_spend"></a>

## Function `capture_reserved_spend`

Capture the actual MIST charge. The oracle signature binds provider cost and the
provider generation hash to the reservation. Any unused maximum is released.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_capture_reserved_spend">capture_reserved_spend</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, reservation_nonce: u64, amount_mist: u64, provider_cost_usd_micros: u64, provider_generation_hash: vector&lt;u8&gt;, timestamp_ms: u64, signature: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_capture_reserved_spend">capture_reserved_spend</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    reservation_nonce: u64,
    amount_mist: u64,
    provider_cost_usd_micros: u64,
    provider_generation_hash: vector&lt;u8&gt;,
    timestamp_ms: u64,
    signature: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(table::contains(&balance.reservations, reservation_nonce), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationNotFound">EReservationNotFound</a>);
    <b>let</b> reservation = *table::borrow(&balance.reservations, reservation_nonce);
    <b>let</b> intent = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CaptureSpendIntent">CaptureSpendIntent</a> {
        balance_id: object::id(balance),
        agent_object_id: reservation.agent_object_id,
        reservation_nonce,
        amount_mist,
        provider_cost_usd_micros,
        provider_generation_hash,
        timestamp_ms,
    };
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_capture_signature">verify_capture_signature</a>(config, &intent, &signature);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_capture">execute_capture</a>(config, balance, intent, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_cancel_reserved_spend"></a>

## Function `cancel_reserved_spend`

Release a reservation only when the oracle confirms that no billable generation
exists. Signed cancellation closes at <code>capture_deadline_ms</code>; after hard expiry anyone
may call <code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_expire_reservation">expire_reservation</a></code> instead.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_cancel_reserved_spend">cancel_reserved_spend</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, reservation_nonce: u64, timestamp_ms: u64, signature: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_cancel_reserved_spend">cancel_reserved_spend</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    reservation_nonce: u64,
    timestamp_ms: u64,
    signature: vector&lt;u8&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(table::contains(&balance.reservations, reservation_nonce), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationNotFound">EReservationNotFound</a>);
    <b>let</b> reservation = *table::borrow(&balance.reservations, reservation_nonce);
    <b>let</b> intent = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CancelSpendIntent">CancelSpendIntent</a> {
        balance_id: object::id(balance),
        agent_object_id: reservation.agent_object_id,
        reservation_nonce,
        timestamp_ms,
    };
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_cancel_signature">verify_cancel_signature</a>(config, &intent, &signature);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_cancellation">execute_cancellation</a>(config, balance, intent, clock);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_expire_reservation"></a>

## Function `expire_reservation`

Permissionless safety valve so gateway failure cannot lock funds indefinitely.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_expire_reservation">expire_reservation</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, reservation_nonce: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_expire_reservation">expire_reservation</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    reservation_nonce: u64,
    clock: &Clock,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <b>assert</b>!(table::contains(&balance.reservations, reservation_nonce), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationNotFound">EReservationNotFound</a>);
    <b>let</b> reservation = *table::borrow(&balance.reservations, reservation_nonce);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &gt;= reservation.hard_expiry_ms, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationNotExpired">EReservationNotExpired</a>);
    <b>let</b> reservation = table::remove(&<b>mut</b> balance.reservations, reservation_nonce);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_release_reservation_counters">release_reservation_counters</a>(balance, &reservation, now);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiSpendExpired">AiSpendExpired</a> {
        balance_id: object::id(balance),
        agent_object_id: reservation.agent_object_id,
        reservation_nonce,
        released_mist: reservation.max_amount_mist,
        expired_at_ms: now,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance),
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_settle_usage"></a>

## Function `settle_usage`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_usage">settle_usage</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, receipt: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">social_contracts::ai_credit::UsageReceipt</a>, signature: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_usage">settle_usage</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    agent: &SubAgent,
    receipt: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a>,
    signature: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_usage_with_signature">settle_usage_with_signature</a>(config, balance, account, agent, receipt, signature, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_settle_usage_batch"></a>

## Function `settle_usage_batch`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_usage_batch">settle_usage_batch</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, receipts: vector&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">social_contracts::ai_credit::UsageReceipt</a>&gt;, signatures: vector&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_usage_batch">settle_usage_batch</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    agent: &SubAgent,
    receipts: vector&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a>&gt;,
    signatures: vector&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> n = vector::length(&receipts);
    <b>assert</b>!(n == vector::length(&signatures), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> receipt = *vector::borrow(&receipts, i);
        <b>let</b> sig = *vector::borrow(&signatures, i);
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_usage_with_signature">settle_usage_with_signature</a>(config, balance, account, agent, receipt, sig, clock, ctx);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_settle_signed_usage"></a>

## Function `settle_signed_usage`

PTB/oracle entry point: receipt fields are primitives; balance and agent IDs are derived
from shared objects (not caller-supplied). Authorization is the oracle Ed25519 signature,
not <code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_signed_usage">settle_signed_usage</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, receipt_id: u128, amount_mist: u64, usage_kind: u8, timestamp_ms: u64, settlement_nonce: u64, signature: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_signed_usage">settle_signed_usage</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    agent: &SubAgent,
    receipt_id: u128,
    amount_mist: u64,
    usage_kind: u8,
    timestamp_ms: u64,
    settlement_nonce: u64,
    signature: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> receipt = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_make_usage_receipt_from_objects">make_usage_receipt_from_objects</a>(
        balance,
        agent,
        receipt_id,
        amount_mist,
        usage_kind,
        timestamp_ms,
        settlement_nonce,
    );
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_usage_with_signature">settle_usage_with_signature</a>(config, balance, account, agent, receipt, signature, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_make_usage_receipt_from_objects"></a>

## Function `make_usage_receipt_from_objects`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_make_usage_receipt_from_objects">make_usage_receipt_from_objects</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, receipt_id: u128, amount_mist: u64, usage_kind: u8, timestamp_ms: u64, settlement_nonce: u64): <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">social_contracts::ai_credit::UsageReceipt</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_make_usage_receipt_from_objects">make_usage_receipt_from_objects</a>(
    balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent: &SubAgent,
    receipt_id: u128,
    amount_mist: u64,
    usage_kind: u8,
    timestamp_ms: u64,
    settlement_nonce: u64,
): <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a> {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a> {
        balance_id: object::id(balance),
        agent_object_id: <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(agent),
        receipt_id,
        amount_mist,
        usage_kind,
        timestamp_ms,
        settlement_nonce,
    }
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_settle_usage_with_signature"></a>

## Function `settle_usage_with_signature`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_usage_with_signature">settle_usage_with_signature</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, receipt: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">social_contracts::ai_credit::UsageReceipt</a>, signature: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_settle_usage_with_signature">settle_usage_with_signature</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    agent: &SubAgent,
    receipt: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a>,
    signature: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_receipt_signature">verify_receipt_signature</a>(config, &receipt, &signature);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_settlement">execute_settlement</a>(config, balance, account, agent, receipt, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_execute_settlement"></a>

## Function `execute_settlement`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_settlement">execute_settlement</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, receipt: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">social_contracts::ai_credit::UsageReceipt</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_settlement">execute_settlement</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    agent: &SubAgent,
    receipt: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance, agent);
    <b>assert</b>!(object::id(account) == balance.memory_account_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(agent) == receipt.agent_object_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>);
    <b>assert</b>!(object::id(balance) == receipt.balance_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(agent, clock);
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_has_cap">memory::has_cap</a>(<a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_capabilities">memory::sub_agent_capabilities</a>(agent), <a href="../social_contracts/memory.md#social_contracts_memory_cap_ai_spend">memory::cap_ai_spend</a>()),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentMissingCap">EAgentMissingCap</a>,
    );
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_receipt_fresh">assert_receipt_fresh</a>(config, &receipt, clock);
    <b>assert</b>!(receipt.amount_mist &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(receipt.amount_mist &lt;= config.max_single_settlement_mist, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>);
    <b>assert</b>!(receipt.settlement_nonce == balance.settlement_nonce + 1, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidNonce">EInvalidNonce</a>);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_account_windows">roll_account_windows</a>(balance, clock::timestamp_ms(clock));
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_account_caps">assert_account_caps</a>(balance, receipt.amount_mist);
    <b>let</b> agent_id = <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(agent);
    <b>if</b> (table::contains(&balance.agent_budgets, agent_id)) {
        <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> balance.agent_budgets, agent_id);
        <b>assert</b>!(<b>entry</b>.enabled, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentDisabled">EAgentDisabled</a>);
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_agent_windows">roll_agent_windows</a>(<b>entry</b>, clock::timestamp_ms(clock));
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_caps">assert_agent_caps</a>(<b>entry</b>, receipt.amount_mist);
        <b>entry</b>.spent_mist = <b>entry</b>.spent_mist + receipt.amount_mist;
        <b>entry</b>.spent_day_mist = <b>entry</b>.spent_day_mist + receipt.amount_mist;
        <b>entry</b>.spent_month_mist = <b>entry</b>.spent_month_mist + receipt.amount_mist;
    };
    // Over-threshold settlements must consume a live spend allowance (the previously
    // unenforced `require_approval_above_mist` gate).
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_maybe_consume_spend_approval">maybe_consume_spend_approval</a>(balance, agent_id, receipt.amount_mist, clock);
    <b>assert</b>!(receipt.amount_mist &lt;= <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInsufficientBalance">EInsufficientBalance</a>);
    balance.settlement_nonce = receipt.settlement_nonce;
    balance.spent_total_mist = balance.spent_total_mist + receipt.amount_mist;
    balance.spent_day_mist = balance.spent_day_mist + receipt.amount_mist;
    balance.spent_month_mist = balance.spent_month_mist + receipt.amount_mist;
    <b>let</b> payout = balance::split(&<b>mut</b> balance.balance, receipt.amount_mist);
    transfer::public_transfer(coin::from_balance(payout, ctx), config.treasury);
    <b>let</b> remaining = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a>(balance);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditUsageSettled">AiCreditUsageSettled</a> {
        balance_id: object::id(balance),
        agent_object_id: agent_id,
        receipt_id: receipt.receipt_id,
        amount_mist: receipt.amount_mist,
        usage_kind: receipt.usage_kind,
        settlement_nonce: receipt.settlement_nonce,
        remaining_mist: remaining,
    });
    <b>if</b> (remaining == 0) {
        event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalanceDepleted">AiCreditBalanceDepleted</a> {
            balance_id: object::id(balance),
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_execute_reservation"></a>

## Function `execute_reservation`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_reservation">execute_reservation</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservationIntent">social_contracts::ai_credit::SpendReservationIntent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_reservation">execute_reservation</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    agent: &SubAgent,
    intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservationIntent">SpendReservationIntent</a>,
    clock: &Clock,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance, agent);
    <b>assert</b>!(object::id(account) == balance.memory_account_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>);
    <b>assert</b>!(object::id(balance) == intent.balance_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationMismatch">EReservationMismatch</a>);
    <b>let</b> agent_id = <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(agent);
    <b>assert</b>!(agent_id == intent.agent_object_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationMismatch">EReservationMismatch</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(agent, clock);
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_has_cap">memory::has_cap</a>(<a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_capabilities">memory::sub_agent_capabilities</a>(agent), <a href="../social_contracts/memory.md#social_contracts_memory_cap_ai_spend">memory::cap_ai_spend</a>()),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentMissingCap">EAgentMissingCap</a>,
    );
    <b>assert</b>!(intent.max_amount_mist &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(intent.max_amount_mist &lt;= config.max_single_settlement_mist, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>);
    <b>assert</b>!(intent.reservation_nonce == balance.reservation_nonce + 1, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidNonce">EInvalidNonce</a>);
    <b>assert</b>!(!table::contains(&balance.reservations, intent.reservation_nonce), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidNonce">EInvalidNonce</a>);
    <b>assert</b>!(vector::length(&intent.provider_envelope_hash) == <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_HASH_LEN">HASH_LEN</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidHash">EInvalidHash</a>);
    <b>assert</b>!(vector::length(&intent.request_hash) == <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_HASH_LEN">HASH_LEN</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidHash">EInvalidHash</a>);
    <b>assert</b>!(vector::length(&intent.fx_quote_id) &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidHash">EInvalidHash</a>);
    <b>assert</b>!(intent.myso_usd_e8 &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(intent.markup_bps == config.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EMarkupMismatch">EMarkupMismatch</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_signed_timestamp_fresh">assert_signed_timestamp_fresh</a>(config, intent.timestamp_ms, now);
    <b>assert</b>!(intent.capture_deadline_ms &gt; now, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidReservationWindow">EInvalidReservationWindow</a>);
    <b>assert</b>!(intent.hard_expiry_ms &gt; intent.capture_deadline_ms, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidReservationWindow">EInvalidReservationWindow</a>);
    <b>assert</b>!(
        intent.capture_deadline_ms - now &lt;= <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MAX_CAPTURE_WINDOW_MS">MAX_CAPTURE_WINDOW_MS</a>,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidReservationWindow">EInvalidReservationWindow</a>,
    );
    <b>assert</b>!(
        intent.hard_expiry_ms - now &lt;= <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MAX_RESERVATION_LIFETIME_MS">MAX_RESERVATION_LIFETIME_MS</a>,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidReservationWindow">EInvalidReservationWindow</a>,
    );
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_account_windows">roll_account_windows</a>(balance, now);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_account_caps">assert_account_caps</a>(balance, intent.max_amount_mist);
    <b>assert</b>!(intent.max_amount_mist &lt;= <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInsufficientBalance">EInsufficientBalance</a>);
    <b>let</b> agent_budget_reserved = table::contains(&balance.agent_budgets, agent_id);
    <b>let</b> (agent_day_anchor_ms, agent_month_anchor_ms) = <b>if</b> (agent_budget_reserved) {
        <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> balance.agent_budgets, agent_id);
        <b>assert</b>!(<b>entry</b>.enabled, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentDisabled">EAgentDisabled</a>);
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_agent_windows">roll_agent_windows</a>(<b>entry</b>, now);
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_caps">assert_agent_caps</a>(<b>entry</b>, intent.max_amount_mist);
        <b>let</b> day_anchor = <b>entry</b>.day_anchor_ms;
        <b>let</b> month_anchor = <b>entry</b>.month_anchor_ms;
        <b>entry</b>.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> = <b>entry</b>.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> + intent.max_amount_mist;
        <b>entry</b>.reserved_day_mist = <b>entry</b>.reserved_day_mist + intent.max_amount_mist;
        <b>entry</b>.reserved_month_mist = <b>entry</b>.reserved_month_mist + intent.max_amount_mist;
        (day_anchor, month_anchor)
    } <b>else</b> {
        (0, 0)
    };
    // Approval is consumed when funds become unavailable, not after provider spend.
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_maybe_consume_spend_approval">maybe_consume_spend_approval</a>(balance, agent_id, intent.max_amount_mist, clock);
    balance.reservation_nonce = intent.reservation_nonce;
    balance.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> = balance.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> + intent.max_amount_mist;
    balance.reserved_day_mist = balance.reserved_day_mist + intent.max_amount_mist;
    balance.reserved_month_mist = balance.reserved_month_mist + intent.max_amount_mist;
    <b>let</b> reservation = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">SpendReservation</a> {
        reservation_nonce: intent.reservation_nonce,
        agent_object_id: agent_id,
        max_amount_mist: intent.max_amount_mist,
        provider_envelope_hash: intent.provider_envelope_hash,
        request_hash: intent.request_hash,
        fx_quote_id: intent.fx_quote_id,
        myso_usd_e8: intent.myso_usd_e8,
        markup_bps: intent.markup_bps,
        created_at_ms: now,
        capture_deadline_ms: intent.capture_deadline_ms,
        hard_expiry_ms: intent.hard_expiry_ms,
        account_day_anchor_ms: balance.day_anchor_ms,
        account_month_anchor_ms: balance.month_anchor_ms,
        agent_budget_reserved,
        agent_day_anchor_ms,
        agent_month_anchor_ms,
    };
    table::add(&<b>mut</b> balance.reservations, intent.reservation_nonce, reservation);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiSpendReserved">AiSpendReserved</a> {
        balance_id: object::id(balance),
        agent_object_id: agent_id,
        reservation_nonce: intent.reservation_nonce,
        max_amount_mist: intent.max_amount_mist,
        provider_envelope_hash: intent.provider_envelope_hash,
        request_hash: intent.request_hash,
        fx_quote_id: intent.fx_quote_id,
        myso_usd_e8: intent.myso_usd_e8,
        markup_bps: intent.markup_bps,
        capture_deadline_ms: intent.capture_deadline_ms,
        hard_expiry_ms: intent.hard_expiry_ms,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance),
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_execute_capture"></a>

## Function `execute_capture`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_capture">execute_capture</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CaptureSpendIntent">social_contracts::ai_credit::CaptureSpendIntent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_capture">execute_capture</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CaptureSpendIntent">CaptureSpendIntent</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <b>assert</b>!(table::contains(&balance.reservations, intent.reservation_nonce), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationNotFound">EReservationNotFound</a>);
    <b>assert</b>!(intent.balance_id == object::id(balance), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationMismatch">EReservationMismatch</a>);
    <b>assert</b>!(intent.amount_mist &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(intent.provider_cost_usd_micros &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(vector::length(&intent.provider_generation_hash) == <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_HASH_LEN">HASH_LEN</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidHash">EInvalidHash</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_signed_timestamp_fresh">assert_signed_timestamp_fresh</a>(config, intent.timestamp_ms, now);
    <b>let</b> reservation = *table::borrow(&balance.reservations, intent.reservation_nonce);
    <b>assert</b>!(intent.agent_object_id == reservation.agent_object_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationMismatch">EReservationMismatch</a>);
    <b>assert</b>!(intent.amount_mist &lt;= reservation.max_amount_mist, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>);
    <b>assert</b>!(now &lt; reservation.hard_expiry_ms, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationExpired">EReservationExpired</a>);
    <b>let</b> reservation = table::remove(&<b>mut</b> balance.reservations, intent.reservation_nonce);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_capture_reservation_counters">capture_reservation_counters</a>(balance, &reservation, intent.amount_mist, now);
    <b>let</b> payout = balance::split(&<b>mut</b> balance.balance, intent.amount_mist);
    transfer::public_transfer(coin::from_balance(payout, ctx), config.treasury);
    <b>let</b> remaining = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a>(balance);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiSpendCaptured">AiSpendCaptured</a> {
        balance_id: object::id(balance),
        agent_object_id: reservation.agent_object_id,
        reservation_nonce: reservation.reservation_nonce,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>: reservation.max_amount_mist,
        captured_mist: intent.amount_mist,
        released_mist: reservation.max_amount_mist - intent.amount_mist,
        provider_cost_usd_micros: intent.provider_cost_usd_micros,
        provider_generation_hash: intent.provider_generation_hash,
        fx_quote_id: reservation.fx_quote_id,
        myso_usd_e8: reservation.myso_usd_e8,
        markup_bps: reservation.markup_bps,
        captured_at_ms: now,
        remaining_mist: remaining,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance),
    });
    <b>if</b> (remaining == 0) {
        event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalanceDepleted">AiCreditBalanceDepleted</a> {
            balance_id: object::id(balance),
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_execute_cancellation"></a>

## Function `execute_cancellation`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_cancellation">execute_cancellation</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CancelSpendIntent">social_contracts::ai_credit::CancelSpendIntent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_execute_cancellation">execute_cancellation</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CancelSpendIntent">CancelSpendIntent</a>,
    clock: &Clock,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <b>assert</b>!(table::contains(&balance.reservations, intent.reservation_nonce), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationNotFound">EReservationNotFound</a>);
    <b>assert</b>!(intent.balance_id == object::id(balance), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationMismatch">EReservationMismatch</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_signed_timestamp_fresh">assert_signed_timestamp_fresh</a>(config, intent.timestamp_ms, now);
    <b>let</b> reservation = *table::borrow(&balance.reservations, intent.reservation_nonce);
    <b>assert</b>!(intent.agent_object_id == reservation.agent_object_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationMismatch">EReservationMismatch</a>);
    <b>assert</b>!(now &lt;= reservation.capture_deadline_ms, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECaptureWindowClosed">ECaptureWindowClosed</a>);
    <b>let</b> reservation = table::remove(&<b>mut</b> balance.reservations, intent.reservation_nonce);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_release_reservation_counters">release_reservation_counters</a>(balance, &reservation, now);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiSpendCancelled">AiSpendCancelled</a> {
        balance_id: object::id(balance),
        agent_object_id: reservation.agent_object_id,
        reservation_nonce: reservation.reservation_nonce,
        released_mist: reservation.max_amount_mist,
        cancelled_at_ms: now,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance),
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_update_oracle_pubkey"></a>

## Function `update_oracle_pubkey`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_oracle_pubkey">update_oracle_pubkey</a>(cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">social_contracts::ai_credit::AiCreditOracleAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, new_pk: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_oracle_pubkey">update_oracle_pubkey</a>(
    cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    new_pk: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_oracle_admin">assert_oracle_admin</a>(cap, ctx);
    <b>assert</b>!(vector::length(&new_pk) == <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ED25519_PK_LEN">ED25519_PK_LEN</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidPubkey">EInvalidPubkey</a>);
    config.oracle_pubkey = new_pk;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOraclePubkeyUpdated">AiCreditOraclePubkeyUpdated</a> {
        updated_by: tx_context::sender(ctx),
        new_pubkey: new_pk,
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_update_oracle_markup"></a>

## Function `update_oracle_markup`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_oracle_markup">update_oracle_markup</a>(cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">social_contracts::ai_credit::AiCreditOracleAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_oracle_markup">update_oracle_markup</a>(
    cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>: u64,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_oracle_admin">assert_oracle_admin</a>(cap, ctx);
    <b>assert</b>!(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a> &lt;= 10000, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    config.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a> = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditMarkupUpdated">AiCreditMarkupUpdated</a> {
        updated_by: tx_context::sender(ctx),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_update_min_deposit"></a>

## Function `update_min_deposit`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_min_deposit">update_min_deposit</a>(cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">social_contracts::ai_credit::AiCreditOracleAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_min_deposit">update_min_deposit</a>(
    cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>: u64,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_oracle_admin">assert_oracle_admin</a>(cap, ctx);
    <b>assert</b>!(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a> &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    config.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a> = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditMinDepositUpdated">AiCreditMinDepositUpdated</a> {
        updated_by: tx_context::sender(ctx),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_update_settlement_limits"></a>

## Function `update_settlement_limits`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_settlement_limits">update_settlement_limits</a>(cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">social_contracts::ai_credit::AiCreditOracleAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, max_single_settlement_mist: u64, receipt_ttl_ms: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_settlement_limits">update_settlement_limits</a>(
    cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    max_single_settlement_mist: u64,
    receipt_ttl_ms: u64,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_oracle_admin">assert_oracle_admin</a>(cap, ctx);
    <b>assert</b>!(max_single_settlement_mist &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    <b>assert</b>!(receipt_ttl_ms &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    config.max_single_settlement_mist = max_single_settlement_mist;
    config.receipt_ttl_ms = receipt_ttl_ms;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditSettlementLimitsUpdated">AiCreditSettlementLimitsUpdated</a> {
        max_single_settlement_mist,
        receipt_ttl_ms,
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_update_treasury"></a>

## Function `update_treasury`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_treasury">update_treasury</a>(cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">social_contracts::ai_credit::AiCreditOracleAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, treasury: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_update_treasury">update_treasury</a>(
      cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a>,
      config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
      treasury: <b>address</b>,
      ctx: &TxContext,
  ) {
      <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_oracle_admin">assert_oracle_admin</a>(cap, ctx);
      config.treasury = treasury;
  }
</code></pre>



</details>

<a name="social_contracts_ai_credit_balance_mist"></a>

## Function `balance_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>): u64 {
    balance::value(&balance.balance)
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_available_mist"></a>

## Function `available_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>): u64 {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a>(balance) - balance.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_reserved_mist"></a>

## Function `reserved_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>): u64 {
    balance.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_latest_reservation_nonce"></a>

## Function `latest_reservation_nonce`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_latest_reservation_nonce">latest_reservation_nonce</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_latest_reservation_nonce">latest_reservation_nonce</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>): u64 {
    balance.reservation_nonce
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_reservation_for"></a>

## Function `reservation_for`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reservation_for">reservation_for</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, reservation_nonce: u64): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">social_contracts::ai_credit::SpendReservation</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reservation_for">reservation_for</a>(
    balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    reservation_nonce: u64,
): Option&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">SpendReservation</a>&gt; {
    <b>if</b> (table::contains(&balance.reservations, reservation_nonce)) {
        option::some(*table::borrow(&balance.reservations, reservation_nonce))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_reservation_max_amount_mist"></a>

## Function `reservation_max_amount_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reservation_max_amount_mist">reservation_max_amount_mist</a>(reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">social_contracts::ai_credit::SpendReservation</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reservation_max_amount_mist">reservation_max_amount_mist</a>(reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">SpendReservation</a>): u64 {
    reservation.max_amount_mist
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_reservation_agent_object_id"></a>

## Function `reservation_agent_object_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reservation_agent_object_id">reservation_agent_object_id</a>(reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">social_contracts::ai_credit::SpendReservation</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reservation_agent_object_id">reservation_agent_object_id</a>(reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">SpendReservation</a>): ID {
    reservation.agent_object_id
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_reservation_hard_expiry_ms"></a>

## Function `reservation_hard_expiry_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reservation_hard_expiry_ms">reservation_hard_expiry_ms</a>(reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">social_contracts::ai_credit::SpendReservation</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reservation_hard_expiry_ms">reservation_hard_expiry_ms</a>(reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">SpendReservation</a>): u64 {
    reservation.hard_expiry_ms
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_oracle_markup_bps"></a>

## Function `oracle_markup_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>): u64 {
    config.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_oracle_markup_bps">oracle_markup_bps</a>
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_min_deposit_mist"></a>

## Function `min_deposit_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>): u64 {
    config.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_min_deposit_mist">min_deposit_mist</a>
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_spend_approval_for"></a>

## Function `spend_approval_for`

Live allowance for an agent, if any (may be expired — check <code>approval_expires_at</code>).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_spend_approval_for">spend_approval_for</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">social_contracts::ai_credit::SpendApproval</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_spend_approval_for">spend_approval_for</a>(
    balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent_object_id: ID,
): Option&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>&gt; {
    <b>let</b> key = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a> { agent_object_id };
    <b>if</b> (df::exists_with_type&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>&gt;(&balance.id, key)) {
        option::some(*df::borrow&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>&gt;(&balance.id, key))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_approval_max_amount_mist"></a>

## Function `approval_max_amount_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_max_amount_mist">approval_max_amount_mist</a>(approval: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">social_contracts::ai_credit::SpendApproval</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_max_amount_mist">approval_max_amount_mist</a>(approval: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>): u64 {
    approval.max_amount_mist
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_approval_expires_at_ms"></a>

## Function `approval_expires_at_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_expires_at_ms">approval_expires_at_ms</a>(approval: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">social_contracts::ai_credit::SpendApproval</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_expires_at_ms">approval_expires_at_ms</a>(approval: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>): u64 {
    approval.expires_at_ms
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_approval_approved_by"></a>

## Function `approval_approved_by`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_approved_by">approval_approved_by</a>(approval: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">social_contracts::ai_credit::SpendApproval</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_approved_by">approval_approved_by</a>(approval: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>): <b>address</b> {
    approval.approved_by
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_approval_nonce"></a>

## Function `approval_nonce`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>(approval: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">social_contracts::ai_credit::SpendApproval</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>(approval: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>): u64 {
    approval.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_agent_approval_threshold"></a>

## Function `agent_approval_threshold`

Approval threshold on the agent's budget entry, if configured.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_agent_approval_threshold">agent_approval_threshold</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_agent_approval_threshold">agent_approval_threshold</a>(
    balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent_object_id: ID,
): Option&lt;u64&gt; {
    <b>if</b> (!table::contains(&balance.agent_budgets, agent_object_id)) {
        <b>return</b> option::none()
    };
    <b>let</b> <b>entry</b> = table::borrow(&balance.agent_budgets, agent_object_id);
    <b>entry</b>.require_approval_above_mist
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_agent_remaining_mist"></a>

## Function `agent_remaining_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_agent_remaining_mist">agent_remaining_mist</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_agent_remaining_mist">agent_remaining_mist</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>, agent_object_id: ID): Option&lt;u64&gt; {
    <b>if</b> (!table::contains(&balance.agent_budgets, agent_object_id)) {
        <b>return</b> option::none()
    };
    <b>let</b> <b>entry</b> = table::borrow(&balance.agent_budgets, agent_object_id);
    <b>if</b> (option::is_none(&<b>entry</b>.budget_mist)) {
        <b>return</b> option::none()
    };
    <b>let</b> max = *option::borrow(&<b>entry</b>.budget_mist);
    <b>let</b> committed = <b>entry</b>.spent_mist + <b>entry</b>.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>;
    <b>if</b> (committed &gt;= max) {
        option::some(0)
    } <b>else</b> {
        option::some(max - committed)
    }
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_usage_inference"></a>

## Function `usage_inference`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_inference">usage_inference</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_inference">usage_inference</a>(): u8 { <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_INFERENCE">USAGE_INFERENCE</a> }
</code></pre>



</details>

<a name="social_contracts_ai_credit_usage_tool"></a>

## Function `usage_tool`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_tool">usage_tool</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_tool">usage_tool</a>(): u8 { <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_TOOL">USAGE_TOOL</a> }
</code></pre>



</details>

<a name="social_contracts_ai_credit_usage_embed"></a>

## Function `usage_embed`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_embed">usage_embed</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_embed">usage_embed</a>(): u8 { <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_EMBED">USAGE_EMBED</a> }
</code></pre>



</details>

<a name="social_contracts_ai_credit_usage_storage"></a>

## Function `usage_storage`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_storage">usage_storage</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_storage">usage_storage</a>(): u8 { <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_STORAGE">USAGE_STORAGE</a> }
</code></pre>



</details>

<a name="social_contracts_ai_credit_usage_workflow"></a>

## Function `usage_workflow`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_workflow">usage_workflow</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_usage_workflow">usage_workflow</a>(): u8 { <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_USAGE_WORKFLOW">USAGE_WORKFLOW</a> }
</code></pre>



</details>

<a name="social_contracts_ai_credit_mist_per_myso"></a>

## Function `mist_per_myso`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_mist_per_myso">mist_per_myso</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_mist_per_myso">mist_per_myso</a>(): u64 { <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MIST_PER_MYSO">MIST_PER_MYSO</a> }
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_agent_may_spend"></a>

## Function `assert_agent_may_spend`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_may_spend">assert_agent_may_spend</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, amount_mist: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_may_spend">assert_agent_may_spend</a>(
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    agent: &SubAgent,
    amount_mist: u64,
    clock: &Clock,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance, agent);
    <b>assert</b>!(object::id(account) == balance.memory_account_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(agent, clock);
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_has_cap">memory::has_cap</a>(<a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_capabilities">memory::sub_agent_capabilities</a>(agent), <a href="../social_contracts/memory.md#social_contracts_memory_cap_ai_spend">memory::cap_ai_spend</a>()),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentMissingCap">EAgentMissingCap</a>,
    );
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_account_windows">roll_account_windows</a>(balance, clock::timestamp_ms(clock));
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_account_caps">assert_account_caps</a>(balance, amount_mist);
    <b>let</b> agent_id = <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(agent);
    <b>if</b> (table::contains(&balance.agent_budgets, agent_id)) {
        <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> balance.agent_budgets, agent_id);
        <b>assert</b>!(<b>entry</b>.enabled, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentDisabled">EAgentDisabled</a>);
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_agent_windows">roll_agent_windows</a>(<b>entry</b>, clock::timestamp_ms(clock));
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_caps">assert_agent_caps</a>(<b>entry</b>, amount_mist);
    };
    <b>assert</b>!(amount_mist &lt;= <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_available_mist">available_mist</a>(balance), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInsufficientBalance">EInsufficientBalance</a>);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_version"></a>

## Function `assert_version`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>, balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>) {
    <b>assert</b>!(config.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(balance.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_owner"></a>

## Function `assert_owner`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>, ctx: &TxContext) {
    <b>assert</b>!(tx_context::sender(ctx) == balance.principal_owner, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ENotOwner">ENotOwner</a>);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_active"></a>

## Function `assert_active`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_active">assert_active</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>) {
    <b>assert</b>!(balance.active, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInactive">EInactive</a>);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_agent_linked"></a>

## Function `assert_agent_linked`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>, agent: &SubAgent) {
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_memory_account_id">memory::sub_agent_memory_account_id</a>(agent) == balance.memory_account_id,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_org_gate_for_agent"></a>

## Function `assert_org_gate_for_agent`

Org role gates require: account matches the balance, org belongs to the account,
and the target agent belongs to that org.


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_org_gate_for_agent">assert_org_gate_for_agent</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_org_gate_for_agent">assert_org_gate_for_agent</a>(
    balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    org: &AgenticOrganization,
    agent: &SubAgent,
) {
    <b>assert</b>!(object::id(account) == balance.memory_account_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>);
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_memory_account_id">memory::organization_memory_account_id</a>(org) == object::id(account),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>,
    );
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_organization_id">memory::sub_agent_organization_id</a>(agent) == <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">memory::organization_id</a>(org),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentNotInOrg">EAgentNotInOrg</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_parent_manages_child"></a>

## Function `assert_parent_manages_child`

Common authorization for parent-delegated budget operations: sender is the parent's
derived address, parent is active with <code>CAP_BUDGET_MANAGE</code>, both agents are linked to
this balance, and the child sits strictly below the parent in the agent tree.


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_parent_manages_child">assert_parent_manages_child</a>(memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, child: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_parent_manages_child">assert_parent_manages_child</a>(
    memory_config: &MemoryConfig,
    balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    account: &MemoryAccount,
    parent: &SubAgent,
    child: &SubAgent,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>assert</b>!(object::id(account) == balance.memory_account_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAccountMismatch">EAccountMismatch</a>);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance, parent);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_linked">assert_agent_linked</a>(balance, child);
    <b>assert</b>!(
        tx_context::sender(ctx) == <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_derived_address">memory::sub_agent_derived_address</a>(parent),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ENotParentSigner">ENotParentSigner</a>,
    );
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">memory::assert_sub_agent_active</a>(parent, clock);
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_has_cap">memory::has_cap</a>(<a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_capabilities">memory::sub_agent_capabilities</a>(parent), <a href="../social_contracts/memory.md#social_contracts_memory_cap_budget_manage">memory::cap_budget_manage</a>()),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentMissingCap">EAgentMissingCap</a>,
    );
    <b>let</b> parent_id = <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(parent);
    <b>let</b> child_id = <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(child);
    <b>assert</b>!(parent_id != child_id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECannotManageSelf">ECannotManageSelf</a>);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_is_descendant_agent">memory::is_descendant_agent</a>(memory_config, account, parent_id, child_id), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ENotDescendant">ENotDescendant</a>);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_child_budget_within_parent_envelope"></a>

## Function `assert_child_budget_within_parent_envelope`

Child budget limits must be at least as strict as the parent's own entry (when the
parent has one; an unconstrained parent may set anything).


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_child_budget_within_parent_envelope">assert_child_budget_within_parent_envelope</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, parent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, budget_mist: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, daily_cap_mist: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, monthly_cap_mist: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, require_approval_above_mist: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_child_budget_within_parent_envelope">assert_child_budget_within_parent_envelope</a>(
    balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    parent_id: ID,
    budget_mist: &Option&lt;u64&gt;,
    daily_cap_mist: &Option&lt;u64&gt;,
    monthly_cap_mist: &Option&lt;u64&gt;,
    require_approval_above_mist: &Option&lt;u64&gt;,
) {
    <b>if</b> (!table::contains(&balance.agent_budgets, parent_id)) {
        <b>return</b>
    };
    <b>let</b> parent_entry = table::borrow(&balance.agent_budgets, parent_id);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_limit_not_looser">assert_limit_not_looser</a>(budget_mist, &parent_entry.budget_mist);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_limit_not_looser">assert_limit_not_looser</a>(daily_cap_mist, &parent_entry.daily_cap_mist);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_limit_not_looser">assert_limit_not_looser</a>(monthly_cap_mist, &parent_entry.monthly_cap_mist);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_limit_not_looser">assert_limit_not_looser</a>(
        require_approval_above_mist,
        &parent_entry.require_approval_above_mist,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_limit_not_looser"></a>

## Function `assert_limit_not_looser`

<code>child</code> is not looser than <code>parent</code>: when the parent limit is set, the child limit
must be set and must not exceed it.


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_limit_not_looser">assert_limit_not_looser</a>(child: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, parent: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_limit_not_looser">assert_limit_not_looser</a>(child: &Option&lt;u64&gt;, parent: &Option&lt;u64&gt;) {
    <b>if</b> (option::is_none(parent)) {
        <b>return</b>
    };
    <b>assert</b>!(option::is_some(child), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EParentEnvelopeExceeded">EParentEnvelopeExceeded</a>);
    <b>assert</b>!(*option::borrow(child) &lt;= *option::borrow(parent), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EParentEnvelopeExceeded">EParentEnvelopeExceeded</a>);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_within_parent_envelope"></a>

## Function `assert_within_parent_envelope`

Parents may only approve amounts they could spend themselves: within their own
approval threshold (if set) and remaining budget/day/month caps (if set).


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_within_parent_envelope">assert_within_parent_envelope</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, parent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, amount_mist: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_within_parent_envelope">assert_within_parent_envelope</a>(
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    parent_id: ID,
    amount_mist: u64,
    clock: &Clock,
) {
    <b>if</b> (!table::contains(&balance.agent_budgets, parent_id)) {
        <b>return</b>
    };
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> balance.agent_budgets, parent_id);
    <b>assert</b>!(<b>entry</b>.enabled, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentDisabled">EAgentDisabled</a>);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_agent_windows">roll_agent_windows</a>(<b>entry</b>, clock::timestamp_ms(clock));
    <b>if</b> (option::is_some(&<b>entry</b>.require_approval_above_mist)) {
        <b>assert</b>!(
            amount_mist &lt;= *option::borrow(&<b>entry</b>.require_approval_above_mist),
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EParentEnvelopeExceeded">EParentEnvelopeExceeded</a>,
        );
    };
    <b>if</b> (option::is_some(&<b>entry</b>.budget_mist)) {
        <b>let</b> max = *option::borrow(&<b>entry</b>.budget_mist);
        <b>assert</b>!(
            <b>entry</b>.spent_mist + <b>entry</b>.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> + amount_mist &lt;= max,
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EParentEnvelopeExceeded">EParentEnvelopeExceeded</a>,
        );
    };
    <b>if</b> (option::is_some(&<b>entry</b>.daily_cap_mist)) {
        <b>let</b> cap = *option::borrow(&<b>entry</b>.daily_cap_mist);
        <b>assert</b>!(
            <b>entry</b>.spent_day_mist + <b>entry</b>.reserved_day_mist + amount_mist &lt;= cap,
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EParentEnvelopeExceeded">EParentEnvelopeExceeded</a>,
        );
    };
    <b>if</b> (option::is_some(&<b>entry</b>.monthly_cap_mist)) {
        <b>let</b> cap = *option::borrow(&<b>entry</b>.monthly_cap_mist);
        <b>assert</b>!(
            <b>entry</b>.spent_month_mist + <b>entry</b>.reserved_month_mist + amount_mist &lt;= cap,
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EParentEnvelopeExceeded">EParentEnvelopeExceeded</a>,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_upsert_agent_budget"></a>

## Function `upsert_agent_budget`

Shared budget upsert used by owner, org-manager, and parent paths. Emits the legacy
<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetUpdated">AiCreditAgentBudgetUpdated</a></code> plus the audit-grade <code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetChanged">AiCreditAgentBudgetChanged</a></code>.


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_upsert_agent_budget">upsert_agent_budget</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, budget_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, daily_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, monthly_cap_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, require_approval_above_mist: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, set_by: <b>address</b>, set_by_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_upsert_agent_budget">upsert_agent_budget</a>(
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent: &SubAgent,
    budget_mist: Option&lt;u64&gt;,
    daily_cap_mist: Option&lt;u64&gt;,
    monthly_cap_mist: Option&lt;u64&gt;,
    require_approval_above_mist: Option&lt;u64&gt;,
    set_by: <b>address</b>,
    set_by_agent_id: Option&lt;ID&gt;,
    organization_id: Option&lt;ID&gt;,
    clock: &Clock,
) {
    <b>let</b> agent_id = <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(agent);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> had_previous_entry = table::contains(&balance.agent_budgets, agent_id);
    <b>let</b> (prev_budget, prev_daily, prev_monthly, prev_approval, prev_enabled) =
        <b>if</b> (had_previous_entry) {
            <b>let</b> prev = table::borrow(&balance.agent_budgets, agent_id);
            (
                prev.budget_mist,
                prev.daily_cap_mist,
                prev.monthly_cap_mist,
                prev.require_approval_above_mist,
                prev.enabled,
            )
        } <b>else</b> {
            (option::none(), option::none(), option::none(), option::none(), <b>false</b>)
        };
    <b>let</b> <b>entry</b> = <b>if</b> (had_previous_entry) {
        <b>let</b> e = table::borrow_mut(&<b>mut</b> balance.agent_budgets, agent_id);
        e.budget_mist = budget_mist;
        e.daily_cap_mist = daily_cap_mist;
        e.monthly_cap_mist = monthly_cap_mist;
        e.require_approval_above_mist = require_approval_above_mist;
        e.enabled = <b>true</b>;
        *e
    } <b>else</b> {
        <b>let</b> e = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AgentBudgetEntry">AgentBudgetEntry</a> {
            agent_object_id: agent_id,
            derived_address: <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_derived_address">memory::sub_agent_derived_address</a>(agent),
            enabled: <b>true</b>,
            budget_mist,
            spent_mist: 0,
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a>: 0,
            daily_cap_mist,
            monthly_cap_mist,
            spent_day_mist: 0,
            reserved_day_mist: 0,
            spent_month_mist: 0,
            reserved_month_mist: 0,
            day_anchor_ms: now,
            month_anchor_ms: now,
            require_approval_above_mist,
        };
        table::add(&<b>mut</b> balance.agent_budgets, agent_id, e);
        e
    };
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetUpdated">AiCreditAgentBudgetUpdated</a> {
        balance_id: object::id(balance),
        agent_object_id: <b>entry</b>.agent_object_id,
        budget_mist: <b>entry</b>.budget_mist,
        daily_cap_mist: <b>entry</b>.daily_cap_mist,
        monthly_cap_mist: <b>entry</b>.monthly_cap_mist,
        require_approval_above_mist: <b>entry</b>.require_approval_above_mist,
    });
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetChanged">AiCreditAgentBudgetChanged</a> {
        balance_id: object::id(balance),
        agent_object_id: agent_id,
        had_previous_entry,
        prev_budget_mist: prev_budget,
        prev_daily_cap_mist: prev_daily,
        prev_monthly_cap_mist: prev_monthly,
        prev_require_approval_above_mist: prev_approval,
        prev_enabled,
        budget_mist,
        daily_cap_mist,
        monthly_cap_mist,
        require_approval_above_mist,
        enabled: <b>true</b>,
        set_by,
        set_by_agent_id,
        organization_id,
        timestamp_ms: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_disable_agent_budget_internal"></a>

## Function `disable_agent_budget_internal`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_agent_budget_internal">disable_agent_budget_internal</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, set_by: <b>address</b>, set_by_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_agent_budget_internal">disable_agent_budget_internal</a>(
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent_object_id: ID,
    set_by: <b>address</b>,
    set_by_agent_id: Option&lt;ID&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(table::contains(&balance.agent_budgets, agent_object_id), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentNotFound">EAgentNotFound</a>);
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> balance.agent_budgets, agent_object_id);
    <b>let</b> prev_enabled = <b>entry</b>.enabled;
    <b>entry</b>.enabled = <b>false</b>;
    <b>let</b> snapshot = *<b>entry</b>;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetDisabled">AiCreditAgentBudgetDisabled</a> {
        balance_id: object::id(balance),
        agent_object_id,
    });
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetChanged">AiCreditAgentBudgetChanged</a> {
        balance_id: object::id(balance),
        agent_object_id,
        had_previous_entry: <b>true</b>,
        prev_budget_mist: snapshot.budget_mist,
        prev_daily_cap_mist: snapshot.daily_cap_mist,
        prev_monthly_cap_mist: snapshot.monthly_cap_mist,
        prev_require_approval_above_mist: snapshot.require_approval_above_mist,
        prev_enabled,
        budget_mist: snapshot.budget_mist,
        daily_cap_mist: snapshot.daily_cap_mist,
        monthly_cap_mist: snapshot.monthly_cap_mist,
        require_approval_above_mist: snapshot.require_approval_above_mist,
        enabled: <b>false</b>,
        set_by,
        set_by_agent_id,
        organization_id: option::none(),
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_store_spend_approval"></a>

## Function `store_spend_approval`

Store (or overwrite) the agent's one-shot allowance and emit the approval event.


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_store_spend_approval">store_spend_approval</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, max_amount_mist: u64, expires_at_ms: u64, approved_by: <b>address</b>, approved_by_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_store_spend_approval">store_spend_approval</a>(
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent_object_id: ID,
    max_amount_mist: u64,
    expires_at_ms: u64,
    approved_by: <b>address</b>,
    approved_by_agent_id: Option&lt;ID&gt;,
    organization_id: Option&lt;ID&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(max_amount_mist &gt; 0, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(expires_at_ms &gt; now, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidExpiry">EInvalidExpiry</a>);
    <b>let</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a> = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_next_approval_nonce">next_approval_nonce</a>(balance);
    <b>let</b> key = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a> { agent_object_id };
    <b>if</b> (df::exists_with_type&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>&gt;(&balance.id, key)) {
        <b>let</b> _old: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a> = df::remove(&<b>mut</b> balance.id, key);
    };
    df::add(&<b>mut</b> balance.id, key, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a> {
        max_amount_mist,
        expires_at_ms,
        approved_by,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>,
    });
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditSpendApproved">AiCreditSpendApproved</a> {
        balance_id: object::id(balance),
        agent_object_id,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>,
        max_amount_mist,
        expires_at_ms,
        approved_by,
        approved_by_agent_id,
        organization_id,
        timestamp_ms: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_next_approval_nonce"></a>

## Function `next_approval_nonce`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_next_approval_nonce">next_approval_nonce</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_next_approval_nonce">next_approval_nonce</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>): u64 {
    <b>if</b> (!df::exists_with_type&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ApprovalNonceKey">ApprovalNonceKey</a>, u64&gt;(&balance.id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ApprovalNonceKey">ApprovalNonceKey</a> {})) {
        df::add(&<b>mut</b> balance.id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ApprovalNonceKey">ApprovalNonceKey</a> {}, 0u64);
    };
    <b>let</b> counter = df::borrow_mut&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ApprovalNonceKey">ApprovalNonceKey</a>, u64&gt;(&<b>mut</b> balance.id, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ApprovalNonceKey">ApprovalNonceKey</a> {});
    *counter = *counter + 1;
    *counter
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_maybe_consume_spend_approval"></a>

## Function `maybe_consume_spend_approval`

Consume the agent's allowance when the settlement amount exceeds its approval
threshold. Aborts when no live, sufficient allowance exists — this is the on-chain
enforcement of <code>require_approval_above_mist</code>.


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_maybe_consume_spend_approval">maybe_consume_spend_approval</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, amount_mist: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_maybe_consume_spend_approval">maybe_consume_spend_approval</a>(
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent_object_id: ID,
    amount_mist: u64,
    clock: &Clock,
) {
    <b>if</b> (!table::contains(&balance.agent_budgets, agent_object_id)) {
        <b>return</b>
    };
    <b>let</b> threshold_opt = {
        <b>let</b> <b>entry</b> = table::borrow(&balance.agent_budgets, agent_object_id);
        <b>entry</b>.require_approval_above_mist
    };
    <b>if</b> (option::is_none(&threshold_opt)) {
        <b>return</b>
    };
    <b>if</b> (amount_mist &lt;= *option::borrow(&threshold_opt)) {
        <b>return</b>
    };
    <b>let</b> key = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a> { agent_object_id };
    <b>assert</b>!(
        df::exists_with_type&lt;<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApprovalKey">SpendApprovalKey</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a>&gt;(&balance.id, key),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EApprovalRequired">EApprovalRequired</a>,
    );
    <b>let</b> approval: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendApproval">SpendApproval</a> = df::remove(&<b>mut</b> balance.id, key);
    <b>assert</b>!(clock::timestamp_ms(clock) &lt;= approval.expires_at_ms, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EApprovalExpired">EApprovalExpired</a>);
    <b>assert</b>!(amount_mist &lt;= approval.max_amount_mist, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EApprovalInsufficient">EApprovalInsufficient</a>);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditSpendApprovalConsumed">AiCreditSpendApprovalConsumed</a> {
        balance_id: object::id(balance),
        agent_object_id,
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>: approval.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approval_nonce">approval_nonce</a>,
        amount_mist,
        approved_by: approval.approved_by,
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_oracle_admin"></a>

## Function `assert_oracle_admin`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_oracle_admin">assert_oracle_admin</a>(_cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">social_contracts::ai_credit::AiCreditOracleAdminCap</a>, _ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_oracle_admin">assert_oracle_admin</a>(_cap: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a>, _ctx: &TxContext) {
    // Holding `<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditOracleAdminCap">AiCreditOracleAdminCap</a>` in the PTB proves admin authority.
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_verify_receipt_signature"></a>

## Function `verify_receipt_signature`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_receipt_signature">verify_receipt_signature</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, receipt: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">social_contracts::ai_credit::UsageReceipt</a>, signature: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_receipt_signature">verify_receipt_signature</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>, receipt: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a>, signature: &vector&lt;u8&gt;) {
    <b>assert</b>!(vector::length(signature) == 64, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidSignature">EInvalidSignature</a>);
    <b>let</b> intent_message = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_IntentMessage">IntentMessage</a> {
        intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_INTENT_AI_CREDIT_USAGE">INTENT_AI_CREDIT_USAGE</a>,
        timestamp_ms: receipt.timestamp_ms,
        payload: *receipt,
    };
    <b>let</b> msg = bcs::to_bytes(&intent_message);
    <b>assert</b>!(
        ed25519::ed25519_verify(signature, &config.oracle_pubkey, &msg),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidSignature">EInvalidSignature</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_verify_reservation_signature"></a>

## Function `verify_reservation_signature`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_reservation_signature">verify_reservation_signature</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, intent: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservationIntent">social_contracts::ai_credit::SpendReservationIntent</a>, signature: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_reservation_signature">verify_reservation_signature</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    intent: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservationIntent">SpendReservationIntent</a>,
    signature: &vector&lt;u8&gt;,
) {
    <b>assert</b>!(vector::length(signature) == 64, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidSignature">EInvalidSignature</a>);
    <b>let</b> intent_message = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_IntentMessage">IntentMessage</a> {
        intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_INTENT_AI_CREDIT_RESERVE">INTENT_AI_CREDIT_RESERVE</a>,
        timestamp_ms: intent.timestamp_ms,
        payload: *intent,
    };
    <b>let</b> msg = bcs::to_bytes(&intent_message);
    <b>assert</b>!(
        ed25519::ed25519_verify(signature, &config.oracle_pubkey, &msg),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidSignature">EInvalidSignature</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_verify_capture_signature"></a>

## Function `verify_capture_signature`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_capture_signature">verify_capture_signature</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, intent: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CaptureSpendIntent">social_contracts::ai_credit::CaptureSpendIntent</a>, signature: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_capture_signature">verify_capture_signature</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    intent: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CaptureSpendIntent">CaptureSpendIntent</a>,
    signature: &vector&lt;u8&gt;,
) {
    <b>assert</b>!(vector::length(signature) == 64, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidSignature">EInvalidSignature</a>);
    <b>let</b> intent_message = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_IntentMessage">IntentMessage</a> {
        intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_INTENT_AI_CREDIT_CAPTURE">INTENT_AI_CREDIT_CAPTURE</a>,
        timestamp_ms: intent.timestamp_ms,
        payload: *intent,
    };
    <b>let</b> msg = bcs::to_bytes(&intent_message);
    <b>assert</b>!(
        ed25519::ed25519_verify(signature, &config.oracle_pubkey, &msg),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidSignature">EInvalidSignature</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_verify_cancel_signature"></a>

## Function `verify_cancel_signature`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_cancel_signature">verify_cancel_signature</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, intent: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CancelSpendIntent">social_contracts::ai_credit::CancelSpendIntent</a>, signature: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_verify_cancel_signature">verify_cancel_signature</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    intent: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_CancelSpendIntent">CancelSpendIntent</a>,
    signature: &vector&lt;u8&gt;,
) {
    <b>assert</b>!(vector::length(signature) == 64, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidSignature">EInvalidSignature</a>);
    <b>let</b> intent_message = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_IntentMessage">IntentMessage</a> {
        intent: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_INTENT_AI_CREDIT_CANCEL">INTENT_AI_CREDIT_CANCEL</a>,
        timestamp_ms: intent.timestamp_ms,
        payload: *intent,
    };
    <b>let</b> msg = bcs::to_bytes(&intent_message);
    <b>assert</b>!(
        ed25519::ed25519_verify(signature, &config.oracle_pubkey, &msg),
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidSignature">EInvalidSignature</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_receipt_fresh"></a>

## Function `assert_receipt_fresh`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_receipt_fresh">assert_receipt_fresh</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, receipt: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">social_contracts::ai_credit::UsageReceipt</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_receipt_fresh">assert_receipt_fresh</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>, receipt: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a>, clock: &Clock) {
    <b>let</b> now = clock::timestamp_ms(clock);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_signed_timestamp_fresh">assert_signed_timestamp_fresh</a>(config, receipt.timestamp_ms, now);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_signed_timestamp_fresh"></a>

## Function `assert_signed_timestamp_fresh`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_signed_timestamp_fresh">assert_signed_timestamp_fresh</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, timestamp_ms: u64, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_signed_timestamp_fresh">assert_signed_timestamp_fresh</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>, timestamp_ms: u64, now: u64) {
    <b>assert</b>!(timestamp_ms &lt;= now, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EStaleReceipt">EStaleReceipt</a>);
    <b>assert</b>!(now - timestamp_ms &lt;= config.receipt_ttl_ms, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EStaleReceipt">EStaleReceipt</a>);
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_roll_account_windows"></a>

## Function `roll_account_windows`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_account_windows">roll_account_windows</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_account_windows">roll_account_windows</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>, now: u64) {
    <b>if</b> (now &gt;= balance.day_anchor_ms + <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_DAY_MS">DAY_MS</a>) {
        balance.spent_day_mist = 0;
        balance.reserved_day_mist = 0;
        balance.day_anchor_ms = now;
    };
    <b>if</b> (now &gt;= balance.month_anchor_ms + <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MONTH_MS">MONTH_MS</a>) {
        balance.spent_month_mist = 0;
        balance.reserved_month_mist = 0;
        balance.month_anchor_ms = now;
    };
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_roll_agent_windows"></a>

## Function `roll_agent_windows`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_agent_windows">roll_agent_windows</a>(<b>entry</b>: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AgentBudgetEntry">social_contracts::ai_credit::AgentBudgetEntry</a>, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_agent_windows">roll_agent_windows</a>(<b>entry</b>: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AgentBudgetEntry">AgentBudgetEntry</a>, now: u64) {
    <b>if</b> (now &gt;= <b>entry</b>.day_anchor_ms + <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_DAY_MS">DAY_MS</a>) {
        <b>entry</b>.spent_day_mist = 0;
        <b>entry</b>.reserved_day_mist = 0;
        <b>entry</b>.day_anchor_ms = now;
    };
    <b>if</b> (now &gt;= <b>entry</b>.month_anchor_ms + <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MONTH_MS">MONTH_MS</a>) {
        <b>entry</b>.spent_month_mist = 0;
        <b>entry</b>.reserved_month_mist = 0;
        <b>entry</b>.month_anchor_ms = now;
    };
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_account_caps"></a>

## Function `assert_account_caps`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_account_caps">assert_account_caps</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, amount_mist: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_account_caps">assert_account_caps</a>(balance: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>, amount_mist: u64) {
    <b>if</b> (option::is_some(&balance.daily_cap_mist)) {
        <b>let</b> cap = *option::borrow(&balance.daily_cap_mist);
        <b>assert</b>!(
            balance.spent_day_mist + balance.reserved_day_mist + amount_mist &lt;= cap,
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>,
        );
    };
    <b>if</b> (option::is_some(&balance.monthly_cap_mist)) {
        <b>let</b> cap = *option::borrow(&balance.monthly_cap_mist);
        <b>assert</b>!(
            balance.spent_month_mist + balance.reserved_month_mist + amount_mist &lt;= cap,
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_assert_agent_caps"></a>

## Function `assert_agent_caps`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_caps">assert_agent_caps</a>(<b>entry</b>: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AgentBudgetEntry">social_contracts::ai_credit::AgentBudgetEntry</a>, amount_mist: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_agent_caps">assert_agent_caps</a>(<b>entry</b>: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AgentBudgetEntry">AgentBudgetEntry</a>, amount_mist: u64) {
    <b>if</b> (option::is_some(&<b>entry</b>.budget_mist)) {
        <b>let</b> cap = *option::borrow(&<b>entry</b>.budget_mist);
        <b>assert</b>!(<b>entry</b>.spent_mist + <b>entry</b>.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> + amount_mist &lt;= cap, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>);
    };
    <b>if</b> (option::is_some(&<b>entry</b>.daily_cap_mist)) {
        <b>let</b> cap = *option::borrow(&<b>entry</b>.daily_cap_mist);
        <b>assert</b>!(
            <b>entry</b>.spent_day_mist + <b>entry</b>.reserved_day_mist + amount_mist &lt;= cap,
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>,
        );
    };
    <b>if</b> (option::is_some(&<b>entry</b>.monthly_cap_mist)) {
        <b>let</b> cap = *option::borrow(&<b>entry</b>.monthly_cap_mist);
        <b>assert</b>!(
            <b>entry</b>.spent_month_mist + <b>entry</b>.reserved_month_mist + amount_mist &lt;= cap,
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_release_reservation_counters"></a>

## Function `release_reservation_counters`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_release_reservation_counters">release_reservation_counters</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">social_contracts::ai_credit::SpendReservation</a>, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_release_reservation_counters">release_reservation_counters</a>(
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">SpendReservation</a>,
    now: u64,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_account_windows">roll_account_windows</a>(balance, now);
    balance.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> = balance.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> - reservation.max_amount_mist;
    <b>if</b> (balance.day_anchor_ms == reservation.account_day_anchor_ms) {
        balance.reserved_day_mist =
            balance.reserved_day_mist - reservation.max_amount_mist;
    };
    <b>if</b> (balance.month_anchor_ms == reservation.account_month_anchor_ms) {
        balance.reserved_month_mist =
            balance.reserved_month_mist - reservation.max_amount_mist;
    };
    <b>if</b> (reservation.agent_budget_reserved) {
        <b>assert</b>!(
            table::contains(&balance.agent_budgets, reservation.agent_object_id),
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationMismatch">EReservationMismatch</a>,
        );
        <b>let</b> <b>entry</b> = table::borrow_mut(
            &<b>mut</b> balance.agent_budgets,
            reservation.agent_object_id,
        );
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_agent_windows">roll_agent_windows</a>(<b>entry</b>, now);
        <b>entry</b>.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> = <b>entry</b>.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> - reservation.max_amount_mist;
        <b>if</b> (<b>entry</b>.day_anchor_ms == reservation.agent_day_anchor_ms) {
            <b>entry</b>.reserved_day_mist =
                <b>entry</b>.reserved_day_mist - reservation.max_amount_mist;
        };
        <b>if</b> (<b>entry</b>.month_anchor_ms == reservation.agent_month_anchor_ms) {
            <b>entry</b>.reserved_month_mist =
                <b>entry</b>.reserved_month_mist - reservation.max_amount_mist;
        };
    };
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_capture_reservation_counters"></a>

## Function `capture_reservation_counters`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_capture_reservation_counters">capture_reservation_counters</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">social_contracts::ai_credit::SpendReservation</a>, amount_mist: u64, now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_capture_reservation_counters">capture_reservation_counters</a>(
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    reservation: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_SpendReservation">SpendReservation</a>,
    amount_mist: u64,
    now: u64,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_account_windows">roll_account_windows</a>(balance, now);
    balance.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> = balance.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> - reservation.max_amount_mist;
    balance.spent_total_mist = balance.spent_total_mist + amount_mist;
    <b>if</b> (balance.day_anchor_ms == reservation.account_day_anchor_ms) {
        balance.reserved_day_mist =
            balance.reserved_day_mist - reservation.max_amount_mist;
        balance.spent_day_mist = balance.spent_day_mist + amount_mist;
    };
    <b>if</b> (balance.month_anchor_ms == reservation.account_month_anchor_ms) {
        balance.reserved_month_mist =
            balance.reserved_month_mist - reservation.max_amount_mist;
        balance.spent_month_mist = balance.spent_month_mist + amount_mist;
    };
    <b>if</b> (reservation.agent_budget_reserved) {
        <b>assert</b>!(
            table::contains(&balance.agent_budgets, reservation.agent_object_id),
            <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EReservationMismatch">EReservationMismatch</a>,
        );
        <b>let</b> <b>entry</b> = table::borrow_mut(
            &<b>mut</b> balance.agent_budgets,
            reservation.agent_object_id,
        );
        <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_roll_agent_windows">roll_agent_windows</a>(<b>entry</b>, now);
        <b>entry</b>.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> = <b>entry</b>.<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_reserved_mist">reserved_mist</a> - reservation.max_amount_mist;
        <b>entry</b>.spent_mist = <b>entry</b>.spent_mist + amount_mist;
        <b>if</b> (<b>entry</b>.day_anchor_ms == reservation.agent_day_anchor_ms) {
            <b>entry</b>.reserved_day_mist =
                <b>entry</b>.reserved_day_mist - reservation.max_amount_mist;
            <b>entry</b>.spent_day_mist = <b>entry</b>.spent_day_mist + amount_mist;
        };
        <b>if</b> (<b>entry</b>.month_anchor_ms == reservation.agent_month_anchor_ms) {
            <b>entry</b>.reserved_month_mist =
                <b>entry</b>.reserved_month_mist - reservation.max_amount_mist;
            <b>entry</b>.spent_month_mist = <b>entry</b>.spent_month_mist + amount_mist;
        };
    };
}
</code></pre>



</details>
