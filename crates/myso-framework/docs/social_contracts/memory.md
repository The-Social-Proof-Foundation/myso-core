---
title: Module `social_contracts::memory`
---

Memory — account and encrypted-memory access policy for delegated keys.

Registry plus shared <code><a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a></code> per owner, linked to <code><a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a></code>.

Register <code><a href="../social_contracts/memory.md#social_contracts_memory_approve_key_policy">approve_key_policy</a></code> with your key service alongside marketplace policies
(see <code><a href="../social_contracts/mydata.md#social_contracts_mydata">social_contracts::mydata</a></code>). <code><a href="../social_contracts/memory.md#social_contracts_memory_owner_key_suffix_bytes">owner_key_suffix_bytes</a></code> is the canonical suffix for owner-scoped
key material clients construct at encrypt time.


-  [Struct `MemoryRegistry`](#social_contracts_memory_MemoryRegistry)
-  [Struct `MemoryAccount`](#social_contracts_memory_MemoryAccount)
-  [Struct `MemoryDelegateKey`](#social_contracts_memory_MemoryDelegateKey)
-  [Struct `MemoryAccountCreated`](#social_contracts_memory_MemoryAccountCreated)
-  [Struct `MemoryDelegateKeyAdded`](#social_contracts_memory_MemoryDelegateKeyAdded)
-  [Struct `MemoryDelegateKeyRemoved`](#social_contracts_memory_MemoryDelegateKeyRemoved)
-  [Struct `MemoryAccountDeactivated`](#social_contracts_memory_MemoryAccountDeactivated)
-  [Struct `MemoryAccountReactivated`](#social_contracts_memory_MemoryAccountReactivated)
-  [Struct `MemoryAccountMigrated`](#social_contracts_memory_MemoryAccountMigrated)
-  [Struct `MemoryRegistryMigrated`](#social_contracts_memory_MemoryRegistryMigrated)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_memory_bootstrap_init)
-  [Function `create_account_for_profile`](#social_contracts_memory_create_account_for_profile)
-  [Function `transfer_account_owner_with_profile`](#social_contracts_memory_transfer_account_owner_with_profile)
-  [Function `add_delegate_key`](#social_contracts_memory_add_delegate_key)
-  [Function `remove_delegate_key`](#social_contracts_memory_remove_delegate_key)
-  [Function `deactivate_account`](#social_contracts_memory_deactivate_account)
-  [Function `reactivate_account`](#social_contracts_memory_reactivate_account)
-  [Function `migrate_account`](#social_contracts_memory_migrate_account)
-  [Function `admin_migrate_account`](#social_contracts_memory_admin_migrate_account)
-  [Function `migrate_registry`](#social_contracts_memory_migrate_registry)
-  [Function `profile_id`](#social_contracts_memory_profile_id)
-  [Function `is_delegate`](#social_contracts_memory_is_delegate)
-  [Function `is_delegate_address`](#social_contracts_memory_is_delegate_address)
-  [Function `owner`](#social_contracts_memory_owner)
-  [Function `delegate_count`](#social_contracts_memory_delegate_count)
-  [Function `has_account`](#social_contracts_memory_has_account)
-  [Function `account_id_for_owner`](#social_contracts_memory_account_id_for_owner)
-  [Function `is_active`](#social_contracts_memory_is_active)
-  [Function `account_version`](#social_contracts_memory_account_version)
-  [Function `registry_version`](#social_contracts_memory_registry_version)
-  [Function `current_contract_version`](#social_contracts_memory_current_contract_version)
-  [Function `approve_key_policy`](#social_contracts_memory_approve_key_policy)
-  [Function `owner_key_suffix_bytes`](#social_contracts_memory_owner_key_suffix_bytes)
-  [Function `get_version`](#social_contracts_memory_get_version)
-  [Function `set_version`](#social_contracts_memory_set_version)
-  [Function `bump_version`](#social_contracts_memory_bump_version)
-  [Function `assert_object_version`](#social_contracts_memory_assert_object_version)
-  [Function `assert_cap_for_this_package`](#social_contracts_memory_assert_cap_for_this_package)
-  [Function `has_suffix`](#social_contracts_memory_has_suffix)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
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



<a name="social_contracts_memory_MemoryRegistry"></a>

## Struct `MemoryRegistry`

Shared singleton — maps owner address → shared <code><a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a></code> id.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a> <b>has</b> key
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
<code>accounts: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccount"></a>

## Struct `MemoryAccount`

Shared memory account — one per owner when linked from profile flows.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a> <b>has</b> key, store
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
<code><a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
 <code>object::uid_to_address</code> of the linked [<code><a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a></code>].
</dd>
<dt>
<code>delegate_keys: vector&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemoryDelegateKey">social_contracts::memory::MemoryDelegateKey</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryDelegateKey"></a>

## Struct `MemoryDelegateKey`

Authorized Ed25519 delegate key and its derived on-chain address.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryDelegateKey">MemoryDelegateKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>label: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccountCreated"></a>

## Struct `MemoryAccountCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountCreated">MemoryAccountCreated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryDelegateKeyAdded"></a>

## Struct `MemoryDelegateKeyAdded`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryDelegateKeyAdded">MemoryDelegateKeyAdded</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>label: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryDelegateKeyRemoved"></a>

## Struct `MemoryDelegateKeyRemoved`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryDelegateKeyRemoved">MemoryDelegateKeyRemoved</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccountDeactivated"></a>

## Struct `MemoryAccountDeactivated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountDeactivated">MemoryAccountDeactivated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccountReactivated"></a>

## Struct `MemoryAccountReactivated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountReactivated">MemoryAccountReactivated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccountMigrated"></a>

## Struct `MemoryAccountMigrated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountMigrated">MemoryAccountMigrated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>from: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>to: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryRegistryMigrated"></a>

## Struct `MemoryRegistryMigrated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistryMigrated">MemoryRegistryMigrated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>registry_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>from: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>to: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_memory_EDelegateKeyAlreadyExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EDelegateKeyAlreadyExists">EDelegateKeyAlreadyExists</a>: u64 = 0;
</code></pre>



<a name="social_contracts_memory_EDelegateKeyNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EDelegateKeyNotFound">EDelegateKeyNotFound</a>: u64 = 1;
</code></pre>



<a name="social_contracts_memory_ETooManyDelegateKeys"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ETooManyDelegateKeys">ETooManyDelegateKeys</a>: u64 = 2;
</code></pre>



<a name="social_contracts_memory_EAccountAlreadyExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EAccountAlreadyExists">EAccountAlreadyExists</a>: u64 = 3;
</code></pre>



<a name="social_contracts_memory_ENotOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>: u64 = 4;
</code></pre>



<a name="social_contracts_memory_EInvalidPublicKeyLength"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidPublicKeyLength">EInvalidPublicKeyLength</a>: u64 = 5;
</code></pre>



<a name="social_contracts_memory_EAccountDeactivated"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>: u64 = 6;
</code></pre>



<a name="social_contracts_memory_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EWrongVersion">EWrongVersion</a>: u64 = 7;
</code></pre>



<a name="social_contracts_memory_ENotUpgradeAuthority"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENotUpgradeAuthority">ENotUpgradeAuthority</a>: u64 = 8;
</code></pre>



<a name="social_contracts_memory_EAlreadyMigrated"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EAlreadyMigrated">EAlreadyMigrated</a>: u64 = 9;
</code></pre>



<a name="social_contracts_memory_ELabelTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ELabelTooLong">ELabelTooLong</a>: u64 = 10;
</code></pre>



<a name="social_contracts_memory_EAccountAlreadyActive"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EAccountAlreadyActive">EAccountAlreadyActive</a>: u64 = 11;
</code></pre>



<a name="social_contracts_memory_ENoAccess"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>: u64 = 100;
</code></pre>



<a name="social_contracts_memory_ENewOwnerHasMemoryAccount"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENewOwnerHasMemoryAccount">ENewOwnerHasMemoryAccount</a>: u64 = 12;
</code></pre>



<a name="social_contracts_memory_ERegistryAccountMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ERegistryAccountMismatch">ERegistryAccountMismatch</a>: u64 = 13;
</code></pre>



<a name="social_contracts_memory_MAX_DELEGATE_KEYS"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_MAX_DELEGATE_KEYS">MAX_DELEGATE_KEYS</a>: u64 = 20;
</code></pre>



<a name="social_contracts_memory_ED25519_PUBLIC_KEY_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ED25519_PUBLIC_KEY_LENGTH">ED25519_PUBLIC_KEY_LENGTH</a>: u64 = 32;
</code></pre>



<a name="social_contracts_memory_MAX_LABEL_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_MAX_LABEL_LENGTH">MAX_LABEL_LENGTH</a>: u64 = 64;
</code></pre>



<a name="social_contracts_memory_VERSION"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>: u64 = 1;
</code></pre>



<a name="social_contracts_memory_VERSION_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>: vector&lt;u8&gt; = vector[109, 101, 109, 111, 114, 121, 95, 118, 101, 114, 115, 105, 111, 110];
</code></pre>



<a name="social_contracts_memory_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> <b>mut</b> registry = <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a> {
        id: object::new(ctx),
        accounts: table::new(ctx),
    };
    <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(&<b>mut</b> registry.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_memory_create_account_for_profile"></a>

## Function `create_account_for_profile`

Create and share a <code><a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a></code>, register by <code>tx_context::sender</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_account_for_profile">create_account_for_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>, <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_account_for_profile">create_account_for_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>,
    <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): ID {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&registry.id);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(!table::contains(&registry.accounts, sender), <a href="../social_contracts/memory.md#social_contracts_memory_EAccountAlreadyExists">EAccountAlreadyExists</a>);
    <b>let</b> <b>mut</b> account = <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a> {
        id: object::new(ctx),
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: sender,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        delegate_keys: vector::empty(),
        created_at: clock::timestamp_ms(clock),
        active: <b>true</b>,
    };
    <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(&<b>mut</b> account.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    <b>let</b> account_id = object::id(&account);
    table::add(&<b>mut</b> registry.accounts, sender, account_id);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountCreated">MemoryAccountCreated</a> {
        account_id,
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: sender,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
    });
    transfer::share_object(account);
    account_id
}
</code></pre>



</details>

<a name="social_contracts_memory_transfer_account_owner_with_profile"></a>

## Function `transfer_account_owner_with_profile`

Keep registry and account owner aligned when the profile is transferred.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_transfer_account_owner_with_profile">transfer_account_owner_with_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b>, old_owner: <b>address</b>, new_owner: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_transfer_account_owner_with_profile">transfer_account_owner_with_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>,
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b>,
    old_owner: <b>address</b>,
    new_owner: <b>address</b>,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&registry.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == old_owner, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a> == <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(table::contains(&registry.accounts, old_owner), <a href="../social_contracts/memory.md#social_contracts_memory_ERegistryAccountMismatch">ERegistryAccountMismatch</a>);
    <b>assert</b>!(*table::borrow(&registry.accounts, old_owner) == object::id(account), <a href="../social_contracts/memory.md#social_contracts_memory_ERegistryAccountMismatch">ERegistryAccountMismatch</a>);
    <b>assert</b>!(!table::contains(&registry.accounts, new_owner), <a href="../social_contracts/memory.md#social_contracts_memory_ENewOwnerHasMemoryAccount">ENewOwnerHasMemoryAccount</a>);
    table::remove(&<b>mut</b> registry.accounts, old_owner);
    account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> = new_owner;
    table::add(&<b>mut</b> registry.accounts, new_owner, object::id(account));
}
</code></pre>



</details>

<a name="social_contracts_memory_add_delegate_key"></a>

## Function `add_delegate_key`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_add_delegate_key">add_delegate_key</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_add_delegate_key">add_delegate_key</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    public_key: vector&lt;u8&gt;,
    derived_address: <b>address</b>,
    label: String,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == tx_context::sender(ctx), <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    <b>assert</b>!(vector::length(&public_key) == <a href="../social_contracts/memory.md#social_contracts_memory_ED25519_PUBLIC_KEY_LENGTH">ED25519_PUBLIC_KEY_LENGTH</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidPublicKeyLength">EInvalidPublicKeyLength</a>);
    <b>assert</b>!(string::length(&label) &lt;= <a href="../social_contracts/memory.md#social_contracts_memory_MAX_LABEL_LENGTH">MAX_LABEL_LENGTH</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ELabelTooLong">ELabelTooLong</a>);
    <b>assert</b>!(vector::length(&account.delegate_keys) &lt; <a href="../social_contracts/memory.md#social_contracts_memory_MAX_DELEGATE_KEYS">MAX_DELEGATE_KEYS</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ETooManyDelegateKeys">ETooManyDelegateKeys</a>);
    <b>let</b> len = vector::length(&account.delegate_keys);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> existing = vector::borrow(&account.delegate_keys, i);
        <b>assert</b>!(existing.public_key != public_key, <a href="../social_contracts/memory.md#social_contracts_memory_EDelegateKeyAlreadyExists">EDelegateKeyAlreadyExists</a>);
        i = i + 1;
    };
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_MemoryDelegateKey">MemoryDelegateKey</a> {
        public_key,
        derived_address,
        label,
        created_at: clock::timestamp_ms(clock),
    };
    <b>let</b> account_id = object::id(account);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryDelegateKeyAdded">MemoryDelegateKeyAdded</a> {
        account_id,
        public_key: key.public_key,
        derived_address: key.derived_address,
        label: key.label,
    });
    vector::push_back(&<b>mut</b> account.delegate_keys, key);
}
</code></pre>



</details>

<a name="social_contracts_memory_remove_delegate_key"></a>

## Function `remove_delegate_key`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_remove_delegate_key">remove_delegate_key</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, public_key: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_remove_delegate_key">remove_delegate_key</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    public_key: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == tx_context::sender(ctx), <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>let</b> <b>mut</b> found = <b>false</b>;
    <b>let</b> <b>mut</b> derived_address = @0x0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&account.delegate_keys);
    <b>while</b> (i &lt; len) {
        <b>let</b> k = vector::borrow(&account.delegate_keys, i);
        <b>if</b> (k.public_key == public_key) {
            derived_address = k.derived_address;
            vector::remove(&<b>mut</b> account.delegate_keys, i);
            found = <b>true</b>;
            <b>break</b>
        };
        i = i + 1;
    };
    <b>assert</b>!(found, <a href="../social_contracts/memory.md#social_contracts_memory_EDelegateKeyNotFound">EDelegateKeyNotFound</a>);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryDelegateKeyRemoved">MemoryDelegateKeyRemoved</a> {
        account_id: object::id(account),
        public_key,
        derived_address,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_deactivate_account"></a>

## Function `deactivate_account`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_deactivate_account">deactivate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_deactivate_account">deactivate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, ctx: &TxContext) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == tx_context::sender(ctx), <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    account.active = <b>false</b>;
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountDeactivated">MemoryAccountDeactivated</a> {
        account_id: object::id(account),
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_reactivate_account"></a>

## Function `reactivate_account`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_reactivate_account">reactivate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_reactivate_account">reactivate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, ctx: &TxContext) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == tx_context::sender(ctx), <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(!account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountAlreadyActive">EAccountAlreadyActive</a>);
    account.active = <b>true</b>;
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountReactivated">MemoryAccountReactivated</a> {
        account_id: object::id(account),
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_migrate_account"></a>

## Function `migrate_account`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_migrate_account">migrate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_migrate_account">migrate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, ctx: &TxContext) {
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == tx_context::sender(ctx), <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>let</b> cur = <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&account.id);
    <b>assert</b>!(cur &lt; <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EAlreadyMigrated">EAlreadyMigrated</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(&<b>mut</b> account.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountMigrated">MemoryAccountMigrated</a> {
        account_id: object::id(account),
        from: cur,
        to: <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_admin_migrate_account"></a>

## Function `admin_migrate_account`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_admin_migrate_account">admin_migrate_account</a>(cap: &<a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_admin_migrate_account">admin_migrate_account</a>(cap: &UpgradeCap, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_cap_for_this_package">assert_cap_for_this_package</a>(cap);
    <b>let</b> cur = <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&account.id);
    <b>assert</b>!(cur &lt; <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EAlreadyMigrated">EAlreadyMigrated</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(&<b>mut</b> account.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountMigrated">MemoryAccountMigrated</a> {
        account_id: object::id(account),
        from: cur,
        to: <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_migrate_registry"></a>

## Function `migrate_registry`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_migrate_registry">migrate_registry</a>(cap: &<a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>, registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_migrate_registry">migrate_registry</a>(cap: &UpgradeCap, registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_cap_for_this_package">assert_cap_for_this_package</a>(cap);
    <b>let</b> cur = <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&registry.id);
    <b>assert</b>!(cur &lt; <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EAlreadyMigrated">EAlreadyMigrated</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(&<b>mut</b> registry.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistryMigrated">MemoryRegistryMigrated</a> {
        registry_id: object::id(registry),
        from: cur,
        to: <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_profile_id"></a>

## Function `profile_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>): <b>address</b> {
    account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>
}
</code></pre>



</details>

<a name="social_contracts_memory_is_delegate"></a>

## Function `is_delegate`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_delegate">is_delegate</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, public_key: &vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_delegate">is_delegate</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, public_key: &vector&lt;u8&gt;): bool {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&account.delegate_keys);
    <b>while</b> (i &lt; len) {
        <b>if</b> (&vector::borrow(&account.delegate_keys, i).public_key == public_key) {
            <b>return</b> <b>true</b>
        };
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_memory_is_delegate_address"></a>

## Function `is_delegate_address`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_delegate_address">is_delegate_address</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_delegate_address">is_delegate_address</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, addr: <b>address</b>): bool {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = vector::length(&account.delegate_keys);
    <b>while</b> (i &lt; len) {
        <b>if</b> (vector::borrow(&account.delegate_keys, i).derived_address == addr) {
            <b>return</b> <b>true</b>
        };
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_memory_owner"></a>

## Function `owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>): <b>address</b> {
    account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>
}
</code></pre>



</details>

<a name="social_contracts_memory_delegate_count"></a>

## Function `delegate_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_delegate_count">delegate_count</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_delegate_count">delegate_count</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>): u64 {
    vector::length(&account.delegate_keys)
}
</code></pre>



</details>

<a name="social_contracts_memory_has_account"></a>

## Function `has_account`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_account">has_account</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_account">has_account</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>, addr: <b>address</b>): bool {
    table::contains(&registry.accounts, addr)
}
</code></pre>



</details>

<a name="social_contracts_memory_account_id_for_owner"></a>

## Function `account_id_for_owner`

Account id registered for owner, if any.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_account_id_for_owner">account_id_for_owner</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>, <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_account_id_for_owner">account_id_for_owner</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>, <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b>): Option&lt;ID&gt; {
    <b>if</b> (table::contains(&registry.accounts, <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>)) {
        option::some(*table::borrow(&registry.accounts, <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_is_active"></a>

## Function `is_active`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_active">is_active</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_active">is_active</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>): bool {
    account.active
}
</code></pre>



</details>

<a name="social_contracts_memory_account_version"></a>

## Function `account_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_account_version">account_version</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_account_version">account_version</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>): u64 {
    <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&account.id)
}
</code></pre>



</details>

<a name="social_contracts_memory_registry_version"></a>

## Function `registry_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_registry_version">registry_version</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_registry_version">registry_version</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>): u64 {
    <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&registry.id)
}
</code></pre>



</details>

<a name="social_contracts_memory_current_contract_version"></a>

## Function `current_contract_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_current_contract_version">current_contract_version</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_current_contract_version">current_contract_version</a>(): u64 {
    <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>
}
</code></pre>



</details>

<a name="social_contracts_memory_approve_key_policy"></a>

## Function `approve_key_policy`

Key-server dry-run entry: allow owner (matching key id suffix) or a registered delegate.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_policy">approve_key_policy</a>(id: vector&lt;u8&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_policy">approve_key_policy</a>(id: vector&lt;u8&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, ctx: &TxContext) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> owner_bytes = bcs::to_bytes(&account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>);
    <b>let</b> is_owner = (caller == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) && <a href="../social_contracts/memory.md#social_contracts_memory_has_suffix">has_suffix</a>(&id, &owner_bytes);
    <b>let</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_delegate">is_delegate</a> = <a href="../social_contracts/memory.md#social_contracts_memory_is_delegate_address">is_delegate_address</a>(account, caller);
    <b>assert</b>!(is_owner || <a href="../social_contracts/memory.md#social_contracts_memory_is_delegate">is_delegate</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_owner_key_suffix_bytes"></a>

## Function `owner_key_suffix_bytes`

Raw bytes for the owner portion of a key id (<code>package_id</code> prefix is added by client tooling).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_owner_key_suffix_bytes">owner_key_suffix_bytes</a>(owner_addr: <b>address</b>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_owner_key_suffix_bytes">owner_key_suffix_bytes</a>(owner_addr: <b>address</b>): vector&lt;u8&gt; {
    bcs::to_bytes(&owner_addr)
}
</code></pre>



</details>

<a name="social_contracts_memory_get_version"></a>

## Function `get_version`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(uid: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(uid: &UID): u64 {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, u64&gt;(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>)) {
        *df::borrow&lt;vector&lt;u8&gt;, u64&gt;(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>)
    } <b>else</b> {
        1
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_set_version"></a>

## Function `set_version`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, v: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(uid: &<b>mut</b> UID, v: u64) {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, u64&gt;(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>)) {
        <b>let</b> r = df::borrow_mut&lt;vector&lt;u8&gt;, u64&gt;(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>);
        *r = v;
    } <b>else</b> {
        df::add(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>, v);
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_bump_version"></a>

## Function `bump_version`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, v: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(uid: &<b>mut</b> UID, v: u64) {
    <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(uid, v)
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_object_version"></a>

## Function `assert_object_version`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(uid: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(uid: &UID) {
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(uid) == <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_cap_for_this_package"></a>

## Function `assert_cap_for_this_package`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_cap_for_this_package">assert_cap_for_this_package</a>(cap: &<a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_cap_for_this_package">assert_cap_for_this_package</a>(cap: &UpgradeCap) {
    <b>let</b> cap_pkg = package::upgrade_package(cap);
    <b>assert</b>!(object::id_to_address(&cap_pkg) == @social_contracts, <a href="../social_contracts/memory.md#social_contracts_memory_ENotUpgradeAuthority">ENotUpgradeAuthority</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_has_suffix"></a>

## Function `has_suffix`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_suffix">has_suffix</a>(data: &vector&lt;u8&gt;, suffix: &vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_suffix">has_suffix</a>(data: &vector&lt;u8&gt;, suffix: &vector&lt;u8&gt;): bool {
    <b>let</b> data_len = vector::length(data);
    <b>let</b> suffix_len = vector::length(suffix);
    <b>if</b> (suffix_len &gt; data_len) <b>return</b> <b>false</b>;
    <b>let</b> offset = data_len - suffix_len;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; suffix_len) {
        <b>if</b> (*vector::borrow(data, offset + i) != *vector::borrow(suffix, i)) <b>return</b> <b>false</b>;
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>
