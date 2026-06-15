---
title: Module `myso::permissions_table`
---

Module: permissions_table

Internal data structure for storing member permissions.
Maps <code><b>address</b> -&gt; VecSet&lt;TypeName&gt;</code> using dynamic fields on a derived object.
Created as a child of <code>PermissionedGroup</code> for easy discoverability.


-  [Struct `PermissionsTable`](#myso_permissions_table_PermissionsTable)
-  [Constants](#@Constants_0)
-  [Function `new_derived`](#myso_permissions_table_new_derived)
    -  [Aborts](#@Aborts_1)
-  [Function `add_member`](#myso_permissions_table_add_member)
-  [Function `remove_member`](#myso_permissions_table_remove_member)
-  [Function `add_permission`](#myso_permissions_table_add_permission)
-  [Function `remove_permission`](#myso_permissions_table_remove_permission)
-  [Function `has_permission`](#myso_permissions_table_has_permission)
-  [Function `is_member`](#myso_permissions_table_is_member)
-  [Function `length`](#myso_permissions_table_length)
-  [Function `destroy_empty`](#myso_permissions_table_destroy_empty)
    -  [Aborts](#@Aborts_2)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="myso_permissions_table_PermissionsTable"></a>

## Struct `PermissionsTable`

A PermissionsTable is a derived object from a parent PermissionedGroup,
that holds all the <code><b>address</b> -&gt; VecSet&lt;TypeName&gt;</code> mappings for permissions.
The permissions are stored as dynamic fields.
This enables easy discoverability, given a PermissionedGroup ID.


<pre><code><b>public</b> <b>struct</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a> <b>has</b> key, store
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
<code><a href="../myso/permissions_table.md#myso_permissions_table_length">length</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_permissions_table_EPermissionsTableAlreadyExists"></a>

Attempted to derive a PermissionsTable that already exists for the given parent.


<pre><code><b>const</b> <a href="../myso/permissions_table.md#myso_permissions_table_EPermissionsTableAlreadyExists">EPermissionsTableAlreadyExists</a>: u64 = 0;
</code></pre>



<a name="myso_permissions_table_EPermissionsTableNotEmpty"></a>

Attempted to destroy a PermissionsTable that still has members.


<pre><code><b>const</b> <a href="../myso/permissions_table.md#myso_permissions_table_EPermissionsTableNotEmpty">EPermissionsTableNotEmpty</a>: u64 = 1;
</code></pre>



<a name="myso_permissions_table_new_derived"></a>

## Function `new_derived`

Creates a new <code><a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a></code> derived from the given parent UID.


<a name="@Aborts_1"></a>

### Aborts

- <code><a href="../myso/permissions_table.md#myso_permissions_table_EPermissionsTableAlreadyExists">EPermissionsTableAlreadyExists</a></code>: if a table already exists for this derivation key


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_new_derived">new_derived</a>(parent_uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, derivation_key: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">myso::permissions_table::PermissionsTable</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_new_derived">new_derived</a>(parent_uid: &<b>mut</b> UID, derivation_key: String): <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a> {
    <b>assert</b>!(!<a href="../myso/derived_object.md#myso_derived_object_exists">derived_object::exists</a>(parent_uid, derivation_key), <a href="../myso/permissions_table.md#myso_permissions_table_EPermissionsTableAlreadyExists">EPermissionsTableAlreadyExists</a>);
    <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a> {
        id: <a href="../myso/derived_object.md#myso_derived_object_claim">derived_object::claim</a>(parent_uid, derivation_key),
        <a href="../myso/permissions_table.md#myso_permissions_table_length">length</a>: 0,
    }
}
</code></pre>



</details>

<a name="myso_permissions_table_add_member"></a>

## Function `add_member`

Adds a new member with the given initial permission set.
Stores the mapping as a dynamic field keyed by the member's address.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_add_member">add_member</a>(self: &<b>mut</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">myso::permissions_table::PermissionsTable</a>, member: <b>address</b>, initial_permissions: <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_add_member">add_member</a>(
    self: &<b>mut</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a>,
    member: <b>address</b>,
    initial_permissions: VecSet&lt;TypeName&gt;,
) {
    field::add(
        &<b>mut</b> self.id,
        member,
        initial_permissions,
    );
    self.<a href="../myso/permissions_table.md#myso_permissions_table_length">length</a> = self.<a href="../myso/permissions_table.md#myso_permissions_table_length">length</a> + 1;
}
</code></pre>



</details>

<a name="myso_permissions_table_remove_member"></a>

## Function `remove_member`

Removes a member and their entire permission set from the table.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_remove_member">remove_member</a>(self: &<b>mut</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">myso::permissions_table::PermissionsTable</a>, member: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_remove_member">remove_member</a>(self: &<b>mut</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a>, member: <b>address</b>) {
    <b>let</b> _permissions_entry = field::remove&lt;<b>address</b>, VecSet&lt;TypeName&gt;&gt;(&<b>mut</b> self.id, member);
    self.<a href="../myso/permissions_table.md#myso_permissions_table_length">length</a> = self.<a href="../myso/permissions_table.md#myso_permissions_table_length">length</a> - 1;
}
</code></pre>



</details>

<a name="myso_permissions_table_add_permission"></a>

## Function `add_permission`

Adds a permission to an existing member's permission set.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_add_permission">add_permission</a>(self: &<b>mut</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">myso::permissions_table::PermissionsTable</a>, member: <b>address</b>, permission: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_add_permission">add_permission</a>(
    self: &<b>mut</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a>,
    member: <b>address</b>,
    permission: TypeName,
) {
    <b>let</b> permissions = field::borrow_mut&lt;<b>address</b>, VecSet&lt;TypeName&gt;&gt;(&<b>mut</b> self.id, member);
    permissions.insert(permission);
}
</code></pre>



</details>

<a name="myso_permissions_table_remove_permission"></a>

## Function `remove_permission`

Removes a permission from a member's set and returns the remaining permissions.
Useful for checking if the member should be removed (empty set).


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_remove_permission">remove_permission</a>(self: &<b>mut</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">myso::permissions_table::PermissionsTable</a>, member: <b>address</b>, permission: &<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>): &<a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_remove_permission">remove_permission</a>(
    self: &<b>mut</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a>,
    member: <b>address</b>,
    permission: &TypeName,
): &VecSet&lt;TypeName&gt; {
    <b>let</b> permissions = field::borrow_mut&lt;<b>address</b>, VecSet&lt;TypeName&gt;&gt;(&<b>mut</b> self.id, member);
    permissions.remove(permission);
    permissions
}
</code></pre>



</details>

<a name="myso_permissions_table_has_permission"></a>

## Function `has_permission`

Returns whether a member has the specified permission.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_has_permission">has_permission</a>(self: &<a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">myso::permissions_table::PermissionsTable</a>, member: <b>address</b>, permission: &<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_has_permission">has_permission</a>(
    self: &<a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a>,
    member: <b>address</b>,
    permission: &TypeName,
): bool {
    <b>if</b> (!field::exists_(&self.id, member)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> permissions = field::borrow&lt;<b>address</b>, VecSet&lt;TypeName&gt;&gt;(&self.id, member);
    permissions.contains(permission)
}
</code></pre>



</details>

<a name="myso_permissions_table_is_member"></a>

## Function `is_member`

Returns whether the given address is a member (has a dynamic field entry).


<pre><code><b>public</b> <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_is_member">is_member</a>(self: &<a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">myso::permissions_table::PermissionsTable</a>, member: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_is_member">is_member</a>(self: &<a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a>, member: <b>address</b>): bool {
    field::exists_with_type&lt;<b>address</b>, VecSet&lt;TypeName&gt;&gt;(&self.id, member)
}
</code></pre>



</details>

<a name="myso_permissions_table_length"></a>

## Function `length`

Returns the number of members in the table.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_length">length</a>(self: &<a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">myso::permissions_table::PermissionsTable</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_length">length</a>(self: &<a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a>): u64 {
    self.<a href="../myso/permissions_table.md#myso_permissions_table_length">length</a>
}
</code></pre>



</details>

<a name="myso_permissions_table_destroy_empty"></a>

## Function `destroy_empty`

Destroys an empty PermissionsTable.


<a name="@Aborts_2"></a>

### Aborts

- <code><a href="../myso/permissions_table.md#myso_permissions_table_EPermissionsTableNotEmpty">EPermissionsTableNotEmpty</a></code>: if the table still has members


<pre><code><b>public</b> <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_destroy_empty">destroy_empty</a>(self: <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">myso::permissions_table::PermissionsTable</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/permissions_table.md#myso_permissions_table_destroy_empty">destroy_empty</a>(self: <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a>) {
    <b>let</b> <a href="../myso/permissions_table.md#myso_permissions_table_PermissionsTable">PermissionsTable</a> { id, <a href="../myso/permissions_table.md#myso_permissions_table_length">length</a> } = self;
    <b>assert</b>!(<a href="../myso/permissions_table.md#myso_permissions_table_length">length</a> == 0, <a href="../myso/permissions_table.md#myso_permissions_table_EPermissionsTableNotEmpty">EPermissionsTableNotEmpty</a>);
    id.delete();
}
</code></pre>



</details>
