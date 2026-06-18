---
title: Module `contra::deny_list`
---



-  [Function `is_receiver_denied`](#contra_deny_list_is_receiver_denied)
-  [Function `is_frozen`](#contra_deny_list_is_frozen)
-  [Function `is_sender_denied`](#contra_deny_list_is_sender_denied)


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



<a name="contra_deny_list_is_receiver_denied"></a>

## Function `is_receiver_denied`

Check if the given address is on the deny list for token type <code>T</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/deny_list.md#contra_deny_list_is_receiver_denied">is_receiver_denied</a>&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &<a href="../myso/deny_list.md#myso_deny_list_DenyList">myso::deny_list::DenyList</a>, receiver: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/deny_list.md#contra_deny_list_is_receiver_denied">is_receiver_denied</a>&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &DenyList, receiver: <b>address</b>): bool {
    deny_list_v2_contains_next_epoch&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>, receiver)
}
</code></pre>



</details>

<a name="contra_deny_list_is_frozen"></a>

## Function `is_frozen`

Check if all transfers for token type <code>T</code> are globally paused.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/deny_list.md#contra_deny_list_is_frozen">is_frozen</a>&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &<a href="../myso/deny_list.md#myso_deny_list_DenyList">myso::deny_list::DenyList</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/deny_list.md#contra_deny_list_is_frozen">is_frozen</a>&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &DenyList): bool {
    deny_list_v2_is_global_pause_enabled_next_epoch&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>)
}
</code></pre>



</details>

<a name="contra_deny_list_is_sender_denied"></a>

## Function `is_sender_denied`

Check if the given sender address is on the deny list for token type <code>T</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/deny_list.md#contra_deny_list_is_sender_denied">is_sender_denied</a>&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &<a href="../myso/deny_list.md#myso_deny_list_DenyList">myso::deny_list::DenyList</a>, sender: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/deny_list.md#contra_deny_list_is_sender_denied">is_sender_denied</a>&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &DenyList, sender: <b>address</b>): bool {
    // Denied addresses in the next epoch will immediately be unable to <b>use</b> objects of this coin type <b>as</b> inputs.
    deny_list_v2_contains_next_epoch&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>, sender)
}
</code></pre>



</details>
