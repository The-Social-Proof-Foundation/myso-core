---
title: Module `myso::permissioned_group_display`
---

Module: permissioned_group_display

Display support for <code>PermissionedGroup&lt;T&gt;</code> types.

Since <code>PermissionedGroup&lt;T&gt;</code> is defined in <code><a href="../myso/permissioned_group.md#myso_permissioned_group">permissioned_group</a></code>, extending
packages cannot directly create <code>Display&lt;PermissionedGroup&lt;T&gt;&gt;</code>.

This module provides a shared <code><a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_PermissionedGroupPublisher">PermissionedGroupPublisher</a></code> that holds the
framework Publisher. Extending packages can call <code><a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_setup_display">setup_display</a>&lt;T&gt;</code>
with their own Publisher to create <code>Display&lt;PermissionedGroup&lt;T&gt;&gt;</code>.


-  [Struct `PERMISSIONED_GROUP_DISPLAY`](#myso_permissioned_group_display_PERMISSIONED_GROUP_DISPLAY)
-  [Struct `PermissionedGroupPublisher`](#myso_permissioned_group_display_PermissionedGroupPublisher)
-  [Constants](#@Constants_0)
-  [Function `init`](#myso_permissioned_group_display_init)
-  [Function `setup_display`](#myso_permissioned_group_display_setup_display)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/display.md#myso_display">myso::display</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/permissioned_group.md#myso_permissioned_group">myso::permissioned_group</a>;
<b>use</b> <a href="../myso/permissions_table.md#myso_permissions_table">myso::permissions_table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
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



<a name="myso_permissioned_group_display_PERMISSIONED_GROUP_DISPLAY"></a>

## Struct `PERMISSIONED_GROUP_DISPLAY`

OTW for claiming Publisher and initializing PermissionedGroupPublisher.


<pre><code><b>public</b> <b>struct</b> <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_PERMISSIONED_GROUP_DISPLAY">PERMISSIONED_GROUP_DISPLAY</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="myso_permissioned_group_display_PermissionedGroupPublisher"></a>

## Struct `PermissionedGroupPublisher`

Shared object holding the framework Publisher for permissioned groups.
Used by extending packages to create <code>Display&lt;PermissionedGroup&lt;T&gt;&gt;</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_PermissionedGroupPublisher">PermissionedGroupPublisher</a> <b>has</b> key
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
<code>publisher: <a href="../myso/package.md#myso_package_Publisher">myso::package::Publisher</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_permissioned_group_display_ETypeNotFromModule"></a>

Type T is not from the same module as the publisher


<pre><code><b>const</b> <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_ETypeNotFromModule">ETypeNotFromModule</a>: u64 = 0;
</code></pre>



<a name="myso_permissioned_group_display_init"></a>

## Function `init`



<pre><code><b>fun</b> <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_init">init</a>(otw: <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_PERMISSIONED_GROUP_DISPLAY">myso::permissioned_group_display::PERMISSIONED_GROUP_DISPLAY</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_init">init</a>(otw: <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_PERMISSIONED_GROUP_DISPLAY">PERMISSIONED_GROUP_DISPLAY</a>, ctx: &<b>mut</b> TxContext) {
    <a href="../myso/transfer.md#myso_transfer_share_object">transfer::share_object</a>(<a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_PermissionedGroupPublisher">PermissionedGroupPublisher</a> {
        id: <a href="../myso/object.md#myso_object_new">object::new</a>(ctx),
        publisher: <a href="../myso/package.md#myso_package_claim">package::claim</a>(otw, ctx),
    });
}
</code></pre>



</details>

<a name="myso_permissioned_group_display_setup_display"></a>

## Function `setup_display`

Creates a <code>Display&lt;PermissionedGroup&lt;T&gt;&gt;</code> using the shared publisher.
The caller must provide their own Publisher to prove they own the module
that defines type T. The Display is transferred to the transaction sender.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_setup_display">setup_display</a>&lt;T: drop&gt;(pg_publisher: &<a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_PermissionedGroupPublisher">myso::permissioned_group_display::PermissionedGroupPublisher</a>, publisher: &<a href="../myso/package.md#myso_package_Publisher">myso::package::Publisher</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, project_url: <a href="../std/string.md#std_string_String">std::string::String</a>, link: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_setup_display">setup_display</a>&lt;T: drop&gt;(
    pg_publisher: &<a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_PermissionedGroupPublisher">PermissionedGroupPublisher</a>,
    publisher: &Publisher,
    name: String,
    description: String,
    image_url: String,
    project_url: String,
    link: String,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(publisher.from_module&lt;T&gt;(), <a href="../myso/permissioned_group_display.md#myso_permissioned_group_display_ETypeNotFromModule">ETypeNotFromModule</a>);
    <b>let</b> <b>mut</b> <a href="../myso/display.md#myso_display">display</a> = <a href="../myso/display.md#myso_display_new">display::new</a>&lt;PermissionedGroup&lt;T&gt;&gt;(&pg_publisher.publisher, ctx);
    <a href="../myso/display.md#myso_display">display</a>.add(b"name".to_string(), name);
    <a href="../myso/display.md#myso_display">display</a>.add(b"description".to_string(), description);
    <a href="../myso/display.md#myso_display">display</a>.add(b"creator".to_string(), b"{creator}".to_string());
    <a href="../myso/display.md#myso_display">display</a>.add(b"image_url".to_string(), image_url);
    <a href="../myso/display.md#myso_display">display</a>.add(b"project_url".to_string(), project_url);
    <a href="../myso/display.md#myso_display">display</a>.add(b"link".to_string(), link);
    <a href="../myso/display.md#myso_display">display</a>.update_version();
    <a href="../myso/transfer.md#myso_transfer_public_transfer">transfer::public_transfer</a>(<a href="../myso/display.md#myso_display">display</a>, ctx.sender());
}
</code></pre>



</details>
