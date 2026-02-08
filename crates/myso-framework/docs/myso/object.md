---
title: Module `myso::object`
---

MySo object identifiers


-  [Struct `ID`](#myso_object_ID)
-  [Struct `UID`](#myso_object_UID)
-  [Constants](#@Constants_0)
-  [Function `id_to_bytes`](#myso_object_id_to_bytes)
-  [Function `id_to_address`](#myso_object_id_to_address)
-  [Function `id_from_bytes`](#myso_object_id_from_bytes)
-  [Function `id_from_address`](#myso_object_id_from_address)
-  [Function `myso_system_state`](#myso_object_myso_system_state)
-  [Function `clock`](#myso_object_clock)
-  [Function `authenticator_state`](#myso_object_authenticator_state)
-  [Function `randomness_state`](#myso_object_randomness_state)
-  [Function `myso_deny_list_object_id`](#myso_object_myso_deny_list_object_id)
-  [Function `myso_accumulator_root_object_id`](#myso_object_myso_accumulator_root_object_id)
-  [Function `myso_accumulator_root_address`](#myso_object_myso_accumulator_root_address)
-  [Function `myso_coin_registry_object_id`](#myso_object_myso_coin_registry_object_id)
-  [Function `myso_coin_registry_address`](#myso_object_myso_coin_registry_address)
-  [Function `bridge`](#myso_object_bridge)
-  [Function `address_alias_state`](#myso_object_address_alias_state)
-  [Function `uid_as_inner`](#myso_object_uid_as_inner)
-  [Function `uid_to_inner`](#myso_object_uid_to_inner)
-  [Function `uid_to_bytes`](#myso_object_uid_to_bytes)
-  [Function `uid_to_address`](#myso_object_uid_to_address)
-  [Function `new`](#myso_object_new)
-  [Function `delete`](#myso_object_delete)
-  [Function `id`](#myso_object_id)
-  [Function `borrow_id`](#myso_object_borrow_id)
-  [Function `id_bytes`](#myso_object_id_bytes)
-  [Function `id_address`](#myso_object_id_address)
-  [Function `borrow_uid`](#myso_object_borrow_uid)
-  [Function `new_uid_from_hash`](#myso_object_new_uid_from_hash)
-  [Function `delete_impl`](#myso_object_delete_impl)
-  [Function `record_new_uid`](#myso_object_record_new_uid)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="myso_object_ID"></a>

## Struct `ID`

An object ID. This is used to reference MySo Objects.
This is *not* guaranteed to be globally unique--anyone can create an <code><a href="../myso/object.md#myso_object_ID">ID</a></code> from a <code><a href="../myso/object.md#myso_object_UID">UID</a></code> or
from an object, and ID's can be freely copied and dropped.
Here, the values are not globally unique because there can be multiple values of type <code><a href="../myso/object.md#myso_object_ID">ID</a></code>
with the same underlying bytes. For example, <code><a href="../myso/object.md#myso_object_id">object::id</a>(&obj)</code> can be called as many times
as you want for a given <code>obj</code>, and each <code><a href="../myso/object.md#myso_object_ID">ID</a></code> value will be identical.


<pre><code><b>public</b> <b>struct</b> <a href="../myso/object.md#myso_object_ID">ID</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bytes: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_object_UID"></a>

## Struct `UID`

Globally unique IDs that define an object's ID in storage. Any MySo Object, that is a struct
with the <code>key</code> ability, must have <code><a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_UID">UID</a></code> as its first field.
These are globally unique in the sense that no two values of type <code><a href="../myso/object.md#myso_object_UID">UID</a></code> are ever equal, in
other words for any two values <code>id1: <a href="../myso/object.md#myso_object_UID">UID</a></code> and <code>id2: <a href="../myso/object.md#myso_object_UID">UID</a></code>, <code>id1</code> != <code>id2</code>.
This is a privileged type that can only be derived from a <code>TxContext</code>.
<code><a href="../myso/object.md#myso_object_UID">UID</a></code> doesn't have the <code>drop</code> ability, so deleting a <code><a href="../myso/object.md#myso_object_UID">UID</a></code> requires a call to <code><a href="../myso/object.md#myso_object_delete">delete</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../myso/object.md#myso_object_UID">UID</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_object_MYSO_SYSTEM_STATE_OBJECT_ID"></a>

The hardcoded ID for the singleton MySo System State Object.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_MYSO_SYSTEM_STATE_OBJECT_ID">MYSO_SYSTEM_STATE_OBJECT_ID</a>: <b>address</b> = 0x5;
</code></pre>



<a name="myso_object_MYSO_CLOCK_OBJECT_ID"></a>

The hardcoded ID for the singleton Clock Object.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_MYSO_CLOCK_OBJECT_ID">MYSO_CLOCK_OBJECT_ID</a>: <b>address</b> = 0x6;
</code></pre>



<a name="myso_object_MYSO_AUTHENTICATOR_STATE_ID"></a>

The hardcoded ID for the singleton AuthenticatorState Object.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_MYSO_AUTHENTICATOR_STATE_ID">MYSO_AUTHENTICATOR_STATE_ID</a>: <b>address</b> = 0x7;
</code></pre>



<a name="myso_object_MYSO_RANDOM_ID"></a>

The hardcoded ID for the singleton Random Object.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_MYSO_RANDOM_ID">MYSO_RANDOM_ID</a>: <b>address</b> = 0x8;
</code></pre>



<a name="myso_object_MYSO_DENY_LIST_OBJECT_ID"></a>

The hardcoded ID for the singleton DenyList.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_MYSO_DENY_LIST_OBJECT_ID">MYSO_DENY_LIST_OBJECT_ID</a>: <b>address</b> = 0x403;
</code></pre>



<a name="myso_object_MYSO_ACCUMULATOR_ROOT_OBJECT_ID"></a>

The hardcoded ID for the singleton AccumulatorRoot Object.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_MYSO_ACCUMULATOR_ROOT_OBJECT_ID">MYSO_ACCUMULATOR_ROOT_OBJECT_ID</a>: <b>address</b> = 0xacc;
</code></pre>



<a name="myso_object_MYSO_BRIDGE_ID"></a>

The hardcoded ID for the Bridge Object.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_MYSO_BRIDGE_ID">MYSO_BRIDGE_ID</a>: <b>address</b> = 0x9;
</code></pre>



<a name="myso_object_MYSO_COIN_REGISTRY_OBJECT_ID"></a>

The hardcoded ID for the Coin Registry Object.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_MYSO_COIN_REGISTRY_OBJECT_ID">MYSO_COIN_REGISTRY_OBJECT_ID</a>: <b>address</b> = 0xc;
</code></pre>



<a name="myso_object_MYSO_ADDRESS_ALIAS_STATE_ID"></a>

The hardcoded ID for the AddressAliasState Object.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_MYSO_ADDRESS_ALIAS_STATE_ID">MYSO_ADDRESS_ALIAS_STATE_ID</a>: <b>address</b> = 0xa;
</code></pre>



<a name="myso_object_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../myso/object.md#myso_object_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="myso_object_id_to_bytes"></a>

## Function `id_to_bytes`

Get the raw bytes of a <code><a href="../myso/object.md#myso_object_ID">ID</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_to_bytes">id_to_bytes</a>(<a href="../myso/object.md#myso_object_id">id</a>: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_to_bytes">id_to_bytes</a>(<a href="../myso/object.md#myso_object_id">id</a>: &<a href="../myso/object.md#myso_object_ID">ID</a>): vector&lt;u8&gt; {
    <a href="../myso/bcs.md#myso_bcs_to_bytes">bcs::to_bytes</a>(&<a href="../myso/object.md#myso_object_id">id</a>.bytes)
}
</code></pre>



</details>

<a name="myso_object_id_to_address"></a>

## Function `id_to_address`

Get the inner bytes of <code><a href="../myso/object.md#myso_object_id">id</a></code> as an address.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_to_address">id_to_address</a>(<a href="../myso/object.md#myso_object_id">id</a>: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_to_address">id_to_address</a>(<a href="../myso/object.md#myso_object_id">id</a>: &<a href="../myso/object.md#myso_object_ID">ID</a>): <b>address</b> {
    <a href="../myso/object.md#myso_object_id">id</a>.bytes
}
</code></pre>



</details>

<a name="myso_object_id_from_bytes"></a>

## Function `id_from_bytes`

Make an <code><a href="../myso/object.md#myso_object_ID">ID</a></code> from raw bytes.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_from_bytes">id_from_bytes</a>(bytes: vector&lt;u8&gt;): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_from_bytes">id_from_bytes</a>(bytes: vector&lt;u8&gt;): <a href="../myso/object.md#myso_object_ID">ID</a> {
    <a href="../myso/address.md#myso_address_from_bytes">address::from_bytes</a>(bytes).to_id()
}
</code></pre>



</details>

<a name="myso_object_id_from_address"></a>

## Function `id_from_address`

Make an <code><a href="../myso/object.md#myso_object_ID">ID</a></code> from an address.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_from_address">id_from_address</a>(bytes: <b>address</b>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_from_address">id_from_address</a>(bytes: <b>address</b>): <a href="../myso/object.md#myso_object_ID">ID</a> {
    <a href="../myso/object.md#myso_object_ID">ID</a> { bytes }
}
</code></pre>



</details>

<a name="myso_object_myso_system_state"></a>

## Function `myso_system_state`

Create the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for the singleton <code>MySoSystemState</code> object.
This should only be called once from <code>myso_system</code>.


<pre><code><b>fun</b> <a href="../myso/object.md#myso_object_myso_system_state">myso_system_state</a>(ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso/object.md#myso_object_myso_system_state">myso_system_state</a>(ctx: &TxContext): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../myso/object.md#myso_object_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: <a href="../myso/object.md#myso_object_MYSO_SYSTEM_STATE_OBJECT_ID">MYSO_SYSTEM_STATE_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="myso_object_clock"></a>

## Function `clock`

Create the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for the singleton <code>Clock</code> object.
This should only be called once from <code><a href="../myso/clock.md#myso_clock">clock</a></code>.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/clock.md#myso_clock">clock</a>(): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/clock.md#myso_clock">clock</a>(): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: <a href="../myso/object.md#myso_object_MYSO_CLOCK_OBJECT_ID">MYSO_CLOCK_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="myso_object_authenticator_state"></a>

## Function `authenticator_state`

Create the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for the singleton <code>AuthenticatorState</code> object.
This should only be called once from <code><a href="../myso/authenticator_state.md#myso_authenticator_state">authenticator_state</a></code>.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/authenticator_state.md#myso_authenticator_state">authenticator_state</a>(): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/authenticator_state.md#myso_authenticator_state">authenticator_state</a>(): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: <a href="../myso/object.md#myso_object_MYSO_AUTHENTICATOR_STATE_ID">MYSO_AUTHENTICATOR_STATE_ID</a> },
    }
}
</code></pre>



</details>

<a name="myso_object_randomness_state"></a>

## Function `randomness_state`

Create the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for the singleton <code>Random</code> object.
This should only be called once from <code><a href="../myso/random.md#myso_random">random</a></code>.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_randomness_state">randomness_state</a>(): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_randomness_state">randomness_state</a>(): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: <a href="../myso/object.md#myso_object_MYSO_RANDOM_ID">MYSO_RANDOM_ID</a> },
    }
}
</code></pre>



</details>

<a name="myso_object_myso_deny_list_object_id"></a>

## Function `myso_deny_list_object_id`

Create the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for the singleton <code>DenyList</code> object.
This should only be called once from <code><a href="../myso/deny_list.md#myso_deny_list">deny_list</a></code>.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_deny_list_object_id">myso_deny_list_object_id</a>(): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_deny_list_object_id">myso_deny_list_object_id</a>(): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: <a href="../myso/object.md#myso_object_MYSO_DENY_LIST_OBJECT_ID">MYSO_DENY_LIST_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="myso_object_myso_accumulator_root_object_id"></a>

## Function `myso_accumulator_root_object_id`



<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_accumulator_root_object_id">myso_accumulator_root_object_id</a>(): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_accumulator_root_object_id">myso_accumulator_root_object_id</a>(): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: <a href="../myso/object.md#myso_object_MYSO_ACCUMULATOR_ROOT_OBJECT_ID">MYSO_ACCUMULATOR_ROOT_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="myso_object_myso_accumulator_root_address"></a>

## Function `myso_accumulator_root_address`



<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_accumulator_root_address">myso_accumulator_root_address</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_accumulator_root_address">myso_accumulator_root_address</a>(): <b>address</b> {
    <a href="../myso/object.md#myso_object_MYSO_ACCUMULATOR_ROOT_OBJECT_ID">MYSO_ACCUMULATOR_ROOT_OBJECT_ID</a>
}
</code></pre>



</details>

<a name="myso_object_myso_coin_registry_object_id"></a>

## Function `myso_coin_registry_object_id`

Create the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for the singleton <code>CoinRegistry</code> object.
This should only be called once from <code><a href="../myso/coin_registry.md#myso_coin_registry">coin_registry</a></code>.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_coin_registry_object_id">myso_coin_registry_object_id</a>(): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_coin_registry_object_id">myso_coin_registry_object_id</a>(): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: <a href="../myso/object.md#myso_object_MYSO_COIN_REGISTRY_OBJECT_ID">MYSO_COIN_REGISTRY_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="myso_object_myso_coin_registry_address"></a>

## Function `myso_coin_registry_address`



<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_coin_registry_address">myso_coin_registry_address</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_myso_coin_registry_address">myso_coin_registry_address</a>(): <b>address</b> {
    <a href="../myso/object.md#myso_object_MYSO_COIN_REGISTRY_OBJECT_ID">MYSO_COIN_REGISTRY_OBJECT_ID</a>
}
</code></pre>



</details>

<a name="myso_object_bridge"></a>

## Function `bridge`

Create the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for the singleton <code>Bridge</code> object.
This should only be called once from <code><a href="../myso/object.md#myso_object_bridge">bridge</a></code>.


<pre><code><b>fun</b> <a href="../myso/object.md#myso_object_bridge">bridge</a>(): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso/object.md#myso_object_bridge">bridge</a>(): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: <a href="../myso/object.md#myso_object_MYSO_BRIDGE_ID">MYSO_BRIDGE_ID</a> },
    }
}
</code></pre>



</details>

<a name="myso_object_address_alias_state"></a>

## Function `address_alias_state`

Create the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for the singleton <code>AddressAliasState</code> object.
This should only be called once from <code><a href="../myso/address_alias.md#myso_address_alias">address_alias</a></code>.


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_address_alias_state">address_alias_state</a>(): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_address_alias_state">address_alias_state</a>(): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: <a href="../myso/object.md#myso_object_MYSO_ADDRESS_ALIAS_STATE_ID">MYSO_ADDRESS_ALIAS_STATE_ID</a> },
    }
}
</code></pre>



</details>

<a name="myso_object_uid_as_inner"></a>

## Function `uid_as_inner`

Get the inner <code><a href="../myso/object.md#myso_object_ID">ID</a></code> of <code>uid</code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_uid_as_inner">uid_as_inner</a>(uid: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_uid_as_inner">uid_as_inner</a>(uid: &<a href="../myso/object.md#myso_object_UID">UID</a>): &<a href="../myso/object.md#myso_object_ID">ID</a> {
    &uid.<a href="../myso/object.md#myso_object_id">id</a>
}
</code></pre>



</details>

<a name="myso_object_uid_to_inner"></a>

## Function `uid_to_inner`

Get the raw bytes of a <code>uid</code>'s inner <code><a href="../myso/object.md#myso_object_ID">ID</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_uid_to_inner">uid_to_inner</a>(uid: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_uid_to_inner">uid_to_inner</a>(uid: &<a href="../myso/object.md#myso_object_UID">UID</a>): <a href="../myso/object.md#myso_object_ID">ID</a> {
    uid.<a href="../myso/object.md#myso_object_id">id</a>
}
</code></pre>



</details>

<a name="myso_object_uid_to_bytes"></a>

## Function `uid_to_bytes`

Get the raw bytes of a <code><a href="../myso/object.md#myso_object_UID">UID</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_uid_to_bytes">uid_to_bytes</a>(uid: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_uid_to_bytes">uid_to_bytes</a>(uid: &<a href="../myso/object.md#myso_object_UID">UID</a>): vector&lt;u8&gt; {
    <a href="../myso/bcs.md#myso_bcs_to_bytes">bcs::to_bytes</a>(&uid.<a href="../myso/object.md#myso_object_id">id</a>.bytes)
}
</code></pre>



</details>

<a name="myso_object_uid_to_address"></a>

## Function `uid_to_address`

Get the inner bytes of <code><a href="../myso/object.md#myso_object_id">id</a></code> as an address.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_uid_to_address">uid_to_address</a>(uid: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_uid_to_address">uid_to_address</a>(uid: &<a href="../myso/object.md#myso_object_UID">UID</a>): <b>address</b> {
    uid.<a href="../myso/object.md#myso_object_id">id</a>.bytes
}
</code></pre>



</details>

<a name="myso_object_new"></a>

## Function `new`

Create a new object. Returns the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> that must be stored in a MySo object.
This is the only way to create <code><a href="../myso/object.md#myso_object_UID">UID</a></code>s.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_new">new</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_UID">UID</a> {
        <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes: ctx.fresh_object_address() },
    }
}
</code></pre>



</details>

<a name="myso_object_delete"></a>

## Function `delete`

Delete the object and its <code><a href="../myso/object.md#myso_object_UID">UID</a></code>. This is the only way to eliminate a <code><a href="../myso/object.md#myso_object_UID">UID</a></code>.
This exists to inform MySo of object deletions. When an object
gets unpacked, the programmer will have to do something with its
<code><a href="../myso/object.md#myso_object_UID">UID</a></code>. The implementation of this function emits a deleted
system event so MySo knows to process the object deletion


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_delete">delete</a>(<a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_delete">delete</a>(<a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_UID">UID</a>) {
    <b>let</b> <a href="../myso/object.md#myso_object_UID">UID</a> { <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes } } = <a href="../myso/object.md#myso_object_id">id</a>;
    <a href="../myso/object.md#myso_object_delete_impl">delete_impl</a>(bytes)
}
</code></pre>



</details>

<a name="myso_object_id"></a>

## Function `id`

Get the underlying <code><a href="../myso/object.md#myso_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id">id</a>&lt;T: key&gt;(obj: &T): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id">id</a>&lt;T: key&gt;(obj: &T): <a href="../myso/object.md#myso_object_ID">ID</a> {
    <a href="../myso/object.md#myso_object_borrow_uid">borrow_uid</a>(obj).<a href="../myso/object.md#myso_object_id">id</a>
}
</code></pre>



</details>

<a name="myso_object_borrow_id"></a>

## Function `borrow_id`

Borrow the underlying <code><a href="../myso/object.md#myso_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_borrow_id">borrow_id</a>&lt;T: key&gt;(obj: &T): &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_borrow_id">borrow_id</a>&lt;T: key&gt;(obj: &T): &<a href="../myso/object.md#myso_object_ID">ID</a> {
    &<a href="../myso/object.md#myso_object_borrow_uid">borrow_uid</a>(obj).<a href="../myso/object.md#myso_object_id">id</a>
}
</code></pre>



</details>

<a name="myso_object_id_bytes"></a>

## Function `id_bytes`

Get the raw bytes for the underlying <code><a href="../myso/object.md#myso_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_bytes">id_bytes</a>&lt;T: key&gt;(obj: &T): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_bytes">id_bytes</a>&lt;T: key&gt;(obj: &T): vector&lt;u8&gt; {
    <a href="../myso/bcs.md#myso_bcs_to_bytes">bcs::to_bytes</a>(&<a href="../myso/object.md#myso_object_borrow_uid">borrow_uid</a>(obj).<a href="../myso/object.md#myso_object_id">id</a>)
}
</code></pre>



</details>

<a name="myso_object_id_address"></a>

## Function `id_address`

Get the inner bytes for the underlying <code><a href="../myso/object.md#myso_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_address">id_address</a>&lt;T: key&gt;(obj: &T): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object.md#myso_object_id_address">id_address</a>&lt;T: key&gt;(obj: &T): <b>address</b> {
    <a href="../myso/object.md#myso_object_borrow_uid">borrow_uid</a>(obj).<a href="../myso/object.md#myso_object_id">id</a>.bytes
}
</code></pre>



</details>

<a name="myso_object_borrow_uid"></a>

## Function `borrow_uid`

Get the <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for <code>obj</code>.
Safe because MySo has an extra bytecode verifier pass that forces every struct with
the <code>key</code> ability to have a distinguished <code><a href="../myso/object.md#myso_object_UID">UID</a></code> field.
Cannot be made public as the access to <code><a href="../myso/object.md#myso_object_UID">UID</a></code> for a given object must be privileged, and
restrictable in the object's module.


<pre><code><b>fun</b> <a href="../myso/object.md#myso_object_borrow_uid">borrow_uid</a>&lt;T: key&gt;(obj: &T): &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../myso/object.md#myso_object_borrow_uid">borrow_uid</a>&lt;T: key&gt;(obj: &T): &<a href="../myso/object.md#myso_object_UID">UID</a>;
</code></pre>



</details>

<a name="myso_object_new_uid_from_hash"></a>

## Function `new_uid_from_hash`

Generate a new UID specifically used for creating a UID from a hash


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_new_uid_from_hash">new_uid_from_hash</a>(bytes: <b>address</b>): <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../myso/package.md#myso_package">package</a>) <b>fun</b> <a href="../myso/object.md#myso_object_new_uid_from_hash">new_uid_from_hash</a>(bytes: <b>address</b>): <a href="../myso/object.md#myso_object_UID">UID</a> {
    <a href="../myso/object.md#myso_object_record_new_uid">record_new_uid</a>(bytes);
    <a href="../myso/object.md#myso_object_UID">UID</a> { <a href="../myso/object.md#myso_object_id">id</a>: <a href="../myso/object.md#myso_object_ID">ID</a> { bytes } }
}
</code></pre>



</details>

<a name="myso_object_delete_impl"></a>

## Function `delete_impl`



<pre><code><b>fun</b> <a href="../myso/object.md#myso_object_delete_impl">delete_impl</a>(<a href="../myso/object.md#myso_object_id">id</a>: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../myso/object.md#myso_object_delete_impl">delete_impl</a>(<a href="../myso/object.md#myso_object_id">id</a>: <b>address</b>);
</code></pre>



</details>

<a name="myso_object_record_new_uid"></a>

## Function `record_new_uid`



<pre><code><b>fun</b> <a href="../myso/object.md#myso_object_record_new_uid">record_new_uid</a>(<a href="../myso/object.md#myso_object_id">id</a>: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../myso/object.md#myso_object_record_new_uid">record_new_uid</a>(<a href="../myso/object.md#myso_object_id">id</a>: <b>address</b>);
</code></pre>



</details>
