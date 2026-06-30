---
title: Module `social_contracts::ai_credit`
---

AI credit escrow — users deposit MYSO; platform oracle settles signed usage receipts
for sub-agent inference, tools, storage, and workflow costs.

Credits display: <code>credits = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a> / <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MIST_PER_MYSO">MIST_PER_MYSO</a></code> (1 MYSO = 1 credit).


-  [Struct `AiCreditOracleAdminCap`](#social_contracts_ai_credit_AiCreditOracleAdminCap)
-  [Struct `AiCreditConfig`](#social_contracts_ai_credit_AiCreditConfig)
-  [Struct `AgentBudgetEntry`](#social_contracts_ai_credit_AgentBudgetEntry)
-  [Struct `AiCreditBalance`](#social_contracts_ai_credit_AiCreditBalance)
-  [Struct `IntentMessage`](#social_contracts_ai_credit_IntentMessage)
-  [Struct `UsageReceipt`](#social_contracts_ai_credit_UsageReceipt)
-  [Struct `AiCreditBalanceCreated`](#social_contracts_ai_credit_AiCreditBalanceCreated)
-  [Struct `AiCreditDeposited`](#social_contracts_ai_credit_AiCreditDeposited)
-  [Struct `AiCreditWithdrawn`](#social_contracts_ai_credit_AiCreditWithdrawn)
-  [Struct `AiCreditAccountCapsUpdated`](#social_contracts_ai_credit_AiCreditAccountCapsUpdated)
-  [Struct `AiCreditAgentBudgetUpdated`](#social_contracts_ai_credit_AiCreditAgentBudgetUpdated)
-  [Struct `AiCreditAgentBudgetDisabled`](#social_contracts_ai_credit_AiCreditAgentBudgetDisabled)
-  [Struct `AiCreditUsageSettled`](#social_contracts_ai_credit_AiCreditUsageSettled)
-  [Struct `AiCreditBalanceDepleted`](#social_contracts_ai_credit_AiCreditBalanceDepleted)
-  [Struct `AiCreditBalancePaused`](#social_contracts_ai_credit_AiCreditBalancePaused)
-  [Struct `AiCreditBalanceReactivated`](#social_contracts_ai_credit_AiCreditBalanceReactivated)
-  [Struct `AiCreditOraclePubkeyUpdated`](#social_contracts_ai_credit_AiCreditOraclePubkeyUpdated)
-  [Struct `AiCreditSettlementLimitsUpdated`](#social_contracts_ai_credit_AiCreditSettlementLimitsUpdated)
-  [Struct `AiCreditConfigInitialized`](#social_contracts_ai_credit_AiCreditConfigInitialized)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_ai_credit_bootstrap_init)
-  [Function `create_oracle_admin_cap`](#social_contracts_ai_credit_create_oracle_admin_cap)
-  [Function `create_and_share_balance`](#social_contracts_ai_credit_create_and_share_balance)
-  [Function `transfer_balance_owner`](#social_contracts_ai_credit_transfer_balance_owner)
-  [Function `new_balance`](#social_contracts_ai_credit_new_balance)
-  [Function `deposit`](#social_contracts_ai_credit_deposit)
-  [Function `withdraw`](#social_contracts_ai_credit_withdraw)
-  [Function `set_account_caps`](#social_contracts_ai_credit_set_account_caps)
-  [Function `set_agent_budget`](#social_contracts_ai_credit_set_agent_budget)
-  [Function `disable_agent_budget`](#social_contracts_ai_credit_disable_agent_budget)
-  [Function `pause_balance`](#social_contracts_ai_credit_pause_balance)
-  [Function `reactivate_balance`](#social_contracts_ai_credit_reactivate_balance)
-  [Function `settle_usage`](#social_contracts_ai_credit_settle_usage)
-  [Function `settle_usage_batch`](#social_contracts_ai_credit_settle_usage_batch)
-  [Function `settle_signed_usage`](#social_contracts_ai_credit_settle_signed_usage)
-  [Function `make_usage_receipt_from_objects`](#social_contracts_ai_credit_make_usage_receipt_from_objects)
-  [Function `settle_usage_with_signature`](#social_contracts_ai_credit_settle_usage_with_signature)
-  [Function `execute_settlement`](#social_contracts_ai_credit_execute_settlement)
-  [Function `update_oracle_pubkey`](#social_contracts_ai_credit_update_oracle_pubkey)
-  [Function `update_settlement_limits`](#social_contracts_ai_credit_update_settlement_limits)
-  [Function `update_treasury`](#social_contracts_ai_credit_update_treasury)
-  [Function `balance_mist`](#social_contracts_ai_credit_balance_mist)
-  [Function `available_mist`](#social_contracts_ai_credit_available_mist)
-  [Function `credits_from_mist`](#social_contracts_ai_credit_credits_from_mist)
-  [Function `mist_from_credits`](#social_contracts_ai_credit_mist_from_credits)
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
-  [Function `assert_oracle_admin`](#social_contracts_ai_credit_assert_oracle_admin)
-  [Function `verify_receipt_signature`](#social_contracts_ai_credit_verify_receipt_signature)
-  [Function `assert_receipt_fresh`](#social_contracts_ai_credit_assert_receipt_fresh)
-  [Function `roll_account_windows`](#social_contracts_ai_credit_roll_account_windows)
-  [Function `roll_agent_windows`](#social_contracts_ai_credit_roll_agent_windows)
-  [Function `assert_account_caps`](#social_contracts_ai_credit_assert_account_caps)
-  [Function `assert_agent_caps`](#social_contracts_ai_credit_assert_agent_caps)


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
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
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
<code>min_deposit_mist: u64</code>
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
<code>spent_month_mist: u64</code>
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
<code>spent_day_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>spent_month_mist: u64</code>
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
<dt>
<code>credits: u64</code>
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
<dt>
<code>credits_remaining: u64</code>
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
<code>min_deposit_mist: u64</code>
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



<a name="social_contracts_ai_credit_ED25519_PK_LEN"></a>



<pre><code><b>const</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ED25519_PK_LEN">ED25519_PK_LEN</a>: u64 = 32;
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



<a name="social_contracts_ai_credit_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_bootstrap_init">bootstrap_init</a>(treasury: <b>address</b>, oracle_pubkey: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_bootstrap_init">bootstrap_init</a>(treasury: <b>address</b>, oracle_pubkey: vector&lt;u8&gt;, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(vector::length(&oracle_pubkey) == <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ED25519_PK_LEN">ED25519_PK_LEN</a>, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidPubkey">EInvalidPubkey</a>);
    <b>let</b> min_deposit_mist = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MIST_PER_MYSO">MIST_PER_MYSO</a>;
    <b>let</b> max_single_settlement_mist = 1000 * <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MIST_PER_MYSO">MIST_PER_MYSO</a>;
    <b>let</b> receipt_ttl_ms = 300_000;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfigInitialized">AiCreditConfigInitialized</a> {
        oracle_pubkey,
        treasury,
        min_deposit_mist,
        max_single_settlement_mist,
        receipt_ttl_ms,
    });
    <b>let</b> config = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a> {
        id: object::new(ctx),
        oracle_pubkey,
        treasury,
        min_deposit_mist,
        max_single_settlement_mist,
        receipt_ttl_ms,
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

<a name="social_contracts_ai_credit_transfer_balance_owner"></a>

## Function `transfer_balance_owner`

Called only from [<code><a href="../social_contracts/profile.md#social_contracts_profile_transfer_profile_with_memory">profile::transfer_profile_with_memory</a></code>].


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_transfer_balance_owner">transfer_balance_owner</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, new_owner: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_transfer_balance_owner">transfer_balance_owner</a>(balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>, new_owner: <b>address</b>) {
    balance.principal_owner = new_owner;
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
        spent_day_mist: 0,
        spent_month_mist: 0,
        day_anchor_ms: now,
        month_anchor_ms: now,
        daily_cap_mist: option::none(),
        monthly_cap_mist: option::none(),
        settlement_nonce: 0,
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
    <b>assert</b>!(amount &gt;= config.min_deposit_mist, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EInvalidAmount">EInvalidAmount</a>);
    balance::join(&<b>mut</b> balance.balance, coin::into_balance(payment));
    <b>let</b> new_balance_mist = <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a>(balance);
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditDeposited">AiCreditDeposited</a> {
        balance_id: object::id(balance),
        amount_mist: amount,
        new_balance_mist,
        credits: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_credits_from_mist">credits_from_mist</a>(new_balance_mist),
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
    <b>let</b> agent_id = <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">memory::agent_object_id</a>(agent);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> <b>entry</b> = <b>if</b> (table::contains(&balance.agent_budgets, agent_id)) {
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
            daily_cap_mist,
            monthly_cap_mist,
            spent_day_mist: 0,
            spent_month_mist: 0,
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
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_disable_agent_budget"></a>

## Function `disable_agent_budget`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_agent_budget">disable_agent_budget</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">social_contracts::ai_credit::AiCreditBalance</a>, agent_object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_disable_agent_budget">disable_agent_budget</a>(
    config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>,
    balance: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalance">AiCreditBalance</a>,
    agent_object_id: ID,
    ctx: &TxContext,
) {
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_version">assert_version</a>(config, balance);
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_owner">assert_owner</a>(balance, ctx);
    <b>assert</b>!(table::contains(&balance.agent_budgets, agent_object_id), <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EAgentNotFound">EAgentNotFound</a>);
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> balance.agent_budgets, agent_object_id);
    <b>entry</b>.enabled = <b>false</b>;
    event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditAgentBudgetDisabled">AiCreditAgentBudgetDisabled</a> {
        balance_id: object::id(balance),
        agent_object_id,
    });
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
        credits_remaining: <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_credits_from_mist">credits_from_mist</a>(remaining),
    });
    <b>if</b> (remaining == 0) {
        event::emit(<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditBalanceDepleted">AiCreditBalanceDepleted</a> {
            balance_id: object::id(balance),
        });
    };
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
    <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_balance_mist">balance_mist</a>(balance)
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_credits_from_mist"></a>

## Function `credits_from_mist`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_credits_from_mist">credits_from_mist</a>(mist: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_credits_from_mist">credits_from_mist</a>(mist: u64): u64 {
    mist / <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MIST_PER_MYSO">MIST_PER_MYSO</a>
}
</code></pre>



</details>

<a name="social_contracts_ai_credit_mist_from_credits"></a>

## Function `mist_from_credits`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_mist_from_credits">mist_from_credits</a>(credits: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_mist_from_credits">mist_from_credits</a>(credits: u64): u64 {
    credits * <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MIST_PER_MYSO">MIST_PER_MYSO</a>
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
    <b>if</b> (<b>entry</b>.spent_mist &gt;= max) {
        option::some(0)
    } <b>else</b> {
        option::some(max - <b>entry</b>.spent_mist)
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

<a name="social_contracts_ai_credit_assert_receipt_fresh"></a>

## Function `assert_receipt_fresh`



<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_receipt_fresh">assert_receipt_fresh</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, receipt: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">social_contracts::ai_credit::UsageReceipt</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_assert_receipt_fresh">assert_receipt_fresh</a>(config: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">AiCreditConfig</a>, receipt: &<a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_UsageReceipt">UsageReceipt</a>, clock: &Clock) {
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(receipt.timestamp_ms &lt;= now, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EStaleReceipt">EStaleReceipt</a>);
    <b>assert</b>!(now - receipt.timestamp_ms &lt;= config.receipt_ttl_ms, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_EStaleReceipt">EStaleReceipt</a>);
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
        balance.day_anchor_ms = now;
    };
    <b>if</b> (now &gt;= balance.month_anchor_ms + <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MONTH_MS">MONTH_MS</a>) {
        balance.spent_month_mist = 0;
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
        <b>entry</b>.day_anchor_ms = now;
    };
    <b>if</b> (now &gt;= <b>entry</b>.month_anchor_ms + <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_MONTH_MS">MONTH_MS</a>) {
        <b>entry</b>.spent_month_mist = 0;
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
        <b>assert</b>!(balance.spent_day_mist + amount_mist &lt;= cap, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>);
    };
    <b>if</b> (option::is_some(&balance.monthly_cap_mist)) {
        <b>let</b> cap = *option::borrow(&balance.monthly_cap_mist);
        <b>assert</b>!(balance.spent_month_mist + amount_mist &lt;= cap, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>);
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
        <b>assert</b>!(<b>entry</b>.spent_mist + amount_mist &lt;= cap, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>);
    };
    <b>if</b> (option::is_some(&<b>entry</b>.daily_cap_mist)) {
        <b>let</b> cap = *option::borrow(&<b>entry</b>.daily_cap_mist);
        <b>assert</b>!(<b>entry</b>.spent_day_mist + amount_mist &lt;= cap, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>);
    };
    <b>if</b> (option::is_some(&<b>entry</b>.monthly_cap_mist)) {
        <b>let</b> cap = *option::borrow(&<b>entry</b>.monthly_cap_mist);
        <b>assert</b>!(<b>entry</b>.spent_month_mist + amount_mist &lt;= cap, <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_ECapExceeded">ECapExceeded</a>);
    };
}
</code></pre>



</details>
