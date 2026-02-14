---
title: Module `orderbook::pool`
---

Public-facing interface for the package.


-  [Struct `Pool`](#orderbook_pool_Pool)
-  [Struct `PoolInner`](#orderbook_pool_PoolInner)
-  [Struct `PoolCreated`](#orderbook_pool_PoolCreated)
-  [Struct `BookParamsUpdated`](#orderbook_pool_BookParamsUpdated)
-  [Struct `MysoBurned`](#orderbook_pool_MysoBurned)
-  [Struct `ReferralRewards`](#orderbook_pool_ReferralRewards)
-  [Struct `ReferralClaimedEvent`](#orderbook_pool_ReferralClaimedEvent)
-  [Struct `ReferralClaimed`](#orderbook_pool_ReferralClaimed)
-  [Struct `ReferralFeeEvent`](#orderbook_pool_ReferralFeeEvent)
-  [Constants](#@Constants_0)
-  [Function `create_permissionless_pool`](#orderbook_pool_create_permissionless_pool)
-  [Function `place_limit_order`](#orderbook_pool_place_limit_order)
-  [Function `place_market_order`](#orderbook_pool_place_market_order)
-  [Function `swap_exact_base_for_quote`](#orderbook_pool_swap_exact_base_for_quote)
-  [Function `swap_exact_base_for_quote_with_manager`](#orderbook_pool_swap_exact_base_for_quote_with_manager)
-  [Function `swap_exact_quote_for_base`](#orderbook_pool_swap_exact_quote_for_base)
-  [Function `swap_exact_quote_for_base_with_manager`](#orderbook_pool_swap_exact_quote_for_base_with_manager)
-  [Function `swap_exact_quantity`](#orderbook_pool_swap_exact_quantity)
-  [Function `swap_exact_quantity_with_manager`](#orderbook_pool_swap_exact_quantity_with_manager)
-  [Function `modify_order`](#orderbook_pool_modify_order)
-  [Function `cancel_order`](#orderbook_pool_cancel_order)
-  [Function `cancel_orders`](#orderbook_pool_cancel_orders)
-  [Function `cancel_all_orders`](#orderbook_pool_cancel_all_orders)
-  [Function `withdraw_settled_amounts`](#orderbook_pool_withdraw_settled_amounts)
-  [Function `withdraw_settled_amounts_permissionless`](#orderbook_pool_withdraw_settled_amounts_permissionless)
-  [Function `stake`](#orderbook_pool_stake)
-  [Function `unstake`](#orderbook_pool_unstake)
-  [Function `submit_proposal`](#orderbook_pool_submit_proposal)
-  [Function `vote`](#orderbook_pool_vote)
-  [Function `claim_rebates`](#orderbook_pool_claim_rebates)
-  [Function `borrow_flashloan_base`](#orderbook_pool_borrow_flashloan_base)
-  [Function `borrow_flashloan_quote`](#orderbook_pool_borrow_flashloan_quote)
-  [Function `return_flashloan_base`](#orderbook_pool_return_flashloan_base)
-  [Function `return_flashloan_quote`](#orderbook_pool_return_flashloan_quote)
-  [Function `add_myso_price_point`](#orderbook_pool_add_myso_price_point)
-  [Function `burn_myso`](#orderbook_pool_burn_myso)
-  [Function `mint_referral`](#orderbook_pool_mint_referral)
-  [Function `update_referral_multiplier`](#orderbook_pool_update_referral_multiplier)
-  [Function `update_orderbook_referral_multiplier`](#orderbook_pool_update_orderbook_referral_multiplier)
-  [Function `update_pool_referral_multiplier`](#orderbook_pool_update_pool_referral_multiplier)
-  [Function `claim_referral_rewards`](#orderbook_pool_claim_referral_rewards)
-  [Function `claim_pool_referral_rewards`](#orderbook_pool_claim_pool_referral_rewards)
-  [Function `create_pool_admin`](#orderbook_pool_create_pool_admin)
-  [Function `unregister_pool_admin`](#orderbook_pool_unregister_pool_admin)
-  [Function `update_allowed_versions`](#orderbook_pool_update_allowed_versions)
-  [Function `update_pool_allowed_versions`](#orderbook_pool_update_pool_allowed_versions)
-  [Function `adjust_tick_size_admin`](#orderbook_pool_adjust_tick_size_admin)
-  [Function `adjust_min_lot_size_admin`](#orderbook_pool_adjust_min_lot_size_admin)
-  [Function `enable_ewma_state`](#orderbook_pool_enable_ewma_state)
-  [Function `set_ewma_params`](#orderbook_pool_set_ewma_params)
-  [Function `whitelisted`](#orderbook_pool_whitelisted)
-  [Function `stable_pool`](#orderbook_pool_stable_pool)
-  [Function `registered_pool`](#orderbook_pool_registered_pool)
-  [Function `get_quote_quantity_out`](#orderbook_pool_get_quote_quantity_out)
-  [Function `get_base_quantity_out`](#orderbook_pool_get_base_quantity_out)
-  [Function `get_quote_quantity_out_input_fee`](#orderbook_pool_get_quote_quantity_out_input_fee)
-  [Function `get_base_quantity_out_input_fee`](#orderbook_pool_get_base_quantity_out_input_fee)
-  [Function `get_quantity_out`](#orderbook_pool_get_quantity_out)
-  [Function `get_quantity_out_input_fee`](#orderbook_pool_get_quantity_out_input_fee)
-  [Function `get_base_quantity_in`](#orderbook_pool_get_base_quantity_in)
-  [Function `get_quote_quantity_in`](#orderbook_pool_get_quote_quantity_in)
-  [Function `mid_price`](#orderbook_pool_mid_price)
-  [Function `account_open_orders`](#orderbook_pool_account_open_orders)
-  [Function `get_level2_range`](#orderbook_pool_get_level2_range)
-  [Function `get_level2_ticks_from_mid`](#orderbook_pool_get_level2_ticks_from_mid)
-  [Function `vault_balances`](#orderbook_pool_vault_balances)
-  [Function `get_pool_id_by_asset`](#orderbook_pool_get_pool_id_by_asset)
-  [Function `get_order`](#orderbook_pool_get_order)
-  [Function `get_orders`](#orderbook_pool_get_orders)
-  [Function `get_account_order_details`](#orderbook_pool_get_account_order_details)
-  [Function `get_order_myso_price`](#orderbook_pool_get_order_myso_price)
-  [Function `get_order_myso_required`](#orderbook_pool_get_order_myso_required)
-  [Function `locked_balance`](#orderbook_pool_locked_balance)
-  [Function `can_place_limit_order`](#orderbook_pool_can_place_limit_order)
-  [Function `can_place_market_order`](#orderbook_pool_can_place_market_order)
-  [Function `check_market_order_params`](#orderbook_pool_check_market_order_params)
-  [Function `check_limit_order_params`](#orderbook_pool_check_limit_order_params)
-  [Function `pool_trade_params`](#orderbook_pool_pool_trade_params)
-  [Function `pool_trade_params_next`](#orderbook_pool_pool_trade_params_next)
-  [Function `pool_book_params`](#orderbook_pool_pool_book_params)
-  [Function `account_exists`](#orderbook_pool_account_exists)
-  [Function `account`](#orderbook_pool_account)
-  [Function `quorum`](#orderbook_pool_quorum)
-  [Function `id`](#orderbook_pool_id)
-  [Function `get_referral_balances`](#orderbook_pool_get_referral_balances)
-  [Function `get_pool_referral_balances`](#orderbook_pool_get_pool_referral_balances)
-  [Function `pool_referral_multiplier`](#orderbook_pool_pool_referral_multiplier)
-  [Function `create_pool`](#orderbook_pool_create_pool)
-  [Function `bids`](#orderbook_pool_bids)
-  [Function `asks`](#orderbook_pool_asks)
-  [Function `load_inner`](#orderbook_pool_load_inner)
-  [Function `load_inner_mut`](#orderbook_pool_load_inner_mut)
-  [Function `load_ewma_state`](#orderbook_pool_load_ewma_state)
-  [Function `place_order_int`](#orderbook_pool_place_order_int)
-  [Function `process_referral_fees`](#orderbook_pool_process_referral_fees)
-  [Function `update_ewma_state`](#orderbook_pool_update_ewma_state)


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
<b>use</b> <a href="../myso/versioned.md#myso_versioned">myso::versioned</a>;
<b>use</b> <a href="../orderbook/account.md#orderbook_account">orderbook::account</a>;
<b>use</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">orderbook::balance_manager</a>;
<b>use</b> <a href="../orderbook/balances.md#orderbook_balances">orderbook::balances</a>;
<b>use</b> <a href="../orderbook/big_vector.md#orderbook_big_vector">orderbook::big_vector</a>;
<b>use</b> <a href="../orderbook/book.md#orderbook_book">orderbook::book</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/ewma.md#orderbook_ewma">orderbook::ewma</a>;
<b>use</b> <a href="../orderbook/fill.md#orderbook_fill">orderbook::fill</a>;
<b>use</b> <a href="../orderbook/governance.md#orderbook_governance">orderbook::governance</a>;
<b>use</b> <a href="../orderbook/history.md#orderbook_history">orderbook::history</a>;
<b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
<b>use</b> <a href="../orderbook/myso_price.md#orderbook_myso_price">orderbook::myso_price</a>;
<b>use</b> <a href="../orderbook/order.md#orderbook_order">orderbook::order</a>;
<b>use</b> <a href="../orderbook/order_info.md#orderbook_order_info">orderbook::order_info</a>;
<b>use</b> <a href="../orderbook/registry.md#orderbook_registry">orderbook::registry</a>;
<b>use</b> <a href="../orderbook/state.md#orderbook_state">orderbook::state</a>;
<b>use</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">orderbook::trade_params</a>;
<b>use</b> <a href="../orderbook/utils.md#orderbook_utils">orderbook::utils</a>;
<b>use</b> <a href="../orderbook/vault.md#orderbook_vault">orderbook::vault</a>;
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



<a name="orderbook_pool_Pool"></a>

## Struct `Pool`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;<b>phantom</b> BaseAsset, <b>phantom</b> QuoteAsset&gt; <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/pool.md#orderbook_pool_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>inner: <a href="../myso/versioned.md#myso_versioned_Versioned">myso::versioned::Versioned</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_pool_PoolInner"></a>

## Struct `PoolInner`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;<b>phantom</b> BaseAsset, <b>phantom</b> QuoteAsset&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>allowed_versions: <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/book.md#orderbook_book">book</a>: <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/state.md#orderbook_state">state</a>: <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/vault.md#orderbook_vault">vault</a>: <a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">orderbook::myso_price::MySoPrice</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/pool.md#orderbook_pool_registered_pool">registered_pool</a>: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_pool_PoolCreated"></a>

## Struct `PoolCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/pool.md#orderbook_pool_PoolCreated">PoolCreated</a>&lt;<b>phantom</b> BaseAsset, <b>phantom</b> QuoteAsset&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>taker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>maker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tick_size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>lot_size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>whitelisted_pool: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_address: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_pool_BookParamsUpdated"></a>

## Struct `BookParamsUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/pool.md#orderbook_pool_BookParamsUpdated">BookParamsUpdated</a>&lt;<b>phantom</b> BaseAsset, <b>phantom</b> QuoteAsset&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>tick_size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>lot_size: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_size: u64</code>
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

<a name="orderbook_pool_MysoBurned"></a>

## Struct `MysoBurned`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/pool.md#orderbook_pool_MysoBurned">MysoBurned</a>&lt;<b>phantom</b> BaseAsset, <b>phantom</b> QuoteAsset&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>myso_burned: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_pool_ReferralRewards"></a>

## Struct `ReferralRewards`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/pool.md#orderbook_pool_ReferralRewards">ReferralRewards</a>&lt;<b>phantom</b> BaseAsset, <b>phantom</b> QuoteAsset&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>multiplier: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;BaseAsset&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>quote: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;QuoteAsset&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>myso: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_pool_ReferralClaimedEvent"></a>

## Struct `ReferralClaimedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/pool.md#orderbook_pool_ReferralClaimedEvent">ReferralClaimedEvent</a>&lt;<b>phantom</b> BaseAsset, <b>phantom</b> QuoteAsset&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>referral_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>base_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quote_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_pool_ReferralClaimed"></a>

## Struct `ReferralClaimed`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/pool.md#orderbook_pool_ReferralClaimed">ReferralClaimed</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>referral_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>base_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quote_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_pool_ReferralFeeEvent"></a>

## Struct `ReferralFeeEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/pool.md#orderbook_pool_ReferralFeeEvent">ReferralFeeEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>referral_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>base_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quote_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_fee: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="orderbook_pool_EInvalidFee"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidFee">EInvalidFee</a>: u64 = 1;
</code></pre>



<a name="orderbook_pool_ESameBaseAndQuote"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_ESameBaseAndQuote">ESameBaseAndQuote</a>: u64 = 2;
</code></pre>



<a name="orderbook_pool_EInvalidTickSize"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidTickSize">EInvalidTickSize</a>: u64 = 3;
</code></pre>



<a name="orderbook_pool_EInvalidLotSize"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidLotSize">EInvalidLotSize</a>: u64 = 4;
</code></pre>



<a name="orderbook_pool_EInvalidMinSize"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidMinSize">EInvalidMinSize</a>: u64 = 5;
</code></pre>



<a name="orderbook_pool_EInvalidQuantityIn"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidQuantityIn">EInvalidQuantityIn</a>: u64 = 6;
</code></pre>



<a name="orderbook_pool_EIneligibleReferencePool"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EIneligibleReferencePool">EIneligibleReferencePool</a>: u64 = 7;
</code></pre>



<a name="orderbook_pool_EInvalidOrderBalanceManager"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidOrderBalanceManager">EInvalidOrderBalanceManager</a>: u64 = 9;
</code></pre>



<a name="orderbook_pool_EIneligibleTargetPool"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EIneligibleTargetPool">EIneligibleTargetPool</a>: u64 = 10;
</code></pre>



<a name="orderbook_pool_EPackageVersionDisabled"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EPackageVersionDisabled">EPackageVersionDisabled</a>: u64 = 11;
</code></pre>



<a name="orderbook_pool_EMinimumQuantityOutNotMet"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EMinimumQuantityOutNotMet">EMinimumQuantityOutNotMet</a>: u64 = 12;
</code></pre>



<a name="orderbook_pool_EInvalidStake"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidStake">EInvalidStake</a>: u64 = 13;
</code></pre>



<a name="orderbook_pool_EPoolNotRegistered"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EPoolNotRegistered">EPoolNotRegistered</a>: u64 = 14;
</code></pre>



<a name="orderbook_pool_EPoolCannotBeBothWhitelistedAndStable"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EPoolCannotBeBothWhitelistedAndStable">EPoolCannotBeBothWhitelistedAndStable</a>: u64 = 15;
</code></pre>



<a name="orderbook_pool_EInvalidReferralMultiplier"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidReferralMultiplier">EInvalidReferralMultiplier</a>: u64 = 16;
</code></pre>



<a name="orderbook_pool_EInvalidEWMAAlpha"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidEWMAAlpha">EInvalidEWMAAlpha</a>: u64 = 17;
</code></pre>



<a name="orderbook_pool_EInvalidZScoreThreshold"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidZScoreThreshold">EInvalidZScoreThreshold</a>: u64 = 18;
</code></pre>



<a name="orderbook_pool_EInvalidAdditionalTakerFee"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EInvalidAdditionalTakerFee">EInvalidAdditionalTakerFee</a>: u64 = 19;
</code></pre>



<a name="orderbook_pool_EWrongPoolReferral"></a>



<pre><code><b>const</b> <a href="../orderbook/pool.md#orderbook_pool_EWrongPoolReferral">EWrongPoolReferral</a>: u64 = 20;
</code></pre>



<a name="orderbook_pool_create_permissionless_pool"></a>

## Function `create_permissionless_pool`

Create a new pool. The pool is registered in the registry.
Checks are performed to ensure the tick size, lot size,
and min size are valid.
The creation fee is transferred to the treasury address.
Returns the id of the pool created


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_create_permissionless_pool">create_permissionless_pool</a>&lt;BaseAsset, QuoteAsset&gt;(<a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> <a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>, tick_size: u64, lot_size: u64, min_size: u64, creation_fee: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_create_permissionless_pool">create_permissionless_pool</a>&lt;BaseAsset, QuoteAsset&gt;(
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> Registry,
    tick_size: u64,
    lot_size: u64,
    min_size: u64,
    creation_fee: Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext,
): ID {
    <b>assert</b>!(creation_fee.value() == <a href="../orderbook/constants.md#orderbook_constants_pool_creation_fee">constants::pool_creation_fee</a>(), <a href="../orderbook/pool.md#orderbook_pool_EInvalidFee">EInvalidFee</a>);
    <b>let</b> base_type = type_name::with_defining_ids&lt;BaseAsset&gt;();
    <b>let</b> quote_type = type_name::with_defining_ids&lt;QuoteAsset&gt;();
    <b>let</b> whitelisted_pool = <b>false</b>;
    <b>let</b> <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a> = <a href="../orderbook/registry.md#orderbook_registry">registry</a>.is_stablecoin(base_type) && <a href="../orderbook/registry.md#orderbook_registry">registry</a>.is_stablecoin(quote_type);
    <a href="../orderbook/pool.md#orderbook_pool_create_pool">create_pool</a>&lt;BaseAsset, QuoteAsset&gt;(
        <a href="../orderbook/registry.md#orderbook_registry">registry</a>,
        tick_size,
        lot_size,
        min_size,
        creation_fee,
        whitelisted_pool,
        <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>,
        ctx,
    )
}
</code></pre>



</details>

<a name="orderbook_pool_place_limit_order"></a>

## Function `place_limit_order`

Place a limit order. Quantity is in base asset terms.
For current version pay_with_myso must be true, so the fee will be paid with
MYSO tokens.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_place_limit_order">place_limit_order</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, client_order_id: u64, order_type: u8, self_matching_option: u8, price: u64, quantity: u64, is_bid: bool, pay_with_myso: bool, expire_timestamp: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_place_limit_order">place_limit_order</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    client_order_id: u64,
    order_type: u8,
    self_matching_option: u8,
    price: u64,
    quantity: u64,
    is_bid: bool,
    pay_with_myso: bool,
    expire_timestamp: u64,
    clock: &Clock,
    ctx: &TxContext,
): OrderInfo {
    self.<a href="../orderbook/pool.md#orderbook_pool_place_order_int">place_order_int</a>(
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>,
        trade_proof,
        client_order_id,
        order_type,
        self_matching_option,
        price,
        quantity,
        is_bid,
        pay_with_myso,
        expire_timestamp,
        clock,
        <b>false</b>,
        ctx,
    )
}
</code></pre>



</details>

<a name="orderbook_pool_place_market_order"></a>

## Function `place_market_order`

Place a market order. Quantity is in base asset terms. Calls
place_limit_order with
a price of MAX_PRICE for bids and MIN_PRICE for asks. Any quantity not
filled is cancelled.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_place_market_order">place_market_order</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, client_order_id: u64, self_matching_option: u8, quantity: u64, is_bid: bool, pay_with_myso: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_place_market_order">place_market_order</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    client_order_id: u64,
    self_matching_option: u8,
    quantity: u64,
    is_bid: bool,
    pay_with_myso: bool,
    clock: &Clock,
    ctx: &TxContext,
): OrderInfo {
    self.<a href="../orderbook/pool.md#orderbook_pool_place_order_int">place_order_int</a>(
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>,
        trade_proof,
        client_order_id,
        <a href="../orderbook/constants.md#orderbook_constants_immediate_or_cancel">constants::immediate_or_cancel</a>(),
        self_matching_option,
        <b>if</b> (is_bid) <a href="../orderbook/constants.md#orderbook_constants_max_price">constants::max_price</a>() <b>else</b> <a href="../orderbook/constants.md#orderbook_constants_min_price">constants::min_price</a>(),
        quantity,
        is_bid,
        pay_with_myso,
        clock.timestamp_ms(),
        clock,
        <b>true</b>,
        ctx,
    )
}
</code></pre>



</details>

<a name="orderbook_pool_swap_exact_base_for_quote"></a>

## Function `swap_exact_base_for_quote`

Swap exact base quantity without needing a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a></code>.
MYSO quantity can be overestimated. Returns three <code>Coin</code> objects:
base, quote, and myso. Some base quantity may be left over, if the
input quantity is not divisible by lot size.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_base_for_quote">swap_exact_base_for_quote</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, base_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, myso_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, min_quote_out: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_base_for_quote">swap_exact_base_for_quote</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    base_in: Coin&lt;BaseAsset&gt;,
    myso_in: Coin&lt;MYSO&gt;,
    min_quote_out: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, Coin&lt;QuoteAsset&gt;, Coin&lt;MYSO&gt;) {
    <b>let</b> quote_in = coin::zero(ctx);
    self.<a href="../orderbook/pool.md#orderbook_pool_swap_exact_quantity">swap_exact_quantity</a>(
        base_in,
        quote_in,
        myso_in,
        min_quote_out,
        clock,
        ctx,
    )
}
</code></pre>



</details>

<a name="orderbook_pool_swap_exact_base_for_quote_with_manager"></a>

## Function `swap_exact_base_for_quote_with_manager`

Swap exact base for quote with a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a></code>.
Assumes fees are paid in MYSO. Assumes balance manager has enough MYSO for fees.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_base_for_quote_with_manager">swap_exact_base_for_quote_with_manager</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>, deposit_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>, withdraw_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>, base_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, min_quote_out: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_base_for_quote_with_manager">swap_exact_base_for_quote_with_manager</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_cap: &TradeCap,
    deposit_cap: &DepositCap,
    withdraw_cap: &WithdrawCap,
    base_in: Coin&lt;BaseAsset&gt;,
    min_quote_out: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, Coin&lt;QuoteAsset&gt;) {
    <b>let</b> quote_in = coin::zero(ctx);
    self.<a href="../orderbook/pool.md#orderbook_pool_swap_exact_quantity_with_manager">swap_exact_quantity_with_manager</a>(
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>,
        trade_cap,
        deposit_cap,
        withdraw_cap,
        base_in,
        quote_in,
        min_quote_out,
        clock,
        ctx,
    )
}
</code></pre>



</details>

<a name="orderbook_pool_swap_exact_quote_for_base"></a>

## Function `swap_exact_quote_for_base`

Swap exact quote quantity without needing a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a></code>.
MYSO quantity can be overestimated. Returns three <code>Coin</code> objects:
base, quote, and myso. Some quote quantity may be left over if the
input quantity is not divisible by lot size.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_quote_for_base">swap_exact_quote_for_base</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, quote_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, myso_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, min_base_out: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_quote_for_base">swap_exact_quote_for_base</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    quote_in: Coin&lt;QuoteAsset&gt;,
    myso_in: Coin&lt;MYSO&gt;,
    min_base_out: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, Coin&lt;QuoteAsset&gt;, Coin&lt;MYSO&gt;) {
    <b>let</b> base_in = coin::zero(ctx);
    self.<a href="../orderbook/pool.md#orderbook_pool_swap_exact_quantity">swap_exact_quantity</a>(
        base_in,
        quote_in,
        myso_in,
        min_base_out,
        clock,
        ctx,
    )
}
</code></pre>



</details>

<a name="orderbook_pool_swap_exact_quote_for_base_with_manager"></a>

## Function `swap_exact_quote_for_base_with_manager`

Swap exact quote for base with a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a></code>.
Assumes fees are paid in MYSO. Assumes balance manager has enough MYSO for fees.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_quote_for_base_with_manager">swap_exact_quote_for_base_with_manager</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>, deposit_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>, withdraw_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>, quote_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, min_base_out: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_quote_for_base_with_manager">swap_exact_quote_for_base_with_manager</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_cap: &TradeCap,
    deposit_cap: &DepositCap,
    withdraw_cap: &WithdrawCap,
    quote_in: Coin&lt;QuoteAsset&gt;,
    min_base_out: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, Coin&lt;QuoteAsset&gt;) {
    <b>let</b> base_in = coin::zero(ctx);
    self.<a href="../orderbook/pool.md#orderbook_pool_swap_exact_quantity_with_manager">swap_exact_quantity_with_manager</a>(
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>,
        trade_cap,
        deposit_cap,
        withdraw_cap,
        base_in,
        quote_in,
        min_base_out,
        clock,
        ctx,
    )
}
</code></pre>



</details>

<a name="orderbook_pool_swap_exact_quantity"></a>

## Function `swap_exact_quantity`

Swap exact quantity without needing a balance_manager.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_quantity">swap_exact_quantity</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, base_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, quote_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, myso_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, min_out: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_quantity">swap_exact_quantity</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    base_in: Coin&lt;BaseAsset&gt;,
    quote_in: Coin&lt;QuoteAsset&gt;,
    myso_in: Coin&lt;MYSO&gt;,
    min_out: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, Coin&lt;QuoteAsset&gt;, Coin&lt;MYSO&gt;) {
    <b>let</b> <b>mut</b> base_quantity = base_in.value();
    <b>let</b> quote_quantity = quote_in.value();
    <b>let</b> taker_fee = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>().taker_fee();
    <b>let</b> input_fee_rate = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(
        taker_fee,
        <a href="../orderbook/constants.md#orderbook_constants_fee_penalty_multiplier">constants::fee_penalty_multiplier</a>(),
    );
    <b>assert</b>!((base_quantity &gt; 0) != (quote_quantity &gt; 0), <a href="../orderbook/pool.md#orderbook_pool_EInvalidQuantityIn">EInvalidQuantityIn</a>);
    <b>let</b> pay_with_myso = myso_in.value() &gt; 0;
    <b>let</b> is_bid = quote_quantity &gt; 0;
    <b>if</b> (is_bid) {
        (base_quantity, _, _) = <b>if</b> (pay_with_myso) {
            self.<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out">get_quantity_out</a>(0, quote_quantity, clock)
        } <b>else</b> {
            self.<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out_input_fee">get_quantity_out_input_fee</a>(0, quote_quantity, clock)
        }
    } <b>else</b> {
        <b>if</b> (!pay_with_myso) {
            base_quantity =
                <a href="../orderbook/math.md#orderbook_math_div">math::div</a>(
                    base_quantity,
                    <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>() + input_fee_rate,
                );
        }
    };
    base_quantity = base_quantity - base_quantity % self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/book.md#orderbook_book">book</a>.lot_size();
    <b>if</b> (base_quantity &lt; self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/book.md#orderbook_book">book</a>.min_size()) {
        <b>return</b> (base_in, quote_in, myso_in)
    };
    <b>let</b> <b>mut</b> temp_balance_manager = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new">balance_manager::new</a>(ctx);
    <b>let</b> trade_proof = temp_balance_manager.generate_proof_as_owner(ctx);
    temp_balance_manager.deposit(base_in, ctx);
    temp_balance_manager.deposit(quote_in, ctx);
    temp_balance_manager.deposit(myso_in, ctx);
    self.<a href="../orderbook/pool.md#orderbook_pool_place_market_order">place_market_order</a>(
        &<b>mut</b> temp_balance_manager,
        &trade_proof,
        0,
        <a href="../orderbook/constants.md#orderbook_constants_self_matching_allowed">constants::self_matching_allowed</a>(),
        base_quantity,
        is_bid,
        pay_with_myso,
        clock,
        ctx,
    );
    <b>let</b> base_out = temp_balance_manager.withdraw_all&lt;BaseAsset&gt;(ctx);
    <b>let</b> quote_out = temp_balance_manager.withdraw_all&lt;QuoteAsset&gt;(ctx);
    <b>let</b> myso_out = temp_balance_manager.withdraw_all&lt;MYSO&gt;(ctx);
    <b>if</b> (is_bid) {
        <b>assert</b>!(base_out.value() &gt;= min_out, <a href="../orderbook/pool.md#orderbook_pool_EMinimumQuantityOutNotMet">EMinimumQuantityOutNotMet</a>);
    } <b>else</b> {
        <b>assert</b>!(quote_out.value() &gt;= min_out, <a href="../orderbook/pool.md#orderbook_pool_EMinimumQuantityOutNotMet">EMinimumQuantityOutNotMet</a>);
    };
    temp_balance_manager.delete();
    (base_out, quote_out, myso_out)
}
</code></pre>



</details>

<a name="orderbook_pool_swap_exact_quantity_with_manager"></a>

## Function `swap_exact_quantity_with_manager`

Swap exact quantity with a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a></code>.
Assumes fees are paid in MYSO. Assumes balance manager has enough MYSO for fees.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_quantity_with_manager">swap_exact_quantity_with_manager</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>, deposit_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>, withdraw_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>, base_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, quote_in: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, min_out: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_swap_exact_quantity_with_manager">swap_exact_quantity_with_manager</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_cap: &TradeCap,
    deposit_cap: &DepositCap,
    withdraw_cap: &WithdrawCap,
    base_in: Coin&lt;BaseAsset&gt;,
    quote_in: Coin&lt;QuoteAsset&gt;,
    min_out: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, Coin&lt;QuoteAsset&gt;) {
    <b>let</b> <b>mut</b> adjusted_base_quantity = base_in.value();
    <b>let</b> base_quantity = base_in.value();
    <b>let</b> quote_quantity = quote_in.value();
    <b>assert</b>!((adjusted_base_quantity &gt; 0) != (quote_quantity &gt; 0), <a href="../orderbook/pool.md#orderbook_pool_EInvalidQuantityIn">EInvalidQuantityIn</a>);
    <b>let</b> is_bid = quote_quantity &gt; 0;
    <b>if</b> (is_bid) {
        (adjusted_base_quantity, _, _) = self.<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out">get_quantity_out</a>(0, quote_quantity, clock)
    } <b>else</b> {
        adjusted_base_quantity =
            adjusted_base_quantity - adjusted_base_quantity % self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/book.md#orderbook_book">book</a>.lot_size();
    };
    <b>if</b> (adjusted_base_quantity &lt; self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/book.md#orderbook_book">book</a>.min_size()) {
        <b>return</b> (base_in, quote_in)
    };
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.deposit_with_cap(deposit_cap, base_in, ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.deposit_with_cap(deposit_cap, quote_in, ctx);
    <b>let</b> trade_proof = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.generate_proof_as_trader(trade_cap, ctx);
    <b>let</b> <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a> = self.<a href="../orderbook/pool.md#orderbook_pool_place_market_order">place_market_order</a>(
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>,
        &trade_proof,
        0,
        <a href="../orderbook/constants.md#orderbook_constants_self_matching_allowed">constants::self_matching_allowed</a>(),
        adjusted_base_quantity,
        is_bid,
        <b>true</b>,
        clock,
        ctx,
    );
    <b>let</b> (base_out_quantity, quote_out_quantity) = <b>if</b> (is_bid) {
        <b>let</b> quote_left = quote_quantity - <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.cumulative_quote_quantity();
        (<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.executed_quantity(), quote_left)
    } <b>else</b> {
        <b>let</b> base_left = base_quantity - <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.executed_quantity();
        (base_left, <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.cumulative_quote_quantity())
    };
    <b>let</b> base_out = <b>if</b> (base_out_quantity &gt; 0) {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.withdraw_with_cap(withdraw_cap, base_out_quantity, ctx)
    } <b>else</b> {
        coin::zero(ctx)
    };
    <b>let</b> quote_out = <b>if</b> (quote_out_quantity &gt; 0) {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.withdraw_with_cap(withdraw_cap, quote_out_quantity, ctx)
    } <b>else</b> {
        coin::zero(ctx)
    };
    <b>if</b> (is_bid) {
        <b>assert</b>!(base_out.value() &gt;= min_out, <a href="../orderbook/pool.md#orderbook_pool_EMinimumQuantityOutNotMet">EMinimumQuantityOutNotMet</a>);
    } <b>else</b> {
        <b>assert</b>!(quote_out.value() &gt;= min_out, <a href="../orderbook/pool.md#orderbook_pool_EMinimumQuantityOutNotMet">EMinimumQuantityOutNotMet</a>);
    };
    (base_out, quote_out)
}
</code></pre>



</details>

<a name="orderbook_pool_modify_order"></a>

## Function `modify_order`

Modifies an order given order_id and new_quantity.
New quantity must be less than the original quantity and more
than the filled quantity. Order must not have already expired.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_modify_order">modify_order</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, order_id: u128, new_quantity: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_modify_order">modify_order</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    order_id: u128,
    new_quantity: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>let</b> previous_quantity = self.<a href="../orderbook/pool.md#orderbook_pool_get_order">get_order</a>(order_id).quantity();
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> (cancel_quantity, <a href="../orderbook/order.md#orderbook_order">order</a>) = self
        .<a href="../orderbook/book.md#orderbook_book">book</a>
        .<a href="../orderbook/pool.md#orderbook_pool_modify_order">modify_order</a>(order_id, new_quantity, clock.timestamp_ms());
    <b>assert</b>!(<a href="../orderbook/order.md#orderbook_order">order</a>.balance_manager_id() == <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(), <a href="../orderbook/pool.md#orderbook_pool_EInvalidOrderBalanceManager">EInvalidOrderBalanceManager</a>);
    <b>let</b> (settled, owed) = self
        .<a href="../orderbook/state.md#orderbook_state">state</a>
        .process_modify(
            <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(),
            cancel_quantity,
            <a href="../orderbook/order.md#orderbook_order">order</a>,
            self.pool_id,
            ctx,
        );
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.settle_balance_manager(settled, owed, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, trade_proof);
    <a href="../orderbook/order.md#orderbook_order">order</a>.emit_order_modified(
        self.pool_id,
        previous_quantity,
        ctx.sender(),
        clock.timestamp_ms(),
    );
}
</code></pre>



</details>

<a name="orderbook_pool_cancel_order"></a>

## Function `cancel_order`

Cancel an order. The order must be owned by the balance_manager.
The order is removed from the book and the balance_manager's open orders.
The balance_manager's balance is updated with the order's remaining
quantity.
Order canceled event is emitted.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_cancel_order">cancel_order</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, order_id: u128, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_cancel_order">cancel_order</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    order_id: u128,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> <b>mut</b> <a href="../orderbook/order.md#orderbook_order">order</a> = self.<a href="../orderbook/book.md#orderbook_book">book</a>.<a href="../orderbook/pool.md#orderbook_pool_cancel_order">cancel_order</a>(order_id);
    <b>assert</b>!(<a href="../orderbook/order.md#orderbook_order">order</a>.balance_manager_id() == <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(), <a href="../orderbook/pool.md#orderbook_pool_EInvalidOrderBalanceManager">EInvalidOrderBalanceManager</a>);
    <b>let</b> (settled, owed) = self
        .<a href="../orderbook/state.md#orderbook_state">state</a>
        .process_cancel(&<b>mut</b> <a href="../orderbook/order.md#orderbook_order">order</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(), self.pool_id, ctx);
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.settle_balance_manager(settled, owed, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, trade_proof);
    <a href="../orderbook/order.md#orderbook_order">order</a>.emit_order_canceled(
        self.pool_id,
        ctx.sender(),
        clock.timestamp_ms(),
    );
}
</code></pre>



</details>

<a name="orderbook_pool_cancel_orders"></a>

## Function `cancel_orders`

Cancel multiple orders within a vector. The orders must be owned by the
balance_manager.
The orders are removed from the book and the balance_manager's open orders.
Order canceled events are emitted.
If any order fails to cancel, no orders will be cancelled.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_cancel_orders">cancel_orders</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, order_ids: vector&lt;u128&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_cancel_orders">cancel_orders</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    order_ids: vector&lt;u128&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> num_orders = order_ids.length();
    <b>while</b> (i &lt; num_orders) {
        <b>let</b> order_id = order_ids[i];
        self.<a href="../orderbook/pool.md#orderbook_pool_cancel_order">cancel_order</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, trade_proof, order_id, clock, ctx);
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="orderbook_pool_cancel_all_orders"></a>

## Function `cancel_all_orders`

Cancel all open orders placed by the balance manager in the pool.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_cancel_all_orders">cancel_all_orders</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_cancel_all_orders">cancel_all_orders</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>let</b> inner = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> <b>mut</b> open_orders = vector[];
    <b>if</b> (inner.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/pool.md#orderbook_pool_account_exists">account_exists</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>())) {
        open_orders = inner.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/account.md#orderbook_account">account</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>()).open_orders().into_keys();
    };
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> num_orders = open_orders.length();
    <b>while</b> (i &lt; num_orders) {
        <b>let</b> order_id = open_orders[i];
        self.<a href="../orderbook/pool.md#orderbook_pool_cancel_order">cancel_order</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, trade_proof, order_id, clock, ctx);
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="orderbook_pool_withdraw_settled_amounts"></a>

## Function `withdraw_settled_amounts`

Withdraw settled amounts to the <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_withdraw_settled_amounts">withdraw_settled_amounts</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_withdraw_settled_amounts">withdraw_settled_amounts</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> (settled, owed) = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/pool.md#orderbook_pool_withdraw_settled_amounts">withdraw_settled_amounts</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>());
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.settle_balance_manager(settled, owed, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, trade_proof);
}
</code></pre>



</details>

<a name="orderbook_pool_withdraw_settled_amounts_permissionless"></a>

## Function `withdraw_settled_amounts_permissionless`

Withdraw settled amounts permissionlessly to the <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_withdraw_settled_amounts_permissionless">withdraw_settled_amounts_permissionless</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_withdraw_settled_amounts_permissionless">withdraw_settled_amounts_permissionless</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> (settled, owed) = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/pool.md#orderbook_pool_withdraw_settled_amounts">withdraw_settled_amounts</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>());
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.settle_balance_manager_permissionless(settled, owed, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>);
}
</code></pre>



</details>

<a name="orderbook_pool_stake"></a>

## Function `stake`

Stake MYSO tokens to the pool. The balance_manager must have enough MYSO
tokens.
The balance_manager's data is updated with the staked amount.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_stake">stake</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, amount: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_stake">stake</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    amount: u64,
    ctx: &TxContext,
) {
    <b>assert</b>!(amount &gt; 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidStake">EInvalidStake</a>);
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> (settled, owed) = self.<a href="../orderbook/state.md#orderbook_state">state</a>.process_stake(self.pool_id, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(), amount, ctx);
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.settle_balance_manager(settled, owed, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, trade_proof);
}
</code></pre>



</details>

<a name="orderbook_pool_unstake"></a>

## Function `unstake`

Unstake MYSO tokens from the pool. The balance_manager must have enough
staked MYSO tokens.
The balance_manager's data is updated with the unstaked amount.
Balance is transferred to the balance_manager immediately.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_unstake">unstake</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_unstake">unstake</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    ctx: &TxContext,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> (settled, owed) = self.<a href="../orderbook/state.md#orderbook_state">state</a>.process_unstake(self.pool_id, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(), ctx);
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.settle_balance_manager(settled, owed, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, trade_proof);
}
</code></pre>



</details>

<a name="orderbook_pool_submit_proposal"></a>

## Function `submit_proposal`

Submit a proposal to change the taker fee, maker fee, and stake required.
The balance_manager must have enough staked MYSO tokens to participate.
Each balance_manager can only submit one proposal per epoch.
If the maximum proposal is reached, the proposal with the lowest vote is
removed.
If the balance_manager has less voting power than the lowest voted proposal,
the proposal is not added.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_submit_proposal">submit_proposal</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, taker_fee: u64, maker_fee: u64, stake_required: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_submit_proposal">submit_proposal</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    taker_fee: u64,
    maker_fee: u64,
    stake_required: u64,
    ctx: &TxContext,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.validate_proof(trade_proof);
    self
        .<a href="../orderbook/state.md#orderbook_state">state</a>
        .process_proposal(
            self.pool_id,
            <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(),
            taker_fee,
            maker_fee,
            stake_required,
            ctx,
        );
}
</code></pre>



</details>

<a name="orderbook_pool_vote"></a>

## Function `vote`

Vote on a proposal. The balance_manager must have enough staked MYSO tokens
to participate.
Full voting power of the balance_manager is used.
Voting for a new proposal will remove the vote from the previous proposal.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_vote">vote</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_vote">vote</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    proposal_id: ID,
    ctx: &TxContext,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.validate_proof(trade_proof);
    self.<a href="../orderbook/state.md#orderbook_state">state</a>.process_vote(self.pool_id, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(), proposal_id, ctx);
}
</code></pre>



</details>

<a name="orderbook_pool_claim_rebates"></a>

## Function `claim_rebates`

Claim the rewards for the balance_manager. The balance_manager must have
rewards to claim.
The balance_manager's data is updated with the claimed rewards.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_claim_rebates">claim_rebates</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_claim_rebates">claim_rebates</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    ctx: &TxContext,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> (settled, owed) = self
        .<a href="../orderbook/state.md#orderbook_state">state</a>
        .process_claim_rebates&lt;BaseAsset, QuoteAsset&gt;(
            self.pool_id,
            <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>,
            ctx,
        );
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.settle_balance_manager(settled, owed, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, trade_proof);
}
</code></pre>



</details>

<a name="orderbook_pool_borrow_flashloan_base"></a>

## Function `borrow_flashloan_base`

Borrow base assets from the Pool. A hot potato is returned,
forcing the borrower to return the assets within the same transaction.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_borrow_flashloan_base">borrow_flashloan_base</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, base_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">orderbook::vault::FlashLoan</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_borrow_flashloan_base">borrow_flashloan_base</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    base_amount: u64,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, FlashLoan) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.<a href="../orderbook/pool.md#orderbook_pool_borrow_flashloan_base">borrow_flashloan_base</a>(self.pool_id, base_amount, ctx)
}
</code></pre>



</details>

<a name="orderbook_pool_borrow_flashloan_quote"></a>

## Function `borrow_flashloan_quote`

Borrow quote assets from the Pool. A hot potato is returned,
forcing the borrower to return the assets within the same transaction.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_borrow_flashloan_quote">borrow_flashloan_quote</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, quote_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">orderbook::vault::FlashLoan</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_borrow_flashloan_quote">borrow_flashloan_quote</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    quote_amount: u64,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;QuoteAsset&gt;, FlashLoan) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.<a href="../orderbook/pool.md#orderbook_pool_borrow_flashloan_quote">borrow_flashloan_quote</a>(self.pool_id, quote_amount, ctx)
}
</code></pre>



</details>

<a name="orderbook_pool_return_flashloan_base"></a>

## Function `return_flashloan_base`

Return the flashloaned base assets to the Pool.
FlashLoan object will only be unwrapped if the assets are returned,
otherwise the transaction will fail.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_return_flashloan_base">return_flashloan_base</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, flash_loan: <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">orderbook::vault::FlashLoan</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_return_flashloan_base">return_flashloan_base</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    coin: Coin&lt;BaseAsset&gt;,
    flash_loan: FlashLoan,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.<a href="../orderbook/pool.md#orderbook_pool_return_flashloan_base">return_flashloan_base</a>(self.pool_id, coin, flash_loan);
}
</code></pre>



</details>

<a name="orderbook_pool_return_flashloan_quote"></a>

## Function `return_flashloan_quote`

Return the flashloaned quote assets to the Pool.
FlashLoan object will only be unwrapped if the assets are returned,
otherwise the transaction will fail.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_return_flashloan_quote">return_flashloan_quote</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, flash_loan: <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">orderbook::vault::FlashLoan</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_return_flashloan_quote">return_flashloan_quote</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    coin: Coin&lt;QuoteAsset&gt;,
    flash_loan: FlashLoan,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.<a href="../orderbook/pool.md#orderbook_pool_return_flashloan_quote">return_flashloan_quote</a>(self.pool_id, coin, flash_loan);
}
</code></pre>



</details>

<a name="orderbook_pool_add_myso_price_point"></a>

## Function `add_myso_price_point`

Adds a price point along with a timestamp to the myso price.
Allows for the calculation of myso price per base asset.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_add_myso_price_point">add_myso_price_point</a>&lt;BaseAsset, QuoteAsset, ReferenceBaseAsset, ReferenceQuoteAsset&gt;(target_pool: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, reference_pool: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;ReferenceBaseAsset, ReferenceQuoteAsset&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_add_myso_price_point">add_myso_price_point</a>&lt;BaseAsset, QuoteAsset, ReferenceBaseAsset, ReferenceQuoteAsset&gt;(
    target_pool: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    reference_pool: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;ReferenceBaseAsset, ReferenceQuoteAsset&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(
        reference_pool.<a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>() && reference_pool.<a href="../orderbook/pool.md#orderbook_pool_registered_pool">registered_pool</a>(),
        <a href="../orderbook/pool.md#orderbook_pool_EIneligibleReferencePool">EIneligibleReferencePool</a>,
    );
    <b>let</b> reference_pool_price = reference_pool.<a href="../orderbook/pool.md#orderbook_pool_mid_price">mid_price</a>(clock);
    <b>let</b> target_pool = target_pool.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> reference_base_type = type_name::with_defining_ids&lt;ReferenceBaseAsset&gt;();
    <b>let</b> reference_quote_type = type_name::with_defining_ids&lt;ReferenceQuoteAsset&gt;();
    <b>let</b> target_base_type = type_name::with_defining_ids&lt;BaseAsset&gt;();
    <b>let</b> target_quote_type = type_name::with_defining_ids&lt;QuoteAsset&gt;();
    <b>let</b> myso_type = type_name::with_defining_ids&lt;MYSO&gt;();
    <b>let</b> timestamp = clock.timestamp_ms();
    <b>assert</b>!(
        reference_base_type == myso_type || reference_quote_type == myso_type,
        <a href="../orderbook/pool.md#orderbook_pool_EIneligibleTargetPool">EIneligibleTargetPool</a>,
    );
    <b>let</b> reference_myso_is_base = reference_base_type == myso_type;
    <b>let</b> reference_other_type = <b>if</b> (reference_myso_is_base) {
        reference_quote_type
    } <b>else</b> {
        reference_base_type
    };
    <b>let</b> reference_other_is_target_base = reference_other_type == target_base_type;
    <b>let</b> reference_other_is_target_quote = reference_other_type == target_quote_type;
    <b>assert</b>!(
        reference_other_is_target_base || reference_other_is_target_quote,
        <a href="../orderbook/pool.md#orderbook_pool_EIneligibleTargetPool">EIneligibleTargetPool</a>,
    );
    // For MYSO/USDC <a href="../orderbook/pool.md#orderbook_pool">pool</a>, reference_myso_is_base is <b>true</b>, MYSO per USDC is
    // reference_pool_price
    // For USDC/MYSO <a href="../orderbook/pool.md#orderbook_pool">pool</a>, reference_myso_is_base is <b>false</b>, USDC per MYSO is
    // reference_pool_price
    <b>let</b> myso_per_reference_other_price = <b>if</b> (reference_myso_is_base) {
        <a href="../orderbook/math.md#orderbook_math_div">math::div</a>(1_000_000_000, reference_pool_price)
    } <b>else</b> {
        reference_pool_price
    };
    // For USDC/MYSO <a href="../orderbook/pool.md#orderbook_pool">pool</a>, reference_other_is_target_base is <b>true</b>, add price
    // point to myso per base
    // For MYSO/USDC <a href="../orderbook/pool.md#orderbook_pool">pool</a>, reference_other_is_target_base is <b>false</b>, add price
    // point to myso per quote
    target_pool
        .<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>
        .add_price_point(
            myso_per_reference_other_price,
            timestamp,
            reference_other_is_target_base,
        );
    emit_myso_price_added(
        myso_per_reference_other_price,
        timestamp,
        reference_other_is_target_base,
        reference_pool.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().pool_id,
        target_pool.pool_id,
    );
}
</code></pre>



</details>

<a name="orderbook_pool_burn_myso"></a>

## Function `burn_myso`

Burns MYSO tokens from the pool by transferring to the zero address.
Amount to burn is within history.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_burn_myso">burn_myso</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_burn_myso">burn_myso</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    ctx: &<b>mut</b> TxContext,
): u64 {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> balance_to_burn = self.<a href="../orderbook/state.md#orderbook_state">state</a>.history_mut().reset_balance_to_burn();
    <b>let</b> myso_to_burn = self.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.withdraw_myso_to_burn(balance_to_burn).into_coin(ctx);
    <b>let</b> amount_burned = myso_to_burn.value();
    transfer::public_transfer(myso_to_burn, @0x0);
    event::emit(<a href="../orderbook/pool.md#orderbook_pool_MysoBurned">MysoBurned</a>&lt;BaseAsset, QuoteAsset&gt; {
        pool_id: self.pool_id,
        myso_burned: amount_burned,
    });
    amount_burned
}
</code></pre>



</details>

<a name="orderbook_pool_mint_referral"></a>

## Function `mint_referral`

Mint a OrderbookReferral and set the additional bps for the referral.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_mint_referral">mint_referral</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, multiplier: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_mint_referral">mint_referral</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    multiplier: u64,
    ctx: &<b>mut</b> TxContext,
): ID {
    <b>assert</b>!(multiplier &lt;= <a href="../orderbook/constants.md#orderbook_constants_referral_max_multiplier">constants::referral_max_multiplier</a>(), <a href="../orderbook/pool.md#orderbook_pool_EInvalidReferralMultiplier">EInvalidReferralMultiplier</a>);
    <b>assert</b>!(multiplier % <a href="../orderbook/constants.md#orderbook_constants_referral_multiplier">constants::referral_multiplier</a>() == 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidReferralMultiplier">EInvalidReferralMultiplier</a>);
    <b>let</b> _ = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> referral_id = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_referral">balance_manager::mint_referral</a>(self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(), ctx);
    self
        .<a href="../orderbook/pool.md#orderbook_pool_id">id</a>
        .add(
            referral_id,
            <a href="../orderbook/pool.md#orderbook_pool_ReferralRewards">ReferralRewards</a>&lt;BaseAsset, QuoteAsset&gt; {
                multiplier,
                base: balance::zero(),
                quote: balance::zero(),
                myso: balance::zero(),
            },
        );
    referral_id
}
</code></pre>



</details>

<a name="orderbook_pool_update_referral_multiplier"></a>

## Function `update_referral_multiplier`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_referral_multiplier">update_referral_multiplier</a>&lt;BaseAsset, QuoteAsset&gt;(_self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, _referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">orderbook::balance_manager::OrderbookReferral</a>, _multiplier: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_referral_multiplier">update_referral_multiplier</a>&lt;BaseAsset, QuoteAsset&gt;(
    _self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    _referral: &OrderbookReferral,
    _multiplier: u64,
) {
    <b>abort</b> 1337
}
</code></pre>



</details>

<a name="orderbook_pool_update_orderbook_referral_multiplier"></a>

## Function `update_orderbook_referral_multiplier`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_orderbook_referral_multiplier">update_orderbook_referral_multiplier</a>&lt;BaseAsset, QuoteAsset&gt;(_self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, _referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">orderbook::balance_manager::OrderbookReferral</a>, _multiplier: u64, _ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_orderbook_referral_multiplier">update_orderbook_referral_multiplier</a>&lt;BaseAsset, QuoteAsset&gt;(
    _self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    _referral: &OrderbookReferral,
    _multiplier: u64,
    _ctx: &TxContext,
) {
    <b>abort</b>
}
</code></pre>



</details>

<a name="orderbook_pool_update_pool_referral_multiplier"></a>

## Function `update_pool_referral_multiplier`

Update the multiplier for the referral.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_pool_referral_multiplier">update_pool_referral_multiplier</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">orderbook::balance_manager::OrderbookPoolReferral</a>, multiplier: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_pool_referral_multiplier">update_pool_referral_multiplier</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    referral: &OrderbookPoolReferral,
    multiplier: u64,
    ctx: &TxContext,
) {
    <b>let</b> _ = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    referral.assert_referral_owner(ctx);
    <b>assert</b>!(multiplier &lt;= <a href="../orderbook/constants.md#orderbook_constants_referral_max_multiplier">constants::referral_max_multiplier</a>(), <a href="../orderbook/pool.md#orderbook_pool_EInvalidReferralMultiplier">EInvalidReferralMultiplier</a>);
    <b>assert</b>!(multiplier % <a href="../orderbook/constants.md#orderbook_constants_referral_multiplier">constants::referral_multiplier</a>() == 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidReferralMultiplier">EInvalidReferralMultiplier</a>);
    <b>let</b> referral_id = object::id(referral);
    <b>let</b> referral_rewards: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_ReferralRewards">ReferralRewards</a>&lt;BaseAsset, QuoteAsset&gt; = self
        .<a href="../orderbook/pool.md#orderbook_pool_id">id</a>
        .borrow_mut(referral_id);
    referral_rewards.multiplier = multiplier;
}
</code></pre>



</details>

<a name="orderbook_pool_claim_referral_rewards"></a>

## Function `claim_referral_rewards`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_claim_referral_rewards">claim_referral_rewards</a>&lt;BaseAsset, QuoteAsset&gt;(_self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, _referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">orderbook::balance_manager::OrderbookReferral</a>, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_claim_referral_rewards">claim_referral_rewards</a>&lt;BaseAsset, QuoteAsset&gt;(
    _self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    _referral: &OrderbookReferral,
    _ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, Coin&lt;QuoteAsset&gt;, Coin&lt;MYSO&gt;) {
    <b>abort</b>
}
</code></pre>



</details>

<a name="orderbook_pool_claim_pool_referral_rewards"></a>

## Function `claim_pool_referral_rewards`

Claim the rewards for the referral.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_claim_pool_referral_rewards">claim_pool_referral_rewards</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">orderbook::balance_manager::OrderbookPoolReferral</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_claim_pool_referral_rewards">claim_pool_referral_rewards</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    referral: &OrderbookPoolReferral,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, Coin&lt;QuoteAsset&gt;, Coin&lt;MYSO&gt;) {
    <b>let</b> _ = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    referral.assert_referral_owner(ctx);
    <b>let</b> referral_id = object::id(referral);
    <b>let</b> referral_rewards: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_ReferralRewards">ReferralRewards</a>&lt;BaseAsset, QuoteAsset&gt; = self
        .<a href="../orderbook/pool.md#orderbook_pool_id">id</a>
        .borrow_mut(referral_id);
    <b>let</b> base = referral_rewards.base.withdraw_all().into_coin(ctx);
    <b>let</b> quote = referral_rewards.quote.withdraw_all().into_coin(ctx);
    <b>let</b> myso_rewards = referral_rewards.myso.withdraw_all().into_coin(ctx);
    event::emit(<a href="../orderbook/pool.md#orderbook_pool_ReferralClaimed">ReferralClaimed</a> {
        pool_id: self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(),
        referral_id,
        owner: ctx.sender(),
        base_amount: base.value(),
        quote_amount: quote.value(),
        myso_amount: myso_rewards.value(),
    });
    (base, quote, myso_rewards)
}
</code></pre>



</details>

<a name="orderbook_pool_create_pool_admin"></a>

## Function `create_pool_admin`

Create a new pool. The pool is registered in the registry.
Checks are performed to ensure the tick size, lot size, and min size are
valid.
Returns the id of the pool created


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_create_pool_admin">create_pool_admin</a>&lt;BaseAsset, QuoteAsset&gt;(<a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> <a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>, tick_size: u64, lot_size: u64, min_size: u64, whitelisted_pool: bool, <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>: bool, _cap: &<a href="../orderbook/registry.md#orderbook_registry_OrderbookAdminCap">orderbook::registry::OrderbookAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_create_pool_admin">create_pool_admin</a>&lt;BaseAsset, QuoteAsset&gt;(
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> Registry,
    tick_size: u64,
    lot_size: u64,
    min_size: u64,
    whitelisted_pool: bool,
    <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>: bool,
    _cap: &OrderbookAdminCap,
    ctx: &<b>mut</b> TxContext,
): ID {
    <b>let</b> creation_fee = coin::zero(ctx);
    <a href="../orderbook/pool.md#orderbook_pool_create_pool">create_pool</a>&lt;BaseAsset, QuoteAsset&gt;(
        <a href="../orderbook/registry.md#orderbook_registry">registry</a>,
        tick_size,
        lot_size,
        min_size,
        creation_fee,
        whitelisted_pool,
        <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>,
        ctx,
    )
}
</code></pre>



</details>

<a name="orderbook_pool_unregister_pool_admin"></a>

## Function `unregister_pool_admin`

Unregister a pool in case it needs to be redeployed.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_unregister_pool_admin">unregister_pool_admin</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> <a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>, _cap: &<a href="../orderbook/registry.md#orderbook_registry_OrderbookAdminCap">orderbook::registry::OrderbookAdminCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_unregister_pool_admin">unregister_pool_admin</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> Registry,
    _cap: &OrderbookAdminCap,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>assert</b>!(self.<a href="../orderbook/pool.md#orderbook_pool_registered_pool">registered_pool</a>, <a href="../orderbook/pool.md#orderbook_pool_EPoolNotRegistered">EPoolNotRegistered</a>);
    self.<a href="../orderbook/pool.md#orderbook_pool_registered_pool">registered_pool</a> = <b>false</b>;
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>.unregister_pool&lt;BaseAsset, QuoteAsset&gt;();
}
</code></pre>



</details>

<a name="orderbook_pool_update_allowed_versions"></a>

## Function `update_allowed_versions`

Takes the registry and updates the allowed version within pool
Only admin can update the allowed versions
This function does not have version restrictions


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_allowed_versions">update_allowed_versions</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>, _cap: &<a href="../orderbook/registry.md#orderbook_registry_OrderbookAdminCap">orderbook::registry::OrderbookAdminCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_allowed_versions">update_allowed_versions</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &Registry,
    _cap: &OrderbookAdminCap,
) {
    <b>let</b> allowed_versions = <a href="../orderbook/registry.md#orderbook_registry">registry</a>.allowed_versions();
    <b>let</b> inner: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;BaseAsset, QuoteAsset&gt; = self.inner.load_value_mut();
    inner.allowed_versions = allowed_versions;
}
</code></pre>



</details>

<a name="orderbook_pool_update_pool_allowed_versions"></a>

## Function `update_pool_allowed_versions`

Takes the registry and updates the allowed version within pool
Permissionless equivalent of <code><a href="../orderbook/pool.md#orderbook_pool_update_allowed_versions">update_allowed_versions</a></code>
This function does not have version restrictions


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_pool_allowed_versions">update_pool_allowed_versions</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_pool_allowed_versions">update_pool_allowed_versions</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &Registry,
) {
    <b>let</b> allowed_versions = <a href="../orderbook/registry.md#orderbook_registry">registry</a>.allowed_versions();
    <b>let</b> inner: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;BaseAsset, QuoteAsset&gt; = self.inner.load_value_mut();
    inner.allowed_versions = allowed_versions;
}
</code></pre>



</details>

<a name="orderbook_pool_adjust_tick_size_admin"></a>

## Function `adjust_tick_size_admin`

Adjust the tick size of the pool. Only admin can adjust the tick size.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_adjust_tick_size_admin">adjust_tick_size_admin</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, new_tick_size: u64, _cap: &<a href="../orderbook/registry.md#orderbook_registry_OrderbookAdminCap">orderbook::registry::OrderbookAdminCap</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_adjust_tick_size_admin">adjust_tick_size_admin</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    new_tick_size: u64,
    _cap: &OrderbookAdminCap,
    clock: &Clock,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>assert</b>!(new_tick_size &gt; 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidTickSize">EInvalidTickSize</a>);
    <b>assert</b>!(<a href="../orderbook/math.md#orderbook_math_is_power_of_ten">math::is_power_of_ten</a>(new_tick_size), <a href="../orderbook/pool.md#orderbook_pool_EInvalidTickSize">EInvalidTickSize</a>);
    self.<a href="../orderbook/book.md#orderbook_book">book</a>.set_tick_size(new_tick_size);
    event::emit(<a href="../orderbook/pool.md#orderbook_pool_BookParamsUpdated">BookParamsUpdated</a>&lt;BaseAsset, QuoteAsset&gt; {
        pool_id: self.pool_id,
        tick_size: self.<a href="../orderbook/book.md#orderbook_book">book</a>.tick_size(),
        lot_size: self.<a href="../orderbook/book.md#orderbook_book">book</a>.lot_size(),
        min_size: self.<a href="../orderbook/book.md#orderbook_book">book</a>.min_size(),
        timestamp: clock.timestamp_ms(),
    });
}
</code></pre>



</details>

<a name="orderbook_pool_adjust_min_lot_size_admin"></a>

## Function `adjust_min_lot_size_admin`

Adjust and lot size and min size of the pool. New lot size must be smaller
than current lot size. Only admin can adjust the min size and lot size.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_adjust_min_lot_size_admin">adjust_min_lot_size_admin</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, new_lot_size: u64, new_min_size: u64, _cap: &<a href="../orderbook/registry.md#orderbook_registry_OrderbookAdminCap">orderbook::registry::OrderbookAdminCap</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_adjust_min_lot_size_admin">adjust_min_lot_size_admin</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    new_lot_size: u64,
    new_min_size: u64,
    _cap: &OrderbookAdminCap,
    clock: &Clock,
) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> lot_size = self.<a href="../orderbook/book.md#orderbook_book">book</a>.lot_size();
    <b>assert</b>!(new_lot_size &gt; 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidLotSize">EInvalidLotSize</a>);
    <b>assert</b>!(lot_size % new_lot_size == 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidLotSize">EInvalidLotSize</a>);
    <b>assert</b>!(<a href="../orderbook/math.md#orderbook_math_is_power_of_ten">math::is_power_of_ten</a>(new_lot_size), <a href="../orderbook/pool.md#orderbook_pool_EInvalidLotSize">EInvalidLotSize</a>);
    <b>assert</b>!(new_min_size &gt; 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidMinSize">EInvalidMinSize</a>);
    <b>assert</b>!(new_min_size % new_lot_size == 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidMinSize">EInvalidMinSize</a>);
    <b>assert</b>!(<a href="../orderbook/math.md#orderbook_math_is_power_of_ten">math::is_power_of_ten</a>(new_min_size), <a href="../orderbook/pool.md#orderbook_pool_EInvalidMinSize">EInvalidMinSize</a>);
    self.<a href="../orderbook/book.md#orderbook_book">book</a>.set_lot_size(new_lot_size);
    self.<a href="../orderbook/book.md#orderbook_book">book</a>.set_min_size(new_min_size);
    event::emit(<a href="../orderbook/pool.md#orderbook_pool_BookParamsUpdated">BookParamsUpdated</a>&lt;BaseAsset, QuoteAsset&gt; {
        pool_id: self.pool_id,
        tick_size: self.<a href="../orderbook/book.md#orderbook_book">book</a>.tick_size(),
        lot_size: self.<a href="../orderbook/book.md#orderbook_book">book</a>.lot_size(),
        min_size: self.<a href="../orderbook/book.md#orderbook_book">book</a>.min_size(),
        timestamp: clock.timestamp_ms(),
    });
}
</code></pre>



</details>

<a name="orderbook_pool_enable_ewma_state"></a>

## Function `enable_ewma_state`

Enable the EWMA state for the pool. This allows the pool to use
the EWMA state for volatility calculations and additional taker fees.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_enable_ewma_state">enable_ewma_state</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, _cap: &<a href="../orderbook/registry.md#orderbook_registry_OrderbookAdminCap">orderbook::registry::OrderbookAdminCap</a>, enable: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_enable_ewma_state">enable_ewma_state</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    _cap: &OrderbookAdminCap,
    enable: bool,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> _ = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> ewma_state = self.<a href="../orderbook/pool.md#orderbook_pool_update_ewma_state">update_ewma_state</a>(clock, ctx);
    <b>if</b> (enable) {
        ewma_state.enable();
    } <b>else</b> {
        ewma_state.disable();
    }
}
</code></pre>



</details>

<a name="orderbook_pool_set_ewma_params"></a>

## Function `set_ewma_params`

Set the EWMA parameters for the pool.
Only admin can set the parameters.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_set_ewma_params">set_ewma_params</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, _cap: &<a href="../orderbook/registry.md#orderbook_registry_OrderbookAdminCap">orderbook::registry::OrderbookAdminCap</a>, alpha: u64, z_score_threshold: u64, additional_taker_fee: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_set_ewma_params">set_ewma_params</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    _cap: &OrderbookAdminCap,
    alpha: u64,
    z_score_threshold: u64,
    additional_taker_fee: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(alpha &lt;= <a href="../orderbook/constants.md#orderbook_constants_max_ewma_alpha">constants::max_ewma_alpha</a>(), <a href="../orderbook/pool.md#orderbook_pool_EInvalidEWMAAlpha">EInvalidEWMAAlpha</a>);
    <b>assert</b>!(z_score_threshold &lt;= <a href="../orderbook/constants.md#orderbook_constants_max_z_score_threshold">constants::max_z_score_threshold</a>(), <a href="../orderbook/pool.md#orderbook_pool_EInvalidZScoreThreshold">EInvalidZScoreThreshold</a>);
    <b>assert</b>!(
        additional_taker_fee &lt;= <a href="../orderbook/constants.md#orderbook_constants_max_additional_taker_fee">constants::max_additional_taker_fee</a>(),
        <a href="../orderbook/pool.md#orderbook_pool_EInvalidAdditionalTakerFee">EInvalidAdditionalTakerFee</a>,
    );
    <b>let</b> _ = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
    <b>let</b> ewma_state = self.<a href="../orderbook/pool.md#orderbook_pool_update_ewma_state">update_ewma_state</a>(clock, ctx);
    ewma_state.set_alpha(alpha);
    ewma_state.set_z_score_threshold(z_score_threshold);
    ewma_state.set_additional_taker_fee(additional_taker_fee);
}
</code></pre>



</details>

<a name="orderbook_pool_whitelisted"></a>

## Function `whitelisted`

Accessor to check if the pool is whitelisted.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;): bool {
    self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>()
}
</code></pre>



</details>

<a name="orderbook_pool_stable_pool"></a>

## Function `stable_pool`

Accessor to check if the pool is a stablecoin pool.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;): bool {
    self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().stable()
}
</code></pre>



</details>

<a name="orderbook_pool_registered_pool"></a>

## Function `registered_pool`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_registered_pool">registered_pool</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_registered_pool">registered_pool</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;): bool {
    self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/pool.md#orderbook_pool_registered_pool">registered_pool</a>
}
</code></pre>



</details>

<a name="orderbook_pool_get_quote_quantity_out"></a>

## Function `get_quote_quantity_out`

Dry run to determine the quote quantity out for a given base quantity.
Uses MYSO token as fee.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quote_quantity_out">get_quote_quantity_out</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, base_quantity: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quote_quantity_out">get_quote_quantity_out</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    base_quantity: u64,
    clock: &Clock,
): (u64, u64, u64) {
    self.<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out">get_quantity_out</a>(base_quantity, 0, clock)
}
</code></pre>



</details>

<a name="orderbook_pool_get_base_quantity_out"></a>

## Function `get_base_quantity_out`

Dry run to determine the base quantity out for a given quote quantity.
Uses MYSO token as fee.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_base_quantity_out">get_base_quantity_out</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, quote_quantity: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_base_quantity_out">get_base_quantity_out</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    quote_quantity: u64,
    clock: &Clock,
): (u64, u64, u64) {
    self.<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out">get_quantity_out</a>(0, quote_quantity, clock)
}
</code></pre>



</details>

<a name="orderbook_pool_get_quote_quantity_out_input_fee"></a>

## Function `get_quote_quantity_out_input_fee`

Dry run to determine the quote quantity out for a given base quantity.
Uses input token as fee.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quote_quantity_out_input_fee">get_quote_quantity_out_input_fee</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, base_quantity: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quote_quantity_out_input_fee">get_quote_quantity_out_input_fee</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    base_quantity: u64,
    clock: &Clock,
): (u64, u64, u64) {
    self.<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out_input_fee">get_quantity_out_input_fee</a>(base_quantity, 0, clock)
}
</code></pre>



</details>

<a name="orderbook_pool_get_base_quantity_out_input_fee"></a>

## Function `get_base_quantity_out_input_fee`

Dry run to determine the base quantity out for a given quote quantity.
Uses input token as fee.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_base_quantity_out_input_fee">get_base_quantity_out_input_fee</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, quote_quantity: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_base_quantity_out_input_fee">get_base_quantity_out_input_fee</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    quote_quantity: u64,
    clock: &Clock,
): (u64, u64, u64) {
    self.<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out_input_fee">get_quantity_out_input_fee</a>(0, quote_quantity, clock)
}
</code></pre>



</details>

<a name="orderbook_pool_get_quantity_out"></a>

## Function `get_quantity_out`

Dry run to determine the quantity out for a given base or quote quantity.
Only one out of base or quote quantity should be non-zero.
Returns the (base_quantity_out, quote_quantity_out, myso_quantity_required)
Uses MYSO token as fee.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quantity_out">get_quantity_out</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, base_quantity: u64, quote_quantity: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quantity_out">get_quantity_out</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    base_quantity: u64,
    quote_quantity: u64,
    clock: &Clock,
): (u64, u64, u64) {
    <b>let</b> whitelist = self.<a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>();
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> params = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>();
    <b>let</b> taker_fee = params.taker_fee();
    <b>let</b> <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a> = self.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.<a href="../orderbook/pool.md#orderbook_pool_get_order_myso_price">get_order_myso_price</a>(whitelist);
    self
        .<a href="../orderbook/book.md#orderbook_book">book</a>
        .<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out">get_quantity_out</a>(
            base_quantity,
            quote_quantity,
            taker_fee,
            <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>,
            self.<a href="../orderbook/book.md#orderbook_book">book</a>.lot_size(),
            <b>true</b>,
            clock.timestamp_ms(),
        )
}
</code></pre>



</details>

<a name="orderbook_pool_get_quantity_out_input_fee"></a>

## Function `get_quantity_out_input_fee`

Dry run to determine the quantity out for a given base or quote quantity.
Only one out of base or quote quantity should be non-zero.
Returns the (base_quantity_out, quote_quantity_out, myso_quantity_required)
Uses input token as fee.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quantity_out_input_fee">get_quantity_out_input_fee</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, base_quantity: u64, quote_quantity: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quantity_out_input_fee">get_quantity_out_input_fee</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    base_quantity: u64,
    quote_quantity: u64,
    clock: &Clock,
): (u64, u64, u64) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> params = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>();
    <b>let</b> taker_fee = params.taker_fee();
    <b>let</b> <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a> = self.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.empty_myso_price();
    self
        .<a href="../orderbook/book.md#orderbook_book">book</a>
        .<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out">get_quantity_out</a>(
            base_quantity,
            quote_quantity,
            taker_fee,
            <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>,
            self.<a href="../orderbook/book.md#orderbook_book">book</a>.lot_size(),
            <b>false</b>,
            clock.timestamp_ms(),
        )
}
</code></pre>



</details>

<a name="orderbook_pool_get_base_quantity_in"></a>

## Function `get_base_quantity_in`

Dry run to determine the base quantity needed to sell to receive a target quote quantity.
Returns (base_quantity_in, actual_quote_quantity_out, myso_quantity_required)
Returns (0, 0, 0) if insufficient liquidity or if result would be below min_size.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_base_quantity_in">get_base_quantity_in</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, target_quote_quantity: u64, pay_with_myso: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_base_quantity_in">get_base_quantity_in</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    target_quote_quantity: u64,
    pay_with_myso: bool,
    clock: &Clock,
): (u64, u64, u64) {
    <b>let</b> whitelist = self.<a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>();
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> params = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>();
    <b>let</b> taker_fee = params.taker_fee();
    <b>let</b> <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a> = <b>if</b> (pay_with_myso) {
        self.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.<a href="../orderbook/pool.md#orderbook_pool_get_order_myso_price">get_order_myso_price</a>(whitelist)
    } <b>else</b> {
        self.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.empty_myso_price()
    };
    self
        .<a href="../orderbook/book.md#orderbook_book">book</a>
        .<a href="../orderbook/pool.md#orderbook_pool_get_base_quantity_in">get_base_quantity_in</a>(
            target_quote_quantity,
            taker_fee,
            <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>,
            pay_with_myso,
            clock.timestamp_ms(),
        )
}
</code></pre>



</details>

<a name="orderbook_pool_get_quote_quantity_in"></a>

## Function `get_quote_quantity_in`

Dry run to determine the quote quantity needed to buy a target base quantity.
Returns (actual_base_quantity_out, quote_quantity_in, myso_quantity_required)
Returns (0, 0, 0) if insufficient liquidity or if result would be below min_size.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quote_quantity_in">get_quote_quantity_in</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, target_base_quantity: u64, pay_with_myso: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_quote_quantity_in">get_quote_quantity_in</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    target_base_quantity: u64,
    pay_with_myso: bool,
    clock: &Clock,
): (u64, u64, u64) {
    <b>let</b> whitelist = self.<a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>();
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> params = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>();
    <b>let</b> taker_fee = params.taker_fee();
    <b>let</b> <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a> = <b>if</b> (pay_with_myso) {
        self.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.<a href="../orderbook/pool.md#orderbook_pool_get_order_myso_price">get_order_myso_price</a>(whitelist)
    } <b>else</b> {
        self.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.empty_myso_price()
    };
    self
        .<a href="../orderbook/book.md#orderbook_book">book</a>
        .<a href="../orderbook/pool.md#orderbook_pool_get_quote_quantity_in">get_quote_quantity_in</a>(
            target_base_quantity,
            taker_fee,
            <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>,
            pay_with_myso,
            clock.timestamp_ms(),
        )
}
</code></pre>



</details>

<a name="orderbook_pool_mid_price"></a>

## Function `mid_price`

Returns the mid price of the pool.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_mid_price">mid_price</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_mid_price">mid_price</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    clock: &Clock,
): u64 {
    self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/book.md#orderbook_book">book</a>.<a href="../orderbook/pool.md#orderbook_pool_mid_price">mid_price</a>(clock.timestamp_ms())
}
</code></pre>



</details>

<a name="orderbook_pool_account_open_orders"></a>

## Function `account_open_orders`

Returns the order_id for all open order for the balance_manager in the pool.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_account_open_orders">account_open_orders</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>): <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;u128&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_account_open_orders">account_open_orders</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &BalanceManager,
): VecSet&lt;u128&gt; {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>if</b> (!self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/pool.md#orderbook_pool_account_exists">account_exists</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>())) {
        <b>return</b> vec_set::empty()
    };
    self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/account.md#orderbook_account">account</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>()).open_orders()
}
</code></pre>



</details>

<a name="orderbook_pool_get_level2_range"></a>

## Function `get_level2_range`

Returns the (price_vec, quantity_vec) for the level2 order book.
The price_low and price_high are inclusive, all orders within the range are
returned.
is_bid is true for bids and false for asks.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_level2_range">get_level2_range</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, price_low: u64, price_high: u64, is_bid: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (vector&lt;u64&gt;, vector&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_level2_range">get_level2_range</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    price_low: u64,
    price_high: u64,
    is_bid: bool,
    clock: &Clock,
): (vector&lt;u64&gt;, vector&lt;u64&gt;) {
    self
        .<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>()
        .<a href="../orderbook/book.md#orderbook_book">book</a>
        .get_level2_range_and_ticks(
            price_low,
            price_high,
            <a href="../orderbook/constants.md#orderbook_constants_max_u64">constants::max_u64</a>(),
            is_bid,
            clock.timestamp_ms(),
        )
}
</code></pre>



</details>

<a name="orderbook_pool_get_level2_ticks_from_mid"></a>

## Function `get_level2_ticks_from_mid`

Returns the (price_vec, quantity_vec) for the level2 order book.
Ticks are the maximum number of ticks to return starting from best bid and
best ask.
(bid_price, bid_quantity, ask_price, ask_quantity) are returned as 4
vectors.
The price vectors are sorted in descending order for bids and ascending
order for asks.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_level2_ticks_from_mid">get_level2_ticks_from_mid</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, ticks: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (vector&lt;u64&gt;, vector&lt;u64&gt;, vector&lt;u64&gt;, vector&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_level2_ticks_from_mid">get_level2_ticks_from_mid</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    ticks: u64,
    clock: &Clock,
): (vector&lt;u64&gt;, vector&lt;u64&gt;, vector&lt;u64&gt;, vector&lt;u64&gt;) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> (bid_price, bid_quantity) = self
        .<a href="../orderbook/book.md#orderbook_book">book</a>
        .get_level2_range_and_ticks(
            <a href="../orderbook/constants.md#orderbook_constants_min_price">constants::min_price</a>(),
            <a href="../orderbook/constants.md#orderbook_constants_max_price">constants::max_price</a>(),
            ticks,
            <b>true</b>,
            clock.timestamp_ms(),
        );
    <b>let</b> (ask_price, ask_quantity) = self
        .<a href="../orderbook/book.md#orderbook_book">book</a>
        .get_level2_range_and_ticks(
            <a href="../orderbook/constants.md#orderbook_constants_min_price">constants::min_price</a>(),
            <a href="../orderbook/constants.md#orderbook_constants_max_price">constants::max_price</a>(),
            ticks,
            <b>false</b>,
            clock.timestamp_ms(),
        );
    (bid_price, bid_quantity, ask_price, ask_quantity)
}
</code></pre>



</details>

<a name="orderbook_pool_vault_balances"></a>

## Function `vault_balances`

Get all balances held in this pool.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_vault_balances">vault_balances</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_vault_balances">vault_balances</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
): (u64, u64, u64) {
    self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/vault.md#orderbook_vault">vault</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>()
}
</code></pre>



</details>

<a name="orderbook_pool_get_pool_id_by_asset"></a>

## Function `get_pool_id_by_asset`

Get the ID of the pool given the asset types.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_pool_id_by_asset">get_pool_id_by_asset</a>&lt;BaseAsset, QuoteAsset&gt;(<a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_pool_id_by_asset">get_pool_id_by_asset</a>&lt;BaseAsset, QuoteAsset&gt;(<a href="../orderbook/registry.md#orderbook_registry">registry</a>: &Registry): ID {
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>.get_pool_id&lt;BaseAsset, QuoteAsset&gt;()
}
</code></pre>



</details>

<a name="orderbook_pool_get_order"></a>

## Function `get_order`

Get the Order struct


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_order">get_order</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, order_id: u128): <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_order">get_order</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    order_id: u128,
): Order {
    self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/book.md#orderbook_book">book</a>.<a href="../orderbook/pool.md#orderbook_pool_get_order">get_order</a>(order_id)
}
</code></pre>



</details>

<a name="orderbook_pool_get_orders"></a>

## Function `get_orders`

Get multiple orders given a vector of order_ids.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_orders">get_orders</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, order_ids: vector&lt;u128&gt;): vector&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_orders">get_orders</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    order_ids: vector&lt;u128&gt;,
): vector&lt;Order&gt; {
    <b>let</b> <b>mut</b> orders = vector[];
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> num_orders = order_ids.length();
    <b>while</b> (i &lt; num_orders) {
        <b>let</b> order_id = order_ids[i];
        orders.push_back(self.<a href="../orderbook/pool.md#orderbook_pool_get_order">get_order</a>(order_id));
        i = i + 1;
    };
    orders
}
</code></pre>



</details>

<a name="orderbook_pool_get_account_order_details"></a>

## Function `get_account_order_details`

Return a copy of all orders that are in the book for this account.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_account_order_details">get_account_order_details</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>): vector&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_account_order_details">get_account_order_details</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &BalanceManager,
): vector&lt;Order&gt; {
    <b>let</b> acct_open_orders = self.<a href="../orderbook/pool.md#orderbook_pool_account_open_orders">account_open_orders</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>).into_keys();
    self.<a href="../orderbook/pool.md#orderbook_pool_get_orders">get_orders</a>(acct_open_orders)
}
</code></pre>



</details>

<a name="orderbook_pool_get_order_myso_price"></a>

## Function `get_order_myso_price`

Return the MYSO price for the pool.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_order_myso_price">get_order_myso_price</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_order_myso_price">get_order_myso_price</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
): OrderMySoPrice {
    <b>let</b> whitelist = self.<a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>();
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    self.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.<a href="../orderbook/pool.md#orderbook_pool_get_order_myso_price">get_order_myso_price</a>(whitelist)
}
</code></pre>



</details>

<a name="orderbook_pool_get_order_myso_required"></a>

## Function `get_order_myso_required`

Returns the myso required for an order if it's taker or maker given quantity
and price
Does not account for discounted taker fees
Returns (myso_required_taker, myso_required_maker)


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_order_myso_required">get_order_myso_required</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, base_quantity: u64, price: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_order_myso_required">get_order_myso_required</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    base_quantity: u64,
    price: u64,
): (u64, u64) {
    <b>let</b> order_myso_price = self.<a href="../orderbook/pool.md#orderbook_pool_get_order_myso_price">get_order_myso_price</a>();
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> maker_fee = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>().maker_fee();
    <b>let</b> taker_fee = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>().taker_fee();
    <b>let</b> myso_quantity = order_myso_price
        .fee_quantity(
            base_quantity,
            <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(base_quantity, price),
            <b>true</b>,
        )
        .myso();
    (<a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(taker_fee, myso_quantity), <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(maker_fee, myso_quantity))
}
</code></pre>



</details>

<a name="orderbook_pool_locked_balance"></a>

## Function `locked_balance`

Returns the locked balance for the balance_manager in the pool
Returns (base_quantity, quote_quantity, myso_quantity)


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_locked_balance">locked_balance</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_locked_balance">locked_balance</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &BalanceManager,
): (u64, u64, u64) {
    <b>let</b> account_orders = self.<a href="../orderbook/pool.md#orderbook_pool_get_account_order_details">get_account_order_details</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>);
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>if</b> (!self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/pool.md#orderbook_pool_account_exists">account_exists</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>())) {
        <b>return</b> (0, 0, 0)
    };
    <b>let</b> <b>mut</b> base_quantity = 0;
    <b>let</b> <b>mut</b> quote_quantity = 0;
    <b>let</b> <b>mut</b> myso_quantity = 0;
    account_orders.do_ref!(|<a href="../orderbook/order.md#orderbook_order">order</a>| {
        <b>let</b> maker_fee = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/history.md#orderbook_history">history</a>().historic_maker_fee(<a href="../orderbook/order.md#orderbook_order">order</a>.epoch());
        <b>let</b> <a href="../orderbook/pool.md#orderbook_pool_locked_balance">locked_balance</a> = <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/pool.md#orderbook_pool_locked_balance">locked_balance</a>(maker_fee);
        base_quantity = base_quantity + <a href="../orderbook/pool.md#orderbook_pool_locked_balance">locked_balance</a>.base();
        quote_quantity = quote_quantity + <a href="../orderbook/pool.md#orderbook_pool_locked_balance">locked_balance</a>.quote();
        myso_quantity = myso_quantity + <a href="../orderbook/pool.md#orderbook_pool_locked_balance">locked_balance</a>.myso();
    });
    <b>let</b> settled_balances = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/account.md#orderbook_account">account</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>()).settled_balances();
    base_quantity = base_quantity + settled_balances.base();
    quote_quantity = quote_quantity + settled_balances.quote();
    myso_quantity = myso_quantity + settled_balances.myso();
    (base_quantity, quote_quantity, myso_quantity)
}
</code></pre>



</details>

<a name="orderbook_pool_can_place_limit_order"></a>

## Function `can_place_limit_order`

Check if a limit order can be placed based on balance manager balances.
Returns true if the balance manager has sufficient balance (accounting for fees) to place the order, false otherwise.
Assumes the limit order is a taker order as a worst case scenario.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_can_place_limit_order">can_place_limit_order</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, price: u64, quantity: u64, is_bid: bool, pay_with_myso: bool, expire_timestamp: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_can_place_limit_order">can_place_limit_order</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &BalanceManager,
    price: u64,
    quantity: u64,
    is_bid: bool,
    pay_with_myso: bool,
    expire_timestamp: u64,
    clock: &Clock,
): bool {
    <b>let</b> whitelist = self.<a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>();
    <b>let</b> pool_inner = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>if</b> (
        !self.<a href="../orderbook/pool.md#orderbook_pool_check_limit_order_params">check_limit_order_params</a>(
            price,
            quantity,
            expire_timestamp,
            clock,
        )
    ) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> order_myso_price = <b>if</b> (pay_with_myso) {
        pool_inner.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.<a href="../orderbook/pool.md#orderbook_pool_get_order_myso_price">get_order_myso_price</a>(whitelist)
    } <b>else</b> {
        pool_inner.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.empty_myso_price()
    };
    <b>let</b> quote_quantity = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(quantity, price);
    // Calculate fee quantity using taker fee (worst case <b>for</b> limit orders)
    <b>let</b> taker_fee = pool_inner.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>().taker_fee();
    <b>let</b> fee_balances = order_myso_price.fee_quantity(quantity, quote_quantity, is_bid);
    // Calculate required <a href="../orderbook/balances.md#orderbook_balances">balances</a>
    <b>let</b> <b>mut</b> required_base = 0;
    <b>let</b> <b>mut</b> required_quote = 0;
    <b>let</b> <b>mut</b> required_myso = 0;
    <b>if</b> (is_bid) {
        required_quote = quote_quantity;
        <b>if</b> (pay_with_myso) {
            required_myso = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(fee_balances.myso(), taker_fee);
        } <b>else</b> {
            <b>let</b> fee_quote = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(fee_balances.quote(), taker_fee);
            required_quote = required_quote + fee_quote;
        };
    } <b>else</b> {
        required_base = quantity;
        <b>if</b> (pay_with_myso) {
            required_myso = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(fee_balances.myso(), taker_fee);
        } <b>else</b> {
            <b>let</b> fee_base = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(fee_balances.base(), taker_fee);
            required_base = required_base + fee_base;
        };
    };
    // Get current <a href="../orderbook/balances.md#orderbook_balances">balances</a> from balance manager. Accounts <b>for</b> settled <a href="../orderbook/balances.md#orderbook_balances">balances</a>.
    <b>let</b> settled_balances = <b>if</b> (!self.<a href="../orderbook/pool.md#orderbook_pool_account_exists">account_exists</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>)) {
        <a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>()
    } <b>else</b> {
        self.<a href="../orderbook/account.md#orderbook_account">account</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>).settled_balances()
    };
    <b>let</b> available_base = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.balance&lt;BaseAsset&gt;() + settled_balances.base();
    <b>let</b> available_quote = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.balance&lt;QuoteAsset&gt;() + settled_balances.quote();
    <b>let</b> available_myso = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.balance&lt;MYSO&gt;() + settled_balances.myso();
    // Check <b>if</b> available <a href="../orderbook/balances.md#orderbook_balances">balances</a> are sufficient
    (available_base &gt;= required_base) && (available_quote &gt;= required_quote) && (available_myso &gt;= required_myso)
}
</code></pre>



</details>

<a name="orderbook_pool_can_place_market_order"></a>

## Function `can_place_market_order`

Check if a market order can be placed based on balance manager balances.
Returns true if the balance manager has sufficient balance (accounting for fees) to place the order, false otherwise.
Does not account for discounted taker fees


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_can_place_market_order">can_place_market_order</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, quantity: u64, is_bid: bool, pay_with_myso: bool, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_can_place_market_order">can_place_market_order</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &BalanceManager,
    quantity: u64,
    is_bid: bool,
    pay_with_myso: bool,
    clock: &Clock,
): bool {
    // Validate <a href="../orderbook/order.md#orderbook_order">order</a> parameters against <a href="../orderbook/pool.md#orderbook_pool">pool</a> <a href="../orderbook/book.md#orderbook_book">book</a> params
    <b>if</b> (!self.<a href="../orderbook/pool.md#orderbook_pool_check_market_order_params">check_market_order_params</a>(quantity)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> <b>mut</b> required_base = 0;
    <b>let</b> <b>mut</b> required_myso = 0;
    // Get current <a href="../orderbook/balances.md#orderbook_balances">balances</a> from balance manager. Accounts <b>for</b> settled <a href="../orderbook/balances.md#orderbook_balances">balances</a>.
    <b>let</b> settled_balances = <b>if</b> (!self.<a href="../orderbook/pool.md#orderbook_pool_account_exists">account_exists</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>)) {
        <a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>()
    } <b>else</b> {
        self.<a href="../orderbook/account.md#orderbook_account">account</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>).settled_balances()
    };
    <b>let</b> available_base = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.balance&lt;BaseAsset&gt;() + settled_balances.base();
    <b>let</b> available_quote = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.balance&lt;QuoteAsset&gt;() + settled_balances.quote();
    <b>let</b> available_myso = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.balance&lt;MYSO&gt;() + settled_balances.myso();
    <b>if</b> (is_bid) {
        // For bid orders: calculate quote needed to acquire desired base quantity
        // <a href="../orderbook/pool.md#orderbook_pool_get_quote_quantity_in">get_quote_quantity_in</a> returns (base_out, quote_needed, myso_required)
        <b>let</b> (base_out, quote_needed, myso_required) = self.<a href="../orderbook/pool.md#orderbook_pool_get_quote_quantity_in">get_quote_quantity_in</a>(
            quantity,
            pay_with_myso,
            clock,
        );
        // Not enough liquidity or available quote <b>for</b> the base quantity
        <b>if</b> (base_out &lt; quantity || available_quote &lt; quote_needed) {
            <b>return</b> <b>false</b>
        };
        <b>if</b> (pay_with_myso) {
            required_myso = myso_required;
        };
    } <b>else</b> {
        // For ask orders: <b>if</b> paying fees in input token (base), need quantity + fees
        // <a href="../orderbook/pool.md#orderbook_pool_get_quantity_out_input_fee">get_quantity_out_input_fee</a> accounts <b>for</b> fees, so we need to check <b>if</b> we have enough base
        // including fees that will be deducted
        <b>let</b> (_, _, myso_required) = <b>if</b> (pay_with_myso) {
            self.<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out">get_quantity_out</a>(quantity, 0, clock)
        } <b>else</b> {
            self.<a href="../orderbook/pool.md#orderbook_pool_get_quantity_out_input_fee">get_quantity_out_input_fee</a>(quantity, 0, clock)
        };
        // If paying fees in base asset, need quantity + fees
        required_base = <b>if</b> (pay_with_myso) {
            quantity
        } <b>else</b> {
            // Fees are deducted from base, so need more base to <a href="../orderbook/account.md#orderbook_account">account</a> <b>for</b> fees
            <b>let</b> (taker_fee, _, _) = self.<a href="../orderbook/pool.md#orderbook_pool_pool_trade_params">pool_trade_params</a>();
            <b>let</b> input_fee_rate = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(taker_fee, <a href="../orderbook/constants.md#orderbook_constants_fee_penalty_multiplier">constants::fee_penalty_multiplier</a>());
            <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(quantity, <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>() + input_fee_rate)
        };
        <b>if</b> (pay_with_myso) {
            required_myso = myso_required;
        };
    };
    // Check <b>if</b> available <a href="../orderbook/balances.md#orderbook_balances">balances</a> are sufficient
    (available_base &gt;= required_base) && (available_myso &gt;= required_myso)
}
</code></pre>



</details>

<a name="orderbook_pool_check_market_order_params"></a>

## Function `check_market_order_params`

Check if a market order can be placed based on pool book params.
Returns true if the order parameters are valid, false otherwise.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_check_market_order_params">check_market_order_params</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, quantity: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_check_market_order_params">check_market_order_params</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    quantity: u64,
): bool {
    <b>let</b> pool_inner = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    pool_inner.<a href="../orderbook/book.md#orderbook_book">book</a>.<a href="../orderbook/pool.md#orderbook_pool_check_market_order_params">check_market_order_params</a>(quantity)
}
</code></pre>



</details>

<a name="orderbook_pool_check_limit_order_params"></a>

## Function `check_limit_order_params`

Check if a limit order can be placed based on pool book params.
Returns true if the order parameters are valid, false otherwise.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_check_limit_order_params">check_limit_order_params</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, price: u64, quantity: u64, expire_timestamp: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_check_limit_order_params">check_limit_order_params</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    price: u64,
    quantity: u64,
    expire_timestamp: u64,
    clock: &Clock,
): bool {
    <b>let</b> pool_inner = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    pool_inner
        .<a href="../orderbook/book.md#orderbook_book">book</a>
        .<a href="../orderbook/pool.md#orderbook_pool_check_limit_order_params">check_limit_order_params</a>(price, quantity, expire_timestamp, clock.timestamp_ms())
}
</code></pre>



</details>

<a name="orderbook_pool_pool_trade_params"></a>

## Function `pool_trade_params`

Returns the trade params for the pool.
Returns (taker_fee, maker_fee, stake_required)


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_pool_trade_params">pool_trade_params</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_pool_trade_params">pool_trade_params</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
): (u64, u64, u64) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a> = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>();
    <b>let</b> taker_fee = <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.taker_fee();
    <b>let</b> maker_fee = <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.maker_fee();
    <b>let</b> stake_required = <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.stake_required();
    (taker_fee, maker_fee, stake_required)
}
</code></pre>



</details>

<a name="orderbook_pool_pool_trade_params_next"></a>

## Function `pool_trade_params_next`

Returns the currently leading trade params for the next epoch for the pool


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_pool_trade_params_next">pool_trade_params_next</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_pool_trade_params_next">pool_trade_params_next</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
): (u64, u64, u64) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a> = self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().next_trade_params();
    <b>let</b> taker_fee = <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.taker_fee();
    <b>let</b> maker_fee = <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.maker_fee();
    <b>let</b> stake_required = <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.stake_required();
    (taker_fee, maker_fee, stake_required)
}
</code></pre>



</details>

<a name="orderbook_pool_pool_book_params"></a>

## Function `pool_book_params`

Returns the tick size, lot size, and min size for the pool.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_pool_book_params">pool_book_params</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_pool_book_params">pool_book_params</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
): (u64, u64, u64) {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>let</b> tick_size = self.<a href="../orderbook/book.md#orderbook_book">book</a>.tick_size();
    <b>let</b> lot_size = self.<a href="../orderbook/book.md#orderbook_book">book</a>.lot_size();
    <b>let</b> min_size = self.<a href="../orderbook/book.md#orderbook_book">book</a>.min_size();
    (tick_size, lot_size, min_size)
}
</code></pre>



</details>

<a name="orderbook_pool_account_exists"></a>

## Function `account_exists`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_account_exists">account_exists</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_account_exists">account_exists</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &BalanceManager,
): bool {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/pool.md#orderbook_pool_account_exists">account_exists</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>())
}
</code></pre>



</details>

<a name="orderbook_pool_account"></a>

## Function `account`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account">account</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>): <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account">account</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &BalanceManager,
): Account {
    <b>let</b> self = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    *self.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/account.md#orderbook_account">account</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>())
}
</code></pre>



</details>

<a name="orderbook_pool_quorum"></a>

## Function `quorum`

Returns the quorum needed to pass proposal in the current epoch


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_quorum">quorum</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_quorum">quorum</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;): u64 {
    self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/pool.md#orderbook_pool_quorum">quorum</a>()
}
</code></pre>



</details>

<a name="orderbook_pool_id"></a>

## Function `id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_id">id</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_id">id</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;): ID {
    self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>().pool_id
}
</code></pre>



</details>

<a name="orderbook_pool_get_referral_balances"></a>

## Function `get_referral_balances`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_referral_balances">get_referral_balances</a>&lt;BaseAsset, QuoteAsset&gt;(_self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, _referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">orderbook::balance_manager::OrderbookReferral</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_referral_balances">get_referral_balances</a>&lt;BaseAsset, QuoteAsset&gt;(
    _self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    _referral: &OrderbookReferral,
): (u64, u64, u64) {
    <b>abort</b>
}
</code></pre>



</details>

<a name="orderbook_pool_get_pool_referral_balances"></a>

## Function `get_pool_referral_balances`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_pool_referral_balances">get_pool_referral_balances</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">orderbook::balance_manager::OrderbookPoolReferral</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_get_pool_referral_balances">get_pool_referral_balances</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    referral: &OrderbookPoolReferral,
): (u64, u64, u64) {
    <b>let</b> _ = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>assert</b>!(referral.balance_manager_referral_pool_id() == self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(), <a href="../orderbook/pool.md#orderbook_pool_EWrongPoolReferral">EWrongPoolReferral</a>);
    <b>let</b> referral_rewards: &<a href="../orderbook/pool.md#orderbook_pool_ReferralRewards">ReferralRewards</a>&lt;BaseAsset, QuoteAsset&gt; = self
        .<a href="../orderbook/pool.md#orderbook_pool_id">id</a>
        .borrow(object::id(referral));
    <b>let</b> base = referral_rewards.base.value();
    <b>let</b> quote = referral_rewards.quote.value();
    <b>let</b> myso_rewards = referral_rewards.myso.value();
    (base, quote, myso_rewards)
}
</code></pre>



</details>

<a name="orderbook_pool_pool_referral_multiplier"></a>

## Function `pool_referral_multiplier`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_pool_referral_multiplier">pool_referral_multiplier</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">orderbook::balance_manager::OrderbookPoolReferral</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_pool_referral_multiplier">pool_referral_multiplier</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    referral: &OrderbookPoolReferral,
): u64 {
    <b>let</b> _ = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>();
    <b>assert</b>!(referral.balance_manager_referral_pool_id() == self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(), <a href="../orderbook/pool.md#orderbook_pool_EWrongPoolReferral">EWrongPoolReferral</a>);
    <b>let</b> referral_rewards: &<a href="../orderbook/pool.md#orderbook_pool_ReferralRewards">ReferralRewards</a>&lt;BaseAsset, QuoteAsset&gt; = self
        .<a href="../orderbook/pool.md#orderbook_pool_id">id</a>
        .borrow(object::id(referral));
    referral_rewards.multiplier
}
</code></pre>



</details>

<a name="orderbook_pool_create_pool"></a>

## Function `create_pool`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_create_pool">create_pool</a>&lt;BaseAsset, QuoteAsset&gt;(<a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> <a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>, tick_size: u64, lot_size: u64, min_size: u64, creation_fee: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, whitelisted_pool: bool, <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_create_pool">create_pool</a>&lt;BaseAsset, QuoteAsset&gt;(
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> Registry,
    tick_size: u64,
    lot_size: u64,
    min_size: u64,
    creation_fee: Coin&lt;MYSO&gt;,
    whitelisted_pool: bool,
    <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>: bool,
    ctx: &<b>mut</b> TxContext,
): ID {
    <b>assert</b>!(tick_size &gt; 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidTickSize">EInvalidTickSize</a>);
    <b>assert</b>!(<a href="../orderbook/math.md#orderbook_math_is_power_of_ten">math::is_power_of_ten</a>(tick_size), <a href="../orderbook/pool.md#orderbook_pool_EInvalidTickSize">EInvalidTickSize</a>);
    <b>assert</b>!(lot_size &gt;= 1000, <a href="../orderbook/pool.md#orderbook_pool_EInvalidLotSize">EInvalidLotSize</a>);
    <b>assert</b>!(<a href="../orderbook/math.md#orderbook_math_is_power_of_ten">math::is_power_of_ten</a>(lot_size), <a href="../orderbook/pool.md#orderbook_pool_EInvalidLotSize">EInvalidLotSize</a>);
    <b>assert</b>!(min_size &gt; 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidMinSize">EInvalidMinSize</a>);
    <b>assert</b>!(min_size % lot_size == 0, <a href="../orderbook/pool.md#orderbook_pool_EInvalidMinSize">EInvalidMinSize</a>);
    <b>assert</b>!(<a href="../orderbook/math.md#orderbook_math_is_power_of_ten">math::is_power_of_ten</a>(min_size), <a href="../orderbook/pool.md#orderbook_pool_EInvalidMinSize">EInvalidMinSize</a>);
    <b>assert</b>!(!(whitelisted_pool && <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>), <a href="../orderbook/pool.md#orderbook_pool_EPoolCannotBeBothWhitelistedAndStable">EPoolCannotBeBothWhitelistedAndStable</a>);
    <b>assert</b>!(
        type_name::with_defining_ids&lt;BaseAsset&gt;() != type_name::with_defining_ids&lt;QuoteAsset&gt;(),
        <a href="../orderbook/pool.md#orderbook_pool_ESameBaseAndQuote">ESameBaseAndQuote</a>,
    );
    <b>let</b> pool_id = object::new(ctx);
    <b>let</b> pool_inner = <a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;BaseAsset, QuoteAsset&gt; {
        allowed_versions: <a href="../orderbook/registry.md#orderbook_registry">registry</a>.allowed_versions(),
        pool_id: pool_id.to_inner(),
        <a href="../orderbook/book.md#orderbook_book">book</a>: <a href="../orderbook/book.md#orderbook_book_empty">book::empty</a>(tick_size, lot_size, min_size, ctx),
        <a href="../orderbook/state.md#orderbook_state">state</a>: <a href="../orderbook/state.md#orderbook_state_empty">state::empty</a>(whitelisted_pool, <a href="../orderbook/pool.md#orderbook_pool_stable_pool">stable_pool</a>, ctx),
        <a href="../orderbook/vault.md#orderbook_vault">vault</a>: <a href="../orderbook/vault.md#orderbook_vault_empty">vault::empty</a>(),
        <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_empty">myso_price::empty</a>(),
        <a href="../orderbook/pool.md#orderbook_pool_registered_pool">registered_pool</a>: <b>true</b>,
    };
    <b>let</b> params = pool_inner.<a href="../orderbook/state.md#orderbook_state">state</a>.<a href="../orderbook/governance.md#orderbook_governance">governance</a>().<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>();
    <b>let</b> taker_fee = params.taker_fee();
    <b>let</b> maker_fee = params.maker_fee();
    <b>let</b> treasury_address = <a href="../orderbook/registry.md#orderbook_registry">registry</a>.treasury_address();
    <b>let</b> <a href="../orderbook/pool.md#orderbook_pool">pool</a> = <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt; {
        <a href="../orderbook/pool.md#orderbook_pool_id">id</a>: pool_id,
        inner: versioned::create(<a href="../orderbook/constants.md#orderbook_constants_current_version">constants::current_version</a>(), pool_inner, ctx),
    };
    <b>let</b> pool_id = object::id(&<a href="../orderbook/pool.md#orderbook_pool">pool</a>);
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>.register_pool&lt;BaseAsset, QuoteAsset&gt;(pool_id);
    event::emit(<a href="../orderbook/pool.md#orderbook_pool_PoolCreated">PoolCreated</a>&lt;BaseAsset, QuoteAsset&gt; {
        pool_id,
        taker_fee,
        maker_fee,
        tick_size,
        lot_size,
        min_size,
        whitelisted_pool,
        treasury_address,
    });
    transfer::public_transfer(creation_fee, treasury_address);
    transfer::share_object(<a href="../orderbook/pool.md#orderbook_pool">pool</a>);
    pool_id
}
</code></pre>



</details>

<a name="orderbook_pool_bids"></a>

## Function `bids`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_bids">bids</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_PoolInner">orderbook::pool::PoolInner</a>&lt;BaseAsset, QuoteAsset&gt;): &<a href="../orderbook/big_vector.md#orderbook_big_vector_BigVector">orderbook::big_vector::BigVector</a>&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_bids">bids</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;BaseAsset, QuoteAsset&gt;,
): &BigVector&lt;Order&gt; {
    self.<a href="../orderbook/book.md#orderbook_book">book</a>.<a href="../orderbook/pool.md#orderbook_pool_bids">bids</a>()
}
</code></pre>



</details>

<a name="orderbook_pool_asks"></a>

## Function `asks`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_asks">asks</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_PoolInner">orderbook::pool::PoolInner</a>&lt;BaseAsset, QuoteAsset&gt;): &<a href="../orderbook/big_vector.md#orderbook_big_vector_BigVector">orderbook::big_vector::BigVector</a>&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_asks">asks</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;BaseAsset, QuoteAsset&gt;,
): &BigVector&lt;Order&gt; {
    self.<a href="../orderbook/book.md#orderbook_book">book</a>.<a href="../orderbook/pool.md#orderbook_pool_asks">asks</a>()
}
</code></pre>



</details>

<a name="orderbook_pool_load_inner"></a>

## Function `load_inner`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): &<a href="../orderbook/pool.md#orderbook_pool_PoolInner">orderbook::pool::PoolInner</a>&lt;BaseAsset, QuoteAsset&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_load_inner">load_inner</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
): &<a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;BaseAsset, QuoteAsset&gt; {
    <b>let</b> inner: &<a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;BaseAsset, QuoteAsset&gt; = self.inner.load_value();
    <b>let</b> package_version = <a href="../orderbook/constants.md#orderbook_constants_current_version">constants::current_version</a>();
    <b>assert</b>!(inner.allowed_versions.contains(&package_version), <a href="../orderbook/pool.md#orderbook_pool_EPackageVersionDisabled">EPackageVersionDisabled</a>);
    inner
}
</code></pre>



</details>

<a name="orderbook_pool_load_inner_mut"></a>

## Function `load_inner_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_PoolInner">orderbook::pool::PoolInner</a>&lt;BaseAsset, QuoteAsset&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
): &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;BaseAsset, QuoteAsset&gt; {
    <b>let</b> inner: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_PoolInner">PoolInner</a>&lt;BaseAsset, QuoteAsset&gt; = self.inner.load_value_mut();
    <b>let</b> package_version = <a href="../orderbook/constants.md#orderbook_constants_current_version">constants::current_version</a>();
    <b>assert</b>!(inner.allowed_versions.contains(&package_version), <a href="../orderbook/pool.md#orderbook_pool_EPackageVersionDisabled">EPackageVersionDisabled</a>);
    inner
}
</code></pre>



</details>

<a name="orderbook_pool_load_ewma_state"></a>

## Function `load_ewma_state`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_load_ewma_state">load_ewma_state</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;): <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_load_ewma_state">load_ewma_state</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
): EWMAState {
    *self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>.borrow(<a href="../orderbook/constants.md#orderbook_constants_ewma_df_key">constants::ewma_df_key</a>())
}
</code></pre>



</details>

<a name="orderbook_pool_place_order_int"></a>

## Function `place_order_int`



<pre><code><b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_place_order_int">place_order_int</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, client_order_id: u64, order_type: u8, self_matching_option: u8, price: u64, quantity: u64, is_bid: bool, pay_with_myso: bool, expire_timestamp: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, market_order: bool, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_place_order_int">place_order_int</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
    client_order_id: u64,
    order_type: u8,
    self_matching_option: u8,
    price: u64,
    quantity: u64,
    is_bid: bool,
    pay_with_myso: bool,
    expire_timestamp: u64,
    clock: &Clock,
    market_order: bool,
    ctx: &TxContext,
): OrderInfo {
    <b>let</b> whitelist = self.<a href="../orderbook/pool.md#orderbook_pool_whitelisted">whitelisted</a>();
    self.<a href="../orderbook/pool.md#orderbook_pool_update_ewma_state">update_ewma_state</a>(clock, ctx);
    <b>let</b> ewma_state = self.<a href="../orderbook/pool.md#orderbook_pool_load_ewma_state">load_ewma_state</a>();
    <b>let</b> <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a> = {
        <b>let</b> pool_inner = self.<a href="../orderbook/pool.md#orderbook_pool_load_inner_mut">load_inner_mut</a>();
        <b>let</b> order_myso_price = <b>if</b> (pay_with_myso) {
            pool_inner.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.<a href="../orderbook/pool.md#orderbook_pool_get_order_myso_price">get_order_myso_price</a>(whitelist)
        } <b>else</b> {
            pool_inner.<a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.empty_myso_price()
        };
        <b>let</b> <b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a> = <a href="../orderbook/order_info.md#orderbook_order_info_new">order_info::new</a>(
            pool_inner.pool_id,
            <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(),
            client_order_id,
            ctx.sender(),
            order_type,
            self_matching_option,
            price,
            quantity,
            is_bid,
            pay_with_myso,
            ctx.epoch(),
            expire_timestamp,
            order_myso_price,
            market_order,
            clock.timestamp_ms(),
        );
        pool_inner.<a href="../orderbook/book.md#orderbook_book">book</a>.create_order(&<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>, clock.timestamp_ms());
        <b>let</b> (settled, owed) = pool_inner
            .<a href="../orderbook/state.md#orderbook_state">state</a>
            .process_create(
                &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>,
                &ewma_state,
                pool_inner.pool_id,
                ctx,
            );
        pool_inner.<a href="../orderbook/vault.md#orderbook_vault">vault</a>.settle_balance_manager(settled, owed, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, trade_proof);
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.emit_order_info();
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.emit_orders_filled(clock.timestamp_ms());
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.emit_order_fully_filled_if_filled(clock.timestamp_ms());
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>
    };
    self.<a href="../orderbook/pool.md#orderbook_pool_process_referral_fees">process_referral_fees</a>&lt;BaseAsset, QuoteAsset&gt;(
        &<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>,
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>,
        trade_proof,
    );
    <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>
}
</code></pre>



</details>

<a name="orderbook_pool_process_referral_fees"></a>

## Function `process_referral_fees`



<pre><code><b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_process_referral_fees">process_referral_fees</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_process_referral_fees">process_referral_fees</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &OrderInfo,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
) {
    <b>let</b> referral_id = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.get_balance_manager_referral_id(self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>());
    <b>if</b> (referral_id.is_some()) {
        <b>let</b> referral_id = referral_id.destroy_some();
        <b>let</b> referral_rewards: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_ReferralRewards">ReferralRewards</a>&lt;BaseAsset, QuoteAsset&gt; = self
            .<a href="../orderbook/pool.md#orderbook_pool_id">id</a>
            .borrow_mut(referral_id);
        <b>let</b> referral_multiplier = referral_rewards.multiplier;
        <b>let</b> referral_fee = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.paid_fees(), referral_multiplier);
        <b>if</b> (referral_fee == 0) {
            <b>return</b>
        };
        <b>let</b> <b>mut</b> base_fee = 0;
        <b>let</b> <b>mut</b> quote_fee = 0;
        <b>let</b> <b>mut</b> myso_fee = 0;
        <b>if</b> (<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.fee_is_myso()) {
            referral_rewards
                .myso
                .join(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.withdraw_with_proof(trade_proof, referral_fee, <b>false</b>));
            myso_fee = referral_fee;
        } <b>else</b> <b>if</b> (!<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.is_bid()) {
            referral_rewards
                .base
                .join(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.withdraw_with_proof(trade_proof, referral_fee, <b>false</b>));
            base_fee = referral_fee;
        } <b>else</b> {
            referral_rewards
                .quote
                .join(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.withdraw_with_proof(trade_proof, referral_fee, <b>false</b>));
            quote_fee = referral_fee;
        };
        event::emit(<a href="../orderbook/pool.md#orderbook_pool_ReferralFeeEvent">ReferralFeeEvent</a> {
            pool_id: self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>(),
            referral_id,
            base_fee,
            quote_fee,
            myso_fee,
        });
    };
}
</code></pre>



</details>

<a name="orderbook_pool_update_ewma_state"></a>

## Function `update_ewma_state`



<pre><code><b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_ewma_state">update_ewma_state</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/pool.md#orderbook_pool_update_ewma_state">update_ewma_state</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/pool.md#orderbook_pool_Pool">Pool</a>&lt;BaseAsset, QuoteAsset&gt;,
    clock: &Clock,
    ctx: &TxContext,
): &<b>mut</b> EWMAState {
    <b>let</b> pool_id = self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>();
    <b>if</b> (!self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>.exists_(<a href="../orderbook/constants.md#orderbook_constants_ewma_df_key">constants::ewma_df_key</a>())) {
        self.<a href="../orderbook/pool.md#orderbook_pool_id">id</a>.add(<a href="../orderbook/constants.md#orderbook_constants_ewma_df_key">constants::ewma_df_key</a>(), init_ewma_state(ctx));
    };
    <b>let</b> ewma_state: &<b>mut</b> EWMAState = self
        .<a href="../orderbook/pool.md#orderbook_pool_id">id</a>
        .borrow_mut(
            <a href="../orderbook/constants.md#orderbook_constants_ewma_df_key">constants::ewma_df_key</a>(),
        );
    ewma_state.update(pool_id, clock, ctx);
    ewma_state
}
</code></pre>



</details>
