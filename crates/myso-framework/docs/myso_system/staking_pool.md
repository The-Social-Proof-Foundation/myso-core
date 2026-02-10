---
title: Module `myso_system::staking_pool`
---



-  [Struct `StakingPool`](#myso_system_staking_pool_StakingPool)
-  [Struct `PoolTokenExchangeRate`](#myso_system_staking_pool_PoolTokenExchangeRate)
-  [Struct `StakedMySo`](#myso_system_staking_pool_StakedMySo)
-  [Struct `FungibleStakedMySo`](#myso_system_staking_pool_FungibleStakedMySo)
-  [Struct `FungibleStakedMySoData`](#myso_system_staking_pool_FungibleStakedMySoData)
-  [Struct `FungibleStakedMySoDataKey`](#myso_system_staking_pool_FungibleStakedMySoDataKey)
-  [Struct `UnderflowMySoBalance`](#myso_system_staking_pool_UnderflowMySoBalance)
-  [Constants](#@Constants_0)
-  [Function `new`](#myso_system_staking_pool_new)
-  [Function `request_add_stake`](#myso_system_staking_pool_request_add_stake)
-  [Function `request_withdraw_stake`](#myso_system_staking_pool_request_withdraw_stake)
-  [Function `redeem_fungible_staked_myso`](#myso_system_staking_pool_redeem_fungible_staked_myso)
-  [Function `calculate_fungible_staked_myso_withdraw_amount`](#myso_system_staking_pool_calculate_fungible_staked_myso_withdraw_amount)
-  [Function `convert_to_fungible_staked_myso`](#myso_system_staking_pool_convert_to_fungible_staked_myso)
-  [Function `withdraw_from_principal`](#myso_system_staking_pool_withdraw_from_principal)
-  [Function `unwrap_staked_myso`](#myso_system_staking_pool_unwrap_staked_myso)
-  [Function `deposit_rewards`](#myso_system_staking_pool_deposit_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#myso_system_staking_pool_process_pending_stakes_and_withdraws)
-  [Function `process_pending_stake_withdraw`](#myso_system_staking_pool_process_pending_stake_withdraw)
-  [Function `process_pending_stake`](#myso_system_staking_pool_process_pending_stake)
-  [Function `withdraw_rewards`](#myso_system_staking_pool_withdraw_rewards)
-  [Function `activate_staking_pool`](#myso_system_staking_pool_activate_staking_pool)
-  [Function `deactivate_staking_pool`](#myso_system_staking_pool_deactivate_staking_pool)
-  [Function `myso_balance`](#myso_system_staking_pool_myso_balance)
-  [Function `pool_id`](#myso_system_staking_pool_pool_id)
-  [Function `fungible_staked_myso_pool_id`](#myso_system_staking_pool_fungible_staked_myso_pool_id)
-  [Function `staked_myso_amount`](#myso_system_staking_pool_staked_myso_amount)
-  [Function `stake_activation_epoch`](#myso_system_staking_pool_stake_activation_epoch)
-  [Function `is_preactive`](#myso_system_staking_pool_is_preactive)
-  [Function `activation_epoch`](#myso_system_staking_pool_activation_epoch)
-  [Function `is_inactive`](#myso_system_staking_pool_is_inactive)
-  [Function `fungible_staked_myso_value`](#myso_system_staking_pool_fungible_staked_myso_value)
-  [Function `split_fungible_staked_myso`](#myso_system_staking_pool_split_fungible_staked_myso)
-  [Function `join_fungible_staked_myso`](#myso_system_staking_pool_join_fungible_staked_myso)
-  [Function `split`](#myso_system_staking_pool_split)
-  [Function `split_staked_myso`](#myso_system_staking_pool_split_staked_myso)
-  [Function `join_staked_myso`](#myso_system_staking_pool_join_staked_myso)
-  [Function `is_equal_staking_metadata`](#myso_system_staking_pool_is_equal_staking_metadata)
-  [Function `pool_token_exchange_rate_at_epoch`](#myso_system_staking_pool_pool_token_exchange_rate_at_epoch)
-  [Function `pending_stake_amount`](#myso_system_staking_pool_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#myso_system_staking_pool_pending_stake_withdraw_amount)
-  [Function `exchange_rates`](#myso_system_staking_pool_exchange_rates)
-  [Function `myso_amount`](#myso_system_staking_pool_myso_amount)
-  [Function `pool_token_amount`](#myso_system_staking_pool_pool_token_amount)
-  [Function `is_preactive_at_epoch`](#myso_system_staking_pool_is_preactive_at_epoch)
-  [Function `get_myso_amount`](#myso_system_staking_pool_get_myso_amount)
-  [Function `get_token_amount`](#myso_system_staking_pool_get_token_amount)
-  [Function `initial_exchange_rate`](#myso_system_staking_pool_initial_exchange_rate)
-  [Function `check_balance_invariants`](#myso_system_staking_pool_check_balance_invariants)
-  [Macro function `mul_div`](#myso_system_staking_pool_mul_div)
-  [Function `calculate_rewards`](#myso_system_staking_pool_calculate_rewards)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/myso.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="myso_system_staking_pool_StakingPool"></a>

## Struct `StakingPool`

A staking pool embedded in each validator struct in the system state object.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a> <b>has</b> key, store
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
<code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this pool became active.
 The value is <code>None</code> if the pool is pre-active and <code>Some(&lt;epoch_number&gt;)</code> if active or inactive.
</dd>
<dt>
<code>deactivation_epoch: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this staking pool ceased to be active. <code>None</code> = {pre-active, active},
 <code>Some(&lt;epoch_number&gt;)</code> if in-active, and it was de-activated at epoch <code>&lt;epoch_number&gt;</code>.
</dd>
<dt>
<code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>: u64</code>
</dt>
<dd>
 The total number of MYSO tokens in this pool, including the MYSO in the rewards_pool, as well as in all the principal
 in the <code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a></code> object, updated at epoch boundaries.
</dd>
<dt>
<code>rewards_pool: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 The epoch stake rewards will be added here at the end of each epoch.
</dd>
<dt>
<code>pool_token_balance: u64</code>
</dt>
<dd>
 Total number of pool tokens issued by the pool.
</dd>
<dt>
<code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_exchange_rates">exchange_rates</a>: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u64, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>&gt;</code>
</dt>
<dd>
 Exchange rate history of previous epochs. Key is the epoch number.
 The entries start from the <code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a></code> of this pool and contains exchange rates at the beginning of each epoch,
 i.e., right after the rewards for the previous epoch have been deposited into the pool.
</dd>
<dt>
<code>pending_stake: u64</code>
</dt>
<dd>
 Pending stake amount for this epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>pending_total_myso_withdraw: u64</code>
</dt>
<dd>
 Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
 This includes both the principal and rewards MYSO withdrawn.
</dd>
<dt>
<code>pending_pool_token_withdraw: u64</code>
</dt>
<dd>
 Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>extra_fields: <a href="../myso/bag.md#myso_bag_Bag">myso::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="myso_system_staking_pool_PoolTokenExchangeRate"></a>

## Struct `PoolTokenExchangeRate`

Struct representing the exchange rate of the stake pool token to MYSO.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_system_staking_pool_StakedMySo"></a>

## Struct `StakedMySo`

A self-custodial object holding the staked MYSO tokens.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a> <b>has</b> key, store
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
<code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
 ID of the staking pool we are staking with.
</dd>
<dt>
<code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: u64</code>
</dt>
<dd>
 The epoch at which the stake becomes active.
</dd>
<dt>
<code>principal: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 The staked MYSO tokens.
</dd>
</dl>


</details>

<a name="myso_system_staking_pool_FungibleStakedMySo"></a>

## Struct `FungibleStakedMySo`

An alternative to <code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a></code> that holds the pool token amount instead of the MYSO balance.
StakedMySo objects can be converted to FungibleStakedMySos after the initial warmup period.
The advantage of this is that you can now merge multiple StakedMySo objects from different
activation epochs into a single FungibleStakedMySo object.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a> <b>has</b> key, store
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
<code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
 ID of the staking pool we are staking with.
</dd>
<dt>
<code>value: u64</code>
</dt>
<dd>
 The pool token amount.
</dd>
</dl>


</details>

<a name="myso_system_staking_pool_FungibleStakedMySoData"></a>

## Struct `FungibleStakedMySoData`

Holds useful information


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySoData">FungibleStakedMySoData</a> <b>has</b> key, store
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
<code>total_supply: u64</code>
</dt>
<dd>
 fungible_staked_myso supply
</dd>
<dt>
<code>principal: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 principal balance. Rewards are withdrawn from the reward pool
</dd>
</dl>


</details>

<a name="myso_system_staking_pool_FungibleStakedMySoDataKey"></a>

## Struct `FungibleStakedMySoDataKey`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySoDataKey">FungibleStakedMySoDataKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="myso_system_staking_pool_UnderflowMySoBalance"></a>

## Struct `UnderflowMySoBalance`

Holds the amount of MYSO that was underflowed when withdrawing from the pool
post safe mode. Cleaned up in the same transaction.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_UnderflowMySoBalance">UnderflowMySoBalance</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_system_staking_pool_MIN_STAKING_THRESHOLD"></a>

StakedMySo objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="myso_system_staking_pool_EInsufficientPoolTokenBalance"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>: u64 = 0;
</code></pre>



<a name="myso_system_staking_pool_EWrongPool"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EWrongPool">EWrongPool</a>: u64 = 1;
</code></pre>



<a name="myso_system_staking_pool_EWithdrawAmountCannotBeZero"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EWithdrawAmountCannotBeZero">EWithdrawAmountCannotBeZero</a>: u64 = 2;
</code></pre>



<a name="myso_system_staking_pool_EInsufficientMySoTokenBalance"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EInsufficientMySoTokenBalance">EInsufficientMySoTokenBalance</a>: u64 = 3;
</code></pre>



<a name="myso_system_staking_pool_EInsufficientRewardsPoolBalance"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EInsufficientRewardsPoolBalance">EInsufficientRewardsPoolBalance</a>: u64 = 4;
</code></pre>



<a name="myso_system_staking_pool_EDestroyNonzeroBalance"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EDestroyNonzeroBalance">EDestroyNonzeroBalance</a>: u64 = 5;
</code></pre>



<a name="myso_system_staking_pool_ETokenTimeLockIsSome"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_ETokenTimeLockIsSome">ETokenTimeLockIsSome</a>: u64 = 6;
</code></pre>



<a name="myso_system_staking_pool_EWrongDelegation"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EWrongDelegation">EWrongDelegation</a>: u64 = 7;
</code></pre>



<a name="myso_system_staking_pool_EPendingDelegationDoesNotExist"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EPendingDelegationDoesNotExist">EPendingDelegationDoesNotExist</a>: u64 = 8;
</code></pre>



<a name="myso_system_staking_pool_ETokenBalancesDoNotMatchExchangeRate"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>: u64 = 9;
</code></pre>



<a name="myso_system_staking_pool_EDelegationToInactivePool"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>: u64 = 10;
</code></pre>



<a name="myso_system_staking_pool_EDeactivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>: u64 = 11;
</code></pre>



<a name="myso_system_staking_pool_EIncompatibleStakedMySo"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EIncompatibleStakedMySo">EIncompatibleStakedMySo</a>: u64 = 12;
</code></pre>



<a name="myso_system_staking_pool_EWithdrawalInSameEpoch"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EWithdrawalInSameEpoch">EWithdrawalInSameEpoch</a>: u64 = 13;
</code></pre>



<a name="myso_system_staking_pool_EPoolAlreadyActive"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>: u64 = 14;
</code></pre>



<a name="myso_system_staking_pool_EPoolPreactiveOrInactive"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EPoolPreactiveOrInactive">EPoolPreactiveOrInactive</a>: u64 = 15;
</code></pre>



<a name="myso_system_staking_pool_EActivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>: u64 = 16;
</code></pre>



<a name="myso_system_staking_pool_EDelegationOfZeroMySo"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EDelegationOfZeroMySo">EDelegationOfZeroMySo</a>: u64 = 17;
</code></pre>



<a name="myso_system_staking_pool_EStakedMySoBelowThreshold"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EStakedMySoBelowThreshold">EStakedMySoBelowThreshold</a>: u64 = 18;
</code></pre>



<a name="myso_system_staking_pool_ECannotMintFungibleStakedMySoYet"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_ECannotMintFungibleStakedMySoYet">ECannotMintFungibleStakedMySoYet</a>: u64 = 19;
</code></pre>



<a name="myso_system_staking_pool_EInvariantFailure"></a>



<pre><code><b>const</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EInvariantFailure">EInvariantFailure</a>: u64 = 20;
</code></pre>



<a name="myso_system_staking_pool_new"></a>

## Function `new`

Create a new, empty staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_new">new</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a> {
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a> {
        id: object::new(ctx),
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>: option::none(),
        deactivation_epoch: option::none(),
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>: 0,
        rewards_pool: balance::zero(),
        pool_token_balance: 0,
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_exchange_rates">exchange_rates</a>: table::new(ctx),
        pending_stake: 0,
        pending_total_myso_withdraw: 0,
        pending_pool_token_withdraw: 0,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="myso_system_staking_pool_request_add_stake"></a>

## Function `request_add_stake`

Request to stake to a staking pool. The stake starts counting at the beginning of the next epoch,


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_request_add_stake">request_add_stake</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, stake: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_request_add_stake">request_add_stake</a>(
    pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>,
    stake: Balance&lt;MYSO&gt;,
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a> {
    <b>let</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a> = stake.value();
    <b>assert</b>!(!pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_inactive">is_inactive</a>(), <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>);
    <b>assert</b>!(<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a> &gt; 0, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EDelegationOfZeroMySo">EDelegationOfZeroMySo</a>);
    pool.pending_stake = pool.pending_stake + <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>;
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a> {
        id: object::new(ctx),
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>: object::id(pool),
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>,
        principal: stake,
    }
}
</code></pre>



</details>

<a name="myso_system_staking_pool_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw the given stake plus rewards from a staking pool.
Both the principal and corresponding rewards in MYSO are withdrawn.
A proportional amount of pool token withdraw is recorded and processed at epoch change time.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(
    pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>,
    staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>,
    ctx: &TxContext,
): Balance&lt;MYSO&gt; {
    // stake is inactive and the pool is not preactive - allow direct withdraw
    // the reason why we exclude preactive pools is to avoid potential underflow
    // on subtraction, and we need to enforce `pending_stake_withdraw` call.
    <b>if</b> (staked_myso.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a> &gt; ctx.epoch() && !pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive">is_preactive</a>()) {
        <b>let</b> principal = staked_myso.into_balance();
        pool.pending_stake = pool.pending_stake - principal.value();
        <b>return</b> principal
    };
    <b>let</b> (pool_token_withdraw_amount, <b>mut</b> principal_withdraw) = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
        staked_myso,
    );
    <b>let</b> principal_withdraw_amount = principal_withdraw.value();
    <b>let</b> rewards_withdraw = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_withdraw_rewards">withdraw_rewards</a>(
        principal_withdraw_amount,
        pool_token_withdraw_amount,
        ctx.epoch(),
    );
    <b>let</b> total_myso_withdraw_amount = principal_withdraw_amount + rewards_withdraw.value();
    pool.pending_total_myso_withdraw = pool.pending_total_myso_withdraw + total_myso_withdraw_amount;
    pool.pending_pool_token_withdraw =
        pool.pending_pool_token_withdraw + pool_token_withdraw_amount;
    // If the pool is inactive or preactive, we immediately process the withdrawal.
    <b>if</b> (pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_inactive">is_inactive</a>() || pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive">is_preactive</a>()) pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>();
    // TODO: implement withdraw bonding period here.
    principal_withdraw.join(rewards_withdraw);
    principal_withdraw
}
</code></pre>



</details>

<a name="myso_system_staking_pool_redeem_fungible_staked_myso"></a>

## Function `redeem_fungible_staked_myso`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_redeem_fungible_staked_myso">redeem_fungible_staked_myso</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, fungible_staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_redeem_fungible_staked_myso">redeem_fungible_staked_myso</a>(
    pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>,
    fungible_staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a>,
    ctx: &TxContext,
): Balance&lt;MYSO&gt; {
    <b>let</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a> { id, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>, value } = fungible_staked_myso;
    <b>assert</b>!(<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a> == object::id(pool), <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EWrongPool">EWrongPool</a>);
    id.delete();
    <b>let</b> latest_exchange_rate = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(ctx.epoch());
    <b>let</b> fungible_staked_myso_data: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySoData">FungibleStakedMySoData</a> =
        &<b>mut</b> pool.extra_fields[<a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySoDataKey">FungibleStakedMySoDataKey</a> {}];
    <b>let</b> (
        principal_amount,
        rewards_amount,
    ) = latest_exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_calculate_fungible_staked_myso_withdraw_amount">calculate_fungible_staked_myso_withdraw_amount</a>(
        value,
        fungible_staked_myso_data.principal.value(),
        fungible_staked_myso_data.total_supply,
    );
    fungible_staked_myso_data.total_supply = fungible_staked_myso_data.total_supply - value;
    <b>let</b> <b>mut</b> myso_out = fungible_staked_myso_data.principal.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_split">split</a>(principal_amount);
    myso_out.join(pool.rewards_pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_split">split</a>(rewards_amount));
    pool.pending_total_myso_withdraw = pool.pending_total_myso_withdraw + myso_out.value();
    pool.pending_pool_token_withdraw = pool.pending_pool_token_withdraw + value;
    myso_out
}
</code></pre>



</details>

<a name="myso_system_staking_pool_calculate_fungible_staked_myso_withdraw_amount"></a>

## Function `calculate_fungible_staked_myso_withdraw_amount`

written in separate function so i can test with random values
returns (principal_withdraw_amount, rewards_withdraw_amount)


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_calculate_fungible_staked_myso_withdraw_amount">calculate_fungible_staked_myso_withdraw_amount</a>(latest_exchange_rate: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_value">fungible_staked_myso_value</a>: u64, fungible_staked_myso_data_principal_amount: u64, fungible_staked_myso_data_total_supply: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_calculate_fungible_staked_myso_withdraw_amount">calculate_fungible_staked_myso_withdraw_amount</a>(
    latest_exchange_rate: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>,
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_value">fungible_staked_myso_value</a>: u64,
    fungible_staked_myso_data_principal_amount: u64, // fungible_staked_myso_data.principal.value()
    fungible_staked_myso_data_total_supply: u64, // fungible_staked_myso_data.total_supply
): (u64, u64) {
    // 1. <b>if</b> the entire <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySoData">FungibleStakedMySoData</a> supply is redeemed, how much myso should we receive?
    <b>let</b> total_myso_amount = latest_exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_myso_amount">get_myso_amount</a>(
        fungible_staked_myso_data_total_supply,
    );
    // min with total_myso_amount to prevent underflow
    <b>let</b> fungible_staked_myso_data_principal_amount = fungible_staked_myso_data_principal_amount.min(
        total_myso_amount,
    );
    // 2. how much do we need to withdraw from the rewards pool?
    <b>let</b> total_rewards = total_myso_amount - fungible_staked_myso_data_principal_amount;
    // 3. proportionally withdraw from both wrt the <a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_value">fungible_staked_myso_value</a>.
    <b>let</b> principal_withdraw_amount = <a href="../myso_system/staking_pool.md#myso_system_staking_pool_mul_div">mul_div</a>!(
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_value">fungible_staked_myso_value</a>,
        fungible_staked_myso_data_principal_amount,
        fungible_staked_myso_data_total_supply,
    );
    <b>let</b> rewards_withdraw_amount = <a href="../myso_system/staking_pool.md#myso_system_staking_pool_mul_div">mul_div</a>!(
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_value">fungible_staked_myso_value</a>,
        total_rewards,
        fungible_staked_myso_data_total_supply,
    );
    // <b>invariant</b> check, just in case
    <b>let</b> expected_myso_amount = latest_exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_myso_amount">get_myso_amount</a>(<a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_value">fungible_staked_myso_value</a>);
    <b>assert</b>!(
        principal_withdraw_amount + rewards_withdraw_amount &lt;= expected_myso_amount,
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EInvariantFailure">EInvariantFailure</a>,
    );
    (principal_withdraw_amount, rewards_withdraw_amount)
}
</code></pre>



</details>

<a name="myso_system_staking_pool_convert_to_fungible_staked_myso"></a>

## Function `convert_to_fungible_staked_myso`

Convert the given staked MYSO to an FungibleStakedMySo object


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_convert_to_fungible_staked_myso">convert_to_fungible_staked_myso</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_convert_to_fungible_staked_myso">convert_to_fungible_staked_myso</a>(
    pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>,
    staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a> {
    <b>let</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a> { id, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>, principal } = staked_myso;
    <b>assert</b>!(<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a> == object::id(pool), <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EWrongPool">EWrongPool</a>);
    <b>assert</b>!(ctx.epoch() &gt;= <a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_ECannotMintFungibleStakedMySoYet">ECannotMintFungibleStakedMySoYet</a>);
    <b>assert</b>!(!pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive">is_preactive</a>() && !pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_inactive">is_inactive</a>(), <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EPoolPreactiveOrInactive">EPoolPreactiveOrInactive</a>);
    id.delete();
    <b>let</b> exchange_rate_at_staking_epoch = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>,
    );
    <b>let</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a> = exchange_rate_at_staking_epoch.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_token_amount">get_token_amount</a>(principal.value());
    <b>let</b> key = <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySoDataKey">FungibleStakedMySoDataKey</a> {};
    <b>if</b> (!pool.extra_fields.contains(key)) {
        pool
            .extra_fields
            .add(
                key,
                <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySoData">FungibleStakedMySoData</a> {
                    id: object::new(ctx),
                    total_supply: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>,
                    principal,
                },
            );
    } <b>else</b> {
        <b>let</b> fungible_staked_myso_data: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySoData">FungibleStakedMySoData</a> = &<b>mut</b> pool.extra_fields[key];
        fungible_staked_myso_data.total_supply =
            fungible_staked_myso_data.total_supply + <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>;
        fungible_staked_myso_data.principal.join(principal);
    };
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a> {
        id: object::new(ctx),
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>,
        value: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>,
    }
}
</code></pre>



</details>

<a name="myso_system_staking_pool_withdraw_from_principal"></a>

## Function `withdraw_from_principal`

Withdraw the principal MYSO stored in the StakedMySo object, and calculate the corresponding amount of pool
tokens using exchange rate at staking epoch.
Returns values are amount of pool tokens withdrawn and withdrawn principal portion of MYSO.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>): (u64, <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
    pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>,
    staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>,
): (u64, Balance&lt;MYSO&gt;) {
    // Check that the stake information matches the pool.
    <b>assert</b>!(staked_myso.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a> == object::id(pool), <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EWrongPool">EWrongPool</a>);
    <b>let</b> exchange_rate_at_staking_epoch = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(staked_myso.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>);
    <b>let</b> principal_withdraw = staked_myso.into_balance();
    <b>let</b> pool_token_withdraw_amount = exchange_rate_at_staking_epoch.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_token_amount">get_token_amount</a>(principal_withdraw.value());
    (pool_token_withdraw_amount, principal_withdraw)
}
</code></pre>



</details>

<a name="myso_system_staking_pool_unwrap_staked_myso"></a>

## Function `unwrap_staked_myso`



<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_unwrap_staked_myso">unwrap_staked_myso</a>(staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_unwrap_staked_myso">unwrap_staked_myso</a>(staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>): Balance&lt;MYSO&gt; {
    <b>let</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a> { id, principal, .. } = staked_myso;
    id.delete();
    principal
}
</code></pre>



</details>

<a name="myso_system_staking_pool_deposit_rewards"></a>

## Function `deposit_rewards`

Called at epoch advancement times to add rewards (in MYSO) to the staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, rewards: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>, rewards: Balance&lt;MYSO&gt;) {
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a> = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a> + rewards.value();
    pool.rewards_pool.join(rewards);
}
</code></pre>



</details>

<a name="myso_system_staking_pool_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>, ctx: &TxContext) {
    <b>let</b> new_epoch = ctx.epoch() + 1;
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>();
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stake">process_pending_stake</a>();
    pool
        .<a href="../myso_system/staking_pool.md#myso_system_staking_pool_exchange_rates">exchange_rates</a>
        .add(
            new_epoch,
            <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
                <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>: pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>,
                <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>: pool.pool_token_balance,
            },
        );
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_check_balance_invariants">check_balance_invariants</a>(new_epoch);
}
</code></pre>



</details>

<a name="myso_system_staking_pool_process_pending_stake_withdraw"></a>

## Function `process_pending_stake_withdraw`

Called at epoch boundaries to process pending stake withdraws requested during the epoch.
Also called immediately upon withdrawal if the pool is inactive.


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>) {
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a> = <b>if</b> (pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a> &gt;= pool.pending_total_myso_withdraw) {
        pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a> - pool.pending_total_myso_withdraw
    } <b>else</b> {
        // the diff will be applied in the `<a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stake">process_pending_stake</a>` function.
        <b>let</b> diff = pool.pending_total_myso_withdraw - pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>;
        pool.extra_fields.add(<a href="../myso_system/staking_pool.md#myso_system_staking_pool_UnderflowMySoBalance">UnderflowMySoBalance</a> {}, diff);
        0
    };
    pool.pool_token_balance = <b>if</b> (pool.pool_token_balance &gt;= pool.pending_pool_token_withdraw) {
        pool.pool_token_balance - pool.pending_pool_token_withdraw
    } <b>else</b> {
        0
    };
    pool.pending_total_myso_withdraw = 0;
    pool.pending_pool_token_withdraw = 0;
}
</code></pre>



</details>

<a name="myso_system_staking_pool_process_pending_stake"></a>

## Function `process_pending_stake`

Called at epoch boundaries to process the pending stake.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>) {
    // Use the most up to date exchange rate with the rewards deposited and withdraws effectuated.
    <b>let</b> latest_exchange_rate = <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>: pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>,
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>: pool.pool_token_balance,
    };
    // This key is only present <b>if</b> the `<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>` underflowed, hence, the current value of `<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>`
    // is `0`. Pool token balance will be recalculated automatically <b>for</b> `0` value.
    <b>let</b> myso_diff = {
        <b>let</b> key = <a href="../myso_system/staking_pool.md#myso_system_staking_pool_UnderflowMySoBalance">UnderflowMySoBalance</a> {};
        <b>if</b> (pool.extra_fields.contains(key)) pool.extra_fields.remove(key) <b>else</b> 0
    };
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a> = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a> + pool.pending_stake - myso_diff;
    pool.pool_token_balance = latest_exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_token_amount">get_token_amount</a>(pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>);
    pool.pending_stake = 0;
}
</code></pre>



</details>

<a name="myso_system_staking_pool_withdraw_rewards"></a>

## Function `withdraw_rewards`

This function does the following:
1. Calculates the total amount of MYSO (including principal and rewards) that the provided pool tokens represent
at the current exchange rate.
2. Using the above number and the given <code>principal_withdraw_amount</code>, calculates the rewards portion of the
stake we should withdraw.
3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
portion because the principal portion was already taken out of the staker's self custodied StakedMySo.


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_withdraw_rewards">withdraw_rewards</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, principal_withdraw_amount: u64, pool_token_withdraw_amount: u64, epoch: u64): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_withdraw_rewards">withdraw_rewards</a>(
    pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>,
    principal_withdraw_amount: u64,
    pool_token_withdraw_amount: u64,
    epoch: u64,
): Balance&lt;MYSO&gt; {
    <b>let</b> exchange_rate = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch);
    <b>let</b> total_myso_withdraw_amount = exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_myso_amount">get_myso_amount</a>(pool_token_withdraw_amount);
    <b>let</b> <b>mut</b> reward_withdraw_amount = <b>if</b> (total_myso_withdraw_amount &gt;= principal_withdraw_amount) {
        total_myso_withdraw_amount - principal_withdraw_amount
    } <b>else</b> 0;
    // This may happen when we are withdrawing everything from the pool and
    // the rewards pool balance may be less than reward_withdraw_amount.
    // TODO: FIGURE OUT EXACTLY WHY THIS CAN HAPPEN.
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());
    pool.rewards_pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_split">split</a>(reward_withdraw_amount)
}
</code></pre>



</details>

<a name="myso_system_staking_pool_activate_staking_pool"></a>

## Function `activate_staking_pool`

Called by <code><a href="../myso_system/validator.md#myso_system_validator">validator</a></code> module to activate a staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>: u64) {
    // Add the initial exchange rate to the table.
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_exchange_rates">exchange_rates</a>.add(<a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>());
    // Check that the pool is preactive and not inactive.
    <b>assert</b>!(pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive">is_preactive</a>(), <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>);
    <b>assert</b>!(!pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_inactive">is_inactive</a>(), <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>);
    // Fill in the active epoch.
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>.fill(<a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>);
}
</code></pre>



</details>

<a name="myso_system_staking_pool_deactivate_staking_pool"></a>

## Function `deactivate_staking_pool`

Deactivate a staking pool by setting the <code>deactivation_epoch</code>. After
this pool deactivation, the pool stops earning rewards. Only stake
withdraws can be made to the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>, deactivation_epoch: u64) {
    // We can't deactivate an already deactivated pool.
    <b>assert</b>!(!pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_inactive">is_inactive</a>(), <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>);
    pool.deactivation_epoch = option::some(deactivation_epoch);
}
</code></pre>



</details>

<a name="myso_system_staking_pool_myso_balance"></a>

## Function `myso_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>): u64 { pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a> }
</code></pre>



</details>

<a name="myso_system_staking_pool_pool_id"></a>

## Function `pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>(staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>(staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>): ID { staked_myso.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a> }
</code></pre>



</details>

<a name="myso_system_staking_pool_fungible_staked_myso_pool_id"></a>

## Function `fungible_staked_myso_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_pool_id">fungible_staked_myso_pool_id</a>(fungible_staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_pool_id">fungible_staked_myso_pool_id</a>(fungible_staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a>): ID {
    fungible_staked_myso.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>
}
</code></pre>



</details>

<a name="myso_system_staking_pool_staked_myso_amount"></a>

## Function `staked_myso_amount`

Returns the principal amount of <code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_staked_myso_amount">staked_myso_amount</a>(staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_staked_myso_amount">staked_myso_amount</a>(staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>): u64 { staked_myso.principal.value() }
</code></pre>



</details>

<a name="myso_system_staking_pool_stake_activation_epoch"></a>

## Function `stake_activation_epoch`

Returns the activation epoch of <code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>(staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>(staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>): u64 {
    staked_myso.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>
}
</code></pre>



</details>

<a name="myso_system_staking_pool_is_preactive"></a>

## Function `is_preactive`

Returns true if the input staking pool is preactive.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive">is_preactive</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive">is_preactive</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>.is_none()
}
</code></pre>



</details>

<a name="myso_system_staking_pool_activation_epoch"></a>

## Function `activation_epoch`

Returns the activation epoch of the <code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a></code>. For validator candidates,
or pending validators, the value returned is <code>None</code>. For active validators,
the value is the epoch before the validator was activated.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>): Option&lt;u64&gt; {
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>
}
</code></pre>



</details>

<a name="myso_system_staking_pool_is_inactive"></a>

## Function `is_inactive`

Returns true if the input staking pool is inactive.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.deactivation_epoch.is_some()
}
</code></pre>



</details>

<a name="myso_system_staking_pool_fungible_staked_myso_value"></a>

## Function `fungible_staked_myso_value`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_value">fungible_staked_myso_value</a>(fungible_staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_fungible_staked_myso_value">fungible_staked_myso_value</a>(fungible_staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a>): u64 {
    fungible_staked_myso.value
}
</code></pre>



</details>

<a name="myso_system_staking_pool_split_fungible_staked_myso"></a>

## Function `split_fungible_staked_myso`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_split_fungible_staked_myso">split_fungible_staked_myso</a>(fungible_staked_myso: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_split_fungible_staked_myso">split_fungible_staked_myso</a>(
    fungible_staked_myso: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a>,
    split_amount: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a> {
    <b>assert</b>!(split_amount &lt;= fungible_staked_myso.value, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>);
    fungible_staked_myso.value = fungible_staked_myso.value - split_amount;
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a> {
        id: object::new(ctx),
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>: fungible_staked_myso.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>,
        value: split_amount,
    }
}
</code></pre>



</details>

<a name="myso_system_staking_pool_join_fungible_staked_myso"></a>

## Function `join_fungible_staked_myso`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_join_fungible_staked_myso">join_fungible_staked_myso</a>(self: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>, other: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_join_fungible_staked_myso">join_fungible_staked_myso</a>(self: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a>, other: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a>) {
    <b>let</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">FungibleStakedMySo</a> { id, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>, value } = other;
    <b>assert</b>!(self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a> == <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EWrongPool">EWrongPool</a>);
    id.delete();
    self.value = self.value + value;
}
</code></pre>



</details>

<a name="myso_system_staking_pool_split"></a>

## Function `split`

Split StakedMySo <code>self</code> to two parts, one with principal <code>split_amount</code>,
and the remaining principal is left in <code>self</code>.
All the other parameters of the StakedMySo like <code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a></code> or <code><a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a></code> remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_split">split</a>(self: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_split">split</a>(self: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>, split_amount: u64, ctx: &<b>mut</b> TxContext): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a> {
    <b>let</b> original_amount = self.principal.value();
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EInsufficientMySoTokenBalance">EInsufficientMySoTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="../myso_system/staking_pool.md#myso_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="../myso_system/staking_pool.md#myso_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EStakedMySoBelowThreshold">EStakedMySoBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="../myso_system/staking_pool.md#myso_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EStakedMySoBelowThreshold">EStakedMySoBelowThreshold</a>);
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a> {
        id: object::new(ctx),
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>: self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>,
        <a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>,
        principal: self.principal.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_split">split</a>(split_amount),
    }
}
</code></pre>



</details>

<a name="myso_system_staking_pool_split_staked_myso"></a>

## Function `split_staked_myso`

Split the given StakedMySo to the two parts, one with principal <code>split_amount</code>,
transfer the newly split part to the sender address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_split_staked_myso">split_staked_myso</a>(stake: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_split_staked_myso">split_staked_myso</a>(stake: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    transfer::transfer(stake.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_split">split</a>(split_amount, ctx), ctx.sender());
}
</code></pre>



</details>

<a name="myso_system_staking_pool_join_staked_myso"></a>

## Function `join_staked_myso`

Consume the staked myso <code>other</code> and add its value to <code>self</code>.
Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_join_staked_myso">join_staked_myso</a>(self: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, other: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_join_staked_myso">join_staked_myso</a>(self: &<b>mut</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>, other: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>) {
    <b>assert</b>!(<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self, &other), <a href="../myso_system/staking_pool.md#myso_system_staking_pool_EIncompatibleStakedMySo">EIncompatibleStakedMySo</a>);
    <b>let</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a> { id, principal, .. } = other;
    id.delete();
    self.principal.join(principal);
}
</code></pre>



</details>

<a name="myso_system_staking_pool_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the staking parameters of the staked myso except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, other: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>, other: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>): bool {
    (self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a> == other.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_id">pool_id</a>) &&
    (self.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a> == other.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>)
}
</code></pre>



</details>

<a name="myso_system_staking_pool_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, epoch: u64): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
    pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>,
    epoch: u64,
): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    // If the pool is preactive then the exchange rate is always 1:1.
    <b>if</b> (pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(epoch)) {
        <b>return</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
    };
    <b>let</b> clamped_epoch = pool.deactivation_epoch.get_with_default(epoch);
    <b>let</b> <b>mut</b> epoch = clamped_epoch.min(epoch);
    <b>let</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a> = *pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>.borrow();
    // Find the latest epoch that's earlier than the given epoch with an <b>entry</b> in the table
    <b>while</b> (epoch &gt;= <a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>) {
        <b>if</b> (pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_exchange_rates">exchange_rates</a>.contains(epoch)) {
            <b>return</b> pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_exchange_rates">exchange_rates</a>[epoch]
        };
        epoch = epoch - 1;
    };
    // This line really should be unreachable. Do we want an <b>assert</b> <b>false</b> here?
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
}
</code></pre>



</details>

<a name="myso_system_staking_pool_pending_stake_amount"></a>

## Function `pending_stake_amount`

Returns the total value of the pending staking requests for this staking pool.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.pending_stake
}
</code></pre>



</details>

<a name="myso_system_staking_pool_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`

Returns the total withdrawal from the staking pool this epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool">staking_pool</a>.pending_total_myso_withdraw
}
</code></pre>



</details>

<a name="myso_system_staking_pool_exchange_rates"></a>

## Function `exchange_rates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>): &<a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u64, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>): &Table&lt;u64, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>&gt; {
    &pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_exchange_rates">exchange_rates</a>
}
</code></pre>



</details>

<a name="myso_system_staking_pool_myso_amount"></a>

## Function `myso_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>(exchange_rate: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>(exchange_rate: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>
}
</code></pre>



</details>

<a name="myso_system_staking_pool_pool_token_amount"></a>

## Function `pool_token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>
}
</code></pre>



</details>

<a name="myso_system_staking_pool_is_preactive_at_epoch"></a>

## Function `is_preactive_at_epoch`

Returns true if the provided staking pool is preactive at the provided epoch.


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>, epoch: u64): bool {
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_is_preactive">is_preactive</a>() || (*pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_activation_epoch">activation_epoch</a>.borrow() &gt; epoch)
}
</code></pre>



</details>

<a name="myso_system_staking_pool_get_myso_amount"></a>

## Function `get_myso_amount`



<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_myso_amount">get_myso_amount</a>(exchange_rate: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>, token_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_myso_amount">get_myso_amount</a>(exchange_rate: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, token_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a> == 0 || exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a> == 0) {
        <b>return</b> token_amount
    };
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_mul_div">mul_div</a>!(exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>, token_amount, exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>)
}
</code></pre>



</details>

<a name="myso_system_staking_pool_get_token_amount"></a>

## Function `get_token_amount`



<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a> == 0 || exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a> == 0) {
        <b>return</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>
    };
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_mul_div">mul_div</a>!(exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>, exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>)
}
</code></pre>



</details>

<a name="myso_system_staking_pool_initial_exchange_rate"></a>

## Function `initial_exchange_rate`



<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { <a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_amount">myso_amount</a>: 0, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_amount">pool_token_amount</a>: 0 }
}
</code></pre>



</details>

<a name="myso_system_staking_pool_check_balance_invariants"></a>

## Function `check_balance_invariants`



<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>, epoch: u64) {
    <b>let</b> exchange_rate = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch);
    // check that the pool token balance and myso balance ratio matches the exchange rate stored.
    <b>let</b> expected = exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_token_amount">get_token_amount</a>(pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_myso_balance">myso_balance</a>);
    <b>let</b> actual = pool.pool_token_balance;
    <b>assert</b>!(expected == actual, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>)
}
</code></pre>



</details>

<a name="myso_system_staking_pool_mul_div"></a>

## Macro function `mul_div`



<pre><code><b>macro</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_mul_div">mul_div</a>($a: u64, $b: u64, $c: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>macro</b> <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_mul_div">mul_div</a>($a: u64, $b: u64, $c: u64): u64 {
    (($a <b>as</b> u128) * ($b <b>as</b> u128) / ($c <b>as</b> u128)) <b>as</b> u64
}
</code></pre>



</details>

<a name="myso_system_staking_pool_calculate_rewards"></a>

## Function `calculate_rewards`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_calculate_rewards">calculate_rewards</a>(pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">myso_system::staking_pool::StakingPool</a>, staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, current_epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool_calculate_rewards">calculate_rewards</a>(
    pool: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakingPool">StakingPool</a>,
    staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">StakedMySo</a>,
    current_epoch: u64,
): u64 {
    <b>let</b> staked_amount = staked_myso.amount();
    <b>let</b> pool_token_withdraw_amount = {
        <b>let</b> exchange_rate_at_staking_epoch = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(staked_myso.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>);
        exchange_rate_at_staking_epoch.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_token_amount">get_token_amount</a>(staked_amount)
    };
    <b>let</b> new_epoch_exchange_rate = pool.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(current_epoch);
    <b>let</b> total_myso_withdraw_amount = new_epoch_exchange_rate.<a href="../myso_system/staking_pool.md#myso_system_staking_pool_get_myso_amount">get_myso_amount</a>(
        pool_token_withdraw_amount,
    );
    <b>let</b> <b>mut</b> reward_withdraw_amount = <b>if</b> (total_myso_withdraw_amount &gt;= staked_amount) {
        total_myso_withdraw_amount - staked_amount
    } <b>else</b> 0;
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());
    reward_withdraw_amount
}
</code></pre>



</details>
