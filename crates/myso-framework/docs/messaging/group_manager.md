---
title: Module `messaging::group_manager`
---

Module: group_manager

Actor object that provides controlled <code>&<b>mut</b> UID</code> access to <code>PermissionedGroup&lt;T&gt;</code> objects.

<code><a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a></code> is a derived singleton object from <code>MessagingNamespace</code>.
It is granted <code>ObjectAdmin</code> on every group created via <code>messaging::create_group</code>,
and exposes functions for:
- Metadata dynamic field management

This module does NOT import <code><a href="../messaging/messaging.md#messaging_messaging">messaging</a>.<b>move</b></code> to avoid a circular dependency.
The generic functions are instantiated with the concrete <code>Messaging</code> type
at the call site in <code><a href="../messaging/messaging.md#messaging_messaging">messaging</a>.<b>move</b></code>.

All public entry points are in the <code><a href="../messaging/messaging.md#messaging_messaging">messaging</a></code> module.


-  [Struct `GroupManager`](#messaging_group_manager_GroupManager)
-  [Constants](#@Constants_0)
-  [Function `new`](#messaging_group_manager_new)
    -  [Parameters](#@Parameters_1)
    -  [Returns](#@Returns_2)
-  [Function `share`](#messaging_group_manager_share)
-  [Function `derivation_key`](#messaging_group_manager_derivation_key)
    -  [Returns](#@Returns_3)
-  [Function `attach_metadata`](#messaging_group_manager_attach_metadata)
-  [Function `remove_metadata`](#messaging_group_manager_remove_metadata)
-  [Function `borrow_metadata`](#messaging_group_manager_borrow_metadata)
-  [Function `borrow_metadata_mut`](#messaging_group_manager_borrow_metadata_mut)


<pre><code><b>use</b> <a href="../messaging/metadata.md#messaging_metadata">messaging::metadata</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
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
<b>use</b> <a href="../myso/permissioned_group.md#myso_permissioned_group">myso::permissioned_group</a>;
<b>use</b> <a href="../myso/permissions_table.md#myso_permissions_table">myso::permissions_table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/unpause_cap.md#myso_unpause_cap">myso::unpause_cap</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="messaging_group_manager_GroupManager"></a>

## Struct `GroupManager`

Actor object that holds <code>ObjectAdmin</code> on all messaging groups.
The <code>id</code> field is intentionally private — no UID getter is exposed.
All operations go through the package-internal functions.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a> <b>has</b> key
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


<a name="messaging_group_manager_GROUP_MANAGER_DERIVATION_KEY"></a>

Fixed derivation key for the singleton <code><a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a></code> derived from <code>MessagingNamespace</code>.


<pre><code><b>const</b> <a href="../messaging/group_manager.md#messaging_group_manager_GROUP_MANAGER_DERIVATION_KEY">GROUP_MANAGER_DERIVATION_KEY</a>: vector&lt;u8&gt; = vector[103, 114, 111, 117, 112, 95, 109, 97, 110, 97, 103, 101, 114];
</code></pre>



<a name="messaging_group_manager_new"></a>

## Function `new`

Creates a new <code><a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a></code> derived from the namespace UID.
Called once during <code>messaging::init</code>.


<a name="@Parameters_1"></a>

### Parameters

- <code>namespace_uid</code>: Mutable reference to the <code>MessagingNamespace</code> UID


<a name="@Returns_2"></a>

### Returns

A new <code><a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a></code> object with a deterministic address.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_new">new</a>(namespace_uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): <a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_new">new</a>(namespace_uid: &<b>mut</b> UID): <a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a> {
    <a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a> {
        id: derived_object::claim(namespace_uid, <a href="../messaging/group_manager.md#messaging_group_manager_GROUP_MANAGER_DERIVATION_KEY">GROUP_MANAGER_DERIVATION_KEY</a>.to_string()),
    }
}
</code></pre>



</details>

<a name="messaging_group_manager_share"></a>

## Function `share`

Shares the <code><a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a></code> object on-chain.
Called once during <code>messaging::init</code> after creating the object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_share">share</a>(self: <a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_share">share</a>(self: <a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a>) {
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="messaging_group_manager_derivation_key"></a>

## Function `derivation_key`

Returns the fixed derivation key string.
Used by <code>messaging::create_group</code> to compute the <code><a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a></code>'s address via
<code>derived_object::derive_address</code> without holding the object.


<a name="@Returns_3"></a>

### Returns

The string key used for address derivation.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_derivation_key">derivation_key</a>(): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_derivation_key">derivation_key</a>(): String {
    <a href="../messaging/group_manager.md#messaging_group_manager_GROUP_MANAGER_DERIVATION_KEY">GROUP_MANAGER_DERIVATION_KEY</a>.to_string()
}
</code></pre>



</details>

<a name="messaging_group_manager_attach_metadata"></a>

## Function `attach_metadata`

Attaches Metadata as a dynamic field on the group.
Called during <code>messaging::create_group</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_attach_metadata">attach_metadata</a>&lt;T: drop&gt;(self: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;T&gt;, m: <a href="../messaging/metadata.md#messaging_metadata_Metadata">messaging::metadata::Metadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_attach_metadata">attach_metadata</a>&lt;T: drop&gt;(
    self: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a>,
    group: &<b>mut</b> PermissionedGroup&lt;T&gt;,
    m: Metadata,
) {
    <b>let</b> uid = group.object_uid_mut&lt;T&gt;(&self.id);
    dynamic_field::add(uid, <a href="../messaging/metadata.md#messaging_metadata_key">metadata::key</a>(), m);
}
</code></pre>



</details>

<a name="messaging_group_manager_remove_metadata"></a>

## Function `remove_metadata`

Removes and returns Metadata from the group.
Used when archiving/destroying a group to preserve metadata.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_remove_metadata">remove_metadata</a>&lt;T: drop&gt;(self: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;T&gt;): <a href="../messaging/metadata.md#messaging_metadata_Metadata">messaging::metadata::Metadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_remove_metadata">remove_metadata</a>&lt;T: drop&gt;(
    self: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a>,
    group: &<b>mut</b> PermissionedGroup&lt;T&gt;,
): Metadata {
    <b>let</b> uid = group.object_uid_mut&lt;T&gt;(&self.id);
    dynamic_field::remove(uid, <a href="../messaging/metadata.md#messaging_metadata_key">metadata::key</a>())
}
</code></pre>



</details>

<a name="messaging_group_manager_borrow_metadata"></a>

## Function `borrow_metadata`

Returns an immutable reference to the group's Metadata.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_borrow_metadata">borrow_metadata</a>&lt;T: drop&gt;(self: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;T&gt;): &<a href="../messaging/metadata.md#messaging_metadata_Metadata">messaging::metadata::Metadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_borrow_metadata">borrow_metadata</a>&lt;T: drop&gt;(
    self: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a>,
    group: &PermissionedGroup&lt;T&gt;,
): &Metadata {
    <b>let</b> uid = group.object_uid&lt;T&gt;(&self.id);
    dynamic_field::borrow(uid, <a href="../messaging/metadata.md#messaging_metadata_key">metadata::key</a>())
}
</code></pre>



</details>

<a name="messaging_group_manager_borrow_metadata_mut"></a>

## Function `borrow_metadata_mut`

Returns a mutable reference to the group's Metadata.
Used by messaging.move to expose field-level setters with permission checks.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_borrow_metadata_mut">borrow_metadata_mut</a>&lt;T: drop&gt;(self: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;T&gt;): &<b>mut</b> <a href="../messaging/metadata.md#messaging_metadata_Metadata">messaging::metadata::Metadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_manager.md#messaging_group_manager_borrow_metadata_mut">borrow_metadata_mut</a>&lt;T: drop&gt;(
    self: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">GroupManager</a>,
    group: &<b>mut</b> PermissionedGroup&lt;T&gt;,
): &<b>mut</b> Metadata {
    <b>let</b> uid = group.object_uid_mut&lt;T&gt;(&self.id);
    dynamic_field::borrow_mut(uid, <a href="../messaging/metadata.md#messaging_metadata_key">metadata::key</a>())
}
</code></pre>



</details>
