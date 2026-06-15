---
title: Module `messaging::group_handle_registry`
---

On-chain registry mapping **canonical group handles** to <code>PermissionedGroup&lt;Messaging&gt;</code> object IDs.

This is intentionally separate from any **profile** <code>UsernameRegistry</code> (user usernames): the same
string may exist as both a user username and a group handle; clients use separate lookup APIs (<code>lookup_profile_by_username</code> vs <code><a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_lookup_group_by_handle">lookup_group_by_handle</a></code>).


-  [Struct `GroupHandleRegistry`](#messaging_group_handle_registry_GroupHandleRegistry)
-  [Constants](#@Constants_0)
-  [Function `new`](#messaging_group_handle_registry_new)
-  [Function `share`](#messaging_group_handle_registry_share)
-  [Function `derivation_key`](#messaging_group_handle_registry_derivation_key)
-  [Function `to_lowercase_bytes`](#messaging_group_handle_registry_to_lowercase_bytes)
-  [Function `canonical_handle`](#messaging_group_handle_registry_canonical_handle)
-  [Function `duplicate_string`](#messaging_group_handle_registry_duplicate_string)
-  [Function `is_valid_handle_chars`](#messaging_group_handle_registry_is_valid_handle_chars)
-  [Function `is_reserved`](#messaging_group_handle_registry_is_reserved)
-  [Function `is_bytes_eq`](#messaging_group_handle_registry_is_bytes_eq)
-  [Function `validate_handle_string`](#messaging_group_handle_registry_validate_handle_string)
-  [Function `set_handle`](#messaging_group_handle_registry_set_handle)
-  [Function `clear_handle`](#messaging_group_handle_registry_clear_handle)
-  [Function `lookup_group_by_handle`](#messaging_group_handle_registry_lookup_group_by_handle)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="messaging_group_handle_registry_GroupHandleRegistry"></a>

## Struct `GroupHandleRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">GroupHandleRegistry</a> <b>has</b> key
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
<code>handle_to_group: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>group_to_handle: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="messaging_group_handle_registry_EInvalidHandle"></a>



<pre><code><b>const</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_EInvalidHandle">EInvalidHandle</a>: u64 = 0;
</code></pre>



<a name="messaging_group_handle_registry_EHandleTaken"></a>



<pre><code><b>const</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_EHandleTaken">EHandleTaken</a>: u64 = 1;
</code></pre>



<a name="messaging_group_handle_registry_GROUP_HANDLE_REGISTRY_DERIVATION_KEY"></a>



<pre><code><b>const</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GROUP_HANDLE_REGISTRY_DERIVATION_KEY">GROUP_HANDLE_REGISTRY_DERIVATION_KEY</a>: vector&lt;u8&gt; = vector[103, 114, 111, 117, 112, 95, 104, 97, 110, 100, 108, 101, 95, 114, 101, 103, 105, 115, 116, 114, 121];
</code></pre>



<a name="messaging_group_handle_registry_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_new">new</a>(namespace_uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">messaging::group_handle_registry::GroupHandleRegistry</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_new">new</a>(namespace_uid: &<b>mut</b> UID, ctx: &<b>mut</b> TxContext): <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">GroupHandleRegistry</a> {
    <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">GroupHandleRegistry</a> {
        id: derived_object::claim(
            namespace_uid,
            <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GROUP_HANDLE_REGISTRY_DERIVATION_KEY">GROUP_HANDLE_REGISTRY_DERIVATION_KEY</a>.to_string(),
        ),
        handle_to_group: table::new(ctx),
        group_to_handle: table::new(ctx),
    }
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_share"></a>

## Function `share`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_share">share</a>(self: <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">messaging::group_handle_registry::GroupHandleRegistry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_share">share</a>(self: <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">GroupHandleRegistry</a>) {
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_derivation_key"></a>

## Function `derivation_key`

Matches [<code><a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GROUP_HANDLE_REGISTRY_DERIVATION_KEY">GROUP_HANDLE_REGISTRY_DERIVATION_KEY</a></code>](group_handle_registry) in TS (<code><a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GROUP_HANDLE_REGISTRY_DERIVATION_KEY">GROUP_HANDLE_REGISTRY_DERIVATION_KEY</a></code>).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_derivation_key">derivation_key</a>(): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_derivation_key">derivation_key</a>(): String {
    <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GROUP_HANDLE_REGISTRY_DERIVATION_KEY">GROUP_HANDLE_REGISTRY_DERIVATION_KEY</a>.to_string()
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_to_lowercase_bytes"></a>

## Function `to_lowercase_bytes`



<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_to_lowercase_bytes">to_lowercase_bytes</a>(bytes: &vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_to_lowercase_bytes">to_lowercase_bytes</a>(bytes: &vector&lt;u8&gt;): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> result = vector::empty&lt;u8&gt;();
    <b>let</b> len = vector::length(bytes);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> b = *vector::borrow(bytes, i);
        <b>let</b> out = <b>if</b> (b &gt;= 65 && b &lt;= 90) {
            b + 32
        } <b>else</b> {
            b
        };
        vector::push_back(&<b>mut</b> result, out);
        i = i + 1;
    };
    result
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_canonical_handle"></a>

## Function `canonical_handle`



<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_canonical_handle">canonical_handle</a>(s: &<a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_canonical_handle">canonical_handle</a>(s: &String): String {
    <b>let</b> lowered = <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_to_lowercase_bytes">to_lowercase_bytes</a>(string::as_bytes(s));
    string::utf8(lowered)
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_duplicate_string"></a>

## Function `duplicate_string`



<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_duplicate_string">duplicate_string</a>(s: &<a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_duplicate_string">duplicate_string</a>(s: &String): String {
    <b>let</b> bytes = string::as_bytes(s);
    <b>let</b> len = vector::length(bytes);
    <b>let</b> <b>mut</b> v = vector::empty&lt;u8&gt;();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        vector::push_back(&<b>mut</b> v, *vector::borrow(bytes, i));
        i = i + 1;
    };
    string::utf8(v)
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_is_valid_handle_chars"></a>

## Function `is_valid_handle_chars`



<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_valid_handle_chars">is_valid_handle_chars</a>(h: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_valid_handle_chars">is_valid_handle_chars</a>(h: &String): bool {
    <b>let</b> bytes = string::as_bytes(h);
    <b>let</b> len = vector::length(bytes);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> b = *vector::borrow(bytes, i);
        <b>let</b> ok = (b &gt;= 48 && b &lt;= 57) || // 0-9
            (b &gt;= 97 && b &lt;= 122) || // a-z
            (b == 95);
        <b>if</b> (!ok) {
            <b>return</b> <b>false</b>
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_is_reserved"></a>

## Function `is_reserved`



<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_reserved">is_reserved</a>(h: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_reserved">is_reserved</a>(h: &String): bool {
    <b>let</b> bytes = string::as_bytes(h);
    <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_bytes_eq">is_bytes_eq</a>(bytes, &b"admin")
        || <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_bytes_eq">is_bytes_eq</a>(bytes, &b"root")
        || <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_bytes_eq">is_bytes_eq</a>(bytes, &b"system")
        || <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_bytes_eq">is_bytes_eq</a>(bytes, &b"myso")
        || <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_bytes_eq">is_bytes_eq</a>(bytes, &b"support")
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_is_bytes_eq"></a>

## Function `is_bytes_eq`



<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_bytes_eq">is_bytes_eq</a>(lhs: &vector&lt;u8&gt;, rhs: &vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_bytes_eq">is_bytes_eq</a>(lhs: &vector&lt;u8&gt;, rhs: &vector&lt;u8&gt;): bool {
    <b>if</b> (vector::length(lhs) != vector::length(rhs)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> len = vector::length(lhs);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>if</b> (*vector::borrow(lhs, i) != *vector::borrow(rhs, i)) {
            <b>return</b> <b>false</b>
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_validate_handle_string"></a>

## Function `validate_handle_string`



<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_validate_handle_string">validate_handle_string</a>(handle: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_validate_handle_string">validate_handle_string</a>(handle: String): String {
    <b>let</b> h = <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_canonical_handle">canonical_handle</a>(&handle);
    <b>let</b> len = string::length(&h);
    <b>assert</b>!(len &gt;= 2 && len &lt;= 50, <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_EInvalidHandle">EInvalidHandle</a>);
    <b>assert</b>!(<a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_valid_handle_chars">is_valid_handle_chars</a>(&h), <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_EInvalidHandle">EInvalidHandle</a>);
    <b>assert</b>!(!<a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_reserved">is_reserved</a>(&h), <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_EInvalidHandle">EInvalidHandle</a>);
    h
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_set_handle"></a>

## Function `set_handle`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_set_handle">set_handle</a>(registry: &<b>mut</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">messaging::group_handle_registry::GroupHandleRegistry</a>, group_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, handle: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_set_handle">set_handle</a>(
    registry: &<b>mut</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">GroupHandleRegistry</a>,
    group_id: ID,
    handle: String,
) {
    <b>let</b> h = <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_validate_handle_string">validate_handle_string</a>(handle);
    // Drop any existing mapping <b>for</b> this group.
    <b>if</b> (table::contains(&registry.group_to_handle, group_id)) {
        <b>let</b> old_h = table::remove(&<b>mut</b> registry.group_to_handle, group_id);
        table::remove(&<b>mut</b> registry.handle_to_group, old_h);
    };
    <b>assert</b>!(!table::contains(&registry.handle_to_group, h), <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_EHandleTaken">EHandleTaken</a>);
    <b>let</b> h_rev = <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_duplicate_string">duplicate_string</a>(&h);
    table::add(&<b>mut</b> registry.handle_to_group, h, group_id);
    table::add(&<b>mut</b> registry.group_to_handle, group_id, h_rev);
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_clear_handle"></a>

## Function `clear_handle`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_clear_handle">clear_handle</a>(registry: &<b>mut</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">messaging::group_handle_registry::GroupHandleRegistry</a>, group_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_clear_handle">clear_handle</a>(registry: &<b>mut</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">GroupHandleRegistry</a>, group_id: ID) {
    <b>if</b> (!table::contains(&registry.group_to_handle, group_id)) {
        <b>return</b>
    };
    <b>let</b> old_h = table::remove(&<b>mut</b> registry.group_to_handle, group_id);
    table::remove(&<b>mut</b> registry.handle_to_group, old_h);
}
</code></pre>



</details>

<a name="messaging_group_handle_registry_lookup_group_by_handle"></a>

## Function `lookup_group_by_handle`

Returns the group object ID for a handle, if registered. No version gate — safe for off-chain indexing.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_lookup_group_by_handle">lookup_group_by_handle</a>(registry: &<a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">messaging::group_handle_registry::GroupHandleRegistry</a>, handle: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_lookup_group_by_handle">lookup_group_by_handle</a>(registry: &<a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">GroupHandleRegistry</a>, handle: String): Option&lt;ID&gt; {
    <b>let</b> h = <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_canonical_handle">canonical_handle</a>(&handle);
    <b>if</b> (string::length(&h) &lt; 2 || string::length(&h) &gt; 50 || !<a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_valid_handle_chars">is_valid_handle_chars</a>(&h)) {
        <b>return</b> option::none()
    };
    <b>if</b> (<a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_is_reserved">is_reserved</a>(&h)) {
        <b>return</b> option::none()
    };
    <b>if</b> (!table::contains(&registry.handle_to_group, h)) {
        <b>return</b> option::none()
    };
    option::some(*table::borrow(&registry.handle_to_group, h))
}
</code></pre>



</details>
