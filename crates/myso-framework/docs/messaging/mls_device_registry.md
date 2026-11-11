---
title: Module `messaging::mls_device_registry`
---

MLS device registry (E2EE M6: certificate renewal).

Anchors each messaging device's long-lived **authority key** (ed25519) to its account on-chain,
so device certificates can be renewed by the device alone (no new zkLogin proof), verified by
anyone at any time, and revoked.

- <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_register">register</a></code>: the **account** (tx sender; for zkLogin the chain checked the proof when the tx ran)
binds <code>(owner, device_id)</code> to an authority key. Once per device id; gas sponsored by the app.
- <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_rotate">rotate</a></code>: the **device itself** replaces its authority key. The tx sender must be the current
authority key's address, so no account signature (and no zkLogin proof) is needed. The old key
keeps verifying for <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ROTATION_GRACE_MS">ROTATION_GRACE_MS</a></code>, so certificates in flight don't fail.
- <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoke">revoke</a></code> (by the account) / <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoke_by_device">revoke_by_device</a></code> (by any active device of the same account,
including itself): permanent for that device id; both keys stop verifying at once.

The registry is a derived object of <code>MessagingNamespace</code> (key <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry">mls_device_registry</a></code>), so its
address is deterministic. It is created once, at genesis, in <code>messaging::create_genesis_objects</code>.

Verification rule (clients; docs/myso-messaging-stack/E2EE-Protocol-MLS.md): a v2 device
certificate signed by key K is valid iff the record for <code>(wallet, device_id)</code> exists, is not
revoked, and K is <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a></code>, or K is <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a></code> and now < <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_valid_until_ms">previous_valid_until_ms</a></code>.


-  [Struct `MlsDeviceRegistry`](#messaging_mls_device_registry_MlsDeviceRegistry)
-  [Struct `DeviceKey`](#messaging_mls_device_registry_DeviceKey)
-  [Struct `DeviceRecord`](#messaging_mls_device_registry_DeviceRecord)
-  [Struct `MlsDeviceRegistered`](#messaging_mls_device_registry_MlsDeviceRegistered)
-  [Struct `MlsDeviceKeyRotated`](#messaging_mls_device_registry_MlsDeviceKeyRotated)
-  [Struct `MlsDeviceRevoked`](#messaging_mls_device_registry_MlsDeviceRevoked)
-  [Constants](#@Constants_0)
-  [Function `new`](#messaging_mls_device_registry_new)
-  [Function `share`](#messaging_mls_device_registry_share)
-  [Function `derivation_key`](#messaging_mls_device_registry_derivation_key)
-  [Function `authority_address`](#messaging_mls_device_registry_authority_address)
-  [Function `assert_key`](#messaging_mls_device_registry_assert_key)
-  [Function `assert_device_id`](#messaging_mls_device_registry_assert_device_id)
-  [Function `do_revoke`](#messaging_mls_device_registry_do_revoke)
-  [Function `sender_device`](#messaging_mls_device_registry_sender_device)
-  [Function `register`](#messaging_mls_device_registry_register)
-  [Function `rotate`](#messaging_mls_device_registry_rotate)
-  [Function `revoke`](#messaging_mls_device_registry_revoke)
-  [Function `revoke_by_device`](#messaging_mls_device_registry_revoke_by_device)
-  [Function `device`](#messaging_mls_device_registry_device)
-  [Function `is_valid_key`](#messaging_mls_device_registry_is_valid_key)
-  [Function `authority_key`](#messaging_mls_device_registry_authority_key)
-  [Function `previous_key`](#messaging_mls_device_registry_previous_key)
-  [Function `previous_valid_until_ms`](#messaging_mls_device_registry_previous_valid_until_ms)
-  [Function `generation`](#messaging_mls_device_registry_generation)
-  [Function `registered_at_ms`](#messaging_mls_device_registry_registered_at_ms)
-  [Function `revoked_at_ms`](#messaging_mls_device_registry_revoked_at_ms)
-  [Function `rotation_grace_ms`](#messaging_mls_device_registry_rotation_grace_ms)


<pre><code><b>use</b> <a href="../messaging/version.md#messaging_version">messaging::version</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="messaging_mls_device_registry_MlsDeviceRegistry"></a>

## Struct `MlsDeviceRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a> <b>has</b> key
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
<code>devices: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">messaging::mls_device_registry::DeviceKey</a>, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">messaging::mls_device_registry::DeviceRecord</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>by_authority: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">messaging::mls_device_registry::DeviceKey</a>&gt;</code>
</dt>
<dd>
 Current authority key address → its device (authorizes <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_rotate">rotate</a></code> / <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoke_by_device">revoke_by_device</a></code>).
</dd>
</dl>


</details>

<a name="messaging_mls_device_registry_DeviceKey"></a>

## Struct `DeviceKey`

Table key. BCS: <code>owner (32 bytes) || device_id (ULEB128 length || UTF-8)</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">DeviceKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>device_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="messaging_mls_device_registry_DeviceRecord"></a>

## Struct `DeviceRecord`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">DeviceRecord</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 Current ed25519 authority public key (32 bytes).
</dd>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 Key replaced by the last rotation (empty if never rotated).
</dd>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_valid_until_ms">previous_valid_until_ms</a>: u64</code>
</dt>
<dd>
 <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a></code> verifies until this time (0 if never rotated).
</dd>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a>: u64</code>
</dt>
<dd>
 1 at registration, +1 per rotation.
</dd>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_registered_at_ms">registered_at_ms</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoked_at_ms">revoked_at_ms</a>: u64</code>
</dt>
<dd>
 0 = active.
</dd>
</dl>


</details>

<a name="messaging_mls_device_registry_MlsDeviceRegistered"></a>

## Struct `MlsDeviceRegistered`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistered">MlsDeviceRegistered</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>device_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="messaging_mls_device_registry_MlsDeviceKeyRotated"></a>

## Struct `MlsDeviceKeyRotated`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceKeyRotated">MlsDeviceKeyRotated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>device_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="messaging_mls_device_registry_MlsDeviceRevoked"></a>

## Struct `MlsDeviceRevoked`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRevoked">MlsDeviceRevoked</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>device_id: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>by_device: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 The device that revoked it (<code>none</code> = the account itself).
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="messaging_mls_device_registry_EBadKeyLength"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_EBadKeyLength">EBadKeyLength</a>: u64 = 0;
</code></pre>



<a name="messaging_mls_device_registry_EAlreadyRegistered"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_EAlreadyRegistered">EAlreadyRegistered</a>: u64 = 1;
</code></pre>



<a name="messaging_mls_device_registry_ENotFound"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ENotFound">ENotFound</a>: u64 = 2;
</code></pre>



<a name="messaging_mls_device_registry_ENotAuthorized"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ENotAuthorized">ENotAuthorized</a>: u64 = 3;
</code></pre>



<a name="messaging_mls_device_registry_ERevoked"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ERevoked">ERevoked</a>: u64 = 4;
</code></pre>



<a name="messaging_mls_device_registry_ESameKey"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ESameKey">ESameKey</a>: u64 = 5;
</code></pre>



<a name="messaging_mls_device_registry_EBadDeviceId"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_EBadDeviceId">EBadDeviceId</a>: u64 = 6;
</code></pre>



<a name="messaging_mls_device_registry_EKeyInUse"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_EKeyInUse">EKeyInUse</a>: u64 = 7;
</code></pre>



<a name="messaging_mls_device_registry_MLS_DEVICE_REGISTRY_DERIVATION_KEY"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MLS_DEVICE_REGISTRY_DERIVATION_KEY">MLS_DEVICE_REGISTRY_DERIVATION_KEY</a>: vector&lt;u8&gt; = vector[109, 108, 115, 95, 100, 101, 118, 105, 99, 101, 95, 114, 101, 103, 105, 115, 116, 114, 121];
</code></pre>



<a name="messaging_mls_device_registry_ED25519_KEY_LENGTH"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ED25519_KEY_LENGTH">ED25519_KEY_LENGTH</a>: u64 = 32;
</code></pre>



<a name="messaging_mls_device_registry_MAX_DEVICE_ID_LENGTH"></a>



<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MAX_DEVICE_ID_LENGTH">MAX_DEVICE_ID_LENGTH</a>: u64 = 128;
</code></pre>



<a name="messaging_mls_device_registry_ROTATION_GRACE_MS"></a>

After <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_rotate">rotate</a></code>, certificates under the previous key keep verifying this long (7 days).


<pre><code><b>const</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ROTATION_GRACE_MS">ROTATION_GRACE_MS</a>: u64 = 604800000;
</code></pre>



<a name="messaging_mls_device_registry_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_new">new</a>(namespace_uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_new">new</a>(namespace_uid: &<b>mut</b> UID, ctx: &<b>mut</b> TxContext): <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a> {
    <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a> {
        id: derived_object::claim(namespace_uid, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MLS_DEVICE_REGISTRY_DERIVATION_KEY">MLS_DEVICE_REGISTRY_DERIVATION_KEY</a>.to_string()),
        devices: table::new(ctx),
        by_authority: table::new(ctx),
    }
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_share"></a>

## Function `share`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_share">share</a>(self: <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_share">share</a>(self: <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a>) {
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_derivation_key"></a>

## Function `derivation_key`

Matches <code><a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MLS_DEVICE_REGISTRY_DERIVATION_KEY">MLS_DEVICE_REGISTRY_DERIVATION_KEY</a></code> in the TS SDK.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_derivation_key">derivation_key</a>(): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_derivation_key">derivation_key</a>(): String {
    <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MLS_DEVICE_REGISTRY_DERIVATION_KEY">MLS_DEVICE_REGISTRY_DERIVATION_KEY</a>.to_string()
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_authority_address"></a>

## Function `authority_address`

The MySo address of an ed25519 public key: <code>blake2b256(0x00 || key)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_address">authority_address</a>(key: &vector&lt;u8&gt;): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_address">authority_address</a>(key: &vector&lt;u8&gt;): <b>address</b> {
    <b>let</b> <b>mut</b> bytes = vector[0u8];
    bytes.append(*key);
    address::from_bytes(hash::blake2b256(&bytes))
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_assert_key"></a>

## Function `assert_key`



<pre><code><b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_assert_key">assert_key</a>(key: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_assert_key">assert_key</a>(key: &vector&lt;u8&gt;) {
    <b>assert</b>!(key.length() == <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ED25519_KEY_LENGTH">ED25519_KEY_LENGTH</a>, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_EBadKeyLength">EBadKeyLength</a>);
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_assert_device_id"></a>

## Function `assert_device_id`



<pre><code><b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_assert_device_id">assert_device_id</a>(device_id: &<a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_assert_device_id">assert_device_id</a>(device_id: &String) {
    <b>let</b> len = device_id.length();
    <b>assert</b>!(len &gt; 0 && len &lt;= <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MAX_DEVICE_ID_LENGTH">MAX_DEVICE_ID_LENGTH</a>, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_EBadDeviceId">EBadDeviceId</a>);
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_do_revoke"></a>

## Function `do_revoke`



<pre><code><b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_do_revoke">do_revoke</a>(registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>, key: <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">messaging::mls_device_registry::DeviceKey</a>, by_device: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_do_revoke">do_revoke</a>(
    registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a>,
    key: <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">DeviceKey</a>,
    by_device: Option&lt;String&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(registry.devices.contains(key), <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ENotFound">ENotFound</a>);
    <b>let</b> record = registry.devices.borrow_mut(key);
    <b>assert</b>!(record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoked_at_ms">revoked_at_ms</a> == 0, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ERevoked">ERevoked</a>);
    record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoked_at_ms">revoked_at_ms</a> = clock.timestamp_ms();
    <b>let</b> authority = <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_address">authority_address</a>(&record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>);
    <b>if</b> (registry.by_authority.contains(authority)) {
        registry.by_authority.remove(authority);
    };
    event::emit(<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRevoked">MlsDeviceRevoked</a> { owner: key.owner, device_id: key.device_id, by_device });
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_sender_device"></a>

## Function `sender_device`

The active device whose current authority key sent this tx.


<pre><code><b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_sender_device">sender_device</a>(registry: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">messaging::mls_device_registry::DeviceKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_sender_device">sender_device</a>(registry: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a>, ctx: &TxContext): <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">DeviceKey</a> {
    <b>let</b> sender = ctx.sender();
    <b>assert</b>!(registry.by_authority.contains(sender), <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ENotAuthorized">ENotAuthorized</a>);
    <b>let</b> key = *registry.by_authority.borrow(sender);
    <b>assert</b>!(registry.devices.borrow(key).<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoked_at_ms">revoked_at_ms</a> == 0, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ERevoked">ERevoked</a>);
    key
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_register"></a>

## Function `register`

The account (tx sender) registers one of its devices. Gas is sponsored by the app.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_register">register</a>(registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>, <a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, device_id: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_register">register</a>(
    registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a>,
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    device_id: String,
    <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_assert_device_id">assert_device_id</a>(&device_id);
    <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_assert_key">assert_key</a>(&<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>);
    <b>let</b> key = <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">DeviceKey</a> { owner: ctx.sender(), device_id };
    <b>assert</b>!(!registry.devices.contains(key), <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_EAlreadyRegistered">EAlreadyRegistered</a>);
    <b>let</b> authority = <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_address">authority_address</a>(&<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>);
    <b>assert</b>!(!registry.by_authority.contains(authority), <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_EKeyInUse">EKeyInUse</a>);
    registry
        .devices
        .add(
            key,
            <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">DeviceRecord</a> {
                <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>,
                <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a>: vector[],
                <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_valid_until_ms">previous_valid_until_ms</a>: 0,
                <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a>: 1,
                <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_registered_at_ms">registered_at_ms</a>: clock.timestamp_ms(),
                <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoked_at_ms">revoked_at_ms</a>: 0,
            },
        );
    registry.by_authority.add(authority, key);
    event::emit(<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistered">MlsDeviceRegistered</a> { owner: key.owner, device_id: key.device_id, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a> });
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_rotate"></a>

## Function `rotate`

The device (tx sender = its current authority key's address) rotates to <code>new_key</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_rotate">rotate</a>(registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>, <a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, new_key: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_rotate">rotate</a>(
    registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a>,
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    new_key: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_assert_key">assert_key</a>(&new_key);
    <b>let</b> key = <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_sender_device">sender_device</a>(registry, ctx);
    <b>assert</b>!(registry.devices.borrow(key).<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a> != new_key, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ESameKey">ESameKey</a>);
    <b>let</b> new_authority = <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_address">authority_address</a>(&new_key);
    <b>assert</b>!(!registry.by_authority.contains(new_authority), <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_EKeyInUse">EKeyInUse</a>);
    <b>let</b> record = registry.devices.borrow_mut(key);
    <b>let</b> old = record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>;
    record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a> = old;
    record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_valid_until_ms">previous_valid_until_ms</a> = clock.timestamp_ms() + <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ROTATION_GRACE_MS">ROTATION_GRACE_MS</a>;
    record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a> = new_key;
    record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a> = record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a> + 1;
    <b>let</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a> = record.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a>;
    registry.by_authority.remove(ctx.sender());
    registry.by_authority.add(new_authority, key);
    event::emit(<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceKeyRotated">MlsDeviceKeyRotated</a> {
        owner: key.owner,
        device_id: key.device_id,
        <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>: new_key,
        <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a>: old,
        <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a>,
    });
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_revoke"></a>

## Function `revoke`

The account revokes one of its devices (permanent for that device id).


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoke">revoke</a>(registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>, <a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, device_id: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoke">revoke</a>(
    registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a>,
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    device_id: String,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_do_revoke">do_revoke</a>(registry, <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">DeviceKey</a> { owner: ctx.sender(), device_id }, option::none(), clock);
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_revoke_by_device"></a>

## Function `revoke_by_device`

An active device of the same account (tx sender = its authority key's address) revokes
<code>device_id</code>: another device, or itself (logout). No account signature needed.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoke_by_device">revoke_by_device</a>(registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>, <a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, device_id: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoke_by_device">revoke_by_device</a>(
    registry: &<b>mut</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a>,
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    device_id: String,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <b>let</b> revoker = <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_sender_device">sender_device</a>(registry, ctx);
    <b>let</b> target = <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">DeviceKey</a> { owner: revoker.owner, device_id };
    <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_do_revoke">do_revoke</a>(registry, target, option::some(revoker.device_id), clock);
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_device"></a>

## Function `device`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_device">device</a>(registry: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>, owner: <b>address</b>, device_id: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">messaging::mls_device_registry::DeviceRecord</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_device">device</a>(
    registry: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a>,
    owner: <b>address</b>,
    device_id: String,
): Option&lt;<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">DeviceRecord</a>&gt; {
    <b>let</b> key = <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">DeviceKey</a> { owner, device_id };
    <b>if</b> (registry.devices.contains(key)) option::some(*registry.devices.borrow(key))
    <b>else</b> option::none()
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_is_valid_key"></a>

## Function `is_valid_key`

Whether a certificate signed by <code>key</code> verifies for <code>(owner, device_id)</code> at <code>now_ms</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_is_valid_key">is_valid_key</a>(registry: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">messaging::mls_device_registry::MlsDeviceRegistry</a>, owner: <b>address</b>, device_id: <a href="../std/string.md#std_string_String">std::string::String</a>, key: vector&lt;u8&gt;, now_ms: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_is_valid_key">is_valid_key</a>(
    registry: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_MlsDeviceRegistry">MlsDeviceRegistry</a>,
    owner: <b>address</b>,
    device_id: String,
    key: vector&lt;u8&gt;,
    now_ms: u64,
): bool {
    <b>let</b> k = <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceKey">DeviceKey</a> { owner, device_id };
    <b>if</b> (!registry.devices.contains(k)) <b>return</b> <b>false</b>;
    <b>let</b> r = registry.devices.borrow(k);
    <b>if</b> (r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoked_at_ms">revoked_at_ms</a> != 0) <b>return</b> <b>false</b>;
    key == r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a> || (key == r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a> && now_ms &lt; r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_valid_until_ms">previous_valid_until_ms</a>)
}
</code></pre>



</details>

<a name="messaging_mls_device_registry_authority_key"></a>

## Function `authority_key`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">messaging::mls_device_registry::DeviceRecord</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">DeviceRecord</a>): vector&lt;u8&gt; { r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_authority_key">authority_key</a> }
</code></pre>



</details>

<a name="messaging_mls_device_registry_previous_key"></a>

## Function `previous_key`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">messaging::mls_device_registry::DeviceRecord</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">DeviceRecord</a>): vector&lt;u8&gt; { r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_key">previous_key</a> }
</code></pre>



</details>

<a name="messaging_mls_device_registry_previous_valid_until_ms"></a>

## Function `previous_valid_until_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_valid_until_ms">previous_valid_until_ms</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">messaging::mls_device_registry::DeviceRecord</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_valid_until_ms">previous_valid_until_ms</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">DeviceRecord</a>): u64 { r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_previous_valid_until_ms">previous_valid_until_ms</a> }
</code></pre>



</details>

<a name="messaging_mls_device_registry_generation"></a>

## Function `generation`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">messaging::mls_device_registry::DeviceRecord</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">DeviceRecord</a>): u64 { r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_generation">generation</a> }
</code></pre>



</details>

<a name="messaging_mls_device_registry_registered_at_ms"></a>

## Function `registered_at_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_registered_at_ms">registered_at_ms</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">messaging::mls_device_registry::DeviceRecord</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_registered_at_ms">registered_at_ms</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">DeviceRecord</a>): u64 { r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_registered_at_ms">registered_at_ms</a> }
</code></pre>



</details>

<a name="messaging_mls_device_registry_revoked_at_ms"></a>

## Function `revoked_at_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoked_at_ms">revoked_at_ms</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">messaging::mls_device_registry::DeviceRecord</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoked_at_ms">revoked_at_ms</a>(r: &<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_DeviceRecord">DeviceRecord</a>): u64 { r.<a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_revoked_at_ms">revoked_at_ms</a> }
</code></pre>



</details>

<a name="messaging_mls_device_registry_rotation_grace_ms"></a>

## Function `rotation_grace_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_rotation_grace_ms">rotation_grace_ms</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_rotation_grace_ms">rotation_grace_ms</a>(): u64 { <a href="../messaging/mls_device_registry.md#messaging_mls_device_registry_ROTATION_GRACE_MS">ROTATION_GRACE_MS</a> }
</code></pre>



</details>
