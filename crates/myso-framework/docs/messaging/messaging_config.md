---
title: Module `messaging::messaging_config`
---

Global configuration for paid messaging (fees, reply rules, dedupe limits).


-  [Struct `MessagingConfig`](#messaging_messaging_config_MessagingConfig)
-  [Struct `MessagingConfigUpdatedEvent`](#messaging_messaging_config_MessagingConfigUpdatedEvent)
-  [Constants](#@Constants_0)
-  [Function `share_initial`](#messaging_messaging_config_share_initial)
-  [Function `update_messaging_config`](#messaging_messaging_config_update_messaging_config)
-  [Function `paid_msg_platform_fee_bps`](#messaging_messaging_config_paid_msg_platform_fee_bps)
-  [Function `paid_msg_treasury_fee_bps`](#messaging_messaging_config_paid_msg_treasury_fee_bps)
-  [Function `payment_expiration_ms`](#messaging_messaging_config_payment_expiration_ms)
-  [Function `min_reply_chars`](#messaging_messaging_config_min_reply_chars)
-  [Function `max_dedupe_key_bytes`](#messaging_messaging_config_max_dedupe_key_bytes)
-  [Function `new_defaults`](#messaging_messaging_config_new_defaults)


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
<b>use</b> <a href="../myso/versioned.md#myso_versioned">myso::versioned</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/registry.md#orderbook_registry">orderbook::registry</a>;
<b>use</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit">social_contracts::ai_credit</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">social_contracts::bootstrap</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/insurance.md#social_contracts_insurance">social_contracts::insurance</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">social_contracts::mydata</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary">social_contracts::poc_username_beneficiary</a>;
<b>use</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault">social_contracts::poc_vault</a>;
<b>use</b> <a href="../social_contracts/post.md#social_contracts_post">social_contracts::post</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity">social_contracts::proof_of_creativity</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
<b>use</b> <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth">social_contracts::social_proof_of_truth</a>;
<b>use</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens">social_contracts::social_proof_tokens</a>;
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
<b>use</b> <a href="../std/u256.md#std_u256">std::u256</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="messaging_messaging_config_MessagingConfig"></a>

## Struct `MessagingConfig`

Shared singleton for paid-messaging parameters.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a> <b>has</b> key
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
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>: u32</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="messaging_messaging_config_MessagingConfigUpdatedEvent"></a>

## Struct `MessagingConfigUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfigUpdatedEvent">MessagingConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>: u32</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="messaging_messaging_config_BPS_DENOM"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_BPS_DENOM">BPS_DENOM</a>: u64 = 10000;
</code></pre>



<a name="messaging_messaging_config_DEFAULT_PAID_MSG_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_PAID_MSG_PLATFORM_FEE_BPS">DEFAULT_PAID_MSG_PLATFORM_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="messaging_messaging_config_DEFAULT_PAID_MSG_TREASURY_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_PAID_MSG_TREASURY_FEE_BPS">DEFAULT_PAID_MSG_TREASURY_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="messaging_messaging_config_DEFAULT_PAYMENT_EXPIRATION_MS"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_PAYMENT_EXPIRATION_MS">DEFAULT_PAYMENT_EXPIRATION_MS</a>: u64 = 2592000000;
</code></pre>



<a name="messaging_messaging_config_DEFAULT_MIN_REPLY_CHARS"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_MIN_REPLY_CHARS">DEFAULT_MIN_REPLY_CHARS</a>: u32 = 6;
</code></pre>



<a name="messaging_messaging_config_DEFAULT_MAX_DEDUPE_KEY_BYTES"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_MAX_DEDUPE_KEY_BYTES">DEFAULT_MAX_DEDUPE_KEY_BYTES</a>: u64 = 256;
</code></pre>



<a name="messaging_messaging_config_EInvalidFeeBps"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_EInvalidFeeBps">EInvalidFeeBps</a>: u64 = 0;
</code></pre>



<a name="messaging_messaging_config_EInvalidPaymentExpiration"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_EInvalidPaymentExpiration">EInvalidPaymentExpiration</a>: u64 = 1;
</code></pre>



<a name="messaging_messaging_config_EInvalidMinReplyChars"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_EInvalidMinReplyChars">EInvalidMinReplyChars</a>: u64 = 2;
</code></pre>



<a name="messaging_messaging_config_EInvalidMaxDedupeKeyBytes"></a>



<pre><code><b>const</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_EInvalidMaxDedupeKeyBytes">EInvalidMaxDedupeKeyBytes</a>: u64 = 3;
</code></pre>



<a name="messaging_messaging_config_share_initial"></a>

## Function `share_initial`

Shares the genesis [<code><a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a></code>] singleton. Called from <code>messaging::init</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_share_initial">share_initial</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_share_initial">share_initial</a>(ctx: &<b>mut</b> TxContext) {
    transfer::share_object(<a href="../messaging/messaging_config.md#messaging_messaging_config_new_defaults">new_defaults</a>(ctx));
}
</code></pre>



</details>

<a name="messaging_messaging_config_update_messaging_config"></a>

## Function `update_messaging_config`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_update_messaging_config">update_messaging_config</a>(_admin: &<a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_MessagingAdminCap">social_contracts::bootstrap::MessagingAdminCap</a>, config: &<b>mut</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">messaging::messaging_config::MessagingConfig</a>, <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>: u64, <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>: u64, <a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>: u64, <a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>: u32, <a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_update_messaging_config">update_messaging_config</a>(
    _admin: &MessagingAdminCap,
    config: &<b>mut</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a>,
    <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>: u64,
    <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>: u64,
    <a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>: u64,
    <a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>: u32,
    <a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a> &lt;= <a href="../messaging/messaging_config.md#messaging_messaging_config_BPS_DENOM">BPS_DENOM</a>, <a href="../messaging/messaging_config.md#messaging_messaging_config_EInvalidFeeBps">EInvalidFeeBps</a>);
    <b>assert</b>!(<a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a> &lt;= <a href="../messaging/messaging_config.md#messaging_messaging_config_BPS_DENOM">BPS_DENOM</a>, <a href="../messaging/messaging_config.md#messaging_messaging_config_EInvalidFeeBps">EInvalidFeeBps</a>);
    <b>assert</b>!(<a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a> &gt; 0, <a href="../messaging/messaging_config.md#messaging_messaging_config_EInvalidPaymentExpiration">EInvalidPaymentExpiration</a>);
    <b>assert</b>!(<a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a> &gt; 0, <a href="../messaging/messaging_config.md#messaging_messaging_config_EInvalidMinReplyChars">EInvalidMinReplyChars</a>);
    <b>assert</b>!(<a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a> &gt; 0, <a href="../messaging/messaging_config.md#messaging_messaging_config_EInvalidMaxDedupeKeyBytes">EInvalidMaxDedupeKeyBytes</a>);
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a> = <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>;
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a> = <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>;
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a> = <a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>;
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a> = <a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>;
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a> = <a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>;
    event::emit(<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfigUpdatedEvent">MessagingConfigUpdatedEvent</a> {
        updated_by: ctx.sender(),
        timestamp: clock.timestamp_ms(),
        <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>,
        <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>,
        <a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>,
        <a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>,
        <a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>,
    });
}
</code></pre>



</details>

<a name="messaging_messaging_config_paid_msg_platform_fee_bps"></a>

## Function `paid_msg_platform_fee_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">messaging::messaging_config::MessagingConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a>): u64 {
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>
}
</code></pre>



</details>

<a name="messaging_messaging_config_paid_msg_treasury_fee_bps"></a>

## Function `paid_msg_treasury_fee_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">messaging::messaging_config::MessagingConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a>): u64 {
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>
}
</code></pre>



</details>

<a name="messaging_messaging_config_payment_expiration_ms"></a>

## Function `payment_expiration_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">messaging::messaging_config::MessagingConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a>): u64 {
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>
}
</code></pre>



</details>

<a name="messaging_messaging_config_min_reply_chars"></a>

## Function `min_reply_chars`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">messaging::messaging_config::MessagingConfig</a>): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a>): u32 {
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>
}
</code></pre>



</details>

<a name="messaging_messaging_config_max_dedupe_key_bytes"></a>

## Function `max_dedupe_key_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">messaging::messaging_config::MessagingConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a>): u64 {
    config.<a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>
}
</code></pre>



</details>

<a name="messaging_messaging_config_new_defaults"></a>

## Function `new_defaults`



<pre><code><b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_new_defaults">new_defaults</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">messaging::messaging_config::MessagingConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging_config.md#messaging_messaging_config_new_defaults">new_defaults</a>(ctx: &<b>mut</b> TxContext): <a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a> {
    <a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">MessagingConfig</a> {
        id: object::new(ctx),
        <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">paid_msg_platform_fee_bps</a>: <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_PAID_MSG_PLATFORM_FEE_BPS">DEFAULT_PAID_MSG_PLATFORM_FEE_BPS</a>,
        <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">paid_msg_treasury_fee_bps</a>: <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_PAID_MSG_TREASURY_FEE_BPS">DEFAULT_PAID_MSG_TREASURY_FEE_BPS</a>,
        <a href="../messaging/messaging_config.md#messaging_messaging_config_payment_expiration_ms">payment_expiration_ms</a>: <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_PAYMENT_EXPIRATION_MS">DEFAULT_PAYMENT_EXPIRATION_MS</a>,
        <a href="../messaging/messaging_config.md#messaging_messaging_config_min_reply_chars">min_reply_chars</a>: <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_MIN_REPLY_CHARS">DEFAULT_MIN_REPLY_CHARS</a>,
        <a href="../messaging/messaging_config.md#messaging_messaging_config_max_dedupe_key_bytes">max_dedupe_key_bytes</a>: <a href="../messaging/messaging_config.md#messaging_messaging_config_DEFAULT_MAX_DEDUPE_KEY_BYTES">DEFAULT_MAX_DEDUPE_KEY_BYTES</a>,
    }
}
</code></pre>



</details>
