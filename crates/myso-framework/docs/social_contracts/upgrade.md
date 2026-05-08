---
title: Module `social_contracts::upgrade`
---

Module to manage package upgrades for MySocialContracts.
Provides versioning support for all shared objects.


-  [Struct `UpgradeAdminCap`](#social_contracts_upgrade_UpgradeAdminCap)
-  [Struct `UpgradeEvent`](#social_contracts_upgrade_UpgradeEvent)
-  [Struct `ObjectMigratedEvent`](#social_contracts_upgrade_ObjectMigratedEvent)
-  [Constants](#@Constants_0)
-  [Function `authorize_upgrade`](#social_contracts_upgrade_authorize_upgrade)
-  [Function `commit_upgrade`](#social_contracts_upgrade_commit_upgrade)
-  [Function `version`](#social_contracts_upgrade_version)
-  [Function `package_id`](#social_contracts_upgrade_package_id)
-  [Function `current_version`](#social_contracts_upgrade_current_version)
-  [Function `assert_version`](#social_contracts_upgrade_assert_version)
-  [Function `emit_migration_event`](#social_contracts_upgrade_emit_migration_event)
-  [Function `create_upgrade_admin_cap`](#social_contracts_upgrade_create_upgrade_admin_cap)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
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



<a name="social_contracts_upgrade_UpgradeAdminCap"></a>

## Struct `UpgradeAdminCap`

Admin capability for package upgrades


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">UpgradeAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_upgrade_UpgradeEvent"></a>

## Struct `UpgradeEvent`

Event emitted when a package is upgraded


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeEvent">UpgradeEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/upgrade.md#social_contracts_upgrade_package_id">package_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_upgrade_ObjectMigratedEvent"></a>

## Struct `ObjectMigratedEvent`

Event emitted when a shared object is migrated to a new version


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_ObjectMigratedEvent">ObjectMigratedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>object_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>old_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>migrated_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_upgrade_EInvalidDigest"></a>



<pre><code><b>const</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_EInvalidDigest">EInvalidDigest</a>: u64 = 0;
</code></pre>



<a name="social_contracts_upgrade_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_EWrongVersion">EWrongVersion</a>: u64 = 1;
</code></pre>



<a name="social_contracts_upgrade_CURRENT_VERSION"></a>



<pre><code><b>const</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_CURRENT_VERSION">CURRENT_VERSION</a>: u64 = 1;
</code></pre>



<a name="social_contracts_upgrade_authorize_upgrade"></a>

## Function `authorize_upgrade`

Authorize an upgrade with the upgrade cap


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_authorize_upgrade">authorize_upgrade</a>(cap: &<b>mut</b> <a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>, digest: vector&lt;u8&gt;): <a href="../myso/package.md#myso_package_UpgradeTicket">myso::package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_authorize_upgrade">authorize_upgrade</a>(
    cap: &<b>mut</b> package::UpgradeCap,
    digest: vector&lt;u8&gt;
): package::UpgradeTicket {
    // Verify digest length is 32 bytes
    <b>assert</b>!(vector::length(&digest) == 32, <a href="../social_contracts/upgrade.md#social_contracts_upgrade_EInvalidDigest">EInvalidDigest</a>);
    // Use default <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> policy
    <b>let</b> policy = cap.policy();
    // Return the <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> ticket
    cap.<a href="../social_contracts/upgrade.md#social_contracts_upgrade_authorize_upgrade">authorize_upgrade</a>(policy, digest)
}
</code></pre>



</details>

<a name="social_contracts_upgrade_commit_upgrade"></a>

## Function `commit_upgrade`

Commit an upgrade with the receipt


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_commit_upgrade">commit_upgrade</a>(cap: &<b>mut</b> <a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>, receipt: <a href="../myso/package.md#myso_package_UpgradeReceipt">myso::package::UpgradeReceipt</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_commit_upgrade">commit_upgrade</a>(
    cap: &<b>mut</b> package::UpgradeCap,
    receipt: package::UpgradeReceipt
) {
    // Emit <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> event
    event::emit(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeEvent">UpgradeEvent</a> {
        <a href="../social_contracts/upgrade.md#social_contracts_upgrade_package_id">package_id</a>: receipt.package(),
        <a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>: cap.<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>() + 1
    });
    // Commit the <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a>
    cap.<a href="../social_contracts/upgrade.md#social_contracts_upgrade_commit_upgrade">commit_upgrade</a>(receipt);
}
</code></pre>



</details>

<a name="social_contracts_upgrade_version"></a>

## Function `version`

Get the current package version


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>(cap: &<a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>(cap: &package::UpgradeCap): u64 {
    cap.<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>()
}
</code></pre>



</details>

<a name="social_contracts_upgrade_package_id"></a>

## Function `package_id`

Get the package ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_package_id">package_id</a>(cap: &<a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_package_id">package_id</a>(cap: &package::UpgradeCap): object::ID {
    cap.package()
}
</code></pre>



</details>

<a name="social_contracts_upgrade_current_version"></a>

## Function `current_version`

Get the current package version constant


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">current_version</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">current_version</a>(): u64 {
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_CURRENT_VERSION">CURRENT_VERSION</a>
}
</code></pre>



</details>

<a name="social_contracts_upgrade_assert_version"></a>

## Function `assert_version`

Check if the version matches the current package version


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_assert_version">assert_version</a>(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_assert_version">assert_version</a>(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>: u64) {
    <b>assert</b>!(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../social_contracts/upgrade.md#social_contracts_upgrade_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_upgrade_emit_migration_event"></a>

## Function `emit_migration_event`

Helper function to emit migration event
This can be called directly by other modules implementing their own migration


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">emit_migration_event</a>(object_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, object_type: <a href="../std/string.md#std_string_String">std::string::String</a>, old_version: u64, migrated_by: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">emit_migration_event</a>(
    object_id: ID,
    object_type: String,
    old_version: u64,
    migrated_by: <b>address</b>
) {
    event::emit(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_ObjectMigratedEvent">ObjectMigratedEvent</a> {
        object_id,
        object_type,
        old_version,
        new_version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_CURRENT_VERSION">CURRENT_VERSION</a>,
        migrated_by
    });
}
</code></pre>



</details>

<a name="social_contracts_upgrade_create_upgrade_admin_cap"></a>

## Function `create_upgrade_admin_cap`

Create an UpgradeAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_create_upgrade_admin_cap">create_upgrade_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_create_upgrade_admin_cap">create_upgrade_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">UpgradeAdminCap</a> {
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">UpgradeAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>
