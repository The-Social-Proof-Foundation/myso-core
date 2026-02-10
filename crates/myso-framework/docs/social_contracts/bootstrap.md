---
title: Module `social_contracts::bootstrap`
---

Bootstrap service for MySocial - claims all admin capabilities in one call.
Uses the framework's centralized BootstrapKey for one-time initialization.


-  [Function `claim_all_admin_capabilities`](#social_contracts_bootstrap_claim_all_admin_capabilities)
-  [Function `is_bootstrap_used`](#social_contracts_bootstrap_is_bootstrap_used)
-  [Function `bootstrap_version`](#social_contracts_bootstrap_bootstrap_version)


<pre><code><b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
<b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
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
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/hmac.md#myso_hmac">myso::hmac</a>;
<b>use</b> <a href="../myso/math.md#myso_math">myso::math</a>;
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
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/insurance.md#social_contracts_insurance">social_contracts::insurance</a>;
<b>use</b> <a href="../social_contracts/message.md#social_contracts_message">social_contracts::message</a>;
<b>use</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">social_contracts::mydata</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
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
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_bootstrap_claim_all_admin_capabilities"></a>

## Function `claim_all_admin_capabilities`

Claim all admin capabilities (one-time only)
Creates and transfers all admin capabilities to caller, then seals the bootstrap key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_claim_all_admin_capabilities">claim_all_admin_capabilities</a>(bootstrap_key: &<b>mut</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">myso::bootstrap_key::BootstrapKey</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_claim_all_admin_capabilities">claim_all_admin_capabilities</a>(
    bootstrap_key: &<b>mut</b> BootstrapKey,
    ctx: &<b>mut</b> TxContext
) {
    bootstrap_key::assert_not_used(bootstrap_key);
    <b>let</b> admin = tx_context::sender(ctx);
    // Initialize shared objects
    <a href="../social_contracts/platform.md#social_contracts_platform_bootstrap_init">social_contracts::platform::bootstrap_init</a>(ctx);
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_bootstrap_init">social_contracts::social_graph::bootstrap_init</a>(ctx);
    <a href="../social_contracts/profile.md#social_contracts_profile_bootstrap_init">social_contracts::profile::bootstrap_init</a>(ctx);
    <a href="../social_contracts/block_list.md#social_contracts_block_list_bootstrap_init">social_contracts::block_list::bootstrap_init</a>(ctx);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_bootstrap_init">social_contracts::mydata::bootstrap_init</a>(ctx);
    <a href="../social_contracts/governance.md#social_contracts_governance_bootstrap_init">social_contracts::governance::bootstrap_init</a>(ctx);
    <a href="../social_contracts/post.md#social_contracts_post_bootstrap_init">social_contracts::post::bootstrap_init</a>(ctx);
    <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_bootstrap_init">social_contracts::social_proof_tokens::bootstrap_init</a>(ctx);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_bootstrap_init">social_contracts::proof_of_creativity::bootstrap_init</a>(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_bootstrap_init">social_contracts::message::bootstrap_init</a>(ctx);
    <a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_bootstrap_init">social_contracts::social_proof_of_truth::bootstrap_init</a>(ctx);
    <a href="../social_contracts/insurance.md#social_contracts_insurance_bootstrap_init">social_contracts::insurance::bootstrap_init</a>(ctx);
    // Create admin capabilities
    transfer::public_transfer(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_create_upgrade_admin_cap">upgrade::create_upgrade_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_create_social_proof_tokens_admin_cap">social_proof_tokens::create_social_proof_tokens_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/post.md#social_contracts_post_create_post_admin_cap">post::create_post_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_create_poc_admin_cap">proof_of_creativity::create_poc_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/platform.md#social_contracts_platform_create_platform_admin_cap">platform::create_platform_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/governance.md#social_contracts_governance_create_governance_admin_cap">governance::create_governance_admin_cap</a>(ctx), admin);
    transfer::public_transfer(mydata::create_mydata_admin_cap(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_admin_cap">social_proof_of_truth::create_spot_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/social_proof_of_truth.md#social_contracts_social_proof_of_truth_create_spot_oracle_admin_cap">social_proof_of_truth::create_spot_oracle_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/profile.md#social_contracts_profile_create_ecosystem_treasury_admin_cap">profile::create_ecosystem_treasury_admin_cap</a>(ctx), admin);
    transfer::public_transfer(<a href="../social_contracts/insurance.md#social_contracts_insurance_create_insurance_admin_cap">insurance::create_insurance_admin_cap</a>(ctx), admin);
    transfer::public_transfer(coin::create_coin_creation_admin_cap_for_bootstrap(bootstrap_key, ctx), admin);
    transfer::public_transfer(package::create_package_publishing_admin_cap_for_bootstrap(bootstrap_key, ctx), admin);
    // Seal the <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a> key permanently (prevents any future <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a> attempts)
    bootstrap_key::finalize_bootstrap(bootstrap_key);
}
</code></pre>



</details>

<a name="social_contracts_bootstrap_is_bootstrap_used"></a>

## Function `is_bootstrap_used`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_is_bootstrap_used">is_bootstrap_used</a>(key: &<a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">myso::bootstrap_key::BootstrapKey</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_is_bootstrap_used">is_bootstrap_used</a>(key: &BootstrapKey): bool {
    bootstrap_key::is_used(key)
}
</code></pre>



</details>

<a name="social_contracts_bootstrap_bootstrap_version"></a>

## Function `bootstrap_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_bootstrap_version">bootstrap_version</a>(key: &<a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">myso::bootstrap_key::BootstrapKey</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap_bootstrap_version">bootstrap_version</a>(key: &BootstrapKey): u64 {
    bootstrap_key::version(key)
}
</code></pre>



</details>
