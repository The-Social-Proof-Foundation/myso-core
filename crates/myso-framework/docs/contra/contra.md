---
title: Module `contra::contra`
---

Confidential transfers on MySo.

Enables token transfers where amounts are encrypted using twisted ElGamal
encryption while remaining verifiable through zero-knowledge proofs.


<a name="@Key_Flows_for_the_Token_Issuer_of_public_token_type_<code>T</code>:_0"></a>

### Key Flows for the Token Issuer of public token type <code>T</code>:

1. Create a new confidential token for a token type <code>T</code> (using the TreasuryCap), optionally
with an initial set of auditor public keys. Creation returns a <code><a href="../contra/contra.md#contra_contra_ManagementCap">ManagementCap</a>&lt;T&gt;</code>.
2. Set the freeze admins who can freeze the token globally or specific accounts (via the
ManagementCap). Those admins may monitor the confidential token and freeze it or
individual accounts if necessary.
3. Unfreeze the token globally or a specific account (using the TreasuryCap).
4. Set the balance of an account directly, to emulate burn/seize (using the TreasuryCap).
5. Freeze specific accounts via the token's deny list, using
<code><a href="../myso/coin.md#myso_coin_deny_list_v2_add">myso::coin::deny_list_v2_add</a></code> / <code><a href="../myso/coin.md#myso_coin_deny_list_v2_remove">myso::coin::deny_list_v2_remove</a></code>. The deny list affects
both the public and the private coin; to freeze only the private coin, see items 2 and 3.
6. Rotate or disable the auditor key set via <code><a href="../contra/contra.md#contra_contra_update_auditors">update_auditors</a></code> (using the ManagementCap).
Setting <code>bump_recommended_min</code> raises the auditors' <code>recommended_min_version</code> to the new
version, signalling to wallets that every user should refresh their key.
Passing an empty <code>public_keys</code> vector disables the auditor flow.
7. [Advanced] Set the policy for the confidential token (using the TreasuryCap). Policies define
which operations are permissioned. Currently supported permissioned operations are:
- <code><a href="../contra/contra.md#contra_contra_register">register</a></code>: Register a token account for a token type <code>T</code>. E.g., caller ensures the user is
KYCed before registering an account. When set, also setting the public key for an account
is permissioned.
- <code><a href="../contra/contra.md#contra_contra_wrap">wrap</a></code>: Wrap a public coin into a private balance. E.g., caller ensures the funds passed
screening before wrapping.
- <code><a href="../contra/contra.md#contra_contra_unwrap">unwrap</a></code>: Unwrap a private balance into a public coin. E.g., caller enforces rate limit on
exiting the system.
Additional permissioned operations may be added in the future.
Permissioned operations are customized flows that should be implemented by the issuer's
contract, and may not be supported by all clients/wallets.
The default policy is fully permissionless.


<a name="@Key_Flows_for_Users:_1"></a>

### Key Flows for Users:

1. Create an account for an address (needed once for all token types).
2. Register a token account for a token type <code>T</code> with a public key <code>pk</code>. If the token has
auditors configured, the user must additionally provide the user's key encrypted to every
auditor public key in the current set, and a proof that the ciphertext encrypts its
secret key.
3. Update the public key for a token account.
4. Wrap a public coin into a confidential token, adding to the pending encrypted balance of an
account.
5. Transfer an encrypted amount to two or more token accounts.
6. Unwrap an encrypted amount from a token account and convert it to public coins.


<a name="@Authentication:_2"></a>

### Authentication:

Some functions require authorization via an <code>&Auth&lt;T&gt;</code> argument. Under the default
permissionless policy any <code>Auth&lt;T&gt;</code> is accepted; permissioning narrows which constructors
produce a valid <code>Auth&lt;T&gt;</code>. The caller constructs the <code>Auth&lt;T&gt;</code> via one of three constructors:
- <code><a href="../contra/contra.md#contra_contra_authorize_as_sender">authorize_as_sender</a></code>: authenticates <code>ctx.sender()</code>. The standard path for end-user wallets
and permissionless operations.
- <code><a href="../contra/contra.md#contra_contra_authorize_as_object">authorize_as_object</a></code>: authenticates the address derived from a given object's <code>UID</code>. Use this
when access is controlled by a Move object (the holder of <code>&<b>mut</b> UID</code> proves ownership).
- <code><a href="../contra/contra.md#contra_contra_authorize_with_witness">authorize_with_witness</a></code>: authenticates <code><a href="../contra/contra.md#contra_contra_owner">owner</a></code> under a witness <code>W</code> required by the policy. Use
this to implement custom permissioned operations: the issuer's contract holds <code>W</code>, performs
its own checks (e.g. KYC, screening, rate limiting), and creates an <code>Auth&lt;T&gt;</code> for the
requested operation.


    -  [Key Flows for the Token Issuer of public token type <code>T</code>:](#@Key_Flows_for_the_Token_Issuer_of_public_token_type_<code>T</code>:_0)
    -  [Key Flows for Users:](#@Key_Flows_for_Users:_1)
    -  [Authentication:](#@Authentication:_2)
-  [Struct `TokenRegistry`](#contra_contra_TokenRegistry)
-  [Struct `AccountRegistry`](#contra_contra_AccountRegistry)
-  [Struct `ConfidentialToken`](#contra_contra_ConfidentialToken)
-  [Struct `Pool`](#contra_contra_Pool)
-  [Struct `Account`](#contra_contra_Account)
-  [Struct `TokenAccount`](#contra_contra_TokenAccount)
-  [Struct `TokenKey`](#contra_contra_TokenKey)
-  [Struct `PoolKey`](#contra_contra_PoolKey)
-  [Struct `TokenAccountKey`](#contra_contra_TokenAccountKey)
-  [Struct `AccountKey`](#contra_contra_AccountKey)
-  [Struct `ManagementCap`](#contra_contra_ManagementCap)
-  [Enum `TransferBatch`](#contra_contra_TransferBatch)
-  [Constants](#@Constants_3)
-  [Function `init`](#contra_contra_init)
-  [Function `authorize_as_sender`](#contra_contra_authorize_as_sender)
-  [Function `authorize_with_witness`](#contra_contra_authorize_with_witness)
-  [Function `authorize_as_object`](#contra_contra_authorize_as_object)
-  [Function `new_confidential_token`](#contra_contra_new_confidential_token)
-  [Function `share_confidential_token`](#contra_contra_share_confidential_token)
-  [Function `new_account`](#contra_contra_new_account)
-  [Function `share_account`](#contra_contra_share_account)
-  [Function `register`](#contra_contra_register)
-  [Function `register_internal`](#contra_contra_register_internal)
-  [Function `set_accepts_encrypted_deposits`](#contra_contra_set_accepts_encrypted_deposits)
-  [Function `set_public_key`](#contra_contra_set_public_key)
-  [Function `try_set_public_key_and_unpause`](#contra_contra_try_set_public_key_and_unpause)
-  [Function `set_public_key_internal`](#contra_contra_set_public_key_internal)
-  [Function `wrap`](#contra_contra_wrap)
-  [Function `batched_transfer`](#contra_contra_batched_transfer)
-  [Function `add_to_batch`](#contra_contra_add_to_batch)
-  [Function `try_finalize`](#contra_contra_try_finalize)
-  [Function `finalize`](#contra_contra_finalize)
-  [Function `merge`](#contra_contra_merge)
-  [Function `update_active_balance`](#contra_contra_update_active_balance)
-  [Function `try_update_active`](#contra_contra_try_update_active)
-  [Function `unwrap`](#contra_contra_unwrap)
-  [Function `try_unwrap`](#contra_contra_try_unwrap)
-  [Function `try_unwrap_internal`](#contra_contra_try_unwrap_internal)
-  [Function `owner`](#contra_contra_owner)
-  [Function `set_balance_by_issuer`](#contra_contra_set_balance_by_issuer)
-  [Function `issue_freeze_cap`](#contra_contra_issue_freeze_cap)
-  [Function `revoke_freeze_cap`](#contra_contra_revoke_freeze_cap)
-  [Function `global_freeze`](#contra_contra_global_freeze)
-  [Function `global_unfreeze`](#contra_contra_global_unfreeze)
-  [Function `account_freeze`](#contra_contra_account_freeze)
-  [Function `account_unfreeze`](#contra_contra_account_unfreeze)
-  [Function `set_policy`](#contra_contra_set_policy)
-  [Function `update_auditors`](#contra_contra_update_auditors)
-  [Function `has_token`](#contra_contra_has_token)
-  [Function `has_deposit_slot`](#contra_contra_has_deposit_slot)
-  [Function `session_id`](#contra_contra_session_id)
-  [Function `dst`](#contra_contra_dst)
-  [Function `borrow`](#contra_contra_borrow)
-  [Function `borrow_mut`](#contra_contra_borrow_mut)


<pre><code><b>use</b> <a href="../contra/auditors.md#contra_auditors">contra::auditors</a>;
<b>use</b> <a href="../contra/balance.md#contra_balance">contra::balance</a>;
<b>use</b> <a href="../contra/deny_list.md#contra_deny_list">contra::deny_list</a>;
<b>use</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount">contra::encrypted_amount</a>;
<b>use</b> <a href="../contra/events.md#contra_events">contra::events</a>;
<b>use</b> <a href="../contra/nizk.md#contra_nizk">contra::nizk</a>;
<b>use</b> <a href="../contra/policy.md#contra_policy">contra::policy</a>;
<b>use</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal">contra::twisted_elgamal</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/rangeproofs.md#myso_rangeproofs">myso::rangeproofs</a>;
<b>use</b> <a href="../myso/ristretto255.md#myso_ristretto255">myso::ristretto255</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="contra_contra_TokenRegistry"></a>

## Struct `TokenRegistry`

Registry of tokens for confidential transactions. Each <code><a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a></code>'s
UID is derived from this registry.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_TokenRegistry">TokenRegistry</a> <b>has</b> key
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

<a name="contra_contra_AccountRegistry"></a>

## Struct `AccountRegistry`

Registry of accounts for confidential transactions. Each <code><a href="../contra/contra.md#contra_contra_Account">Account</a></code>'s UID is
derived from this registry.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_AccountRegistry">AccountRegistry</a> <b>has</b> key
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

<a name="contra_contra_ConfidentialToken"></a>

## Struct `ConfidentialToken`

The representation of a confidential token. Each <code><a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a></code> corresponds to a public
token type <code>T</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;<b>phantom</b> T&gt; <b>has</b> key
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
<code>is_active: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>freeze_admins: <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../contra/policy.md#contra_policy">policy</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/policy.md#contra_policy_Policy">contra::policy::Policy</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../contra/auditors.md#contra_auditors">auditors</a>: <a href="../contra/auditors.md#contra_auditors_Auditors">contra::auditors::Auditors</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_contra_Pool"></a>

## Struct `Pool`

The representation of the pool of tokens of type <code>T</code> in circulation as confidential tokens.
Stored as a derived object of <code><a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;</code> to reduce contention on non-unwrap
operations.
Tokens are held at this object's address via MySo address balance to reduce contention on wrap
operations.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_Pool">Pool</a>&lt;<b>phantom</b> T&gt; <b>has</b> key
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

<a name="contra_contra_Account"></a>

## Struct `Account`

Base account that stores token accounts as dynamic fields.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_Account">Account</a> <b>has</b> key
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
<code><a href="../contra/contra.md#contra_contra_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_contra_TokenAccount"></a>

## Struct `TokenAccount`

A user's account for one confidential token.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_TokenAccount">TokenAccount</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>verified_key_encryption: <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../contra/contra.md#contra_contra_session_id">session_id</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>is_frozen: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>accepts_deposits: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>active: <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>pending: <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>public_balance: <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_contra_TokenKey"></a>

## Struct `TokenKey`

Key used for <code><a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a></code> UID derivation.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_TokenKey">TokenKey</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="contra_contra_PoolKey"></a>

## Struct `PoolKey`

Key used for <code><a href="../contra/contra.md#contra_contra_Pool">Pool</a></code> UID derivation from <code><a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a></code>.
There is only one pool per token, so no parameter is needed.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_PoolKey">PoolKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="contra_contra_TokenAccountKey"></a>

## Struct `TokenAccountKey`

Dynamic field key used for storing <code><a href="../contra/contra.md#contra_contra_TokenAccount">TokenAccount</a></code>s in <code><a href="../contra/contra.md#contra_contra_Account">Account</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="contra_contra_AccountKey"></a>

## Struct `AccountKey`

Key used for <code><a href="../contra/contra.md#contra_contra_Account">Account</a></code> UID derivation.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_AccountKey">AccountKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>0: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_contra_ManagementCap"></a>

## Struct `ManagementCap`

Capability granting management of the freeze admins and auditor keys.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/contra.md#contra_contra_ManagementCap">ManagementCap</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
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

<a name="contra_contra_TransferBatch"></a>

## Enum `TransferBatch`

State machine for batched transfers from a single sender to multiple receivers.
Created by <code><a href="../contra/contra.md#contra_contra_batched_transfer">batched_transfer</a></code>, consumed by calling <code>add</code> for each receiver then <code><a href="../contra/contra.md#contra_contra_finalize">finalize</a></code>.


<pre><code><b>public</b> <b>enum</b> <a href="../contra/contra.md#contra_contra_TransferBatch">TransferBatch</a>&lt;<b>phantom</b> T&gt;
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>BalanceProofFailed</code>
</dt>
<dd>
 The sender's balance proof failed. Subsequent <code>add</code> calls are no-ops; <code><a href="../contra/contra.md#contra_contra_try_finalize">try_finalize</a></code>
 returns <code><b>false</b></code> and <code><a href="../contra/contra.md#contra_contra_finalize">finalize</a></code> aborts.
</dd>
<dt>
Variant <code>Ok</code>
</dt>
<dd>
 The balance proof succeeded. Holds the receiver-keyed <code>EncryptedCoin</code>s split off the
 sender's balance, one per transfer. <code><a href="../contra/contra.md#contra_contra_add_to_batch">add_to_batch</a></code> pops one per receiver and credits it to
 their pending deposits. <code>sender_amounts</code> is the parallel vector of sender-keyed encryptions
 of the same *total* (individual values aren't constrained — see the <code>TransferEvent</code> doc),
 carried only so each <code><a href="../contra/contra.md#contra_contra_add_to_batch">add_to_batch</a></code> can emit one in the <code>TransferEvent</code>. <code>sender_pk</code> is
 likewise carried only for the event.
</dd>

<dl>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>sender_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>coins: vector&lt;<a href="../contra/balance.md#contra_balance_EncryptedCoin">contra::balance::EncryptedCoin</a>&lt;T&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>sender_amounts: vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>

</dl>


</details>

<a name="@Constants_3"></a>

## Constants


<a name="contra_contra_EAccountAlreadyRegistered"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_EAccountAlreadyRegistered">EAccountAlreadyRegistered</a>: u64 = 0;
</code></pre>



<a name="contra_contra_ETransferDenied"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>: u64 = 1;
</code></pre>



<a name="contra_contra_EAuthorizationError"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>: u64 = 2;
</code></pre>



<a name="contra_contra_ETokenAlreadyRegistered"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_ETokenAlreadyRegistered">ETokenAlreadyRegistered</a>: u64 = 3;
</code></pre>



<a name="contra_contra_EPendingDepositsMustBeMerged"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_EPendingDepositsMustBeMerged">EPendingDepositsMustBeMerged</a>: u64 = 4;
</code></pre>



<a name="contra_contra_EBalanceProofFailed"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_EBalanceProofFailed">EBalanceProofFailed</a>: u64 = 5;
</code></pre>



<a name="contra_contra_EAllAmountsMustBeUsed"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_EAllAmountsMustBeUsed">EAllAmountsMustBeUsed</a>: u64 = 6;
</code></pre>



<a name="contra_contra_EAmountsEqualityProofFailed"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_EAmountsEqualityProofFailed">EAmountsEqualityProofFailed</a>: u64 = 7;
</code></pre>



<a name="contra_contra_EEmptyTransferBatch"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_EEmptyTransferBatch">EEmptyTransferBatch</a>: u64 = 8;
</code></pre>



<a name="contra_contra_ETooManyReceivers"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_ETooManyReceivers">ETooManyReceivers</a>: u64 = 9;
</code></pre>



<a name="contra_contra_EBalancesFull"></a>

Recovery: transfer or update active balance.


<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_EBalancesFull">EBalancesFull</a>: u64 = 10;
</code></pre>



<a name="contra_contra_EIdentityPublicKey"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_EIdentityPublicKey">EIdentityPublicKey</a>: u64 = 11;
</code></pre>



<a name="contra_contra_PERMISSIONED_REGISTER"></a>

(Potentially) permissioned operations.


<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_PERMISSIONED_REGISTER">PERMISSIONED_REGISTER</a>: u8 = 0;
</code></pre>



<a name="contra_contra_PERMISSIONED_WRAP"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_PERMISSIONED_WRAP">PERMISSIONED_WRAP</a>: u8 = 1;
</code></pre>



<a name="contra_contra_PERMISSIONED_UNWRAP"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_PERMISSIONED_UNWRAP">PERMISSIONED_UNWRAP</a>: u8 = 2;
</code></pre>



<a name="contra_contra_DST_DDH"></a>

Protocol IDs for Fiat-Shamir domain separation.
Protocol-id <code>100</code> is also reserved by the ts-sdk for <code>PROTOCOL_VERIFIED_DEC</code>


<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_DST_DDH">DST_DDH</a>: u8 = 1;
</code></pre>



<a name="contra_contra_DST_ELGAMAL"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_DST_ELGAMAL">DST_ELGAMAL</a>: u8 = 2;
</code></pre>



<a name="contra_contra_DST_KEY_CONSISTENCY"></a>



<pre><code><b>const</b> <a href="../contra/contra.md#contra_contra_DST_KEY_CONSISTENCY">DST_KEY_CONSISTENCY</a>: u8 = 3;
</code></pre>



<a name="contra_contra_init"></a>

## Function `init`

On initialization, we create and share the <code><a href="../contra/contra.md#contra_contra_AccountRegistry">AccountRegistry</a></code> and <code><a href="../contra/contra.md#contra_contra_TokenRegistry">TokenRegistry</a></code> objects.


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_init">init</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_init">init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> account_registry = <a href="../contra/contra.md#contra_contra_AccountRegistry">AccountRegistry</a> { id: object::new(ctx) };
    <b>let</b> token_registry = <a href="../contra/contra.md#contra_contra_TokenRegistry">TokenRegistry</a> { id: object::new(ctx) };
    transfer::share_object(account_registry);
    transfer::share_object(token_registry);
}
</code></pre>



</details>

<a name="contra_contra_authorize_as_sender"></a>

## Function `authorize_as_sender`

Create an <code>Auth&lt;T&gt;</code> for <code>ctx.sender()</code> covering every operation the policy on <code>ct</code> leaves
permissionless.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_authorize_as_sender">authorize_as_sender</a>&lt;T&gt;(ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_authorize_as_sender">authorize_as_sender</a>&lt;T&gt;(ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;, ctx: &TxContext): Auth&lt;T&gt; {
    <a href="../contra/policy.md#contra_policy_as_sender">policy::as_sender</a>&lt;T&gt;(&ct.<a href="../contra/policy.md#contra_policy">policy</a>, ctx)
}
</code></pre>



</details>

<a name="contra_contra_authorize_with_witness"></a>

## Function `authorize_with_witness`

Create an <code>Auth&lt;T&gt;</code> on behalf of <code><a href="../contra/contra.md#contra_contra_owner">owner</a></code> covering the requested <code>operation</code>, authorized by
witness <code>W</code>. Aborts unless the policy on <code>ct</code> is set, its witness type is <code>W</code>, and <code>operation</code>
is permissioned. The witness-holding contract is fully responsible for authenticating <code><a href="../contra/contra.md#contra_contra_owner">owner</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_authorize_with_witness">authorize_with_witness</a>&lt;T, W: drop&gt;(ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, operation: u8, <a href="../contra/contra.md#contra_contra_owner">owner</a>: <b>address</b>, witness: W): <a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_authorize_with_witness">authorize_with_witness</a>&lt;T, W: drop&gt;(
    ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    operation: u8,
    <a href="../contra/contra.md#contra_contra_owner">owner</a>: <b>address</b>,
    witness: W,
): Auth&lt;T&gt; {
    <a href="../contra/policy.md#contra_policy_with_witness">policy::with_witness</a>&lt;T, W&gt;(&ct.<a href="../contra/policy.md#contra_policy">policy</a>, operation, <a href="../contra/contra.md#contra_contra_owner">owner</a>, witness)
}
</code></pre>



</details>

<a name="contra_contra_authorize_as_object"></a>

## Function `authorize_as_object`

Create an <code>Auth&lt;T&gt;</code> on behalf of an object identified by <code>uid</code>, covering every operation the
policy on <code>ct</code> leaves permissionless. Holding <code>&<b>mut</b> UID</code> proves custody of the object, so the
object self-authenticates as its own <code><a href="../contra/contra.md#contra_contra_owner">owner</a></code> (the address derived from the UID).


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_authorize_as_object">authorize_as_object</a>&lt;T&gt;(ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): <a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_authorize_as_object">authorize_as_object</a>&lt;T&gt;(ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;, uid: &<b>mut</b> UID): Auth&lt;T&gt; {
    <a href="../contra/policy.md#contra_policy_as_object">policy::as_object</a>&lt;T&gt;(&ct.<a href="../contra/policy.md#contra_policy">policy</a>, uid)
}
</code></pre>



</details>

<a name="contra_contra_new_confidential_token"></a>

## Function `new_confidential_token`

Create a new confidential token for the given token type. Can only happen
once per token type, and the token object is immediately shared.

Requires a <code>&<b>mut</b> TreasuryCap</code> for authorization, this is to prevent frozen
TreasuryCaps from being used.

Creates an <code>Auditors</code> object for the confidential token using the provided public keys.
The auditor public keys can be empty initially and updated later by the issuer.

Returns the created <code><a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a></code> and a <code><a href="../contra/contra.md#contra_contra_ManagementCap">ManagementCap</a></code> that can be used to perform
administrative operations for this token.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_new_confidential_token">new_confidential_token</a>&lt;T&gt;(registry: &<b>mut</b> <a href="../contra/contra.md#contra_contra_TokenRegistry">contra::contra::TokenRegistry</a>, _t: &<b>mut</b> <a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;T&gt;, auditor_public_keys: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, <a href="../contra/contra.md#contra_contra_ManagementCap">contra::contra::ManagementCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_new_confidential_token">new_confidential_token</a>&lt;T&gt;(
    registry: &<b>mut</b> <a href="../contra/contra.md#contra_contra_TokenRegistry">TokenRegistry</a>,
    _t: &<b>mut</b> TreasuryCap&lt;T&gt;,
    auditor_public_keys: vector&lt;Element&lt;G&gt;&gt;,
    ctx: &<b>mut</b> TxContext,
): (<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;, <a href="../contra/contra.md#contra_contra_ManagementCap">ManagementCap</a>&lt;T&gt;) {
    <b>assert</b>!(!derived_object::exists(&registry.id, <a href="../contra/contra.md#contra_contra_TokenKey">TokenKey</a>&lt;T&gt;()), <a href="../contra/contra.md#contra_contra_ETokenAlreadyRegistered">ETokenAlreadyRegistered</a>);
    <b>let</b> <b>mut</b> id = derived_object::claim(&<b>mut</b> registry.id, <a href="../contra/contra.md#contra_contra_TokenKey">TokenKey</a>&lt;T&gt;());
    <b>let</b> pool_id = derived_object::claim(&<b>mut</b> id, <a href="../contra/contra.md#contra_contra_PoolKey">PoolKey</a>());
    transfer::share_object(<a href="../contra/contra.md#contra_contra_Pool">Pool</a>&lt;T&gt; { id: pool_id });
    <a href="../contra/events.md#contra_events_emit_new_confidential_token">events::emit_new_confidential_token</a>&lt;T&gt;();
    (
        <a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt; {
            id,
            is_active: <b>true</b>,
            freeze_admins: vec_set::empty(),
            <a href="../contra/policy.md#contra_policy">policy</a>: <a href="../contra/policy.md#contra_policy_permissionless">policy::permissionless</a>(),
            <a href="../contra/auditors.md#contra_auditors">auditors</a>: new_auditors(auditor_public_keys),
        },
        <a href="../contra/contra.md#contra_contra_ManagementCap">ManagementCap</a> { id: object::new(ctx) },
    )
}
</code></pre>



</details>

<a name="contra_contra_share_confidential_token"></a>

## Function `share_confidential_token`

Share the confidential token object.
This is needed to allow the issuer to interact with the confidential token, e.g.,
to set permissions, in the same PTB.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_share_confidential_token">share_confidential_token</a>&lt;T&gt;(ct: <a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_share_confidential_token">share_confidential_token</a>&lt;T&gt;(ct: <a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;) {
    transfer::share_object(ct);
}
</code></pre>



</details>

<a name="contra_contra_new_account"></a>

## Function `new_account`

Create a new account for the given address. Can only happen once per address.

Note: the <code><a href="../contra/contra.md#contra_contra_owner">owner</a></code> argument is not tied to <code>ctx.sender()</code> — anyone can create an
<code><a href="../contra/contra.md#contra_contra_Account">Account</a></code> on behalf of any address. Since <code><a href="../contra/contra.md#contra_contra_Account">Account</a></code> has <code>key</code> only (no <code>store</code>),
the only way to dispose of it outside this module is via <code><a href="../contra/contra.md#contra_contra_share_account">share_account</a></code>, and
all authenticated operations still gate on <code>account.<a href="../contra/contra.md#contra_contra_owner">owner</a> == ctx.sender()</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_new_account">new_account</a>(registry: &<b>mut</b> <a href="../contra/contra.md#contra_contra_AccountRegistry">contra::contra::AccountRegistry</a>, <a href="../contra/contra.md#contra_contra_owner">owner</a>: <b>address</b>): <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_new_account">new_account</a>(registry: &<b>mut</b> <a href="../contra/contra.md#contra_contra_AccountRegistry">AccountRegistry</a>, <a href="../contra/contra.md#contra_contra_owner">owner</a>: <b>address</b>): <a href="../contra/contra.md#contra_contra_Account">Account</a> {
    <b>assert</b>!(!derived_object::exists(&registry.id, <a href="../contra/contra.md#contra_contra_AccountKey">AccountKey</a>(<a href="../contra/contra.md#contra_contra_owner">owner</a>)), <a href="../contra/contra.md#contra_contra_EAccountAlreadyRegistered">EAccountAlreadyRegistered</a>);
    <b>let</b> id = derived_object::claim(&<b>mut</b> registry.id, <a href="../contra/contra.md#contra_contra_AccountKey">AccountKey</a>(<a href="../contra/contra.md#contra_contra_owner">owner</a>));
    <a href="../contra/contra.md#contra_contra_Account">Account</a> { id, <a href="../contra/contra.md#contra_contra_owner">owner</a> }
}
</code></pre>



</details>

<a name="contra_contra_share_account"></a>

## Function `share_account`

Share the account object.
This has do be done after <code><a href="../contra/contra.md#contra_contra_new_account">new_account</a></code>, but it allows the user to create token
accounts for confidential tokens immediately.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_share_account">share_account</a>(account: <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_share_account">share_account</a>(account: <a href="../contra/contra.md#contra_contra_Account">Account</a>) {
    transfer::share_object(account);
}
</code></pre>



</details>

<a name="contra_contra_register"></a>

## Function `register`

Create a <code><a href="../contra/contra.md#contra_contra_TokenAccount">TokenAccount</a></code> for token <code>T</code> with the given <code>pk</code>. Authorized by <code>auth</code>, which must
be for the <code><a href="../contra/contra.md#contra_contra_PERMISSIONED_REGISTER">PERMISSIONED_REGISTER</a></code> operation and for <code>account.<a href="../contra/contra.md#contra_contra_owner">owner</a></code>.
If <code><a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;</code> has auditors enabled, a <code>KeyEncryption</code> must be provided.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_register">register</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, key_encryption: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/auditors.md#contra_auditors_KeyEncryption">contra::auditors::KeyEncryption</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_register">register</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    pk: Element&lt;G&gt;,
    key_encryption: Option&lt;KeyEncryption&gt;,
) {
    <b>let</b> <a href="../contra/contra.md#contra_contra_session_id">session_id</a> = account.<a href="../contra/contra.md#contra_contra_session_id">session_id</a>&lt;T&gt;();
    <b>let</b> verified_key_encription = ct
        .<a href="../contra/auditors.md#contra_auditors">auditors</a>
        .verify_key_encryption(
            &pk,
            key_encryption,
            <a href="../contra/contra.md#contra_contra_session_id">session_id</a>.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_KEY_CONSISTENCY">DST_KEY_CONSISTENCY</a>),
        );
    <a href="../contra/contra.md#contra_contra_register_internal">register_internal</a>(
        account,
        auth,
        pk,
        verified_key_encription,
        <a href="../contra/contra.md#contra_contra_session_id">session_id</a>,
    );
}
</code></pre>



</details>

<a name="contra_contra_register_internal"></a>

## Function `register_internal`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/contra.md#contra_contra_register_internal">register_internal</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, verified_key_encryption: <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a>, <a href="../contra/contra.md#contra_contra_session_id">session_id</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/contra.md#contra_contra_register_internal">register_internal</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    pk: Element&lt;G&gt;,
    verified_key_encryption: VerifiedKeyEncryption,
    <a href="../contra/contra.md#contra_contra_session_id">session_id</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(auth.is_allowed(<a href="../contra/contra.md#contra_contra_PERMISSIONED_REGISTER">PERMISSIONED_REGISTER</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>assert</b>!(auth.is_authenticated(account.<a href="../contra/contra.md#contra_contra_owner">owner</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>assert</b>!(!account.<a href="../contra/contra.md#contra_contra_has_token">has_token</a>&lt;T&gt;(), <a href="../contra/contra.md#contra_contra_EAccountAlreadyRegistered">EAccountAlreadyRegistered</a>);
    // TODO: Can we skip the next check? what should we check instead in the zk proofs?
    <b>assert</b>!(pk != g_identity(), <a href="../contra/contra.md#contra_contra_EIdentityPublicKey">EIdentityPublicKey</a>);
    <a href="../contra/events.md#contra_events_emit_new_registration">events::emit_new_registration</a>&lt;T&gt;(account.<a href="../contra/contra.md#contra_contra_owner">owner</a>, pk, verified_key_encryption);
    df::add(
        &<b>mut</b> account.id,
        <a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;(),
        <a href="../contra/contra.md#contra_contra_TokenAccount">TokenAccount</a>&lt;T&gt; {
            pk,
            verified_key_encryption,
            <a href="../contra/contra.md#contra_contra_session_id">session_id</a>,
            is_frozen: <b>false</b>,
            accepts_deposits: <b>true</b>,
            active: <a href="../contra/balance.md#contra_balance_new">balance::new</a>&lt;T&gt;(),
            pending: <a href="../contra/balance.md#contra_balance_empty">balance::empty</a>&lt;T&gt;(),
            public_balance: <a href="../contra/balance.md#contra_balance_zero">balance::zero</a>&lt;T&gt;(),
        },
    );
}
</code></pre>



</details>

<a name="contra_contra_set_accepts_encrypted_deposits"></a>

## Function `set_accepts_encrypted_deposits`

Set whether this account for token <code>T</code> accepts new encrypted deposits.
This is used to prevent receiving new encrypted deposits during token account key rotation.
Authorized by <code>auth</code>, which must be for <code>account.<a href="../contra/contra.md#contra_contra_owner">owner</a></code>. Any <code>Auth&lt;T&gt;</code> is accepted regardless
of which operation it covers.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_set_accepts_encrypted_deposits">set_accepts_encrypted_deposits</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, accepts_encrypted_deposits: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_set_accepts_encrypted_deposits">set_accepts_encrypted_deposits</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    accepts_encrypted_deposits: bool,
) {
    // TODO: consider checking <a href="../contra/contra.md#contra_contra_PERMISSIONED_REGISTER">PERMISSIONED_REGISTER</a>
    <b>assert</b>!(auth.is_authenticated(account.<a href="../contra/contra.md#contra_contra_owner">owner</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()].accepts_deposits = accepts_encrypted_deposits;
}
</code></pre>



</details>

<a name="contra_contra_set_public_key"></a>

## Function `set_public_key`

Update the public key for the account of token <code>T</code>. Authorized by <code>auth</code>, which must be for
the <code><a href="../contra/contra.md#contra_contra_PERMISSIONED_REGISTER">PERMISSIONED_REGISTER</a></code> operation and for <code>account.<a href="../contra/contra.md#contra_contra_owner">owner</a></code> -- key rotation reuses the registration
authorization since the same flow gates account onboarding.
This aborts if there are pending deposits that need to be merged, so the caller should:
- Call <code><a href="../contra/contra.md#contra_contra_merge">merge</a></code> to merge pending deposits and <code><a href="../contra/contra.md#contra_contra_set_accepts_encrypted_deposits">set_accepts_encrypted_deposits</a></code> to false to
prevent new encrypted deposits.
- Call <code><a href="../contra/contra.md#contra_contra_set_public_key">set_public_key</a></code> to update the public key and <code><a href="../contra/contra.md#contra_contra_set_accepts_encrypted_deposits">set_accepts_encrypted_deposits</a></code> to true
to allow new encrypted deposits again.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_set_public_key">set_public_key</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, new_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, new_balance_proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, handle_eq_proof: <a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, key_encryption: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/auditors.md#contra_auditors_KeyEncryption">contra::auditors::KeyEncryption</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_set_public_key">set_public_key</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    new_pk: Element&lt;G&gt;,
    new_balance: EncryptedAmount,
    new_balance_proof: WellFormedProof,
    handle_eq_proof: DdhProof,
    key_encryption: Option&lt;KeyEncryption&gt;,
) {
    <b>let</b> sid = account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()].<a href="../contra/contra.md#contra_contra_session_id">session_id</a>;
    <a href="../contra/contra.md#contra_contra_set_public_key_internal">set_public_key_internal</a>(
        account,
        auth,
        new_pk,
        new_balance,
        new_balance_proof,
        handle_eq_proof,
        ct.<a href="../contra/auditors.md#contra_auditors">auditors</a>.verify_key_encryption(&new_pk, key_encryption, sid.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_KEY_CONSISTENCY">DST_KEY_CONSISTENCY</a>)),
        sid.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_DDH">DST_DDH</a>),
    );
}
</code></pre>



</details>

<a name="contra_contra_try_set_public_key_and_unpause"></a>

## Function `try_set_public_key_and_unpause`

Optimistic key rotation: re-state the balance under a fresh blinding, re-key it to <code>new_pk</code>, and
unpause. If the restate's <code>balance_proof</code> fails (e.g. a deposit raced the caller's read), emits
<code>TrySetPublicKeyFailedEvent</code> and leaves the account paused for a retry. The caller must <code><a href="../contra/contra.md#contra_contra_merge">merge</a></code>
(and pause) first.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_try_set_public_key_and_unpause">try_set_public_key_and_unpause</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, new_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, restated_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, restated_balance_proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, balance_proof: <a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, new_balance_proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, handle_eq_proof: <a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, key_encryption: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/auditors.md#contra_auditors_KeyEncryption">contra::auditors::KeyEncryption</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_try_set_public_key_and_unpause">try_set_public_key_and_unpause</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    new_pk: Element&lt;G&gt;,
    restated_balance: EncryptedAmount,
    restated_balance_proof: WellFormedProof,
    balance_proof: DdhProof,
    new_balance: EncryptedAmount,
    new_balance_proof: WellFormedProof,
    handle_eq_proof: DdhProof,
    key_encryption: Option&lt;KeyEncryption&gt;,
) {
    <b>assert</b>!(auth.is_allowed(<a href="../contra/contra.md#contra_contra_PERMISSIONED_REGISTER">PERMISSIONED_REGISTER</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>assert</b>!(auth.is_authenticated(account.<a href="../contra/contra.md#contra_contra_owner">owner</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    // Optimistic restate under the old key — the only step that can fail on a race. On failure we
    // bail before touching the key, leaving the account paused and merged <b>for</b> a retry.
    <b>let</b> token_account = &<b>mut</b> account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()];
    <b>let</b> sid = token_account.<a href="../contra/contra.md#contra_contra_session_id">session_id</a>;
    <b>let</b> update_successful = token_account.<a href="../contra/contra.md#contra_contra_try_update_active">try_update_active</a>(
        restated_balance,
        restated_balance_proof,
        &balance_proof,
        sid,
    );
    <b>if</b> (!update_successful) {
        <a href="../contra/events.md#contra_events_emit_try_set_public_key_failed">events::emit_try_set_public_key_failed</a>();
        <b>return</b>
    };
    <a href="../contra/contra.md#contra_contra_set_public_key_internal">set_public_key_internal</a>(
        account,
        auth,
        new_pk,
        new_balance,
        new_balance_proof,
        handle_eq_proof,
        ct.<a href="../contra/auditors.md#contra_auditors">auditors</a>.verify_key_encryption(&new_pk, key_encryption, sid.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_KEY_CONSISTENCY">DST_KEY_CONSISTENCY</a>)),
        sid.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_DDH">DST_DDH</a>),
    );
    account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()].accepts_deposits = <b>true</b>;
}
</code></pre>



</details>

<a name="contra_contra_set_public_key_internal"></a>

## Function `set_public_key_internal`

Re-key the active balance to <code>new_pk</code>, aborting if any proof or precondition fails.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/contra.md#contra_contra_set_public_key_internal">set_public_key_internal</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, new_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, new_balance_proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, handle_eq_proof: <a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, new_verified_key_encryption: <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a>, <a href="../contra/contra.md#contra_contra_dst">dst</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/contra.md#contra_contra_set_public_key_internal">set_public_key_internal</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    new_pk: Element&lt;G&gt;,
    new_balance: EncryptedAmount,
    new_balance_proof: WellFormedProof,
    handle_eq_proof: DdhProof,
    new_verified_key_encryption: VerifiedKeyEncryption,
    <a href="../contra/contra.md#contra_contra_dst">dst</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(auth.is_allowed(<a href="../contra/contra.md#contra_contra_PERMISSIONED_REGISTER">PERMISSIONED_REGISTER</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>assert</b>!(auth.is_authenticated(account.<a href="../contra/contra.md#contra_contra_owner">owner</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>assert</b>!(new_pk != g_identity(), <a href="../contra/contra.md#contra_contra_EIdentityPublicKey">EIdentityPublicKey</a>);
    <b>let</b> <a href="../contra/contra.md#contra_contra_owner">owner</a> = account.<a href="../contra/contra.md#contra_contra_owner">owner</a>;
    <b>let</b> token_account = &<b>mut</b> account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()];
    <b>assert</b>!(token_account.pending.is_empty(), <a href="../contra/contra.md#contra_contra_EPendingDepositsMustBeMerged">EPendingDepositsMustBeMerged</a>);
    <b>let</b> new_balance = new_balance.into_well_formed(
        token_account.<a href="../contra/contra.md#contra_contra_session_id">session_id</a>.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_ELGAMAL">DST_ELGAMAL</a>),
        new_pk,
        new_balance_proof,
    );
    <b>assert</b>!(
        token_account
            .active
            .try_set_public_key(
                &token_account.pk,
                &new_pk,
                &new_balance,
                handle_eq_proof,
                <a href="../contra/contra.md#contra_contra_dst">dst</a>,
            ),
        <a href="../contra/contra.md#contra_contra_EAmountsEqualityProofFailed">EAmountsEqualityProofFailed</a>,
    );
    token_account.pk = new_pk;
    token_account.verified_key_encryption = new_verified_key_encryption;
    <a href="../contra/events.md#contra_events_emit_updated_public_key">events::emit_updated_public_key</a>&lt;T&gt;(<a href="../contra/contra.md#contra_contra_owner">owner</a>, new_pk, token_account.verified_key_encryption);
}
</code></pre>



</details>

<a name="contra_contra_wrap"></a>

## Function `wrap`

Convert public coin to private tokens and add them to the public pending balance of <code>receiver</code>.
Authorized by <code>auth</code>, which must be for the <code><a href="../contra/contra.md#contra_contra_PERMISSIONED_WRAP">PERMISSIONED_WRAP</a></code> operation; <code>auth</code> may be for any owner.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_wrap">wrap</a>&lt;T&gt;(receiver: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &<a href="../myso/deny_list.md#myso_deny_list_DenyList">myso::deny_list::DenyList</a>, pool: &<a href="../contra/contra.md#contra_contra_Pool">contra::contra::Pool</a>&lt;T&gt;, coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, memo: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_wrap">wrap</a>&lt;T&gt;(
    receiver: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &DenyList,
    pool: &<a href="../contra/contra.md#contra_contra_Pool">Pool</a>&lt;T&gt;,
    coin: Coin&lt;T&gt;,
    memo: vector&lt;u8&gt;,
) {
    <b>assert</b>!(auth.is_allowed(<a href="../contra/contra.md#contra_contra_PERMISSIONED_WRAP">PERMISSIONED_WRAP</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>assert</b>!(
        ct.is_active &&
        !is_frozen&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>) &&
        !is_receiver_denied&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>, receiver.<a href="../contra/contra.md#contra_contra_owner">owner</a>),
        <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>,
    );
    <b>let</b> acc = &<b>mut</b> receiver[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()];
    <b>assert</b>!(!acc.is_frozen, <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>);
    <b>assert</b>!(acc.accepts_deposits, <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>);
    <b>assert</b>!(acc.public_balance.value() &gt; 0
        || acc.<a href="../contra/contra.md#contra_contra_has_deposit_slot">has_deposit_slot</a>(), <a href="../contra/contra.md#contra_contra_EBalancesFull">EBalancesFull</a>);
    <b>let</b> amount = coin.value();
    <b>let</b> public_coin = <a href="../contra/balance.md#contra_balance_wrap">balance::wrap</a>(coin, &pool.id);
    acc.public_balance.join(public_coin);
    <a href="../contra/events.md#contra_events_emit_wrap">events::emit_wrap</a>&lt;T&gt;(receiver.<a href="../contra/contra.md#contra_contra_owner">owner</a>, amount, memo);
}
</code></pre>



</details>

<a name="contra_contra_batched_transfer"></a>

## Function `batched_transfer`

Start a batched transfer from <code>sender</code>. <code>receiver_amounts[i]</code> is the transferred value
re-encrypted under <code>receiver_pks[i]</code>; <code>sender_amounts[i]</code> is the same value under the
sender's key, forwarded to the events and otherwise only checked as a sum.
<code>well_formed_proofs</code> is a single batched <code>WellFormedProof</code> covering
<code>receiver_amounts ++ [new_balance]</code> under <code>receiver_pks ++ [sender_pk]</code> — one aggregate
Bulletproof for the whole transfer. <code>consistency_proof</code> and <code>balance_proof</code> together prove
the sender's balance drops by exactly the transfer total (see <code><a href="../contra/balance.md#contra_balance_try_split_batch">balance::try_split_batch</a></code>).

Returns <code>TransferBatch::Ok</code> when <code>balance_proof</code> verifies, else <code>BalanceProofFailed</code>. Aborts
if <code>well_formed_proofs</code> does not verify, the sender amounts don't sum to the receivers, or
<code>consistency_proof</code> fails. Call <code>add</code> once per receiver, in <code>receiver_amounts</code> order, then
<code><a href="../contra/contra.md#contra_contra_finalize">finalize</a></code>. Authorized by any <code>Auth&lt;T&gt;</code> for <code>sender.<a href="../contra/contra.md#contra_contra_owner">owner</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_batched_transfer">batched_transfer</a>&lt;T&gt;(sender: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &<a href="../myso/deny_list.md#myso_deny_list_DenyList">myso::deny_list::DenyList</a>, receiver_pks: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, receiver_amounts: vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>&gt;, well_formed_proofs: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, sender_amounts: vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>&gt;, consistency_proof: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, balance_proof: <a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>): <a href="../contra/contra.md#contra_contra_TransferBatch">contra::contra::TransferBatch</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_batched_transfer">batched_transfer</a>&lt;T&gt;(
    sender: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &DenyList,
    <b>mut</b> receiver_pks: vector&lt;Element&lt;G&gt;&gt;,
    <b>mut</b> receiver_amounts: vector&lt;EncryptedAmount&gt;,
    well_formed_proofs: WellFormedProof,
    <b>mut</b> sender_amounts: vector&lt;EncryptedAmount&gt;,
    consistency_proof: ElGamalProof,
    new_balance: EncryptedAmount,
    balance_proof: DdhProof,
): <a href="../contra/contra.md#contra_contra_TransferBatch">TransferBatch</a>&lt;T&gt; {
    <b>assert</b>!(ct.is_active, <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>);
    <b>assert</b>!(auth.is_authenticated(sender.<a href="../contra/contra.md#contra_contra_owner">owner</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>assert</b>!(
        !is_sender_denied&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>, sender.<a href="../contra/contra.md#contra_contra_owner">owner</a>) && !is_frozen&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>),
        <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>,
    );
    <b>assert</b>!(!receiver_amounts.is_empty(), <a href="../contra/contra.md#contra_contra_EEmptyTransferBatch">EEmptyTransferBatch</a>);
    <b>assert</b>!(receiver_amounts.length() == receiver_pks.length(), <a href="../contra/contra.md#contra_contra_EEmptyTransferBatch">EEmptyTransferBatch</a>);
    <b>let</b> sender_addr = sender.<a href="../contra/contra.md#contra_contra_owner">owner</a>;
    <b>let</b> sender = &<b>mut</b> sender[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()];
    <b>assert</b>!(!sender.is_frozen, <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>);
    // `well_formed_proofs` is one aggregate proof over `[receiver_amounts..., new_balance]`
    // under `[receiver_pks..., sender.pk]`; verify and <a href="../contra/contra.md#contra_contra_wrap">wrap</a> into WFEAs in one call, then peel
    // the last <b>entry</b> off <b>as</b> the sender's new-<a href="../contra/balance.md#contra_balance">balance</a> WFEA.
    receiver_amounts.push_back(new_balance);
    receiver_pks.push_back(sender.pk);
    <b>let</b> <b>mut</b> wfeas = <a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_into_well_formed">encrypted_amount::batch_into_well_formed</a>(
        receiver_amounts,
        sender.<a href="../contra/contra.md#contra_contra_session_id">session_id</a>.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_ELGAMAL">DST_ELGAMAL</a>),
        receiver_pks,
        well_formed_proofs,
    );
    <b>let</b> new_balance = wfeas.pop_back();
    <b>let</b> receiver_amounts = wfeas;
    <b>let</b> withdrawn = sender
        .active
        .try_split_batch(
            &sender.pk,
            new_balance,
            receiver_amounts,
            &sender_amounts,
            consistency_proof,
            sender.<a href="../contra/contra.md#contra_contra_session_id">session_id</a>.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_ELGAMAL">DST_ELGAMAL</a>),
            &balance_proof,
            sender.<a href="../contra/contra.md#contra_contra_session_id">session_id</a>.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_DDH">DST_DDH</a>),
        );
    <b>if</b> (withdrawn.is_some()) {
        <b>let</b> <b>mut</b> coins = withdrawn.destroy_some();
        // Reverse both so `<a href="../contra/contra.md#contra_contra_add_to_batch">add_to_batch</a>`'s `pop_back` consumes them in submission order.
        coins.reverse();
        sender_amounts.reverse();
        TransferBatch::Ok {
            sender: sender_addr,
            sender_pk: sender.pk,
            coins,
            sender_amounts,
        }
    } <b>else</b> {
        withdrawn.destroy_none();
        TransferBatch::BalanceProofFailed
    }
}
</code></pre>



</details>

<a name="contra_contra_add_to_batch"></a>

## Function `add_to_batch`

Add a receiver to a batched transfer: pop the next receiver-keyed <code>EncryptedCoin</code> and credit it
to the receiver's pending deposits. Aborts if:
* the receiver is not registered, frozen, or on the deny list,
* <code><a href="../contra/contra.md#contra_contra_add_to_batch">add_to_batch</a></code> is called more times than there were <code>receiver_amounts</code> in <code><a href="../contra/contra.md#contra_contra_batched_transfer">batched_transfer</a></code>,
* the coin is not encrypted under the receiver's registered public key.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_add_to_batch">add_to_batch</a>&lt;T&gt;(batch: <a href="../contra/contra.md#contra_contra_TransferBatch">contra::contra::TransferBatch</a>&lt;T&gt;, receiver: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, memo: vector&lt;u8&gt;, <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &<a href="../myso/deny_list.md#myso_deny_list_DenyList">myso::deny_list::DenyList</a>): <a href="../contra/contra.md#contra_contra_TransferBatch">contra::contra::TransferBatch</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_add_to_batch">add_to_batch</a>&lt;T&gt;(
    batch: <a href="../contra/contra.md#contra_contra_TransferBatch">TransferBatch</a>&lt;T&gt;,
    receiver: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    memo: vector&lt;u8&gt;,
    <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &DenyList,
): <a href="../contra/contra.md#contra_contra_TransferBatch">TransferBatch</a>&lt;T&gt; {
    match (batch) {
        // If batch is already failed, nothing should be mutated or emitted and the function should immediately <b>return</b> TransferBatch::BalanceProofFailed.
        TransferBatch::BalanceProofFailed =&gt; TransferBatch::BalanceProofFailed,
        // If batch is Ok, all mutations and checks must either succeed or <b>assert</b>, but never fail silently.
        TransferBatch::Ok { sender, sender_pk, <b>mut</b> coins, <b>mut</b> sender_amounts } =&gt; {
            <b>assert</b>!(!coins.is_empty(), <a href="../contra/contra.md#contra_contra_ETooManyReceivers">ETooManyReceivers</a>);
            <b>let</b> receiver_addr = receiver.<a href="../contra/contra.md#contra_contra_owner">owner</a>;
            <b>assert</b>!(!is_receiver_denied&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>, receiver_addr), <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>);
            <b>let</b> coin = coins.pop_back();
            <b>let</b> encrypted_amount_sender = sender_amounts.pop_back();
            <b>let</b> receiver = &<b>mut</b> receiver[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()];
            <b>assert</b>!(!receiver.is_frozen, <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>);
            <b>assert</b>!(receiver.accepts_deposits, <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>);
            <b>assert</b>!(receiver.<a href="../contra/contra.md#contra_contra_has_deposit_slot">has_deposit_slot</a>(), <a href="../contra/contra.md#contra_contra_EBalancesFull">EBalancesFull</a>);
            <a href="../contra/events.md#contra_events_emit_transfer">events::emit_transfer</a>&lt;T&gt;(
                sender,
                sender_pk,
                encrypted_amount_sender,
                receiver_addr,
                receiver.pk,
                *coin.amount().amount(),
                memo,
            );
            receiver.pending.merge_encrypted(&receiver.pk, coin);
            TransferBatch::Ok { sender, sender_pk, coins, sender_amounts }
        },
    }
}
</code></pre>



</details>

<a name="contra_contra_try_finalize"></a>

## Function `try_finalize`

Consume the <code><a href="../contra/contra.md#contra_contra_TransferBatch">TransferBatch</a></code> to complete the transfer batch and return <code><b>true</b></code> if the transfer
succeeded and <code><b>false</b></code> if the balance proof failed.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_try_finalize">try_finalize</a>&lt;T&gt;(batch: <a href="../contra/contra.md#contra_contra_TransferBatch">contra::contra::TransferBatch</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_try_finalize">try_finalize</a>&lt;T&gt;(batch: <a href="../contra/contra.md#contra_contra_TransferBatch">TransferBatch</a>&lt;T&gt;): bool {
    match (batch) {
        TransferBatch::BalanceProofFailed =&gt; {
            // It is critical to make sure no <a href="../contra/events.md#contra_events">events</a> were emitted, or mutations were made before
            // this point.
            <a href="../contra/events.md#contra_events_emit_try_transfer_failed">events::emit_try_transfer_failed</a>();
            <b>false</b>
        },
        TransferBatch::Ok { coins, sender_amounts, .. } =&gt; {
            <b>assert</b>!(coins.is_empty() && sender_amounts.is_empty(), <a href="../contra/contra.md#contra_contra_EAllAmountsMustBeUsed">EAllAmountsMustBeUsed</a>);
            coins.destroy_empty();
            <b>true</b>
        },
    }
}
</code></pre>



</details>

<a name="contra_contra_finalize"></a>

## Function `finalize`

Consume the <code><a href="../contra/contra.md#contra_contra_TransferBatch">TransferBatch</a></code> to complete the transfer batch. Aborts if any check, including the
balance proof, failed.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_finalize">finalize</a>&lt;T&gt;(batch: <a href="../contra/contra.md#contra_contra_TransferBatch">contra::contra::TransferBatch</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_finalize">finalize</a>&lt;T&gt;(batch: <a href="../contra/contra.md#contra_contra_TransferBatch">TransferBatch</a>&lt;T&gt;) {
    <b>assert</b>!(batch.<a href="../contra/contra.md#contra_contra_try_finalize">try_finalize</a>(), <a href="../contra/contra.md#contra_contra_EBalanceProofFailed">EBalanceProofFailed</a>);
}
</code></pre>



</details>

<a name="contra_contra_merge"></a>

## Function `merge`

Merge all pending deposits into the active balance.
This must be done before pending encrypted and public deposits can be used in a transfer.
To prevent overflows, the number of additions done with the active balance is limited,
including the number of additions done with the pending deposits.
Authorized by <code>auth</code>, which must be for <code>account.<a href="../contra/contra.md#contra_contra_owner">owner</a></code>. Any <code>Auth&lt;T&gt;</code> is accepted regardless
of which operation it covers.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_merge">merge</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_merge">merge</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>, auth: &Auth&lt;T&gt;) {
    <b>assert</b>!(auth.is_authenticated(account.<a href="../contra/contra.md#contra_contra_owner">owner</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>let</b> <a href="../contra/contra.md#contra_contra_owner">owner</a> = account.<a href="../contra/contra.md#contra_contra_owner">owner</a>;
    <b>let</b> acc = &<b>mut</b> account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()];
    acc.active.merge_into(&<b>mut</b> acc.pending);
    acc.active.merge_public(acc.public_balance.take());
    <a href="../contra/events.md#contra_events_emit_merge_deposits">events::emit_merge_deposits</a>&lt;T&gt;(<a href="../contra/contra.md#contra_contra_owner">owner</a>);
}
</code></pre>



</details>

<a name="contra_contra_update_active_balance"></a>

## Function `update_active_balance`

This may be used to update the balance after merging many pending deposits before
merging new deposits.
Authorized by <code>auth</code>, which must be for <code>account.<a href="../contra/contra.md#contra_contra_owner">owner</a></code>. Any <code>Auth&lt;T&gt;</code> is accepted regardless
of which operation it covers.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_update_active_balance">update_active_balance</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, new_balance_proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, balance_proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_update_active_balance">update_active_balance</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    new_balance: EncryptedAmount,
    new_balance_proof: WellFormedProof,
    balance_proof: &DdhProof,
) {
    <b>assert</b>!(auth.is_authenticated(account.<a href="../contra/contra.md#contra_contra_owner">owner</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>let</b> <a href="../contra/contra.md#contra_contra_owner">owner</a> = account.<a href="../contra/contra.md#contra_contra_owner">owner</a>;
    <b>let</b> token_account = &<b>mut</b> account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()];
    <b>let</b> sid = token_account.<a href="../contra/contra.md#contra_contra_session_id">session_id</a>;
    <b>assert</b>!(
        token_account.<a href="../contra/contra.md#contra_contra_try_update_active">try_update_active</a>(new_balance, new_balance_proof, balance_proof, sid),
        <a href="../contra/contra.md#contra_contra_EBalanceProofFailed">EBalanceProofFailed</a>,
    );
    <a href="../contra/events.md#contra_events_emit_update_balance">events::emit_update_balance</a>&lt;T&gt;(<a href="../contra/contra.md#contra_contra_owner">owner</a>);
}
</code></pre>



</details>

<a name="contra_contra_try_update_active"></a>

## Function `try_update_active`

Re-state <code>self.active</code> as the well-formed <code>new_balance</code> (same key), proven equal in value by
<code>balance_proof</code>. Returns whether the proof verified; adds no authorization or event. Shared by
<code><a href="../contra/contra.md#contra_contra_update_active_balance">update_active_balance</a></code> and the restate step of <code><a href="../contra/contra.md#contra_contra_try_set_public_key_and_unpause">try_set_public_key_and_unpause</a></code>.


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_try_update_active">try_update_active</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/contra.md#contra_contra_TokenAccount">contra::contra::TokenAccount</a>&lt;T&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, new_balance_proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, balance_proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, sid: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_try_update_active">try_update_active</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../contra/contra.md#contra_contra_TokenAccount">TokenAccount</a>&lt;T&gt;,
    new_balance: EncryptedAmount,
    new_balance_proof: WellFormedProof,
    balance_proof: &DdhProof,
    sid: vector&lt;u8&gt;,
): bool {
    <b>let</b> new_balance = new_balance.into_well_formed(
        sid.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_ELGAMAL">DST_ELGAMAL</a>),
        self.pk,
        new_balance_proof,
    );
    self.active.try_update(&self.pk, new_balance, balance_proof, sid.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_DDH">DST_DDH</a>))
}
</code></pre>



</details>

<a name="contra_contra_unwrap"></a>

## Function `unwrap`

Take an amount of <code>Coin&lt;T&gt;</code> from the encrypted balance of <code>account</code>. Authorized by <code>auth</code>,
which must be for the <code><a href="../contra/contra.md#contra_contra_PERMISSIONED_UNWRAP">PERMISSIONED_UNWRAP</a></code> operation and for <code>account.<a href="../contra/contra.md#contra_contra_owner">owner</a></code>.
The caller needs to provide a proof that the new balance is correct after taking the amount:
- <code>new_balance</code> is the new encrypted balance of the account after taking the amount,
- <code>amount</code> is the amount of coins taken from the balance,
- <code>balance_proof</code> is a proof that <code>account.<a href="../contra/balance.md#contra_balance">balance</a> = new_balance + amount</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_unwrap">unwrap</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &<a href="../myso/deny_list.md#myso_deny_list_DenyList">myso::deny_list::DenyList</a>, pool: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Pool">contra::contra::Pool</a>&lt;T&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, new_balance_proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, amount: u64, balance_proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_unwrap">unwrap</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &DenyList,
    pool: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Pool">Pool</a>&lt;T&gt;,
    new_balance: EncryptedAmount,
    new_balance_proof: WellFormedProof,
    amount: u64,
    balance_proof: &DdhProof,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;T&gt; {
    <b>let</b> (success, coin) = account.<a href="../contra/contra.md#contra_contra_try_unwrap_internal">try_unwrap_internal</a>(
        auth,
        ct,
        <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>,
        pool,
        new_balance,
        new_balance_proof,
        amount,
        balance_proof,
        ctx,
    );
    <b>assert</b>!(success, <a href="../contra/contra.md#contra_contra_EBalanceProofFailed">EBalanceProofFailed</a>);
    coin
}
</code></pre>



</details>

<a name="contra_contra_try_unwrap"></a>

## Function `try_unwrap`

Same as <code><a href="../contra/contra.md#contra_contra_unwrap">unwrap</a></code> but does not abort if the balance proof fails. Instead, it emits a
<code>TryUnwrapFailedEvent</code> and returns a zero-value coin.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_try_unwrap">try_unwrap</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &<a href="../myso/deny_list.md#myso_deny_list_DenyList">myso::deny_list::DenyList</a>, pool: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Pool">contra::contra::Pool</a>&lt;T&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, new_balance_proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, amount: u64, balance_proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_try_unwrap">try_unwrap</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &DenyList,
    pool: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Pool">Pool</a>&lt;T&gt;,
    new_balance: EncryptedAmount,
    new_balance_proof: WellFormedProof,
    amount: u64,
    balance_proof: &DdhProof,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;T&gt; {
    <b>let</b> (success, coin) = account.<a href="../contra/contra.md#contra_contra_try_unwrap_internal">try_unwrap_internal</a>(
        auth,
        ct,
        <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>,
        pool,
        new_balance,
        new_balance_proof,
        amount,
        balance_proof,
        ctx,
    );
    <b>if</b> (!success) {
        <a href="../contra/events.md#contra_events_emit_try_unwrap_failed">events::emit_try_unwrap_failed</a>();
    };
    coin
}
</code></pre>



</details>

<a name="contra_contra_try_unwrap_internal"></a>

## Function `try_unwrap_internal`

Common unwrap logic shared by <code><a href="../contra/contra.md#contra_contra_unwrap">unwrap</a></code> and <code><a href="../contra/contra.md#contra_contra_try_unwrap">try_unwrap</a></code>.
Returns <code>(<b>true</b>, coin)</code> if the balance proof succeeds, or <code>(<b>false</b>, zero_coin)</code> if it fails.


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_try_unwrap_internal">try_unwrap_internal</a>&lt;T&gt;(account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, auth: &<a href="../contra/policy.md#contra_policy_Auth">contra::policy::Auth</a>&lt;T&gt;, ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &<a href="../myso/deny_list.md#myso_deny_list_DenyList">myso::deny_list::DenyList</a>, pool: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Pool">contra::contra::Pool</a>&lt;T&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, new_balance_proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, amount: u64, balance_proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (bool, <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_try_unwrap_internal">try_unwrap_internal</a>&lt;T&gt;(
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    auth: &Auth&lt;T&gt;,
    ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    <a href="../contra/deny_list.md#contra_deny_list">deny_list</a>: &DenyList,
    pool: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Pool">Pool</a>&lt;T&gt;,
    new_balance: EncryptedAmount,
    new_balance_proof: WellFormedProof,
    amount: u64,
    balance_proof: &DdhProof,
    ctx: &<b>mut</b> TxContext,
): (bool, Coin&lt;T&gt;) {
    <b>assert</b>!(auth.is_allowed(<a href="../contra/contra.md#contra_contra_PERMISSIONED_UNWRAP">PERMISSIONED_UNWRAP</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>assert</b>!(auth.is_authenticated(account.<a href="../contra/contra.md#contra_contra_owner">owner</a>), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>assert</b>!(ct.is_active, <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>);
    <b>assert</b>!(
        !is_frozen&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>) && !is_sender_denied&lt;T&gt;(<a href="../contra/deny_list.md#contra_deny_list">deny_list</a>, account.<a href="../contra/contra.md#contra_contra_owner">owner</a>),
        <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>,
    );
    <b>let</b> <a href="../contra/contra.md#contra_contra_owner">owner</a> = account.<a href="../contra/contra.md#contra_contra_owner">owner</a>;
    <b>let</b> account = &<b>mut</b> account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()];
    <b>assert</b>!(!account.is_frozen, <a href="../contra/contra.md#contra_contra_ETransferDenied">ETransferDenied</a>);
    <b>let</b> sid = account.<a href="../contra/contra.md#contra_contra_session_id">session_id</a>;
    <b>let</b> new_balance = new_balance.into_well_formed(
        sid.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_ELGAMAL">DST_ELGAMAL</a>),
        account.pk,
        new_balance_proof,
    );
    <b>let</b> withdrawn = account
        .active
        .try_split_to_public(
            &account.pk,
            new_balance,
            amount,
            balance_proof,
            sid.<a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_DST_DDH">DST_DDH</a>),
        );
    <b>if</b> (withdrawn.is_some()) {
        <b>let</b> coin = withdrawn.destroy_some().<a href="../contra/contra.md#contra_contra_unwrap">unwrap</a>(&<b>mut</b> pool.id, ctx);
        <a href="../contra/events.md#contra_events_emit_unwrap">events::emit_unwrap</a>&lt;T&gt;(<a href="../contra/contra.md#contra_contra_owner">owner</a>, amount);
        (<b>true</b>, coin)
    } <b>else</b> {
        withdrawn.destroy_none();
        (<b>false</b>, coin::zero(ctx))
    }
}
</code></pre>



</details>

<a name="contra_contra_owner"></a>

## Function `owner`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_owner">owner</a>(account: &<a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_owner">owner</a>(account: &<a href="../contra/contra.md#contra_contra_Account">Account</a>): <b>address</b> {
    account.<a href="../contra/contra.md#contra_contra_owner">owner</a>
}
</code></pre>



</details>

<a name="contra_contra_set_balance_by_issuer"></a>

## Function `set_balance_by_issuer`

A function for the issuer to set the balance of an account directly.
This is used in cases where the issuer needs to intervene.

WARNING: This may break the consistency of the balance such that the number of confidential
tokens in circulation does not match the amount of coins in the pool. It is the responsibility
of the caller to ensure consistency is maintained when using this function.
The <code>upper_bound</code> is set to 1, so the caller is responsible for ensuring that the
<code>EncryptedAmount</code> is well-formed.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_set_balance_by_issuer">set_balance_by_issuer</a>&lt;T&gt;(t: &<b>mut</b> <a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;T&gt;, account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_set_balance_by_issuer">set_balance_by_issuer</a>&lt;T&gt;(
    t: &<b>mut</b> TreasuryCap&lt;T&gt;,
    account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>,
    new_balance: EncryptedAmount,
) {
    <b>let</b> <a href="../contra/contra.md#contra_contra_owner">owner</a> = account.<a href="../contra/contra.md#contra_contra_owner">owner</a>;
    <b>let</b> account = &<b>mut</b> account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()];
    account.active.overwrite_unchecked(t, new_balance);
    account.pending.clear_unchecked(t);
    account.public_balance.set_zero_unchecked(t);
    <a href="../contra/events.md#contra_events_emit_set_balance_by_issuer">events::emit_set_balance_by_issuer</a>&lt;T&gt;(<a href="../contra/contra.md#contra_contra_owner">owner</a>, new_balance);
}
</code></pre>



</details>

<a name="contra_contra_issue_freeze_cap"></a>

## Function `issue_freeze_cap`

Allow the given address to freeze the token globally or freeze individual accounts
(via the ManagementCap). Only the issuer can unfreeze (globally or per-account).
Aborts if the address already has the freeze capability.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_issue_freeze_cap">issue_freeze_cap</a>&lt;T&gt;(ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, _t: &<a href="../contra/contra.md#contra_contra_ManagementCap">contra::contra::ManagementCap</a>&lt;T&gt;, addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_issue_freeze_cap">issue_freeze_cap</a>&lt;T&gt;(
    ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    _t: &<a href="../contra/contra.md#contra_contra_ManagementCap">ManagementCap</a>&lt;T&gt;,
    addr: <b>address</b>,
) {
    ct.freeze_admins.insert(addr);
}
</code></pre>



</details>

<a name="contra_contra_revoke_freeze_cap"></a>

## Function `revoke_freeze_cap`

Revoke the freeze capability from the given address.
Aborts if the address does not have the freeze capability.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_revoke_freeze_cap">revoke_freeze_cap</a>&lt;T&gt;(ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, _t: &<a href="../contra/contra.md#contra_contra_ManagementCap">contra::contra::ManagementCap</a>&lt;T&gt;, addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_revoke_freeze_cap">revoke_freeze_cap</a>&lt;T&gt;(
    ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    _t: &<a href="../contra/contra.md#contra_contra_ManagementCap">ManagementCap</a>&lt;T&gt;,
    addr: <b>address</b>,
) {
    ct.freeze_admins.remove(&addr);
}
</code></pre>



</details>

<a name="contra_contra_global_freeze"></a>

## Function `global_freeze`

Freeze the token globally. This prevents any transfers from happening until the token is
unfrozen again.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_global_freeze">global_freeze</a>&lt;T&gt;(ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_global_freeze">global_freeze</a>&lt;T&gt;(ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(ct.freeze_admins.contains(&ctx.sender()), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    ct.is_active = <b>false</b>;
    <a href="../contra/events.md#contra_events_emit_global_freeze">events::emit_global_freeze</a>&lt;T&gt;();
}
</code></pre>



</details>

<a name="contra_contra_global_unfreeze"></a>

## Function `global_unfreeze`

Unfreeze the token globally. This allows transfers to happen again and can only be done by the
token issuer.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_global_unfreeze">global_unfreeze</a>&lt;T&gt;(ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, _cap: &<a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_global_unfreeze">global_unfreeze</a>&lt;T&gt;(ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;, _cap: &TreasuryCap&lt;T&gt;) {
    ct.is_active = <b>true</b>;
    <a href="../contra/events.md#contra_events_emit_global_unfreeze">events::emit_global_unfreeze</a>&lt;T&gt;();
}
</code></pre>



</details>

<a name="contra_contra_account_freeze"></a>

## Function `account_freeze`

Freeze the given account for token <code>T</code>. A frozen account cannot transfer, receive, wrap,
or unwrap until unfrozen. Only addresses in <code>ct.freeze_admins</code> may call this.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_account_freeze">account_freeze</a>&lt;T&gt;(ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_account_freeze">account_freeze</a>&lt;T&gt;(ct: &<a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;, account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>, ctx: &TxContext) {
    <b>let</b> admin = ctx.sender();
    <b>assert</b>!(ct.freeze_admins.contains(&admin), <a href="../contra/contra.md#contra_contra_EAuthorizationError">EAuthorizationError</a>);
    <b>let</b> <a href="../contra/contra.md#contra_contra_owner">owner</a> = account.<a href="../contra/contra.md#contra_contra_owner">owner</a>;
    account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()].is_frozen = <b>true</b>;
    <a href="../contra/events.md#contra_events_emit_account_freeze">events::emit_account_freeze</a>&lt;T&gt;(admin, <a href="../contra/contra.md#contra_contra_owner">owner</a>);
}
</code></pre>



</details>

<a name="contra_contra_account_unfreeze"></a>

## Function `account_unfreeze`

Unfreeze the given account for token <code>T</code>. Only the token issuer (holder of <code>&TreasuryCap&lt;T&gt;</code>)
may call this. The asymmetry — admins freeze, only the issuer unfreezes — mirrors
<code><a href="../contra/contra.md#contra_contra_global_freeze">global_freeze</a></code> / <code><a href="../contra/contra.md#contra_contra_global_unfreeze">global_unfreeze</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_account_unfreeze">account_unfreeze</a>&lt;T&gt;(_cap: &<a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;T&gt;, account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_account_unfreeze">account_unfreeze</a>&lt;T&gt;(_cap: &TreasuryCap&lt;T&gt;, account: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>) {
    <b>let</b> <a href="../contra/contra.md#contra_contra_owner">owner</a> = account.<a href="../contra/contra.md#contra_contra_owner">owner</a>;
    account[<a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()].is_frozen = <b>false</b>;
    <a href="../contra/events.md#contra_events_emit_account_unfreeze">events::emit_account_unfreeze</a>&lt;T&gt;(<a href="../contra/contra.md#contra_contra_owner">owner</a>);
}
</code></pre>



</details>

<a name="contra_contra_set_policy"></a>

## Function `set_policy`

Set a policy for the confidential token.
This allows implementing permissioned operations, but only the witness type is stored here - the
logic must be handled in the corresponding flows.
See <code>register_permissioned</code> for an example of how this can be implemented.
Changing the witness type will break all in-flight permissioned calls using the old witness,
and thus highly discouraged.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_set_policy">set_policy</a>&lt;T, W&gt;(ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, _t: &<b>mut</b> <a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;T&gt;, permissioned_operations: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_set_policy">set_policy</a>&lt;T, W&gt;(
    ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    _t: &<b>mut</b> TreasuryCap&lt;T&gt;,
    permissioned_operations: vector&lt;u8&gt;,
) {
    <a href="../contra/policy.md#contra_policy_set">policy::set</a>&lt;W&gt;(&<b>mut</b> ct.<a href="../contra/policy.md#contra_policy">policy</a>, permissioned_operations);
    <a href="../contra/events.md#contra_events_emit_policy_update">events::emit_policy_update</a>&lt;T, W&gt;(permissioned_operations);
}
</code></pre>



</details>

<a name="contra_contra_update_auditors"></a>

## Function `update_auditors`

Update the auditors for this confidential token by setting their new public keys in the
corresponding <code><a href="../contra/auditors.md#contra_auditors">auditors</a></code> struct. If <code>bump_recommended_min</code> is true, the auditors'
<code>recommended_min_version</code> is raised to the new version, signalling that all users should
call <code><a href="../contra/contra.md#contra_contra_set_public_key">set_public_key</a></code> with a valid viewing key encrypted towards the new auditor keys.
The floor is advisory; the chain does not enforce it on transfer.
The auditor flow can be disabled by inputting an empty <code>public_keys</code> vector.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_update_auditors">update_auditors</a>&lt;T&gt;(ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">contra::contra::ConfidentialToken</a>&lt;T&gt;, _cap: &<a href="../contra/contra.md#contra_contra_ManagementCap">contra::contra::ManagementCap</a>&lt;T&gt;, public_keys: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, bump_recommended_min: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/contra.md#contra_contra_update_auditors">update_auditors</a>&lt;T&gt;(
    ct: &<b>mut</b> <a href="../contra/contra.md#contra_contra_ConfidentialToken">ConfidentialToken</a>&lt;T&gt;,
    _cap: &<a href="../contra/contra.md#contra_contra_ManagementCap">ManagementCap</a>&lt;T&gt;,
    public_keys: vector&lt;Element&lt;G&gt;&gt;,
    bump_recommended_min: bool,
) {
    ct.<a href="../contra/auditors.md#contra_auditors">auditors</a>.update(public_keys, bump_recommended_min);
    <a href="../contra/events.md#contra_events_emit_update_auditors">events::emit_update_auditors</a>&lt;T&gt;(
        *ct.<a href="../contra/auditors.md#contra_auditors">auditors</a>.pks(),
        ct.<a href="../contra/auditors.md#contra_auditors">auditors</a>.version(),
        ct.<a href="../contra/auditors.md#contra_auditors">auditors</a>.recommended_min_version(),
    );
}
</code></pre>



</details>

<a name="contra_contra_has_token"></a>

## Function `has_token`

Return whether the given account has registered for the given token type.


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_has_token">has_token</a>&lt;T&gt;(account: &<a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_has_token">has_token</a>&lt;T&gt;(account: &<a href="../contra/contra.md#contra_contra_Account">Account</a>): bool {
    df::exists_(&account.id, <a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;())
}
</code></pre>



</details>

<a name="contra_contra_has_deposit_slot"></a>

## Function `has_deposit_slot`

Slots available for new pending deposits. Always reserves one slot for a possible future
<code>merge_public</code> bump, so the cap compared against is <code>max_upper_bound() - 1</code> rather than
<code>max_upper_bound()</code>.


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_has_deposit_slot">has_deposit_slot</a>&lt;T&gt;(self: &<a href="../contra/contra.md#contra_contra_TokenAccount">contra::contra::TokenAccount</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_has_deposit_slot">has_deposit_slot</a>&lt;T&gt;(self: &<a href="../contra/contra.md#contra_contra_TokenAccount">TokenAccount</a>&lt;T&gt;): bool {
    <b>let</b> cap = <a href="../contra/balance.md#contra_balance_max_upper_bound_minus_1">balance::max_upper_bound_minus_1</a>();
    <b>let</b> used = self.active.upper_bound() + self.pending.upper_bound();
    cap &gt; used
}
</code></pre>



</details>

<a name="contra_contra_session_id"></a>

## Function `session_id`

20-byte session_id for <code>account</code>'s <code><a href="../contra/contra.md#contra_contra_TokenAccount">TokenAccount</a>&lt;T&gt;</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/contra.md#contra_contra_session_id">session_id</a>&lt;T&gt;(account: &<a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/contra.md#contra_contra_session_id">session_id</a>&lt;T&gt;(account: &<a href="../contra/contra.md#contra_contra_Account">Account</a>): vector&lt;u8&gt; {
    // TODO: Switch to a simple hash of the account ID and token type.
    // TODO: Must be unique across different chains.
    derived_object::derive_address(account.id.to_inner(), <a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;()).to_bytes().take(20)
}
</code></pre>



</details>

<a name="contra_contra_dst"></a>

## Function `dst`

21-byte Fiat-Shamir DST <code><a href="../contra/contra.md#contra_contra_session_id">session_id</a> || protocol_id</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_session_id">session_id</a>: vector&lt;u8&gt;, protocol_id: u8): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/contra.md#contra_contra_dst">dst</a>(<a href="../contra/contra.md#contra_contra_session_id">session_id</a>: vector&lt;u8&gt;, protocol_id: u8): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> bytes = <a href="../contra/contra.md#contra_contra_session_id">session_id</a>;
    bytes.push_back(protocol_id);
    bytes
}
</code></pre>



</details>

<a name="contra_contra_borrow"></a>

## Function `borrow`



<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_borrow">borrow</a>&lt;T&gt;(acc: &<a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, key: <a href="../contra/contra.md#contra_contra_TokenAccountKey">contra::contra::TokenAccountKey</a>&lt;T&gt;): &<a href="../contra/contra.md#contra_contra_TokenAccount">contra::contra::TokenAccount</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_borrow">borrow</a>&lt;T&gt;(acc: &<a href="../contra/contra.md#contra_contra_Account">Account</a>, key: <a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;): &<a href="../contra/contra.md#contra_contra_TokenAccount">TokenAccount</a>&lt;T&gt; {
    df::borrow(&acc.id, key)
}
</code></pre>



</details>

<a name="contra_contra_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_borrow_mut">borrow_mut</a>&lt;T&gt;(acc: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">contra::contra::Account</a>, key: <a href="../contra/contra.md#contra_contra_TokenAccountKey">contra::contra::TokenAccountKey</a>&lt;T&gt;): &<b>mut</b> <a href="../contra/contra.md#contra_contra_TokenAccount">contra::contra::TokenAccount</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/contra.md#contra_contra_borrow_mut">borrow_mut</a>&lt;T&gt;(acc: &<b>mut</b> <a href="../contra/contra.md#contra_contra_Account">Account</a>, key: <a href="../contra/contra.md#contra_contra_TokenAccountKey">TokenAccountKey</a>&lt;T&gt;): &<b>mut</b> <a href="../contra/contra.md#contra_contra_TokenAccount">TokenAccount</a>&lt;T&gt; {
    df::borrow_mut(&<b>mut</b> acc.id, key)
}
</code></pre>



</details>
