---
title: Module `social_contracts::poc_vault`
---



-  [Struct `VaultBalanceKey`](#social_contracts_poc_vault_VaultBalanceKey)
-  [Struct `PoCVaultDirectory`](#social_contracts_poc_vault_PoCVaultDirectory)
-  [Struct `PoCBeneficiaryVault`](#social_contracts_poc_vault_PoCBeneficiaryVault)
-  [Struct `PoCBadgeObject`](#social_contracts_poc_vault_PoCBadgeObject)
-  [Struct `PoCBeneficiaryVaultDepositEvent`](#social_contracts_poc_vault_PoCBeneficiaryVaultDepositEvent)
-  [Struct `PoCBeneficiaryVaultClaimedEvent`](#social_contracts_poc_vault_PoCBeneficiaryVaultClaimedEvent)
-  [Constants](#@Constants_0)
-  [Function `media_index_unspecified`](#social_contracts_poc_vault_media_index_unspecified)
-  [Function `bootstrap_init_directory`](#social_contracts_poc_vault_bootstrap_init_directory)
-  [Function `beneficiary_address`](#social_contracts_poc_vault_beneficiary_address)
-  [Function `ensure_beneficiary_vault`](#social_contracts_poc_vault_ensure_beneficiary_vault)
-  [Function `deposit_coin`](#social_contracts_poc_vault_deposit_coin)
-  [Function `claim_vault_balance`](#social_contracts_poc_vault_claim_vault_balance)
-  [Function `new_poc_badge_object`](#social_contracts_poc_vault_new_poc_badge_object)
-  [Function `share_po_badge_object`](#social_contracts_poc_vault_share_po_badge_object)
-  [Function `po_badge_object_address`](#social_contracts_poc_vault_po_badge_object_address)


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
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">social_contracts::subscription</a>;
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



<a name="social_contracts_poc_vault_VaultBalanceKey"></a>

## Struct `VaultBalanceKey`

Bag key for <code>Balance&lt;T&gt;</code> buckets (same phantom-key pattern as orderbook <code>BalanceKey</code>).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_VaultBalanceKey">VaultBalanceKey</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_poc_vault_PoCVaultDirectory"></a>

## Struct `PoCVaultDirectory`

Maps beneficiary wallet → shared <code><a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">PoCBeneficiaryVault</a></code> object address (lookup only).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCVaultDirectory">PoCVaultDirectory</a> <b>has</b> key
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
<code>vault_by_beneficiary: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
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

<a name="social_contracts_poc_vault_PoCBeneficiaryVault"></a>

## Struct `PoCBeneficiaryVault`

One shared vault per beneficiary; anyone may deposit; only beneficiary may claim per coin type.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">PoCBeneficiaryVault</a> <b>has</b> key
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
<code>beneficiary: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>balances: <a href="../myso/bag.md#myso_bag_Bag">myso::bag::Bag</a></code>
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

<a name="social_contracts_poc_vault_PoCBadgeObject"></a>

## Struct `PoCBadgeObject`

Authoritative on-chain PoC badge record for a post (shared object).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBadgeObject">PoCBadgeObject</a> <b>has</b> key
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
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_beneficiary_address">beneficiary_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>matched_anchor_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>media_index: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>similarity_score: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>media_type: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>oracle_address: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>analyzed_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
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

<a name="social_contracts_poc_vault_PoCBeneficiaryVaultDepositEvent"></a>

## Struct `PoCBeneficiaryVaultDepositEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVaultDepositEvent">PoCBeneficiaryVaultDepositEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>beneficiary: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>coin_type: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>source_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
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

<a name="social_contracts_poc_vault_PoCBeneficiaryVaultClaimedEvent"></a>

## Struct `PoCBeneficiaryVaultClaimedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVaultClaimedEvent">PoCBeneficiaryVaultClaimedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>beneficiary: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>coin_type: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>referrer: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>referrer_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>beneficiary_amount: u64</code>
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

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_poc_vault_EUnauthorized"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EUnauthorized">EUnauthorized</a>: u64 = 1;
</code></pre>



<a name="social_contracts_poc_vault_EWrongBeneficiary"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EWrongBeneficiary">EWrongBeneficiary</a>: u64 = 2;
</code></pre>



<a name="social_contracts_poc_vault_EVaultEmpty"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EVaultEmpty">EVaultEmpty</a>: u64 = 3;
</code></pre>



<a name="social_contracts_poc_vault_EInvalidReferrer"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EInvalidReferrer">EInvalidReferrer</a>: u64 = 4;
</code></pre>



<a name="social_contracts_poc_vault_EBpsTooLarge"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EBpsTooLarge">EBpsTooLarge</a>: u64 = 5;
</code></pre>



<a name="social_contracts_poc_vault_EDEPOSIT_BELOW_MINIMUM"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EDEPOSIT_BELOW_MINIMUM">EDEPOSIT_BELOW_MINIMUM</a>: u64 = 6;
</code></pre>



<a name="social_contracts_poc_vault_EClaimInvariant"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EClaimInvariant">EClaimInvariant</a>: u64 = 7;
</code></pre>



<a name="social_contracts_poc_vault_MIN_VAULT_DEPOSIT_AMOUNT"></a>

Minimum amount (per asset) accepted into the vault; configurable later via governance if needed.


<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_MIN_VAULT_DEPOSIT_AMOUNT">MIN_VAULT_DEPOSIT_AMOUNT</a>: u64 = 1;
</code></pre>



<a name="social_contracts_poc_vault_media_index_unspecified"></a>

## Function `media_index_unspecified`

Sentinel media index when oracle did not bind to a specific attachment slot.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_media_index_unspecified">media_index_unspecified</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_media_index_unspecified">media_index_unspecified</a>(): u8 {
    255
}
</code></pre>



</details>

<a name="social_contracts_poc_vault_bootstrap_init_directory"></a>

## Function `bootstrap_init_directory`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_bootstrap_init_directory">bootstrap_init_directory</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_bootstrap_init_directory">bootstrap_init_directory</a>(ctx: &<b>mut</b> TxContext) {
    transfer::share_object(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCVaultDirectory">PoCVaultDirectory</a> {
        id: object::new(ctx),
        vault_by_beneficiary: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    });
}
</code></pre>



</details>

<a name="social_contracts_poc_vault_beneficiary_address"></a>

## Function `beneficiary_address`

Beneficiary wallet for this vault (depositor routing assertions).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_beneficiary_address">beneficiary_address</a>(vault: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_beneficiary_address">beneficiary_address</a>(vault: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">PoCBeneficiaryVault</a>): <b>address</b> {
    vault.beneficiary
}
</code></pre>



</details>

<a name="social_contracts_poc_vault_ensure_beneficiary_vault"></a>

## Function `ensure_beneficiary_vault`

Returns the vault object address for <code>beneficiary</code>, creating and sharing a vault if needed.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_ensure_beneficiary_vault">ensure_beneficiary_vault</a>(directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCVaultDirectory">social_contracts::poc_vault::PoCVaultDirectory</a>, beneficiary: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_ensure_beneficiary_vault">ensure_beneficiary_vault</a>(
    directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCVaultDirectory">PoCVaultDirectory</a>,
    beneficiary: <b>address</b>,
    ctx: &<b>mut</b> TxContext
): <b>address</b> {
    <b>if</b> (table::contains(&directory.vault_by_beneficiary, beneficiary)) {
        *table::borrow(&directory.vault_by_beneficiary, beneficiary)
    } <b>else</b> {
        <b>let</b> vault = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">PoCBeneficiaryVault</a> {
            id: object::new(ctx),
            beneficiary,
            balances: bag::new(ctx),
            version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        };
        <b>let</b> vault_address = object::uid_to_address(&vault.id);
        transfer::share_object(vault);
        table::add(&<b>mut</b> directory.vault_by_beneficiary, beneficiary, vault_address);
        vault_address
    }
}
</code></pre>



</details>

<a name="social_contracts_poc_vault_deposit_coin"></a>

## Function `deposit_coin`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_deposit_coin">deposit_coin</a>&lt;T&gt;(vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, expected_beneficiary: <b>address</b>, fee_coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, source_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_deposit_coin">deposit_coin</a>&lt;T&gt;(
    vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">PoCBeneficiaryVault</a>,
    expected_beneficiary: <b>address</b>,
    fee_coin: Coin&lt;T&gt;,
    source_post_id: Option&lt;<b>address</b>&gt;,
    ctx: &TxContext
) {
    <b>assert</b>!(vault.beneficiary == expected_beneficiary, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EWrongBeneficiary">EWrongBeneficiary</a>);
    <b>let</b> amount = coin::value(&fee_coin);
    <b>if</b> (amount == 0) {
        coin::destroy_zero(fee_coin);
        <b>return</b>
    };
    <b>assert</b>!(amount &gt;= <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_MIN_VAULT_DEPOSIT_AMOUNT">MIN_VAULT_DEPOSIT_AMOUNT</a>, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EDEPOSIT_BELOW_MINIMUM">EDEPOSIT_BELOW_MINIMUM</a>);
    <b>let</b> vault_id = object::uid_to_address(&vault.id);
    <b>let</b> coin_type = type_name::with_defining_ids&lt;T&gt;();
    <b>let</b> key = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_VaultBalanceKey">VaultBalanceKey</a>&lt;T&gt; {};
    <b>let</b> incoming = coin::into_balance(fee_coin);
    <b>if</b> (bag::contains(&vault.balances, key)) {
        <b>let</b> slot: &<b>mut</b> Balance&lt;T&gt; = bag::borrow_mut(&<b>mut</b> vault.balances, key);
        balance::join(slot, incoming);
    } <b>else</b> {
        bag::add(&<b>mut</b> vault.balances, key, incoming);
    };
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVaultDepositEvent">PoCBeneficiaryVaultDepositEvent</a> {
        vault_id,
        beneficiary: vault.beneficiary,
        coin_type,
        amount,
        source_post_id,
        timestamp: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_poc_vault_claim_vault_balance"></a>

## Function `claim_vault_balance`

Claim entire balance for coin type <code>T</code> with treasury fee (bps) and optional referrer slice.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_claim_vault_balance">claim_vault_balance</a>&lt;T&gt;(vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, treasury_fee_bps: u64, max_referral_bps: u64, referrer_opt: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_claim_vault_balance">claim_vault_balance</a>&lt;T&gt;(
    vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">PoCBeneficiaryVault</a>,
    treasury: &EcosystemTreasury,
    treasury_fee_bps: u64,
    max_referral_bps: u64,
    referrer_opt: Option&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == vault.beneficiary, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(treasury_fee_bps &lt;= 10000 && max_referral_bps &lt;= 10000, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EBpsTooLarge">EBpsTooLarge</a>);
    <b>let</b> key = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_VaultBalanceKey">VaultBalanceKey</a>&lt;T&gt; {};
    <b>assert</b>!(
        bag::contains_with_type&lt;<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_VaultBalanceKey">VaultBalanceKey</a>&lt;T&gt;, Balance&lt;T&gt;&gt;(&vault.balances, key),
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EVaultEmpty">EVaultEmpty</a>
    );
    <b>let</b> stored_balance = bag::remove&lt;<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_VaultBalanceKey">VaultBalanceKey</a>&lt;T&gt;, Balance&lt;T&gt;&gt;(&<b>mut</b> vault.balances, key);
    <b>let</b> gross = balance::value(&stored_balance);
    <b>assert</b>!(gross &gt; 0, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EVaultEmpty">EVaultEmpty</a>);
    <b>if</b> (option::is_some(&referrer_opt)) {
        <b>let</b> r = *option::borrow(&referrer_opt);
        <b>assert</b>!(r != @0x0 && r != vault.beneficiary, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EInvalidReferrer">EInvalidReferrer</a>);
    };
    <b>let</b> treasury_amt = (gross * treasury_fee_bps) / 10000;
    <b>let</b> after_treasury = gross - treasury_amt;
    <b>let</b> referrer_amt = <b>if</b> (option::is_some(&referrer_opt)) {
        (after_treasury * max_referral_bps) / 10000
    } <b>else</b> {
        0
    };
    <b>let</b> beneficiary_amt = gross - treasury_amt - referrer_amt;
    <b>assert</b>!(
        treasury_amt + referrer_amt + beneficiary_amt == gross,
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_EClaimInvariant">EClaimInvariant</a>
    );
    <b>let</b> <b>mut</b> all_coin = coin::from_balance(stored_balance, ctx);
    <b>if</b> (treasury_amt &gt; 0) {
        <b>let</b> treasury_coin = coin::split(&<b>mut</b> all_coin, treasury_amt, ctx);
        transfer::public_transfer(treasury_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    <b>if</b> (referrer_amt &gt; 0) {
        <b>let</b> ref_addr = *option::borrow(&referrer_opt);
        <b>let</b> ref_coin = coin::split(&<b>mut</b> all_coin, referrer_amt, ctx);
        transfer::public_transfer(ref_coin, ref_addr);
    };
    <b>if</b> (beneficiary_amt &gt; 0) {
        <b>let</b> ben_coin = coin::split(&<b>mut</b> all_coin, beneficiary_amt, ctx);
        transfer::public_transfer(ben_coin, vault.beneficiary);
    };
    coin::destroy_zero(all_coin);
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVaultClaimedEvent">PoCBeneficiaryVaultClaimedEvent</a> {
        vault_id: object::uid_to_address(&vault.id),
        beneficiary: vault.beneficiary,
        coin_type: type_name::with_defining_ids&lt;T&gt;(),
        referrer: referrer_opt,
        treasury_amount: treasury_amt,
        referrer_amount: referrer_amt,
        beneficiary_amount: beneficiary_amt,
        timestamp: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_poc_vault_new_poc_badge_object"></a>

## Function `new_poc_badge_object`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_new_poc_badge_object">new_poc_badge_object</a>(post_id: <b>address</b>, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_beneficiary_address">beneficiary_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, matched_anchor_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, media_index: u8, reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, evidence_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, similarity_score: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, media_type: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;, oracle_address: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, analyzed_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBadgeObject">social_contracts::poc_vault::PoCBadgeObject</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_new_poc_badge_object">new_poc_badge_object</a>(
    post_id: <b>address</b>,
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_beneficiary_address">beneficiary_address</a>: Option&lt;<b>address</b>&gt;,
    matched_anchor_id: Option&lt;<b>address</b>&gt;,
    media_index: u8,
    reasoning: Option&lt;String&gt;,
    evidence_urls: Option&lt;vector&lt;String&gt;&gt;,
    similarity_score: Option&lt;u64&gt;,
    media_type: Option&lt;u8&gt;,
    oracle_address: Option&lt;<b>address</b>&gt;,
    analyzed_at: Option&lt;u64&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBadgeObject">PoCBadgeObject</a> {
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBadgeObject">PoCBadgeObject</a> {
        id: object::new(ctx),
        post_id,
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_beneficiary_address">beneficiary_address</a>,
        matched_anchor_id,
        media_index,
        reasoning,
        evidence_urls,
        similarity_score,
        media_type,
        oracle_address,
        analyzed_at,
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    }
}
</code></pre>



</details>

<a name="social_contracts_poc_vault_share_po_badge_object"></a>

## Function `share_po_badge_object`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_share_po_badge_object">share_po_badge_object</a>(badge: <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBadgeObject">social_contracts::poc_vault::PoCBadgeObject</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_share_po_badge_object">share_po_badge_object</a>(badge: <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBadgeObject">PoCBadgeObject</a>) {
    transfer::share_object(badge);
}
</code></pre>



</details>

<a name="social_contracts_poc_vault_po_badge_object_address"></a>

## Function `po_badge_object_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_po_badge_object_address">po_badge_object_address</a>(badge: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBadgeObject">social_contracts::poc_vault::PoCBadgeObject</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_po_badge_object_address">po_badge_object_address</a>(badge: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBadgeObject">PoCBadgeObject</a>): <b>address</b> {
    object::uid_to_address(&badge.id)
}
</code></pre>



</details>
