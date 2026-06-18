---
title: Module `contra::policy`
---

Access policies for <code>ConfidentialToken&lt;T&gt;</code>. A <code><a href="../contra/policy.md#contra_policy_Policy">Policy</a></code> records the set of operations that
require a witness of a specific type, used to gate permissioned versions of those operations.


-  [Struct `Policy`](#contra_policy_Policy)
-  [Struct `Auth`](#contra_policy_Auth)
-  [Constants](#@Constants_0)
-  [Function `new`](#contra_policy_new)
-  [Function `permissionless`](#contra_policy_permissionless)
-  [Function `set`](#contra_policy_set)
-  [Function `as_sender`](#contra_policy_as_sender)
-  [Function `as_object`](#contra_policy_as_object)
-  [Function `with_witness`](#contra_policy_with_witness)
-  [Function `is_allowed`](#contra_policy_is_allowed)
-  [Function `is_authenticated`](#contra_policy_is_authenticated)
-  [Function `is_unique`](#contra_policy_is_unique)
-  [Function `create_bitmap`](#contra_policy_create_bitmap)
-  [Function `permissionless_bitmap`](#contra_policy_permissionless_bitmap)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="contra_policy_Policy"></a>

## Struct `Policy`

Access policies for a confidential token.
If set, some operations, as defined in the policy, must be called using their permissioned
version with a witness.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/policy.md#contra_policy_Policy">Policy</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>witness_type: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>permissioned_operations_bitmap: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_policy_Auth"></a>

## Struct `Auth`

A capability authorizing a set of operations on behalf of <code>owner</code>. The phantom <code>T</code> tags the
auth with the consuming domain so an auth minted for one context cannot be used in another.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/policy.md#contra_policy_Auth">Auth</a>&lt;<b>phantom</b> T&gt; <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>operations: u32</code>
</dt>
<dd>
 Bitmap with bit <code>o</code> set iff operation <code>o</code> is allowed.
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="contra_policy_EInvalidOperation"></a>



<pre><code><b>const</b> <a href="../contra/policy.md#contra_policy_EInvalidOperation">EInvalidOperation</a>: u64 = 0;
</code></pre>



<a name="contra_policy_EDuplicateOperation"></a>



<pre><code><b>const</b> <a href="../contra/policy.md#contra_policy_EDuplicateOperation">EDuplicateOperation</a>: u64 = 1;
</code></pre>



<a name="contra_policy_EAuthorizationError"></a>



<pre><code><b>const</b> <a href="../contra/policy.md#contra_policy_EAuthorizationError">EAuthorizationError</a>: u64 = 2;
</code></pre>



<a name="contra_policy_MAX_OPERATION_INDEX"></a>

Permissioned operations are stored as a 32 bit bitmap.


<pre><code><b>const</b> <a href="../contra/policy.md#contra_policy_MAX_OPERATION_INDEX">MAX_OPERATION_INDEX</a>: u8 = 31;
</code></pre>



<a name="contra_policy_new"></a>

## Function `new`

Create a new <code><a href="../contra/policy.md#contra_policy_Policy">Policy</a></code> for witness type <code>W</code> covering the given operations. Aborts if any
operation index exceeds <code><a href="../contra/policy.md#contra_policy_MAX_OPERATION_INDEX">MAX_OPERATION_INDEX</a></code> or if <code>permissioned_operations</code> contains
duplicates.


<pre><code><b>fun</b> <a href="../contra/policy.md#contra_policy_new">new</a>&lt;W&gt;(permissioned_operations: vector&lt;u8&gt;): <a href="../contra/policy.md#contra_policy_Policy">contra::policy::Policy</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/policy.md#contra_policy_new">new</a>&lt;W&gt;(permissioned_operations: vector&lt;u8&gt;): <a href="../contra/policy.md#contra_policy_Policy">Policy</a> {
    <b>assert</b>!(permissioned_operations.all!(|o| *o &lt;= <a href="../contra/policy.md#contra_policy_MAX_OPERATION_INDEX">MAX_OPERATION_INDEX</a>), <a href="../contra/policy.md#contra_policy_EInvalidOperation">EInvalidOperation</a>);
    <b>assert</b>!(<a href="../contra/policy.md#contra_policy_is_unique">is_unique</a>(&permissioned_operations), <a href="../contra/policy.md#contra_policy_EDuplicateOperation">EDuplicateOperation</a>);
    <a href="../contra/policy.md#contra_policy_Policy">Policy</a> {
        witness_type: type_name::with_defining_ids&lt;W&gt;(),
        permissioned_operations_bitmap: <a href="../contra/policy.md#contra_policy_create_bitmap">create_bitmap</a>(permissioned_operations),
    }
}
</code></pre>



</details>

<a name="contra_policy_permissionless"></a>

## Function `permissionless`

A fully permissionless policy slot: every operation is allowed without a witness.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_permissionless">permissionless</a>(): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/policy.md#contra_policy_Policy">contra::policy::Policy</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_permissionless">permissionless</a>(): Option&lt;<a href="../contra/policy.md#contra_policy_Policy">Policy</a>&gt; {
    option::none()
}
</code></pre>



</details>

<a name="contra_policy_set"></a>

## Function `set`

Update <code><a href="../contra/policy.md#contra_policy">policy</a></code> to gate <code>permissioned_operations</code> behind witness <code>W</code>. An empty
<code>permissioned_operations</code> clears the policy entirely (every operation becomes permissionless
again). Aborts if any operation index exceeds <code><a href="../contra/policy.md#contra_policy_MAX_OPERATION_INDEX">MAX_OPERATION_INDEX</a></code> or if
<code>permissioned_operations</code> contains duplicates.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_set">set</a>&lt;W&gt;(<a href="../contra/policy.md#contra_policy">policy</a>: &<b>mut</b> <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/policy.md#contra_policy_Policy">contra::policy::Policy</a>&gt;, permissioned_operations: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_set">set</a>&lt;W&gt;(<a href="../contra/policy.md#contra_policy">policy</a>: &<b>mut</b> Option&lt;<a href="../contra/policy.md#contra_policy_Policy">Policy</a>&gt;, permissioned_operations: vector&lt;u8&gt;) {
    <b>if</b> (permissioned_operations.is_empty()) {
        *<a href="../contra/policy.md#contra_policy">policy</a> = <a href="../contra/policy.md#contra_policy_permissionless">permissionless</a>();
    } <b>else</b> {
        <a href="../contra/policy.md#contra_policy">policy</a>.swap_or_fill(<a href="../contra/policy.md#contra_policy_new">new</a>&lt;W&gt;(permissioned_operations));
    }
}
</code></pre>



</details>

<a name="contra_policy_as_sender"></a>

## Function `as_sender`

Create an <code><a href="../contra/policy.md#contra_policy_Auth">Auth</a>&lt;T&gt;</code> for <code>ctx.sender()</code> covering every operation the policy leaves
permissionless (i.e. every operation NOT listed in the policy's permissioned bitmap). When
<code><a href="../contra/policy.md#contra_policy">policy</a></code> is empty, all operations are permissionless.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_as_sender">as_sender</a>&lt;T&gt;(<a href="../contra/policy.md#contra_policy">policy</a>: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/policy.md#contra_policy_Policy">contra::policy::Policy</a>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_as_sender">as_sender</a>&lt;T&gt;(<a href="../contra/policy.md#contra_policy">policy</a>: &Option&lt;<a href="../contra/policy.md#contra_policy_Policy">Policy</a>&gt;, ctx: &TxContext): <a href="../contra/policy.md#contra_policy_Auth">Auth</a>&lt;T&gt; {
    <a href="../contra/policy.md#contra_policy_Auth">Auth</a> { operations: <a href="../contra/policy.md#contra_policy_permissionless_bitmap">permissionless_bitmap</a>(<a href="../contra/policy.md#contra_policy">policy</a>), owner: ctx.sender() }
}
</code></pre>



</details>

<a name="contra_policy_as_object"></a>

## Function `as_object`

Create an <code><a href="../contra/policy.md#contra_policy_Auth">Auth</a>&lt;T&gt;</code> on behalf of the object identified by <code>uid</code> covering every operation the
policy leaves permissionless. Holding <code>&<b>mut</b> UID</code> proves custody of the object, so the object
self-authenticates as its own <code>owner</code>. Owner address is the inner value of the <code>UID</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_as_object">as_object</a>&lt;T&gt;(<a href="../contra/policy.md#contra_policy">policy</a>: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/policy.md#contra_policy_Policy">contra::policy::Policy</a>&gt;, uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): <a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_as_object">as_object</a>&lt;T&gt;(<a href="../contra/policy.md#contra_policy">policy</a>: &Option&lt;<a href="../contra/policy.md#contra_policy_Policy">Policy</a>&gt;, uid: &<b>mut</b> UID): <a href="../contra/policy.md#contra_policy_Auth">Auth</a>&lt;T&gt; {
    <a href="../contra/policy.md#contra_policy_Auth">Auth</a> { operations: <a href="../contra/policy.md#contra_policy_permissionless_bitmap">permissionless_bitmap</a>(<a href="../contra/policy.md#contra_policy">policy</a>), owner: uid.to_inner().to_address() }
}
</code></pre>



</details>

<a name="contra_policy_with_witness"></a>

## Function `with_witness`

Create an <code><a href="../contra/policy.md#contra_policy_Auth">Auth</a>&lt;T&gt;</code> on behalf of <code>owner</code> covering the requested <code>operation</code>, authorized by
witness <code>W</code>. Aborts unless <code><a href="../contra/policy.md#contra_policy">policy</a></code> is set, its witness type is <code>W</code>, and <code>operation</code> is
permissioned in <code><a href="../contra/policy.md#contra_policy">policy</a></code>. The witness-holding contract is fully responsible for authenticating
<code>owner</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_with_witness">with_witness</a>&lt;T, W: drop&gt;(<a href="../contra/policy.md#contra_policy">policy</a>: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/policy.md#contra_policy_Policy">contra::policy::Policy</a>&gt;, operation: u8, owner: <b>address</b>, _witness: W): <a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_with_witness">with_witness</a>&lt;T, W: drop&gt;(
    <a href="../contra/policy.md#contra_policy">policy</a>: &Option&lt;<a href="../contra/policy.md#contra_policy_Policy">Policy</a>&gt;,
    operation: u8,
    owner: <b>address</b>,
    _witness: W,
): <a href="../contra/policy.md#contra_policy_Auth">Auth</a>&lt;T&gt; {
    <b>assert</b>!(operation &lt;= <a href="../contra/policy.md#contra_policy_MAX_OPERATION_INDEX">MAX_OPERATION_INDEX</a>, <a href="../contra/policy.md#contra_policy_EInvalidOperation">EInvalidOperation</a>);
    <b>assert</b>!(
        <a href="../contra/policy.md#contra_policy">policy</a>.is_some_and!(|p| p.witness_type == type_name::with_defining_ids&lt;W&gt;()),
        <a href="../contra/policy.md#contra_policy_EAuthorizationError">EAuthorizationError</a>,
    );
    <a href="../contra/policy.md#contra_policy_Auth">Auth</a> { operations: 1u32 &lt;&lt; operation, owner }
}
</code></pre>



</details>

<a name="contra_policy_is_allowed"></a>

## Function `is_allowed`

True if <code>auth</code> authorizes <code>operation</code>. Aborts if <code>operation &gt; <a href="../contra/policy.md#contra_policy_MAX_OPERATION_INDEX">MAX_OPERATION_INDEX</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_is_allowed">is_allowed</a>&lt;T&gt;(auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, operation: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_is_allowed">is_allowed</a>&lt;T&gt;(auth: &<a href="../contra/policy.md#contra_policy_Auth">Auth</a>&lt;T&gt;, operation: u8): bool {
    <b>assert</b>!(operation &lt;= <a href="../contra/policy.md#contra_policy_MAX_OPERATION_INDEX">MAX_OPERATION_INDEX</a>, <a href="../contra/policy.md#contra_policy_EInvalidOperation">EInvalidOperation</a>);
    auth.operations & (1 &lt;&lt; operation) != 0
}
</code></pre>



</details>

<a name="contra_policy_is_authenticated"></a>

## Function `is_authenticated`

True if <code>auth</code> authenticates <code>owner</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_is_authenticated">is_authenticated</a>&lt;T&gt;(auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, owner: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/policy.md#contra_policy_is_authenticated">is_authenticated</a>&lt;T&gt;(auth: &<a href="../contra/policy.md#contra_policy_Auth">Auth</a>&lt;T&gt;, owner: <b>address</b>): bool {
    auth.owner == owner
}
</code></pre>



</details>

<a name="contra_policy_is_unique"></a>

## Function `is_unique`

Return <code><b>true</b></code> iff <code>v</code> contains no duplicates.


<pre><code><b>fun</b> <a href="../contra/policy.md#contra_policy_is_unique">is_unique</a>&lt;T: <b>copy</b>, drop&gt;(v: &vector&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/policy.md#contra_policy_is_unique">is_unique</a>&lt;T: <b>copy</b> + drop&gt;(v: &vector&lt;T&gt;): bool {
    <b>let</b> <b>mut</b> seen = vector[];
    v.all!(|item| {
        <b>if</b> (seen.contains(item)) <b>false</b>
        <b>else</b> { seen.push_back(*item); <b>true</b> }
    })
}
</code></pre>



</details>

<a name="contra_policy_create_bitmap"></a>

## Function `create_bitmap`

Build a <code>u32</code> bitmap with bit <code>o</code> set for each <code>o</code> in <code>operations</code>. Caller must ensure each
<code>o &lt;= <a href="../contra/policy.md#contra_policy_MAX_OPERATION_INDEX">MAX_OPERATION_INDEX</a></code>.


<pre><code><b>fun</b> <a href="../contra/policy.md#contra_policy_create_bitmap">create_bitmap</a>(operations: vector&lt;u8&gt;): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/policy.md#contra_policy_create_bitmap">create_bitmap</a>(operations: vector&lt;u8&gt;): u32 {
    operations.fold!(0, |acc, operation| acc | (1 &lt;&lt; operation))
}
</code></pre>



</details>

<a name="contra_policy_permissionless_bitmap"></a>

## Function `permissionless_bitmap`

The bitmap of operations that <code><a href="../contra/policy.md#contra_policy">policy</a></code> leaves permissionless. When <code><a href="../contra/policy.md#contra_policy">policy</a></code> is <code>None</code>, every
operation is permissionless (<code>u32::MAX</code>); otherwise it's the bitwise complement of the
policy's permissioned bitmap.


<pre><code><b>fun</b> <a href="../contra/policy.md#contra_policy_permissionless_bitmap">permissionless_bitmap</a>(<a href="../contra/policy.md#contra_policy">policy</a>: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/policy.md#contra_policy_Policy">contra::policy::Policy</a>&gt;): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/policy.md#contra_policy_permissionless_bitmap">permissionless_bitmap</a>(<a href="../contra/policy.md#contra_policy">policy</a>: &Option&lt;<a href="../contra/policy.md#contra_policy_Policy">Policy</a>&gt;): u32 {
    <a href="../contra/policy.md#contra_policy">policy</a>.map_ref!(|p| p.permissioned_operations_bitmap ^ 0xFFFFFFFFu32).destroy_or!(0xFFFFFFFFu32)
}
</code></pre>



</details>
