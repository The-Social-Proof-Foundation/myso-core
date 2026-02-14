---
title: Module `orderbook::balance_manager`
---

The BalanceManager is a shared object that holds all of the balances for different assets. A combination of <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a></code> and
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code> are passed into a pool to perform trades. A <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code> can be generated in two ways: by the
owner directly, or by any <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code> owner. The owner can generate a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code> without the risk of
equivocation. The <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code> owner, due to it being an owned object, risks equivocation when generating
a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code>. Generally, a high frequency trading engine will trade as the default owner.


-  [Struct `BalanceManager`](#orderbook_balance_manager_BalanceManager)
-  [Struct `BalanceManagerEvent`](#orderbook_balance_manager_BalanceManagerEvent)
-  [Struct `BalanceEvent`](#orderbook_balance_manager_BalanceEvent)
-  [Struct `BalanceKey`](#orderbook_balance_manager_BalanceKey)
-  [Struct `ReferralKey`](#orderbook_balance_manager_ReferralKey)
-  [Struct `TradeCap`](#orderbook_balance_manager_TradeCap)
-  [Struct `DepositCap`](#orderbook_balance_manager_DepositCap)
-  [Struct `WithdrawCap`](#orderbook_balance_manager_WithdrawCap)
-  [Struct `OrderbookReferral`](#orderbook_balance_manager_OrderbookReferral)
-  [Struct `OrderbookPoolReferral`](#orderbook_balance_manager_OrderbookPoolReferral)
-  [Struct `OrderbookReferralCreatedEvent`](#orderbook_balance_manager_OrderbookReferralCreatedEvent)
-  [Struct `OrderbookReferralSetEvent`](#orderbook_balance_manager_OrderbookReferralSetEvent)
-  [Struct `TradeProof`](#orderbook_balance_manager_TradeProof)
-  [Constants](#@Constants_0)
-  [Function `new`](#orderbook_balance_manager_new)
-  [Function `new_with_owner`](#orderbook_balance_manager_new_with_owner)
-  [Function `new_with_custom_owner`](#orderbook_balance_manager_new_with_custom_owner)
-  [Function `new_with_custom_owner_and_caps`](#orderbook_balance_manager_new_with_custom_owner_and_caps)
-  [Function `new_with_custom_owner_caps`](#orderbook_balance_manager_new_with_custom_owner_caps)
-  [Function `set_referral`](#orderbook_balance_manager_set_referral)
-  [Function `set_balance_manager_referral`](#orderbook_balance_manager_set_balance_manager_referral)
-  [Function `unset_referral`](#orderbook_balance_manager_unset_referral)
-  [Function `unset_balance_manager_referral`](#orderbook_balance_manager_unset_balance_manager_referral)
-  [Function `balance`](#orderbook_balance_manager_balance)
-  [Function `mint_trade_cap`](#orderbook_balance_manager_mint_trade_cap)
-  [Function `mint_deposit_cap`](#orderbook_balance_manager_mint_deposit_cap)
-  [Function `mint_withdraw_cap`](#orderbook_balance_manager_mint_withdraw_cap)
-  [Function `revoke_trade_cap`](#orderbook_balance_manager_revoke_trade_cap)
-  [Function `generate_proof_as_owner`](#orderbook_balance_manager_generate_proof_as_owner)
-  [Function `generate_proof_as_trader`](#orderbook_balance_manager_generate_proof_as_trader)
-  [Function `deposit`](#orderbook_balance_manager_deposit)
-  [Function `deposit_with_cap`](#orderbook_balance_manager_deposit_with_cap)
-  [Function `withdraw_with_cap`](#orderbook_balance_manager_withdraw_with_cap)
-  [Function `withdraw`](#orderbook_balance_manager_withdraw)
-  [Function `withdraw_all`](#orderbook_balance_manager_withdraw_all)
-  [Function `register_manager`](#orderbook_balance_manager_register_manager)
-  [Function `register_balance_manager`](#orderbook_balance_manager_register_balance_manager)
-  [Function `get_referral_id`](#orderbook_balance_manager_get_referral_id)
-  [Function `get_balance_manager_referral_id`](#orderbook_balance_manager_get_balance_manager_referral_id)
-  [Function `validate_proof`](#orderbook_balance_manager_validate_proof)
-  [Function `owner`](#orderbook_balance_manager_owner)
-  [Function `id`](#orderbook_balance_manager_id)
-  [Function `referral_owner`](#orderbook_balance_manager_referral_owner)
-  [Function `balance_manager_referral_owner`](#orderbook_balance_manager_balance_manager_referral_owner)
-  [Function `balance_manager_referral_pool_id`](#orderbook_balance_manager_balance_manager_referral_pool_id)
-  [Function `mint_referral`](#orderbook_balance_manager_mint_referral)
-  [Function `assert_referral_owner`](#orderbook_balance_manager_assert_referral_owner)
-  [Function `deposit_with_proof`](#orderbook_balance_manager_deposit_with_proof)
-  [Function `deposit_permissionless`](#orderbook_balance_manager_deposit_permissionless)
-  [Function `generate_proof_as_depositor`](#orderbook_balance_manager_generate_proof_as_depositor)
-  [Function `generate_proof_as_withdrawer`](#orderbook_balance_manager_generate_proof_as_withdrawer)
-  [Function `withdraw_with_proof`](#orderbook_balance_manager_withdraw_with_proof)
-  [Function `delete`](#orderbook_balance_manager_delete)
-  [Function `trader`](#orderbook_balance_manager_trader)
-  [Function `emit_balance_event`](#orderbook_balance_manager_emit_balance_event)
-  [Function `mint_trade_cap_internal`](#orderbook_balance_manager_mint_trade_cap_internal)
-  [Function `mint_deposit_cap_internal`](#orderbook_balance_manager_mint_deposit_cap_internal)
-  [Function `mint_withdraw_cap_internal`](#orderbook_balance_manager_mint_withdraw_cap_internal)
-  [Function `validate_owner`](#orderbook_balance_manager_validate_owner)
-  [Function `validate_trader`](#orderbook_balance_manager_validate_trader)
-  [Function `validate_deposit_cap`](#orderbook_balance_manager_validate_deposit_cap)
-  [Function `validate_withdraw_cap`](#orderbook_balance_manager_validate_withdraw_cap)


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
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/registry.md#orderbook_registry">orderbook::registry</a>;
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



<a name="orderbook_balance_manager_BalanceManager"></a>

## Struct `BalanceManager`

A shared object that is passed into pools for placing orders.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/balances.md#orderbook_balances">balances</a>: <a href="../myso/bag.md#myso_bag_Bag">myso::bag::Bag</a></code>
</dt>
<dd>
</dd>
<dt>
<code>allow_listed: <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_BalanceManagerEvent"></a>

## Struct `BalanceManagerEvent`

Event emitted when a new balance_manager is created.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManagerEvent">BalanceManagerEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_BalanceEvent"></a>

## Struct `BalanceEvent`

Event emitted when a deposit or withdrawal occurs.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceEvent">BalanceEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>asset: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit">deposit</a>: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_BalanceKey"></a>

## Struct `BalanceKey`

Balance identifier.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceKey">BalanceKey</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="orderbook_balance_manager_ReferralKey"></a>

## Struct `ReferralKey`

Referral identifier.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_ReferralKey">ReferralKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>0: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_TradeCap"></a>

## Struct `TradeCap`

Owners of a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code> need to get a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code> to trade across pools in a single PTB (drops after).


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_DepositCap"></a>

## Struct `DepositCap`

<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a></code> is used to deposit funds to a balance_manager by a non-owner.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_WithdrawCap"></a>

## Struct `WithdrawCap`

WithdrawCap is used to withdraw funds from a balance_manager by a non-owner.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_OrderbookReferral"></a>

## Struct `OrderbookReferral`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">OrderbookReferral</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_OrderbookPoolReferral"></a>

## Struct `OrderbookPoolReferral`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">OrderbookPoolReferral</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_OrderbookReferralCreatedEvent"></a>

## Struct `OrderbookReferralCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferralCreatedEvent">OrderbookReferralCreatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_OrderbookReferralSetEvent"></a>

## Struct `OrderbookReferralSetEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferralSetEvent">OrderbookReferralSetEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balance_manager_TradeProof"></a>

## Struct `TradeProof`

BalanceManager owner and <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code> owners can generate a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code>.
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code> is used to validate the balance_manager when trading on Orderbook.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_trader">trader</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="orderbook_balance_manager_EInvalidOwner"></a>



<pre><code><b>const</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidOwner">EInvalidOwner</a>: u64 = 0;
</code></pre>



<a name="orderbook_balance_manager_EInvalidTrader"></a>



<pre><code><b>const</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidTrader">EInvalidTrader</a>: u64 = 1;
</code></pre>



<a name="orderbook_balance_manager_EInvalidProof"></a>



<pre><code><b>const</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidProof">EInvalidProof</a>: u64 = 2;
</code></pre>



<a name="orderbook_balance_manager_EBalanceManagerBalanceTooLow"></a>



<pre><code><b>const</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EBalanceManagerBalanceTooLow">EBalanceManagerBalanceTooLow</a>: u64 = 3;
</code></pre>



<a name="orderbook_balance_manager_EMaxCapsReached"></a>



<pre><code><b>const</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EMaxCapsReached">EMaxCapsReached</a>: u64 = 4;
</code></pre>



<a name="orderbook_balance_manager_ECapNotInList"></a>



<pre><code><b>const</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_ECapNotInList">ECapNotInList</a>: u64 = 5;
</code></pre>



<a name="orderbook_balance_manager_EInvalidReferralOwner"></a>



<pre><code><b>const</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidReferralOwner">EInvalidReferralOwner</a>: u64 = 6;
</code></pre>



<a name="orderbook_balance_manager_MAX_TRADE_CAPS"></a>



<pre><code><b>const</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_MAX_TRADE_CAPS">MAX_TRADE_CAPS</a>: u64 = 1000;
</code></pre>



<a name="orderbook_balance_manager_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new">new</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a> {
    <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a> = object::new(ctx);
    event::emit(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManagerEvent">BalanceManagerEvent</a> {
        balance_manager_id: <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner(),
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: ctx.sender(),
    });
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a> {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>,
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: ctx.sender(),
        <a href="../orderbook/balances.md#orderbook_balances">balances</a>: bag::new(ctx),
        allow_listed: vec_set::empty(),
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_new_with_owner"></a>

## Function `new_with_owner`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new_with_owner">new_with_owner</a>(_ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>, _owner: <b>address</b>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new_with_owner">new_with_owner</a>(_ctx: &<b>mut</b> TxContext, _owner: <b>address</b>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a> {
    <b>abort</b> 1337
}
</code></pre>



</details>

<a name="orderbook_balance_manager_new_with_custom_owner"></a>

## Function `new_with_custom_owner`

Create a new balance manager with an owner.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new_with_custom_owner">new_with_custom_owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new_with_custom_owner">new_with_custom_owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: <b>address</b>, ctx: &<b>mut</b> TxContext): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a> {
    <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a> = object::new(ctx);
    event::emit(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManagerEvent">BalanceManagerEvent</a> {
        balance_manager_id: <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner(),
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>,
    });
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a> {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>,
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>,
        <a href="../orderbook/balances.md#orderbook_balances">balances</a>: bag::new(ctx),
        allow_listed: vec_set::empty(),
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_new_with_custom_owner_and_caps"></a>

## Function `new_with_custom_owner_and_caps`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new_with_custom_owner_and_caps">new_with_custom_owner_and_caps</a>(_owner: <b>address</b>, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new_with_custom_owner_and_caps">new_with_custom_owner_and_caps</a>(
    _owner: <b>address</b>,
    _ctx: &<b>mut</b> TxContext,
): (<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a>) { <b>abort</b> 1337 }
</code></pre>



</details>

<a name="orderbook_balance_manager_new_with_custom_owner_caps"></a>

## Function `new_with_custom_owner_caps`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new_with_custom_owner_caps">new_with_custom_owner_caps</a>&lt;App: drop&gt;(orderbook_registry: &<a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new_with_custom_owner_caps">new_with_custom_owner_caps</a>&lt;App: drop&gt;(
    orderbook_registry: &Registry,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
): (<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a>) {
    orderbook_registry.assert_app_is_authorized&lt;App&gt;();
    <b>let</b> <b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a> = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_new_with_custom_owner">new_with_custom_owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>, ctx);
    <b>let</b> deposit_cap = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_deposit_cap_internal">mint_deposit_cap_internal</a>(&<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, ctx);
    <b>let</b> withdraw_cap = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_withdraw_cap_internal">mint_withdraw_cap_internal</a>(&<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, ctx);
    <b>let</b> trade_cap = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_trade_cap_internal">mint_trade_cap_internal</a>(&<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, ctx);
    (<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, deposit_cap, withdraw_cap, trade_cap)
}
</code></pre>



</details>

<a name="orderbook_balance_manager_set_referral"></a>

## Function `set_referral`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_set_referral">set_referral</a>(_balance_manager: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, _referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">orderbook::balance_manager::OrderbookReferral</a>, _trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_set_referral">set_referral</a>(
    _balance_manager: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    _referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">OrderbookReferral</a>,
    _trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a>,
) { <b>abort</b> }
</code></pre>



</details>

<a name="orderbook_balance_manager_set_balance_manager_referral"></a>

## Function `set_balance_manager_referral`

Set the referral for the balance manager.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_set_balance_manager_referral">set_balance_manager_referral</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">orderbook::balance_manager::OrderbookPoolReferral</a>, trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_set_balance_manager_referral">set_balance_manager_referral</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">OrderbookPoolReferral</a>,
    trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a>,
) {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_trader">validate_trader</a>(trade_cap);
    <b>let</b> _: Option&lt;ID&gt; = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.remove_if_exists(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_ReferralKey">ReferralKey</a>(referral.pool_id));
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.add(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_ReferralKey">ReferralKey</a>(referral.pool_id), referral.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner());
    event::emit(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferralSetEvent">OrderbookReferralSetEvent</a> {
        referral_id: referral.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner(),
        balance_manager_id: <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner(),
    });
}
</code></pre>



</details>

<a name="orderbook_balance_manager_unset_referral"></a>

## Function `unset_referral`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_unset_referral">unset_referral</a>(_balance_manager: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, _trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_unset_referral">unset_referral</a>(_balance_manager: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, _trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a>) {
    <b>abort</b>
}
</code></pre>



</details>

<a name="orderbook_balance_manager_unset_balance_manager_referral"></a>

## Function `unset_balance_manager_referral`

Unset the referral for the balance manager.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_unset_balance_manager_referral">unset_balance_manager_referral</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_unset_balance_manager_referral">unset_balance_manager_referral</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    pool_id: ID,
    trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a>,
) {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_trader">validate_trader</a>(trade_cap);
    <b>let</b> _: Option&lt;ID&gt; = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.remove_if_exists(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_ReferralKey">ReferralKey</a>(pool_id));
    event::emit(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferralSetEvent">OrderbookReferralSetEvent</a> {
        referral_id: id_from_address(@0x0),
        balance_manager_id: <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner(),
    });
}
</code></pre>



</details>

<a name="orderbook_balance_manager_balance"></a>

## Function `balance`

Returns the balance of a Coin in a balance manager.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance">balance</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance">balance</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>): u64 {
    <b>let</b> key = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceKey">BalanceKey</a>&lt;T&gt; {};
    <b>if</b> (!<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>.contains(key)) {
        0
    } <b>else</b> {
        <b>let</b> acc_balance: &Balance&lt;T&gt; = &<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>[key];
        acc_balance.value()
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_mint_trade_cap"></a>

## Function `mint_trade_cap`

Mint a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code>, only owner can mint a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_trade_cap">mint_trade_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_trade_cap">mint_trade_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, ctx: &<b>mut</b> TxContext): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a> {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_owner">validate_owner</a>(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_trade_cap_internal">mint_trade_cap_internal</a>(ctx)
}
</code></pre>



</details>

<a name="orderbook_balance_manager_mint_deposit_cap"></a>

## Function `mint_deposit_cap`

Mint a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a></code>, only owner can mint.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_deposit_cap">mint_deposit_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_deposit_cap">mint_deposit_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, ctx: &<b>mut</b> TxContext): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a> {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_owner">validate_owner</a>(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_deposit_cap_internal">mint_deposit_cap_internal</a>(ctx)
}
</code></pre>



</details>

<a name="orderbook_balance_manager_mint_withdraw_cap"></a>

## Function `mint_withdraw_cap`

Mint a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a></code>, only owner can mint.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_withdraw_cap">mint_withdraw_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_withdraw_cap">mint_withdraw_cap</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a> {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_owner">validate_owner</a>(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_withdraw_cap_internal">mint_withdraw_cap_internal</a>(ctx)
}
</code></pre>



</details>

<a name="orderbook_balance_manager_revoke_trade_cap"></a>

## Function `revoke_trade_cap`

Revoke a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code>. Only the owner can revoke a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code>.
Can also be used to revoke <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a></code> and <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_revoke_trade_cap">revoke_trade_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_cap_id: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_revoke_trade_cap">revoke_trade_cap</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    trade_cap_id: &ID,
    ctx: &TxContext,
) {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_owner">validate_owner</a>(ctx);
    <b>assert</b>!(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.contains(trade_cap_id), <a href="../orderbook/balance_manager.md#orderbook_balance_manager_ECapNotInList">ECapNotInList</a>);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.remove(trade_cap_id);
}
</code></pre>



</details>

<a name="orderbook_balance_manager_generate_proof_as_owner"></a>

## Function `generate_proof_as_owner`

Generate a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code> by the owner. The owner does not require a capability
and can generate TradeProofs without the risk of equivocation.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_owner">generate_proof_as_owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_owner">generate_proof_as_owner</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    ctx: &TxContext,
): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a> {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_owner">validate_owner</a>(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a> {
        balance_manager_id: object::id(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>),
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_trader">trader</a>: ctx.sender(),
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_generate_proof_as_trader"></a>

## Function `generate_proof_as_trader`

Generate a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code> with a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code>.
Risk of equivocation since <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a></code> is an owned object.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_trader">generate_proof_as_trader</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_trader">generate_proof_as_trader</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a>,
    ctx: &TxContext,
): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a> {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_trader">validate_trader</a>(trade_cap);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a> {
        balance_manager_id: object::id(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>),
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_trader">trader</a>: ctx.sender(),
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_deposit"></a>

## Function `deposit`

Deposit funds to a balance manager. Only owner can call this directly.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit">deposit</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit">deposit</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, coin: Coin&lt;T&gt;, ctx: &<b>mut</b> TxContext) {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_emit_balance_event">emit_balance_event</a>(
        type_name::with_defining_ids&lt;T&gt;(),
        coin.value(),
        <b>true</b>,
    );
    <b>let</b> proof = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_owner">generate_proof_as_owner</a>(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit_with_proof">deposit_with_proof</a>(&proof, coin.into_balance());
}
</code></pre>



</details>

<a name="orderbook_balance_manager_deposit_with_cap"></a>

## Function `deposit_with_cap`

Deposit funds into a balance manager by a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a></code> owner.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit_with_cap">deposit_with_cap</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, deposit_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>, coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit_with_cap">deposit_with_cap</a>&lt;T&gt;(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    deposit_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a>,
    coin: Coin&lt;T&gt;,
    ctx: &TxContext,
) {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_emit_balance_event">emit_balance_event</a>(
        type_name::with_defining_ids&lt;T&gt;(),
        coin.value(),
        <b>true</b>,
    );
    <b>let</b> proof = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_depositor">generate_proof_as_depositor</a>(deposit_cap, ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit_with_proof">deposit_with_proof</a>(&proof, coin.into_balance());
}
</code></pre>



</details>

<a name="orderbook_balance_manager_withdraw_with_cap"></a>

## Function `withdraw_with_cap`

Withdraw funds from a balance manager by a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a></code> owner.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_with_cap">withdraw_with_cap</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, withdraw_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>, withdraw_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_with_cap">withdraw_with_cap</a>&lt;T&gt;(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    withdraw_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a>,
    withdraw_amount: u64,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;T&gt; {
    <b>let</b> proof = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_withdrawer">generate_proof_as_withdrawer</a>(
        withdraw_cap,
        ctx,
    );
    <b>let</b> coin = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_with_proof">withdraw_with_proof</a>(&proof, withdraw_amount, <b>false</b>).into_coin(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_emit_balance_event">emit_balance_event</a>(
        type_name::with_defining_ids&lt;T&gt;(),
        coin.value(),
        <b>false</b>,
    );
    coin
}
</code></pre>



</details>

<a name="orderbook_balance_manager_withdraw"></a>

## Function `withdraw`

Withdraw funds from a balance_manager. Only owner can call this directly.
If withdraw_all is true, amount is ignored and full balance withdrawn.
If withdraw_all is false, withdraw_amount will be withdrawn.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw">withdraw</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, withdraw_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw">withdraw</a>&lt;T&gt;(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    withdraw_amount: u64,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;T&gt; {
    <b>let</b> proof = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_owner">generate_proof_as_owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, ctx);
    <b>let</b> coin = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_with_proof">withdraw_with_proof</a>(&proof, withdraw_amount, <b>false</b>).into_coin(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_emit_balance_event">emit_balance_event</a>(
        type_name::with_defining_ids&lt;T&gt;(),
        coin.value(),
        <b>false</b>,
    );
    coin
}
</code></pre>



</details>

<a name="orderbook_balance_manager_withdraw_all"></a>

## Function `withdraw_all`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_all">withdraw_all</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_all">withdraw_all</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, ctx: &<b>mut</b> TxContext): Coin&lt;T&gt; {
    <b>let</b> proof = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_owner">generate_proof_as_owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>, ctx);
    <b>let</b> coin = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_with_proof">withdraw_with_proof</a>(&proof, 0, <b>true</b>).into_coin(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_emit_balance_event">emit_balance_event</a>(
        type_name::with_defining_ids&lt;T&gt;(),
        coin.value(),
        <b>false</b>,
    );
    coin
}
</code></pre>



</details>

<a name="orderbook_balance_manager_register_manager"></a>

## Function `register_manager`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_register_manager">register_manager</a>(_balance_manager: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, _registry: &<b>mut</b> <a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_register_manager">register_manager</a>(_balance_manager: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, _registry: &<b>mut</b> Registry) {
    <b>abort</b> 1337
}
</code></pre>



</details>

<a name="orderbook_balance_manager_register_balance_manager"></a>

## Function `register_balance_manager`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_register_balance_manager">register_balance_manager</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> <a href="../orderbook/registry.md#orderbook_registry_Registry">orderbook::registry::Registry</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_register_balance_manager">register_balance_manager</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>: &<b>mut</b> Registry,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_owner">validate_owner</a>(ctx);
    <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a> = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>();
    <b>let</b> manager_id = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>();
    <a href="../orderbook/registry.md#orderbook_registry">registry</a>.add_balance_manager(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>, manager_id);
}
</code></pre>



</details>

<a name="orderbook_balance_manager_get_referral_id"></a>

## Function `get_referral_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_get_referral_id">get_referral_id</a>(_balance_manager: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_get_referral_id">get_referral_id</a>(_balance_manager: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>): Option&lt;ID&gt; {
    <b>abort</b>
}
</code></pre>



</details>

<a name="orderbook_balance_manager_get_balance_manager_referral_id"></a>

## Function `get_balance_manager_referral_id`

Get the referral id from the balance manager.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_get_balance_manager_referral_id">get_balance_manager_referral_id</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_get_balance_manager_referral_id">get_balance_manager_referral_id</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    pool_id: ID,
): Option&lt;ID&gt; {
    <b>let</b> ref_key = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_ReferralKey">ReferralKey</a>(pool_id);
    <b>if</b> (!<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.exists_(ref_key)) {
        <b>return</b> option::none()
    };
    <b>let</b> referral_id: &ID = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.borrow(ref_key);
    option::some(*referral_id)
}
</code></pre>



</details>

<a name="orderbook_balance_manager_validate_proof"></a>

## Function `validate_proof`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_proof">validate_proof</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_proof">validate_proof</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a>) {
    <b>assert</b>!(object::id(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>) == proof.balance_manager_id, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidProof">EInvalidProof</a>);
}
</code></pre>



</details>

<a name="orderbook_balance_manager_owner"></a>

## Function `owner`

Returns the owner of the balance_manager.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>): <b>address</b> {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>
}
</code></pre>



</details>

<a name="orderbook_balance_manager_id"></a>

## Function `id`

Returns the owner of the balance_manager.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>): ID {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner()
}
</code></pre>



</details>

<a name="orderbook_balance_manager_referral_owner"></a>

## Function `referral_owner`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_referral_owner">referral_owner</a>(_referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">orderbook::balance_manager::OrderbookReferral</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_referral_owner">referral_owner</a>(_referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">OrderbookReferral</a>): <b>address</b> {
    <b>abort</b>
}
</code></pre>



</details>

<a name="orderbook_balance_manager_balance_manager_referral_owner"></a>

## Function `balance_manager_referral_owner`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance_manager_referral_owner">balance_manager_referral_owner</a>(referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">orderbook::balance_manager::OrderbookPoolReferral</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance_manager_referral_owner">balance_manager_referral_owner</a>(referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">OrderbookPoolReferral</a>): <b>address</b> {
    referral.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>
}
</code></pre>



</details>

<a name="orderbook_balance_manager_balance_manager_referral_pool_id"></a>

## Function `balance_manager_referral_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance_manager_referral_pool_id">balance_manager_referral_pool_id</a>(referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">orderbook::balance_manager::OrderbookPoolReferral</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance_manager_referral_pool_id">balance_manager_referral_pool_id</a>(referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">OrderbookPoolReferral</a>): ID {
    referral.pool_id
}
</code></pre>



</details>

<a name="orderbook_balance_manager_mint_referral"></a>

## Function `mint_referral`

Mint a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferral">OrderbookReferral</a></code> and share it.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_referral">mint_referral</a>(pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_referral">mint_referral</a>(pool_id: ID, ctx: &<b>mut</b> TxContext): ID {
    <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a> = object::new(ctx);
    <b>let</b> referral_id = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner();
    <b>let</b> referral = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">OrderbookPoolReferral</a> {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>,
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: ctx.sender(),
        pool_id,
    };
    event::emit(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookReferralCreatedEvent">OrderbookReferralCreatedEvent</a> {
        referral_id,
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: ctx.sender(),
    });
    transfer::share_object(referral);
    referral_id
}
</code></pre>



</details>

<a name="orderbook_balance_manager_assert_referral_owner"></a>

## Function `assert_referral_owner`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_assert_referral_owner">assert_referral_owner</a>(referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">orderbook::balance_manager::OrderbookPoolReferral</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_assert_referral_owner">assert_referral_owner</a>(referral: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_OrderbookPoolReferral">OrderbookPoolReferral</a>, ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == referral.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidReferralOwner">EInvalidReferralOwner</a>);
}
</code></pre>



</details>

<a name="orderbook_balance_manager_deposit_with_proof"></a>

## Function `deposit_with_proof`

Deposit funds to a balance_manager. Pool will call this to deposit funds.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit_with_proof">deposit_with_proof</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, to_deposit: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit_with_proof">deposit_with_proof</a>&lt;T&gt;(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a>,
    to_deposit: Balance&lt;T&gt;,
) {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_proof">validate_proof</a>(proof);
    <b>let</b> key = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceKey">BalanceKey</a>&lt;T&gt; {};
    <b>if</b> (<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>.contains(key)) {
        <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance">balance</a>: &<b>mut</b> Balance&lt;T&gt; = &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>[key];
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance">balance</a>.join(to_deposit);
    } <b>else</b> {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>.add(key, to_deposit);
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_deposit_permissionless"></a>

## Function `deposit_permissionless`

Deposit funds to a balance_manager. Pool will call this to deposit funds.
This function is used by withdraw_settled_amounts_permissionless to deposit funds.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit_permissionless">deposit_permissionless</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, to_deposit: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit_permissionless">deposit_permissionless</a>&lt;T&gt;(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    to_deposit: Balance&lt;T&gt;,
) {
    <b>let</b> key = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceKey">BalanceKey</a>&lt;T&gt; {};
    <b>if</b> (<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>.contains(key)) {
        <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance">balance</a>: &<b>mut</b> Balance&lt;T&gt; = &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>[key];
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_balance">balance</a>.join(to_deposit);
    } <b>else</b> {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>.add(key, to_deposit);
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_generate_proof_as_depositor"></a>

## Function `generate_proof_as_depositor`

Generate a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code> by a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a></code> owner.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_depositor">generate_proof_as_depositor</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, deposit_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_depositor">generate_proof_as_depositor</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    deposit_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a>,
    ctx: &TxContext,
): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a> {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_deposit_cap">validate_deposit_cap</a>(deposit_cap);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a> {
        balance_manager_id: object::id(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>),
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_trader">trader</a>: ctx.sender(),
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_generate_proof_as_withdrawer"></a>

## Function `generate_proof_as_withdrawer`

Generate a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a></code> by a <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a></code> owner.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_withdrawer">generate_proof_as_withdrawer</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, withdraw_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_generate_proof_as_withdrawer">generate_proof_as_withdrawer</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    withdraw_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a>,
    ctx: &TxContext,
): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a> {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_withdraw_cap">validate_withdraw_cap</a>(withdraw_cap);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a> {
        balance_manager_id: object::id(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>),
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_trader">trader</a>: ctx.sender(),
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_withdraw_with_proof"></a>

## Function `withdraw_with_proof`

Withdraw funds from a balance_manager. Pool will call this to withdraw funds.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_with_proof">withdraw_with_proof</a>&lt;T&gt;(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>, withdraw_amount: u64, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_all">withdraw_all</a>: bool): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_with_proof">withdraw_with_proof</a>&lt;T&gt;(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a>,
    withdraw_amount: u64,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_all">withdraw_all</a>: bool,
): Balance&lt;T&gt; {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_proof">validate_proof</a>(proof);
    <b>let</b> key = <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceKey">BalanceKey</a>&lt;T&gt; {};
    <b>let</b> key_exists = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>.contains(key);
    <b>if</b> (!key_exists) {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>.add(key, balance::zero&lt;T&gt;());
    };
    <b>if</b> (<a href="../orderbook/balance_manager.md#orderbook_balance_manager_withdraw_all">withdraw_all</a>) {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>.remove(key)
    } <b>else</b> {
        <b>let</b> acc_balance: &<b>mut</b> Balance&lt;T&gt; = &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>[key];
        <b>let</b> acc_value = acc_balance.value();
        <b>assert</b>!(acc_value &gt;= withdraw_amount, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EBalanceManagerBalanceTooLow">EBalanceManagerBalanceTooLow</a>);
        <b>if</b> (withdraw_amount == acc_value) {
            <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balances.md#orderbook_balances">balances</a>.remove(key)
        } <b>else</b> {
            acc_balance.split(withdraw_amount)
        }
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_delete"></a>

## Function `delete`

Deletes a balance_manager.
This is used for deleting temporary balance_managers for direct swap with pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_delete">delete</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_delete">delete</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>) {
    <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a> {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>,
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>: _,
        <a href="../orderbook/balances.md#orderbook_balances">balances</a>,
        allow_listed: _,
    } = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>;
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_delete">delete</a>();
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.destroy_empty();
}
</code></pre>



</details>

<a name="orderbook_balance_manager_trader"></a>

## Function `trader`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_trader">trader</a>(trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_trader">trader</a>(trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">TradeProof</a>): <b>address</b> {
    trade_proof.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_trader">trader</a>
}
</code></pre>



</details>

<a name="orderbook_balance_manager_emit_balance_event"></a>

## Function `emit_balance_event`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_emit_balance_event">emit_balance_event</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, asset: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>, amount: u64, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit">deposit</a>: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_emit_balance_event">emit_balance_event</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    asset: TypeName,
    amount: u64,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit">deposit</a>: bool,
) {
    event::emit(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceEvent">BalanceEvent</a> {
        balance_manager_id: <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>(),
        asset,
        amount,
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_deposit">deposit</a>,
    });
}
</code></pre>



</details>

<a name="orderbook_balance_manager_mint_trade_cap_internal"></a>

## Function `mint_trade_cap_internal`



<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_trade_cap_internal">mint_trade_cap_internal</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_trade_cap_internal">mint_trade_cap_internal</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, ctx: &<b>mut</b> TxContext): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a> {
    <b>assert</b>!(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.length() &lt; <a href="../orderbook/balance_manager.md#orderbook_balance_manager_MAX_TRADE_CAPS">MAX_TRADE_CAPS</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EMaxCapsReached">EMaxCapsReached</a>);
    <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a> = object::new(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.insert(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner());
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a> {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>,
        balance_manager_id: object::id(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>),
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_mint_deposit_cap_internal"></a>

## Function `mint_deposit_cap_internal`



<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_deposit_cap_internal">mint_deposit_cap_internal</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_deposit_cap_internal">mint_deposit_cap_internal</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a> {
    <b>assert</b>!(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.length() &lt; <a href="../orderbook/balance_manager.md#orderbook_balance_manager_MAX_TRADE_CAPS">MAX_TRADE_CAPS</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EMaxCapsReached">EMaxCapsReached</a>);
    <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a> = object::new(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.insert(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner());
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a> {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>,
        balance_manager_id: object::id(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>),
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_mint_withdraw_cap_internal"></a>

## Function `mint_withdraw_cap_internal`



<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_withdraw_cap_internal">mint_withdraw_cap_internal</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_mint_withdraw_cap_internal">mint_withdraw_cap_internal</a>(
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a> {
    <b>assert</b>!(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.length() &lt; <a href="../orderbook/balance_manager.md#orderbook_balance_manager_MAX_TRADE_CAPS">MAX_TRADE_CAPS</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EMaxCapsReached">EMaxCapsReached</a>);
    <b>let</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a> = object::new(ctx);
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.insert(<a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>.to_inner());
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a> {
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager_id">id</a>,
        balance_manager_id: object::id(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>),
    }
}
</code></pre>



</details>

<a name="orderbook_balance_manager_validate_owner"></a>

## Function `validate_owner`



<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_owner">validate_owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_owner">validate_owner</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.<a href="../orderbook/balance_manager.md#orderbook_balance_manager_owner">owner</a>(), <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidOwner">EInvalidOwner</a>);
}
</code></pre>



</details>

<a name="orderbook_balance_manager_validate_trader"></a>

## Function `validate_trader`



<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_trader">validate_trader</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">orderbook::balance_manager::TradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_trader">validate_trader</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, trade_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeCap">TradeCap</a>) {
    <b>assert</b>!(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.contains(object::borrow_id(trade_cap)), <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidTrader">EInvalidTrader</a>);
}
</code></pre>



</details>

<a name="orderbook_balance_manager_validate_deposit_cap"></a>

## Function `validate_deposit_cap`



<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_deposit_cap">validate_deposit_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, deposit_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">orderbook::balance_manager::DepositCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_deposit_cap">validate_deposit_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, deposit_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_DepositCap">DepositCap</a>) {
    <b>assert</b>!(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.contains(object::borrow_id(deposit_cap)), <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidTrader">EInvalidTrader</a>);
}
</code></pre>



</details>

<a name="orderbook_balance_manager_validate_withdraw_cap"></a>

## Function `validate_withdraw_cap`



<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_withdraw_cap">validate_withdraw_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, withdraw_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">orderbook::balance_manager::WithdrawCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_validate_withdraw_cap">validate_withdraw_cap</a>(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">BalanceManager</a>, withdraw_cap: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_WithdrawCap">WithdrawCap</a>) {
    <b>assert</b>!(<a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.allow_listed.contains(object::borrow_id(withdraw_cap)), <a href="../orderbook/balance_manager.md#orderbook_balance_manager_EInvalidTrader">EInvalidTrader</a>);
}
</code></pre>



</details>
