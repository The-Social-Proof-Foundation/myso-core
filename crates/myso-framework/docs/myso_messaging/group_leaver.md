---
title: Module `myso_messaging::group_leaver`
---

Module: group_leaver

Actor object that allows group members to leave a <code>PermissionedGroup&lt;T&gt;</code>.

<code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a></code> is a derived singleton object from <code>MessagingNamespace</code>.
It is granted <code>PermissionsAdmin</code> on every group created via <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging_create_group">messaging::create_group</a></code>,
and exposes a <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_leave">leave</a></code> function that calls <code>object_remove_member</code> on behalf of the caller.

This module does NOT import <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging">messaging</a>.<b>move</b></code> to avoid a circular dependency.
The generic <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_leave">leave</a>&lt;T: drop&gt;</code> is instantiated with the concrete <code>Messaging</code> type
at the call site in <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging">messaging</a>.<b>move</b></code>.

All public entry points are in the <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging">messaging</a></code> module:
- <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging_leave">messaging::leave</a></code> - removes the caller from a group


-  [Struct `GroupLeaver`](#myso_messaging_group_leaver_GroupLeaver)
-  [Constants](#@Constants_0)
-  [Function `new`](#myso_messaging_group_leaver_new)
    -  [Parameters](#@Parameters_1)
    -  [Returns](#@Returns_2)
-  [Function `share`](#myso_messaging_group_leaver_share)
-  [Function `derivation_key`](#myso_messaging_group_leaver_derivation_key)
    -  [Returns](#@Returns_3)
-  [Function `leave`](#myso_messaging_group_leaver_leave)
    -  [Aborts](#@Aborts_4)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../myso_groups/permissioned_group.md#myso_groups_permissioned_group">myso_groups::permissioned_group</a>;
<b>use</b> <a href="../myso_groups/permissions_table.md#myso_groups_permissions_table">myso_groups::permissions_table</a>;
<b>use</b> <a href="../myso_groups/unpause_cap.md#myso_groups_unpause_cap">myso_groups::unpause_cap</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="myso_messaging_group_leaver_GroupLeaver"></a>

## Struct `GroupLeaver`

Actor object that holds <code>PermissionsAdmin</code> on all messaging groups.
The <code>id</code> field is intentionally private — no UID getter is exposed.
All leave operations go through the package-internal <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_leave">leave</a>&lt;T&gt;</code> function.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_messaging_group_leaver_GROUP_LEAVER_DERIVATION_KEY"></a>

Fixed derivation key for the singleton <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a></code> derived from <code>MessagingNamespace</code>.


<pre><code><b>const</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GROUP_LEAVER_DERIVATION_KEY">GROUP_LEAVER_DERIVATION_KEY</a>: vector&lt;u8&gt; = vector[103, 114, 111, 117, 112, 95, 108, 101, 97, 118, 101, 114];
</code></pre>



<a name="myso_messaging_group_leaver_new"></a>

## Function `new`

Creates a new <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a></code> derived from the namespace UID.
Called once during <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging_init">messaging::init</a></code>.


<a name="@Parameters_1"></a>

### Parameters

- <code>namespace_uid</code>: Mutable reference to the <code>MessagingNamespace</code> UID


<a name="@Returns_2"></a>

### Returns

A new <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a></code> object with a deterministic address.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_new">new</a>(namespace_uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">myso_messaging::group_leaver::GroupLeaver</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_new">new</a>(namespace_uid: &<b>mut</b> UID): <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a> {
    <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a> {
        id: derived_object::claim(namespace_uid, <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GROUP_LEAVER_DERIVATION_KEY">GROUP_LEAVER_DERIVATION_KEY</a>.to_string()),
    }
}
</code></pre>



</details>

<a name="myso_messaging_group_leaver_share"></a>

## Function `share`

Shares the <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a></code> object on-chain.
Called once during <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging_init">messaging::init</a></code> after creating the object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_share">share</a>(self: <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">myso_messaging::group_leaver::GroupLeaver</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_share">share</a>(self: <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a>) {
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="myso_messaging_group_leaver_derivation_key"></a>

## Function `derivation_key`

Returns the fixed derivation key string.
Used by <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging_create_group">messaging::create_group</a></code> to compute the <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a></code>'s address via
<code>derived_object::derive_address</code> without holding the object.


<a name="@Returns_3"></a>

### Returns

The string key used for address derivation.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_derivation_key">derivation_key</a>(): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_derivation_key">derivation_key</a>(): String {
    <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GROUP_LEAVER_DERIVATION_KEY">GROUP_LEAVER_DERIVATION_KEY</a>.to_string()
}
</code></pre>



</details>

<a name="myso_messaging_group_leaver_leave"></a>

## Function `leave`

Removes the caller (<code>ctx.sender()</code>) from the group.
The <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a></code> must have <code>PermissionsAdmin</code> on the group (granted at creation time).

Generic over <code>T: drop</code> so this module does not need to import <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging">messaging</a>.<b>move</b></code>.
Instantiated as <code><a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_leave">leave</a>&lt;Messaging&gt;</code> at the call site in <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging">messaging</a>.<b>move</b></code>.


<a name="@Aborts_4"></a>

### Aborts

- <code>ENotPermitted</code>: if this actor doesn't have <code>PermissionsAdmin</code> on the group
- <code>EMemberNotFound</code>: if the caller is not a member of the group
- <code>ELastPermissionsAdmin</code>: if the caller is the last <code>PermissionsAdmin</code>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_leave">leave</a>&lt;T: drop&gt;(self: &<a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">myso_messaging::group_leaver::GroupLeaver</a>, group: &<b>mut</b> <a href="../myso_groups/permissioned_group.md#myso_groups_permissioned_group_PermissionedGroup">myso_groups::permissioned_group::PermissionedGroup</a>&lt;T&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_leave">leave</a>&lt;T: drop&gt;(
    self: &<a href="../myso_messaging/group_leaver.md#myso_messaging_group_leaver_GroupLeaver">GroupLeaver</a>,
    group: &<b>mut</b> PermissionedGroup&lt;T&gt;,
    ctx: &TxContext,
) {
    group.object_remove_member&lt;T&gt;(&self.id, ctx.sender());
}
</code></pre>



</details>
