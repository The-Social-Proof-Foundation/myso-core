---
title: Module `orderbook::vault`
---

The vault holds all of the assets for this pool. At the end of all
transaction processing, the vault is used to settle the balances for the user.


-  [Struct `Vault`](#orderbook_vault_Vault)
-  [Struct `FlashLoan`](#orderbook_vault_FlashLoan)
-  [Struct `FlashLoanBorrowed`](#orderbook_vault_FlashLoanBorrowed)
-  [Constants](#@Constants_0)
-  [Function `balances`](#orderbook_vault_balances)
-  [Function `empty`](#orderbook_vault_empty)
-  [Function `settle_balance_manager`](#orderbook_vault_settle_balance_manager)
-  [Function `settle_balance_manager_permissionless`](#orderbook_vault_settle_balance_manager_permissionless)
-  [Function `withdraw_myso_to_burn`](#orderbook_vault_withdraw_myso_to_burn)
-  [Function `borrow_flashloan_base`](#orderbook_vault_borrow_flashloan_base)
-  [Function `borrow_flashloan_quote`](#orderbook_vault_borrow_flashloan_quote)
-  [Function `return_flashloan_base`](#orderbook_vault_return_flashloan_base)
-  [Function `return_flashloan_quote`](#orderbook_vault_return_flashloan_quote)


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
<b>use</b> <a href="../myso/versioned.md#myso_versioned">myso::versioned</a>;
<b>use</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">orderbook::balance_manager</a>;
<b>use</b> <a href="../orderbook/balances.md#orderbook_balances">orderbook::balances</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
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



<a name="orderbook_vault_Vault"></a>

## Struct `Vault`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;<b>phantom</b> BaseAsset, <b>phantom</b> QuoteAsset&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>base_balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;BaseAsset&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>quote_balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;QuoteAsset&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_vault_FlashLoan"></a>

## Struct `FlashLoan`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">FlashLoan</a>
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
<code>borrow_quantity: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>type_name: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_vault_FlashLoanBorrowed"></a>

## Struct `FlashLoanBorrowed`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/vault.md#orderbook_vault_FlashLoanBorrowed">FlashLoanBorrowed</a> <b>has</b> <b>copy</b>, drop
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
<code>borrow_quantity: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>type_name: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="orderbook_vault_ENotEnoughBaseForLoan"></a>



<pre><code><b>const</b> <a href="../orderbook/vault.md#orderbook_vault_ENotEnoughBaseForLoan">ENotEnoughBaseForLoan</a>: u64 = 1;
</code></pre>



<a name="orderbook_vault_ENotEnoughQuoteForLoan"></a>



<pre><code><b>const</b> <a href="../orderbook/vault.md#orderbook_vault_ENotEnoughQuoteForLoan">ENotEnoughQuoteForLoan</a>: u64 = 2;
</code></pre>



<a name="orderbook_vault_EInvalidLoanQuantity"></a>



<pre><code><b>const</b> <a href="../orderbook/vault.md#orderbook_vault_EInvalidLoanQuantity">EInvalidLoanQuantity</a>: u64 = 3;
</code></pre>



<a name="orderbook_vault_EIncorrectLoanPool"></a>



<pre><code><b>const</b> <a href="../orderbook/vault.md#orderbook_vault_EIncorrectLoanPool">EIncorrectLoanPool</a>: u64 = 4;
</code></pre>



<a name="orderbook_vault_EIncorrectTypeReturned"></a>



<pre><code><b>const</b> <a href="../orderbook/vault.md#orderbook_vault_EIncorrectTypeReturned">EIncorrectTypeReturned</a>: u64 = 5;
</code></pre>



<a name="orderbook_vault_EIncorrectQuantityReturned"></a>



<pre><code><b>const</b> <a href="../orderbook/vault.md#orderbook_vault_EIncorrectQuantityReturned">EIncorrectQuantityReturned</a>: u64 = 6;
</code></pre>



<a name="orderbook_vault_ENoBalanceToSettle"></a>



<pre><code><b>const</b> <a href="../orderbook/vault.md#orderbook_vault_ENoBalanceToSettle">ENoBalanceToSettle</a>: u64 = 7;
</code></pre>



<a name="orderbook_vault_EHasOwedBalances"></a>



<pre><code><b>const</b> <a href="../orderbook/vault.md#orderbook_vault_EHasOwedBalances">EHasOwedBalances</a>: u64 = 8;
</code></pre>



<a name="orderbook_vault_balances"></a>

## Function `balances`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances">balances</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances">balances</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;BaseAsset, QuoteAsset&gt;,
): (u64, u64, u64) {
    (self.base_balance.value(), self.quote_balance.value(), self.myso_balance.value())
}
</code></pre>



</details>

<a name="orderbook_vault_empty"></a>

## Function `empty`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_empty">empty</a>&lt;BaseAsset, QuoteAsset&gt;(): <a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_empty">empty</a>&lt;BaseAsset, QuoteAsset&gt;(): <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;BaseAsset, QuoteAsset&gt; {
    <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a> {
        base_balance: balance::zero(),
        quote_balance: balance::zero(),
        myso_balance: balance::zero(),
    }
}
</code></pre>



</details>

<a name="orderbook_vault_settle_balance_manager"></a>

## Function `settle_balance_manager`

Transfer any settled amounts for the <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_settle_balance_manager">settle_balance_manager</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;, balances_out: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, balances_in: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>, trade_proof: &<a href="../orderbook/balance_manager.md#orderbook_balance_manager_TradeProof">orderbook::balance_manager::TradeProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_settle_balance_manager">settle_balance_manager</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;BaseAsset, QuoteAsset&gt;,
    balances_out: Balances,
    balances_in: Balances,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
    trade_proof: &TradeProof,
) {
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.validate_proof(trade_proof);
    <b>if</b> (balances_out.base() &gt; balances_in.base()) {
        <b>let</b> balance = self.base_balance.split(balances_out.base() - balances_in.base());
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.deposit_with_proof(trade_proof, balance);
    };
    <b>if</b> (balances_out.quote() &gt; balances_in.quote()) {
        <b>let</b> balance = self.quote_balance.split(balances_out.quote() - balances_in.quote());
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.deposit_with_proof(trade_proof, balance);
    };
    <b>if</b> (balances_out.myso() &gt; balances_in.myso()) {
        <b>let</b> balance = self.myso_balance.split(balances_out.myso() - balances_in.myso());
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.deposit_with_proof(trade_proof, balance);
    };
    <b>if</b> (balances_in.base() &gt; balances_out.base()) {
        <b>let</b> balance = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.withdraw_with_proof(
            trade_proof,
            balances_in.base() - balances_out.base(),
            <b>false</b>,
        );
        self.base_balance.join(balance);
    };
    <b>if</b> (balances_in.quote() &gt; balances_out.quote()) {
        <b>let</b> balance = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.withdraw_with_proof(
            trade_proof,
            balances_in.quote() - balances_out.quote(),
            <b>false</b>,
        );
        self.quote_balance.join(balance);
    };
    <b>if</b> (balances_in.myso() &gt; balances_out.myso()) {
        <b>let</b> balance = <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.withdraw_with_proof(
            trade_proof,
            balances_in.myso() - balances_out.myso(),
            <b>false</b>,
        );
        self.myso_balance.join(balance);
    };
}
</code></pre>



</details>

<a name="orderbook_vault_settle_balance_manager_permissionless"></a>

## Function `settle_balance_manager_permissionless`

Transfer any settled amounts for the <code><a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_settle_balance_manager_permissionless">settle_balance_manager_permissionless</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;, balances_out: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, balances_in: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager_BalanceManager">orderbook::balance_manager::BalanceManager</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_settle_balance_manager_permissionless">settle_balance_manager_permissionless</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;BaseAsset, QuoteAsset&gt;,
    balances_out: Balances,
    balances_in: Balances,
    <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>: &<b>mut</b> BalanceManager,
) {
    <b>assert</b>!(
        balances_in.base() == 0 && balances_in.quote() == 0 && balances_in.myso() == 0,
        <a href="../orderbook/vault.md#orderbook_vault_EHasOwedBalances">EHasOwedBalances</a>,
    );
    <b>let</b> has_settled_balances =
        balances_out.base() &gt; 0
        || balances_out.quote() &gt; 0
        || balances_out.myso() &gt; 0;
    <b>assert</b>!(has_settled_balances, <a href="../orderbook/vault.md#orderbook_vault_ENoBalanceToSettle">ENoBalanceToSettle</a>);
    <b>if</b> (balances_out.base() &gt; 0) {
        <b>let</b> balance = self.base_balance.split(balances_out.base());
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.deposit_permissionless(balance);
    };
    <b>if</b> (balances_out.quote() &gt; 0) {
        <b>let</b> balance = self.quote_balance.split(balances_out.quote());
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.deposit_permissionless(balance);
    };
    <b>if</b> (balances_out.myso() &gt; 0) {
        <b>let</b> balance = self.myso_balance.split(balances_out.myso());
        <a href="../orderbook/balance_manager.md#orderbook_balance_manager">balance_manager</a>.deposit_permissionless(balance);
    };
}
</code></pre>



</details>

<a name="orderbook_vault_withdraw_myso_to_burn"></a>

## Function `withdraw_myso_to_burn`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_withdraw_myso_to_burn">withdraw_myso_to_burn</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;, amount_to_burn: u64): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_withdraw_myso_to_burn">withdraw_myso_to_burn</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;BaseAsset, QuoteAsset&gt;,
    amount_to_burn: u64,
): Balance&lt;MYSO&gt; {
    self.myso_balance.split(amount_to_burn)
}
</code></pre>



</details>

<a name="orderbook_vault_borrow_flashloan_base"></a>

## Function `borrow_flashloan_base`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_borrow_flashloan_base">borrow_flashloan_base</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, borrow_quantity: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">orderbook::vault::FlashLoan</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_borrow_flashloan_base">borrow_flashloan_base</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;BaseAsset, QuoteAsset&gt;,
    pool_id: ID,
    borrow_quantity: u64,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;BaseAsset&gt;, <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">FlashLoan</a>) {
    <b>assert</b>!(borrow_quantity &gt; 0, <a href="../orderbook/vault.md#orderbook_vault_EInvalidLoanQuantity">EInvalidLoanQuantity</a>);
    <b>assert</b>!(self.base_balance.value() &gt;= borrow_quantity, <a href="../orderbook/vault.md#orderbook_vault_ENotEnoughBaseForLoan">ENotEnoughBaseForLoan</a>);
    <b>let</b> borrow_type_name = type_name::with_defining_ids&lt;BaseAsset&gt;();
    <b>let</b> borrow: Coin&lt;BaseAsset&gt; = self.base_balance.split(borrow_quantity).into_coin(ctx);
    <b>let</b> flash_loan = <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">FlashLoan</a> {
        pool_id,
        borrow_quantity,
        type_name: borrow_type_name,
    };
    event::emit(<a href="../orderbook/vault.md#orderbook_vault_FlashLoanBorrowed">FlashLoanBorrowed</a> {
        pool_id,
        borrow_quantity,
        type_name: borrow_type_name,
    });
    (borrow, flash_loan)
}
</code></pre>



</details>

<a name="orderbook_vault_borrow_flashloan_quote"></a>

## Function `borrow_flashloan_quote`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_borrow_flashloan_quote">borrow_flashloan_quote</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, borrow_quantity: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">orderbook::vault::FlashLoan</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_borrow_flashloan_quote">borrow_flashloan_quote</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;BaseAsset, QuoteAsset&gt;,
    pool_id: ID,
    borrow_quantity: u64,
    ctx: &<b>mut</b> TxContext,
): (Coin&lt;QuoteAsset&gt;, <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">FlashLoan</a>) {
    <b>assert</b>!(borrow_quantity &gt; 0, <a href="../orderbook/vault.md#orderbook_vault_EInvalidLoanQuantity">EInvalidLoanQuantity</a>);
    <b>assert</b>!(self.quote_balance.value() &gt;= borrow_quantity, <a href="../orderbook/vault.md#orderbook_vault_ENotEnoughQuoteForLoan">ENotEnoughQuoteForLoan</a>);
    <b>let</b> borrow_type_name = type_name::with_defining_ids&lt;QuoteAsset&gt;();
    <b>let</b> borrow: Coin&lt;QuoteAsset&gt; = self.quote_balance.split(borrow_quantity).into_coin(ctx);
    <b>let</b> flash_loan = <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">FlashLoan</a> {
        pool_id,
        borrow_quantity,
        type_name: borrow_type_name,
    };
    event::emit(<a href="../orderbook/vault.md#orderbook_vault_FlashLoanBorrowed">FlashLoanBorrowed</a> {
        pool_id,
        borrow_quantity,
        type_name: borrow_type_name,
    });
    (borrow, flash_loan)
}
</code></pre>



</details>

<a name="orderbook_vault_return_flashloan_base"></a>

## Function `return_flashloan_base`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_return_flashloan_base">return_flashloan_base</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;BaseAsset&gt;, flash_loan: <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">orderbook::vault::FlashLoan</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_return_flashloan_base">return_flashloan_base</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;BaseAsset, QuoteAsset&gt;,
    pool_id: ID,
    coin: Coin&lt;BaseAsset&gt;,
    flash_loan: <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">FlashLoan</a>,
) {
    <b>assert</b>!(pool_id == flash_loan.pool_id, <a href="../orderbook/vault.md#orderbook_vault_EIncorrectLoanPool">EIncorrectLoanPool</a>);
    <b>assert</b>!(
        type_name::with_defining_ids&lt;BaseAsset&gt;() == flash_loan.type_name,
        <a href="../orderbook/vault.md#orderbook_vault_EIncorrectTypeReturned">EIncorrectTypeReturned</a>,
    );
    <b>assert</b>!(coin.value() == flash_loan.borrow_quantity, <a href="../orderbook/vault.md#orderbook_vault_EIncorrectQuantityReturned">EIncorrectQuantityReturned</a>);
    self.base_balance.join(coin.into_balance&lt;BaseAsset&gt;());
    <b>let</b> <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">FlashLoan</a> {
        pool_id: _,
        borrow_quantity: _,
        type_name: _,
    } = flash_loan;
}
</code></pre>



</details>

<a name="orderbook_vault_return_flashloan_quote"></a>

## Function `return_flashloan_quote`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_return_flashloan_quote">return_flashloan_quote</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">orderbook::vault::Vault</a>&lt;BaseAsset, QuoteAsset&gt;, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;QuoteAsset&gt;, flash_loan: <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">orderbook::vault::FlashLoan</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/vault.md#orderbook_vault_return_flashloan_quote">return_flashloan_quote</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &<b>mut</b> <a href="../orderbook/vault.md#orderbook_vault_Vault">Vault</a>&lt;BaseAsset, QuoteAsset&gt;,
    pool_id: ID,
    coin: Coin&lt;QuoteAsset&gt;,
    flash_loan: <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">FlashLoan</a>,
) {
    <b>assert</b>!(pool_id == flash_loan.pool_id, <a href="../orderbook/vault.md#orderbook_vault_EIncorrectLoanPool">EIncorrectLoanPool</a>);
    <b>assert</b>!(
        type_name::with_defining_ids&lt;QuoteAsset&gt;() == flash_loan.type_name,
        <a href="../orderbook/vault.md#orderbook_vault_EIncorrectTypeReturned">EIncorrectTypeReturned</a>,
    );
    <b>assert</b>!(coin.value() == flash_loan.borrow_quantity, <a href="../orderbook/vault.md#orderbook_vault_EIncorrectQuantityReturned">EIncorrectQuantityReturned</a>);
    self.quote_balance.join(coin.into_balance&lt;QuoteAsset&gt;());
    <b>let</b> <a href="../orderbook/vault.md#orderbook_vault_FlashLoan">FlashLoan</a> {
        pool_id: _,
        borrow_quantity: _,
        type_name: _,
    } = flash_loan;
}
</code></pre>



</details>
