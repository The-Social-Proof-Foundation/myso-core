---
title: Module `messaging::paid_escrow_settlement`
---

Fee distribution for claimed paid-message escrow (<code>MYSO</code>).

Fee BPS are read from [<code><a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">messaging_config::MessagingConfig</a></code>].

When <code>platform_fee_recipient</code> is [<code>NO_PLATFORM_FEE_RECIPIENT</code>] (<code>@0x0</code>), the platform share is
combined with the ecosystem share and sent to <code>ecosystem_fee_recipient</code> (wallet paid DMs with no
associated platform).

Uses <code>transfer::public_transfer</code> to fee recipients. Credits to the live <code>Platform</code> treasury balance
require <code><a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">social_contracts::platform::add_to_treasury</a></code> (same-package); see
<code>ref_social_contract/sources/messaging_paid_fee_bridge.<b>move</b></code> for a foundation-side helper.


-  [Struct `EscrowFeeTotals`](#messaging_paid_escrow_settlement_EscrowFeeTotals)
-  [Function `no_platform_fee_recipient`](#messaging_paid_escrow_settlement_no_platform_fee_recipient)
-  [Function `total_amount`](#messaging_paid_escrow_settlement_total_amount)
-  [Function `platform_fee`](#messaging_paid_escrow_settlement_platform_fee)
-  [Function `treasury_fee`](#messaging_paid_escrow_settlement_treasury_fee)
-  [Function `net_amount`](#messaging_paid_escrow_settlement_net_amount)
-  [Function `distribute_escrow_to_recipients`](#messaging_paid_escrow_settlement_distribute_escrow_to_recipients)


<pre><code><b>use</b> <a href="../messaging/messaging_config.md#messaging_messaging_config">messaging::messaging_config</a>;
<b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
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



<a name="messaging_paid_escrow_settlement_EscrowFeeTotals"></a>

## Struct `EscrowFeeTotals`

Totals from a settled escrow split (for events and testing).


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">EscrowFeeTotals</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_total_amount">total_amount</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_net_amount">net_amount</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="messaging_paid_escrow_settlement_no_platform_fee_recipient"></a>

## Function `no_platform_fee_recipient`

Sentinel: pass as <code>platform_fee_recipient</code> when no platform is associated with the paid DM.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_no_platform_fee_recipient">no_platform_fee_recipient</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_no_platform_fee_recipient">no_platform_fee_recipient</a>(): <b>address</b> {
    @0x0
}
</code></pre>



</details>

<a name="messaging_paid_escrow_settlement_total_amount"></a>

## Function `total_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_total_amount">total_amount</a>(t: &<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">messaging::paid_escrow_settlement::EscrowFeeTotals</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_total_amount">total_amount</a>(t: &<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">EscrowFeeTotals</a>): u64 {
    t.<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_total_amount">total_amount</a>
}
</code></pre>



</details>

<a name="messaging_paid_escrow_settlement_platform_fee"></a>

## Function `platform_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a>(t: &<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">messaging::paid_escrow_settlement::EscrowFeeTotals</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a>(t: &<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">EscrowFeeTotals</a>): u64 {
    t.<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a>
}
</code></pre>



</details>

<a name="messaging_paid_escrow_settlement_treasury_fee"></a>

## Function `treasury_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a>(t: &<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">messaging::paid_escrow_settlement::EscrowFeeTotals</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a>(t: &<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">EscrowFeeTotals</a>): u64 {
    t.<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a>
}
</code></pre>



</details>

<a name="messaging_paid_escrow_settlement_net_amount"></a>

## Function `net_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_net_amount">net_amount</a>(t: &<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">messaging::paid_escrow_settlement::EscrowFeeTotals</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_net_amount">net_amount</a>(t: &<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">EscrowFeeTotals</a>): u64 {
    t.<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_net_amount">net_amount</a>
}
</code></pre>



</details>

<a name="messaging_paid_escrow_settlement_distribute_escrow_to_recipients"></a>

## Function `distribute_escrow_to_recipients`

Splits <code>escrow_coin</code> per paid-message BPS: platform, ecosystem, then <code>primary_recipient</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_distribute_escrow_to_recipients">distribute_escrow_to_recipients</a>(config: &<a href="../messaging/messaging_config.md#messaging_messaging_config_MessagingConfig">messaging::messaging_config::MessagingConfig</a>, escrow_coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, platform_fee_recipient: <b>address</b>, ecosystem_fee_recipient: <b>address</b>, primary_recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">messaging::paid_escrow_settlement::EscrowFeeTotals</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_distribute_escrow_to_recipients">distribute_escrow_to_recipients</a>(
    config: &MessagingConfig,
    <b>mut</b> escrow_coin: Coin&lt;MYSO&gt;,
    platform_fee_recipient: <b>address</b>,
    ecosystem_fee_recipient: <b>address</b>,
    primary_recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
): <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">EscrowFeeTotals</a> {
    <b>let</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_total_amount">total_amount</a> = coin::value(&escrow_coin);
    <b>let</b> platform_fee_bps = <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_platform_fee_bps">messaging_config::paid_msg_platform_fee_bps</a>(config);
    <b>let</b> treasury_fee_bps = <a href="../messaging/messaging_config.md#messaging_messaging_config_paid_msg_treasury_fee_bps">messaging_config::paid_msg_treasury_fee_bps</a>(config);
    <b>let</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a> = (((<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_total_amount">total_amount</a> <b>as</b> u128) * (platform_fee_bps <b>as</b> u128)) / 10000u128) <b>as</b> u64;
    <b>let</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a> = (((<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_total_amount">total_amount</a> <b>as</b> u128) * (treasury_fee_bps <b>as</b> u128)) / 10000u128) <b>as</b> u64;
    <b>let</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_net_amount">net_amount</a> = <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_total_amount">total_amount</a> - <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a> - <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a>;
    <b>if</b> (platform_fee_recipient == <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_no_platform_fee_recipient">no_platform_fee_recipient</a>()) {
        <b>let</b> ecosystem_total = <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a> + <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a>;
        <b>if</b> (ecosystem_total &gt; 0) {
            transfer::public_transfer(
                coin::split(&<b>mut</b> escrow_coin, ecosystem_total, ctx),
                ecosystem_fee_recipient,
            );
        };
    } <b>else</b> {
        <b>if</b> (<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a> &gt; 0) {
            transfer::public_transfer(
                coin::split(&<b>mut</b> escrow_coin, <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a>, ctx),
                platform_fee_recipient,
            );
        };
        <b>if</b> (<a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a> &gt; 0) {
            transfer::public_transfer(
                coin::split(&<b>mut</b> escrow_coin, <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a>, ctx),
                ecosystem_fee_recipient,
            );
        };
    };
    transfer::public_transfer(escrow_coin, primary_recipient);
    <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_EscrowFeeTotals">EscrowFeeTotals</a> { <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_total_amount">total_amount</a>, <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_platform_fee">platform_fee</a>, <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_treasury_fee">treasury_fee</a>, <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement_net_amount">net_amount</a> }
}
</code></pre>



</details>
