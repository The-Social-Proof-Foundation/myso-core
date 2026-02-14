---
title: Module `orderbook::state`
---

State module represents the current state of the pool. It maintains all
the accounts, history, and governance information. It also processes all
the transactions and updates the state accordingly.


-  [Struct `State`](#orderbook_state_State)
-  [Struct `StakeEvent`](#orderbook_state_StakeEvent)
-  [Struct `ProposalEvent`](#orderbook_state_ProposalEvent)
-  [Struct `VoteEvent`](#orderbook_state_VoteEvent)
-  [Struct `RebateEventV2`](#orderbook_state_RebateEventV2)
-  [Struct `RebateEvent`](#orderbook_state_RebateEvent)
-  [Struct `TakerFeePenaltyApplied`](#orderbook_state_TakerFeePenaltyApplied)
-  [Constants](#@Constants_0)
-  [Function `empty`](#orderbook_state_empty)
-  [Function `process_create`](#orderbook_state_process_create)
-  [Function `withdraw_settled_amounts`](#orderbook_state_withdraw_settled_amounts)
-  [Function `process_cancel`](#orderbook_state_process_cancel)
-  [Function `process_modify`](#orderbook_state_process_modify)
-  [Function `process_stake`](#orderbook_state_process_stake)
-  [Function `process_unstake`](#orderbook_state_process_unstake)
-  [Function `process_proposal`](#orderbook_state_process_proposal)
-  [Function `process_vote`](#orderbook_state_process_vote)
-  [Function `process_claim_rebates`](#orderbook_state_process_claim_rebates)
-  [Function `governance`](#orderbook_state_governance)
-  [Function `governance_mut`](#orderbook_state_governance_mut)
-  [Function `account_exists`](#orderbook_state_account_exists)
-  [Function `account`](#orderbook_state_account)
-  [Function `history_mut`](#orderbook_state_history_mut)
-  [Function `history`](#orderbook_state_history)
-  [Function `process_fills`](#orderbook_state_process_fills)
-  [Function `update_account`](#orderbook_state_update_account)


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
<b>use</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">orderbook::trade_params</a>;
<b>use</b> <a href="../orderbook/utils.md#orderbook_utils">orderbook::utils</a>;
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



<a name="orderbook_state_State"></a>

## Struct `State`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/state.md#orderbook_state_State">State</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>accounts: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/history.md#orderbook_history">history</a>: <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/governance.md#orderbook_governance">governance</a>: <a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_state_StakeEvent"></a>

## Struct `StakeEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/state.md#orderbook_state_StakeEvent">StakeEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_state_ProposalEvent"></a>

## Struct `ProposalEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/state.md#orderbook_state_ProposalEvent">ProposalEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
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
<code>stake_required: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_state_VoteEvent"></a>

## Struct `VoteEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/state.md#orderbook_state_VoteEvent">VoteEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>from_proposal_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>to_proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stake: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_state_RebateEventV2"></a>

## Struct `RebateEventV2`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/state.md#orderbook_state_RebateEventV2">RebateEventV2</a> <b>has</b> <b>copy</b>, drop
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
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_amount: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_state_RebateEvent"></a>

## Struct `RebateEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/state.md#orderbook_state_RebateEvent">RebateEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_state_TakerFeePenaltyApplied"></a>

## Struct `TakerFeePenaltyApplied`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/state.md#orderbook_state_TakerFeePenaltyApplied">TakerFeePenaltyApplied</a> <b>has</b> <b>copy</b>, drop
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
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>order_id: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>taker_fee_without_penalty: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>taker_fee: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="orderbook_state_ENoStake"></a>



<pre><code><b>const</b> <a href="../orderbook/state.md#orderbook_state_ENoStake">ENoStake</a>: u64 = 1;
</code></pre>



<a name="orderbook_state_EMaxOpenOrders"></a>



<pre><code><b>const</b> <a href="../orderbook/state.md#orderbook_state_EMaxOpenOrders">EMaxOpenOrders</a>: u64 = 2;
</code></pre>



<a name="orderbook_state_EAlreadyProposed"></a>



<pre><code><b>const</b> <a href="../orderbook/state.md#orderbook_state_EAlreadyProposed">EAlreadyProposed</a>: u64 = 3;
</code></pre>



<a name="orderbook_state_empty"></a>

## Function `empty`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_empty">empty</a>(whitelisted: bool, stable_pool: bool, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_empty">empty</a>(whitelisted: bool, stable_pool: bool, ctx: &<b>mut</b> TxContext): <a href="../orderbook/state.md#orderbook_state_State">State</a> {
    <b>let</b> <a href="../orderbook/governance.md#orderbook_governance">governance</a> = <a href="../orderbook/governance.md#orderbook_governance_empty">governance::empty</a>(
        whitelisted,
        stable_pool,
        ctx,
    );
    <b>let</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a> = <a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>();
    <b>let</b> <a href="../orderbook/history.md#orderbook_history">history</a> = <a href="../orderbook/history.md#orderbook_history_empty">history::empty</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>, ctx.epoch(), ctx);
    <a href="../orderbook/state.md#orderbook_state_State">State</a> { <a href="../orderbook/history.md#orderbook_history">history</a>, <a href="../orderbook/governance.md#orderbook_governance">governance</a>, accounts: table::new(ctx) }
}
</code></pre>



</details>

<a name="orderbook_state_process_create"></a>

## Function `process_create`

Up until this point, an OrderInfo object has been created and potentially
filled. The OrderInfo object contains all of the necessary information to
update the state of the pool. This includes the volumes for the taker and
potentially multiple makers.
First, fills are iterated and processed, updating the appropriate user's
volumes. Funds are settled for those makers. Then, the taker's trading fee
is calculated and the taker's volumes are updated. Finally, the taker's
balances are settled.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_create">process_create</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, ewma_state: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_create">process_create</a>(
    self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>,
    <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<b>mut</b> OrderInfo,
    ewma_state: &EWMAState,
    pool_id: ID,
    ctx: &TxContext,
): (Balances, Balances) {
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.update(ctx);
    self.<a href="../orderbook/history.md#orderbook_history">history</a>.update(self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(), pool_id, ctx);
    <b>let</b> fills = <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.fills_ref();
    self.<a href="../orderbook/state.md#orderbook_state_process_fills">process_fills</a>(fills, ctx);
    self.<a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.balance_manager_id(), ctx);
    <b>let</b> <a href="../orderbook/account.md#orderbook_account">account</a> = &<b>mut</b> self.accounts[<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.balance_manager_id()];
    <b>let</b> account_volume = <a href="../orderbook/account.md#orderbook_account">account</a>.total_volume();
    <b>let</b> account_stake = <a href="../orderbook/account.md#orderbook_account">account</a>.active_stake();
    // avg exucuted price <b>for</b> taker
    <b>let</b> avg_executed_price = <b>if</b> (<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.executed_quantity() &gt; 0) {
        <a href="../orderbook/math.md#orderbook_math_div">math::div</a>(
            <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.cumulative_quote_quantity(),
            <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.executed_quantity(),
        )
    } <b>else</b> {
        0
    };
    <b>let</b> account_volume_in_deep = <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>
        .order_myso_price()
        .myso_quantity_u128(
            account_volume,
            <a href="../orderbook/math.md#orderbook_math_mul_u128">math::mul_u128</a>(account_volume, avg_executed_price <b>as</b> u128),
        );
    // taker fee will always be calculated <b>as</b> 0 <b>for</b> whitelisted pools by
    // default, <b>as</b> account_volume_in_deep is 0
    <b>let</b> taker_fee_without_penalty = self
        .<a href="../orderbook/governance.md#orderbook_governance">governance</a>
        .<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>()
        .taker_fee_for_user(account_stake, account_volume_in_deep);
    <b>let</b> taker_fee = ewma_state.apply_taker_penalty(taker_fee_without_penalty, ctx);
    <b>if</b> (taker_fee &gt; taker_fee_without_penalty) {
        event::emit(<a href="../orderbook/state.md#orderbook_state_TakerFeePenaltyApplied">TakerFeePenaltyApplied</a> {
            pool_id,
            balance_manager_id: <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.balance_manager_id(),
            order_id: <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.order_id(),
            taker_fee_without_penalty,
            taker_fee,
        });
    };
    <b>let</b> maker_fee = self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>().maker_fee();
    <b>if</b> (<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.order_inserted()) {
        <b>assert</b>!(<a href="../orderbook/account.md#orderbook_account">account</a>.open_orders().length() &lt; <a href="../orderbook/constants.md#orderbook_constants_max_open_orders">constants::max_open_orders</a>(), <a href="../orderbook/state.md#orderbook_state_EMaxOpenOrders">EMaxOpenOrders</a>);
        <a href="../orderbook/account.md#orderbook_account">account</a>.add_order(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.order_id());
    };
    <a href="../orderbook/account.md#orderbook_account">account</a>.add_taker_volume(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.executed_quantity());
    <b>let</b> (<b>mut</b> settled, <b>mut</b> owed) = <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.calculate_partial_fill_balances(
        taker_fee,
        maker_fee,
    );
    <b>let</b> (old_settled, old_owed) = <a href="../orderbook/account.md#orderbook_account">account</a>.settle();
    self.<a href="../orderbook/history.md#orderbook_history">history</a>.add_total_fees_collected(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.paid_fees_balances());
    settled.add_balances(old_settled);
    owed.add_balances(old_owed);
    (settled, owed)
}
</code></pre>



</details>

<a name="orderbook_state_withdraw_settled_amounts"></a>

## Function `withdraw_settled_amounts`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_withdraw_settled_amounts">withdraw_settled_amounts</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): (<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_withdraw_settled_amounts">withdraw_settled_amounts</a>(
    self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>,
    balance_manager_id: ID,
): (Balances, Balances) {
    <b>if</b> (self.accounts.contains(balance_manager_id)) {
        <b>let</b> <a href="../orderbook/account.md#orderbook_account">account</a> = &<b>mut</b> self.accounts[balance_manager_id];
        <a href="../orderbook/account.md#orderbook_account">account</a>.settle()
    } <b>else</b> {
        (<a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>(), <a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>())
    }
}
</code></pre>



</details>

<a name="orderbook_state_process_cancel"></a>

## Function `process_cancel`

Update account settled balances and volumes.
Remove order from account orders.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_cancel">process_cancel</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, <a href="../orderbook/order.md#orderbook_order">order</a>: &<b>mut</b> <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_cancel">process_cancel</a>(
    self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>,
    <a href="../orderbook/order.md#orderbook_order">order</a>: &<b>mut</b> Order,
    balance_manager_id: ID,
    pool_id: ID,
    ctx: &TxContext,
): (Balances, Balances) {
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.update(ctx);
    self.<a href="../orderbook/history.md#orderbook_history">history</a>.update(self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(), pool_id, ctx);
    self.<a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(balance_manager_id, ctx);
    <a href="../orderbook/order.md#orderbook_order">order</a>.set_canceled();
    <b>let</b> epoch = <a href="../orderbook/order.md#orderbook_order">order</a>.epoch();
    <b>let</b> maker_fee = self.<a href="../orderbook/history.md#orderbook_history">history</a>.historic_maker_fee(epoch);
    <b>let</b> <a href="../orderbook/balances.md#orderbook_balances">balances</a> = <a href="../orderbook/order.md#orderbook_order">order</a>.calculate_cancel_refund(maker_fee, option::none());
    <b>let</b> <a href="../orderbook/account.md#orderbook_account">account</a> = &<b>mut</b> self.accounts[balance_manager_id];
    <a href="../orderbook/account.md#orderbook_account">account</a>.remove_order(<a href="../orderbook/order.md#orderbook_order">order</a>.order_id());
    <a href="../orderbook/account.md#orderbook_account">account</a>.add_settled_balances(<a href="../orderbook/balances.md#orderbook_balances">balances</a>);
    <a href="../orderbook/account.md#orderbook_account">account</a>.settle()
}
</code></pre>



</details>

<a name="orderbook_state_process_modify"></a>

## Function `process_modify`

Given the modified quantity, update account settled balances and volumes.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_modify">process_modify</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, cancel_quantity: u64, <a href="../orderbook/order.md#orderbook_order">order</a>: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_modify">process_modify</a>(
    self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>,
    balance_manager_id: ID,
    cancel_quantity: u64,
    <a href="../orderbook/order.md#orderbook_order">order</a>: &Order,
    pool_id: ID,
    ctx: &TxContext,
): (Balances, Balances) {
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.update(ctx);
    self.<a href="../orderbook/history.md#orderbook_history">history</a>.update(self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(), pool_id, ctx);
    self.<a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(balance_manager_id, ctx);
    <b>let</b> epoch = <a href="../orderbook/order.md#orderbook_order">order</a>.epoch();
    <b>let</b> maker_fee = self.<a href="../orderbook/history.md#orderbook_history">history</a>.historic_maker_fee(epoch);
    <b>let</b> <a href="../orderbook/balances.md#orderbook_balances">balances</a> = <a href="../orderbook/order.md#orderbook_order">order</a>.calculate_cancel_refund(
        maker_fee,
        option::some(cancel_quantity),
    );
    self.accounts[balance_manager_id].add_settled_balances(<a href="../orderbook/balances.md#orderbook_balances">balances</a>);
    self.accounts[balance_manager_id].settle()
}
</code></pre>



</details>

<a name="orderbook_state_process_stake"></a>

## Function `process_stake`

Process stake transaction. Add stake to account and update governance.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_stake">process_stake</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, new_stake: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_stake">process_stake</a>(
    self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>,
    pool_id: ID,
    balance_manager_id: ID,
    new_stake: u64,
    ctx: &TxContext,
): (Balances, Balances) {
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.update(ctx);
    self.<a href="../orderbook/history.md#orderbook_history">history</a>.update(self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(), pool_id, ctx);
    self.<a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(balance_manager_id, ctx);
    <b>let</b> (stake_before, stake_after) = self.accounts[balance_manager_id].add_stake(new_stake);
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.adjust_voting_power(stake_before, stake_after);
    event::emit(<a href="../orderbook/state.md#orderbook_state_StakeEvent">StakeEvent</a> {
        pool_id,
        balance_manager_id,
        epoch: ctx.epoch(),
        amount: new_stake,
        stake: <b>true</b>,
    });
    self.accounts[balance_manager_id].settle()
}
</code></pre>



</details>

<a name="orderbook_state_process_unstake"></a>

## Function `process_unstake`

Process unstake transaction.
Remove stake from account and update governance.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_unstake">process_unstake</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_unstake">process_unstake</a>(
    self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>,
    pool_id: ID,
    balance_manager_id: ID,
    ctx: &TxContext,
): (Balances, Balances) {
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.update(ctx);
    self.<a href="../orderbook/history.md#orderbook_history">history</a>.update(self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(), pool_id, ctx);
    self.<a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(balance_manager_id, ctx);
    <b>let</b> <a href="../orderbook/account.md#orderbook_account">account</a> = &<b>mut</b> self.accounts[balance_manager_id];
    <b>let</b> active_stake = <a href="../orderbook/account.md#orderbook_account">account</a>.active_stake();
    <b>let</b> inactive_stake = <a href="../orderbook/account.md#orderbook_account">account</a>.inactive_stake();
    <b>let</b> voted_proposal = <a href="../orderbook/account.md#orderbook_account">account</a>.voted_proposal();
    <a href="../orderbook/account.md#orderbook_account">account</a>.remove_stake();
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.adjust_voting_power(active_stake + inactive_stake, 0);
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.adjust_vote(voted_proposal, option::none(), active_stake);
    event::emit(<a href="../orderbook/state.md#orderbook_state_StakeEvent">StakeEvent</a> {
        pool_id,
        balance_manager_id,
        epoch: ctx.epoch(),
        amount: active_stake + inactive_stake,
        stake: <b>false</b>,
    });
    <a href="../orderbook/account.md#orderbook_account">account</a>.settle()
}
</code></pre>



</details>

<a name="orderbook_state_process_proposal"></a>

## Function `process_proposal`

Process proposal transaction. Add proposal to governance and update account.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_proposal">process_proposal</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, taker_fee: u64, maker_fee: u64, stake_required: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_proposal">process_proposal</a>(
    self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>,
    pool_id: ID,
    balance_manager_id: ID,
    taker_fee: u64,
    maker_fee: u64,
    stake_required: u64,
    ctx: &TxContext,
) {
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.update(ctx);
    self.<a href="../orderbook/history.md#orderbook_history">history</a>.update(self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(), pool_id, ctx);
    self.<a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(balance_manager_id, ctx);
    <b>let</b> <a href="../orderbook/account.md#orderbook_account">account</a> = &<b>mut</b> self.accounts[balance_manager_id];
    <b>let</b> stake = <a href="../orderbook/account.md#orderbook_account">account</a>.active_stake();
    <b>let</b> proposal_created = <a href="../orderbook/account.md#orderbook_account">account</a>.created_proposal();
    <b>assert</b>!(stake &gt; 0, <a href="../orderbook/state.md#orderbook_state_ENoStake">ENoStake</a>);
    <b>assert</b>!(!proposal_created, <a href="../orderbook/state.md#orderbook_state_EAlreadyProposed">EAlreadyProposed</a>);
    <a href="../orderbook/account.md#orderbook_account">account</a>.set_created_proposal(<b>true</b>);
    self
        .<a href="../orderbook/governance.md#orderbook_governance">governance</a>
        .add_proposal(
            taker_fee,
            maker_fee,
            stake_required,
            stake,
            balance_manager_id,
        );
    self.<a href="../orderbook/state.md#orderbook_state_process_vote">process_vote</a>(pool_id, balance_manager_id, balance_manager_id, ctx);
    event::emit(<a href="../orderbook/state.md#orderbook_state_ProposalEvent">ProposalEvent</a> {
        pool_id,
        balance_manager_id,
        epoch: ctx.epoch(),
        taker_fee,
        maker_fee,
        stake_required,
    });
}
</code></pre>



</details>

<a name="orderbook_state_process_vote"></a>

## Function `process_vote`

Process vote transaction. Update account voted proposal and governance.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_vote">process_vote</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, proposal_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_vote">process_vote</a>(
    self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>,
    pool_id: ID,
    balance_manager_id: ID,
    proposal_id: ID,
    ctx: &TxContext,
) {
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.update(ctx);
    self.<a href="../orderbook/history.md#orderbook_history">history</a>.update(self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(), pool_id, ctx);
    self.<a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(balance_manager_id, ctx);
    <b>let</b> <a href="../orderbook/account.md#orderbook_account">account</a> = &<b>mut</b> self.accounts[balance_manager_id];
    <b>assert</b>!(<a href="../orderbook/account.md#orderbook_account">account</a>.active_stake() &gt; 0, <a href="../orderbook/state.md#orderbook_state_ENoStake">ENoStake</a>);
    <b>let</b> prev_proposal = <a href="../orderbook/account.md#orderbook_account">account</a>.set_voted_proposal(option::some(proposal_id));
    self
        .<a href="../orderbook/governance.md#orderbook_governance">governance</a>
        .adjust_vote(
            prev_proposal,
            option::some(proposal_id),
            <a href="../orderbook/account.md#orderbook_account">account</a>.active_stake(),
        );
    event::emit(<a href="../orderbook/state.md#orderbook_state_VoteEvent">VoteEvent</a> {
        pool_id,
        balance_manager_id,
        epoch: ctx.epoch(),
        from_proposal_id: prev_proposal,
        to_proposal_id: proposal_id,
        stake: <a href="../orderbook/account.md#orderbook_account">account</a>.active_stake(),
    });
}
</code></pre>



</details>

<a name="orderbook_state_process_claim_rebates"></a>

## Function `process_claim_rebates`

Process claim rebates transaction.
Update account rebates and settle balances.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_claim_rebates">process_claim_rebates</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_claim_rebates">process_claim_rebates</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>,
    pool_id: ID,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &BalanceManager,
    ctx: &TxContext,
): (Balances, Balances) {
    <b>let</b> balance_manager_id = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.id();
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.update(ctx);
    self.<a href="../orderbook/history.md#orderbook_history">history</a>.update(self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>(), pool_id, ctx);
    self.<a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(balance_manager_id, ctx);
    <b>let</b> <a href="../orderbook/account.md#orderbook_account">account</a> = &<b>mut</b> self.accounts[balance_manager_id];
    <b>let</b> claim_amount = <a href="../orderbook/account.md#orderbook_account">account</a>.claim_rebates();
    event::emit(<a href="../orderbook/state.md#orderbook_state_RebateEventV2">RebateEventV2</a> {
        pool_id,
        balance_manager_id,
        epoch: ctx.epoch(),
        claim_amount,
    });
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.emit_balance_event(
        type_name::with_defining_ids&lt;MYSO&gt;(),
        claim_amount.myso(),
        <b>true</b>,
    );
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.emit_balance_event(
        type_name::with_defining_ids&lt;BaseAsset&gt;(),
        claim_amount.base(),
        <b>true</b>,
    );
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.emit_balance_event(
        type_name::with_defining_ids&lt;QuoteAsset&gt;(),
        claim_amount.quote(),
        <b>true</b>,
    );
    <a href="../orderbook/account.md#orderbook_account">account</a>.settle()
}
</code></pre>



</details>

<a name="orderbook_state_governance"></a>

## Function `governance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance">governance</a>(self: &<a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>): &<a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/governance.md#orderbook_governance">governance</a>(self: &<a href="../orderbook/state.md#orderbook_state_State">State</a>): &Governance {
    &self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>
}
</code></pre>



</details>

<a name="orderbook_state_governance_mut"></a>

## Function `governance_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_governance_mut">governance_mut</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): &<b>mut</b> <a href="../orderbook/governance.md#orderbook_governance_Governance">orderbook::governance::Governance</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_governance_mut">governance_mut</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>, ctx: &TxContext): &<b>mut</b> Governance {
    self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>.update(ctx);
    &<b>mut</b> self.<a href="../orderbook/governance.md#orderbook_governance">governance</a>
}
</code></pre>



</details>

<a name="orderbook_state_account_exists"></a>

## Function `account_exists`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_account_exists">account_exists</a>(self: &<a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_account_exists">account_exists</a>(self: &<a href="../orderbook/state.md#orderbook_state_State">State</a>, balance_manager_id: ID): bool {
    self.accounts.contains(balance_manager_id)
}
</code></pre>



</details>

<a name="orderbook_state_account"></a>

## Function `account`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account">account</a>(self: &<a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account">account</a>(self: &<a href="../orderbook/state.md#orderbook_state_State">State</a>, balance_manager_id: ID): &Account {
    &self.accounts[balance_manager_id]
}
</code></pre>



</details>

<a name="orderbook_state_history_mut"></a>

## Function `history_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_history_mut">history_mut</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>): &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/state.md#orderbook_state_history_mut">history_mut</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>): &<b>mut</b> History {
    &<b>mut</b> self.<a href="../orderbook/history.md#orderbook_history">history</a>
}
</code></pre>



</details>

<a name="orderbook_state_history"></a>

## Function `history`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history">history</a>(self: &<a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>): &<a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history">history</a>(self: &<a href="../orderbook/state.md#orderbook_state_State">State</a>): &History {
    &self.<a href="../orderbook/history.md#orderbook_history">history</a>
}
</code></pre>



</details>

<a name="orderbook_state_process_fills"></a>

## Function `process_fills`

Process fills for all makers. Update maker accounts and history.


<pre><code><b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_fills">process_fills</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, fills: &<b>mut</b> vector&lt;<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/state.md#orderbook_state_process_fills">process_fills</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>, fills: &<b>mut</b> vector&lt;Fill&gt;, ctx: &TxContext) {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> num_fills = fills.length();
    <b>while</b> (i &lt; num_fills) {
        <b>let</b> <a href="../orderbook/fill.md#orderbook_fill">fill</a> = &<b>mut</b> fills[i];
        <b>let</b> maker = <a href="../orderbook/fill.md#orderbook_fill">fill</a>.balance_manager_id();
        self.<a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(maker, ctx);
        <b>let</b> <a href="../orderbook/account.md#orderbook_account">account</a> = &<b>mut</b> self.accounts[maker];
        <a href="../orderbook/account.md#orderbook_account">account</a>.process_maker_fill(<a href="../orderbook/fill.md#orderbook_fill">fill</a>);
        <b>let</b> base_volume = <a href="../orderbook/fill.md#orderbook_fill">fill</a>.base_quantity();
        <b>let</b> quote_volume = <a href="../orderbook/fill.md#orderbook_fill">fill</a>.quote_quantity();
        <b>let</b> historic_maker_fee = self.<a href="../orderbook/history.md#orderbook_history">history</a>.historic_maker_fee(<a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_epoch());
        <b>let</b> maker_is_bid = !<a href="../orderbook/fill.md#orderbook_fill">fill</a>.taker_is_bid();
        <b>let</b> <b>mut</b> fee_quantity = <a href="../orderbook/fill.md#orderbook_fill">fill</a>
            .maker_myso_price()
            .fee_quantity(base_volume, quote_volume, maker_is_bid);
        fee_quantity.mul(historic_maker_fee);
        <b>if</b> (!<a href="../orderbook/fill.md#orderbook_fill">fill</a>.expired()) {
            <a href="../orderbook/fill.md#orderbook_fill">fill</a>.set_fill_maker_fee(&fee_quantity);
            self.<a href="../orderbook/history.md#orderbook_history">history</a>.add_volume(base_volume, <a href="../orderbook/account.md#orderbook_account">account</a>.active_stake());
            self.<a href="../orderbook/history.md#orderbook_history">history</a>.add_total_fees_collected(fee_quantity);
        } <b>else</b> {
            <a href="../orderbook/account.md#orderbook_account">account</a>.add_settled_balances(fee_quantity);
        };
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="orderbook_state_update_account"></a>

## Function `update_account`

If account doesn't exist, create it. Update account volumes and rebates.


<pre><code><b>fun</b> <a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">orderbook::state::State</a>, balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/state.md#orderbook_state_update_account">update_account</a>(self: &<b>mut</b> <a href="../orderbook/state.md#orderbook_state_State">State</a>, balance_manager_id: ID, ctx: &TxContext) {
    <b>if</b> (!self.accounts.contains(balance_manager_id)) {
        self.accounts.add(balance_manager_id, <a href="../orderbook/account.md#orderbook_account_empty">account::empty</a>(ctx));
    };
    <b>let</b> <a href="../orderbook/account.md#orderbook_account">account</a> = &<b>mut</b> self.accounts[balance_manager_id];
    <b>let</b> (prev_epoch, maker_volume, active_stake) = <a href="../orderbook/account.md#orderbook_account">account</a>.update(ctx);
    <b>if</b> (prev_epoch &gt; 0 && maker_volume &gt; 0 && active_stake &gt; 0) {
        <b>let</b> rebates = self.<a href="../orderbook/history.md#orderbook_history">history</a>.calculate_rebate_amount(prev_epoch, maker_volume, active_stake);
        <a href="../orderbook/account.md#orderbook_account">account</a>.add_rebates(rebates);
    }
}
</code></pre>



</details>
