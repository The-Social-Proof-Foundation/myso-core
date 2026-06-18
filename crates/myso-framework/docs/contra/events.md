---
title: Module `contra::events`
---



-  [Struct `NewConfidentialTokenEvent`](#contra_events_NewConfidentialTokenEvent)
-  [Struct `PolicyUpdateEvent`](#contra_events_PolicyUpdateEvent)
-  [Struct `NewRegistrationEvent`](#contra_events_NewRegistrationEvent)
-  [Struct `UpdatedPublicKeyEvent`](#contra_events_UpdatedPublicKeyEvent)
-  [Struct `WrapEvent`](#contra_events_WrapEvent)
-  [Struct `TransferEvent`](#contra_events_TransferEvent)
-  [Struct `MergeDepositsEvent`](#contra_events_MergeDepositsEvent)
-  [Struct `TryTransferFailedEvent`](#contra_events_TryTransferFailedEvent)
-  [Struct `TryUnwrapFailedEvent`](#contra_events_TryUnwrapFailedEvent)
-  [Struct `TrySetPublicKeyFailedEvent`](#contra_events_TrySetPublicKeyFailedEvent)
-  [Struct `UnwrapEvent`](#contra_events_UnwrapEvent)
-  [Struct `UpdateBalanceEvent`](#contra_events_UpdateBalanceEvent)
-  [Struct `SetBalanceByIssuerEvent`](#contra_events_SetBalanceByIssuerEvent)
-  [Struct `GlobalFreezeEvent`](#contra_events_GlobalFreezeEvent)
-  [Struct `GlobalUnfreezeEvent`](#contra_events_GlobalUnfreezeEvent)
-  [Struct `AccountFreezeEvent`](#contra_events_AccountFreezeEvent)
-  [Struct `AccountUnfreezeEvent`](#contra_events_AccountUnfreezeEvent)
-  [Struct `UpdateAuditorsEvent`](#contra_events_UpdateAuditorsEvent)
-  [Function `emit_new_confidential_token`](#contra_events_emit_new_confidential_token)
-  [Function `emit_policy_update`](#contra_events_emit_policy_update)
-  [Function `emit_new_registration`](#contra_events_emit_new_registration)
-  [Function `emit_updated_public_key`](#contra_events_emit_updated_public_key)
-  [Function `emit_wrap`](#contra_events_emit_wrap)
-  [Function `emit_transfer`](#contra_events_emit_transfer)
-  [Function `emit_merge_deposits`](#contra_events_emit_merge_deposits)
-  [Function `emit_try_transfer_failed`](#contra_events_emit_try_transfer_failed)
-  [Function `emit_try_unwrap_failed`](#contra_events_emit_try_unwrap_failed)
-  [Function `emit_try_set_public_key_failed`](#contra_events_emit_try_set_public_key_failed)
-  [Function `emit_unwrap`](#contra_events_emit_unwrap)
-  [Function `emit_update_balance`](#contra_events_emit_update_balance)
-  [Function `emit_set_balance_by_issuer`](#contra_events_emit_set_balance_by_issuer)
-  [Function `emit_global_freeze`](#contra_events_emit_global_freeze)
-  [Function `emit_global_unfreeze`](#contra_events_emit_global_unfreeze)
-  [Function `emit_account_freeze`](#contra_events_emit_account_freeze)
-  [Function `emit_account_unfreeze`](#contra_events_emit_account_unfreeze)
-  [Function `emit_update_auditors`](#contra_events_emit_update_auditors)


<pre><code><b>use</b> <a href="../contra/auditors.md#contra_auditors">contra::auditors</a>;
<b>use</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount">contra::encrypted_amount</a>;
<b>use</b> <a href="../contra/nizk.md#contra_nizk">contra::nizk</a>;
<b>use</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal">contra::twisted_elgamal</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/rangeproofs.md#myso_rangeproofs">myso::rangeproofs</a>;
<b>use</b> <a href="../myso/ristretto255.md#myso_ristretto255">myso::ristretto255</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="contra_events_NewConfidentialTokenEvent"></a>

## Struct `NewConfidentialTokenEvent`

A new confidential token is created for a token type <code>T</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_NewConfidentialTokenEvent">NewConfidentialTokenEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="contra_events_PolicyUpdateEvent"></a>

## Struct `PolicyUpdateEvent`

A policy is updated for a confidential token.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_PolicyUpdateEvent">PolicyUpdateEvent</a>&lt;<b>phantom</b> T, <b>phantom</b> W&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>0: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_NewRegistrationEvent"></a>

## Struct `NewRegistrationEvent`

A new token account is registered for an account for a token type <code>T</code> with a public key <code>pk</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_NewRegistrationEvent">NewRegistrationEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
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
<code>pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>verified_key_encryption: <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_UpdatedPublicKeyEvent"></a>

## Struct `UpdatedPublicKeyEvent`

An account has updated the public key for a token type <code>T</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_UpdatedPublicKeyEvent">UpdatedPublicKeyEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
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
<code>new_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>verified_key_encryption: <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_WrapEvent"></a>

## Struct `WrapEvent`

A public coin is wrapped into a confidential token, adding to the pending encrypted balance of
an account. <code>memo</code> is an opaque caller-supplied blob, empty if none was provided.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_WrapEvent">WrapEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>receiver: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>memo: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_TransferEvent"></a>

## Struct `TransferEvent`

A confidential transfer is made from a sender to a receiver. The transferred amount is emitted
twice: <code>encrypted_amount_receiver</code>, the well-formed four-limb encryption under <code>receiver_pk</code>,
and <code>encrypted_amount_sender</code>, the same value under <code>sender_pk</code> so the sender can recognize
its own outgoing transfers. <code>memo</code> is an opaque caller-supplied blob, empty if none was provided.

TODO: <code>encrypted_amount_sender</code> is only verified as part of the batch total (its individual
value is not range-checked); this representation may change in a future revision.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_TransferEvent">TransferEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>sender_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>encrypted_amount_sender: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a></code>
</dt>
<dd>
</dd>
<dt>
<code>receiver: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>receiver_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>encrypted_amount_receiver: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a></code>
</dt>
<dd>
</dd>
<dt>
<code>memo: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_MergeDepositsEvent"></a>

## Struct `MergeDepositsEvent`

An account merges pending encrypted and public deposits to the active balance.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_MergeDepositsEvent">MergeDepositsEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_TryTransferFailedEvent"></a>

## Struct `TryTransferFailedEvent`

An try_finalize fails because the balance proof did not verify.
This is only emitted s.t. the client can detect that the transfer failed and alert the user.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_TryTransferFailedEvent">TryTransferFailedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="contra_events_TryUnwrapFailedEvent"></a>

## Struct `TryUnwrapFailedEvent`

Emitted when a <code>try_unwrap</code> fails due to an invalid balance proof.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_TryUnwrapFailedEvent">TryUnwrapFailedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="contra_events_TrySetPublicKeyFailedEvent"></a>

## Struct `TrySetPublicKeyFailedEvent`

Emitted when a <code>try_set_public_key_and_unpause</code> fails its optimistic restate.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_TrySetPublicKeyFailedEvent">TrySetPublicKeyFailedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="contra_events_UnwrapEvent"></a>

## Struct `UnwrapEvent`

An amount is taken from the balance of an account and converted to public coins.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_UnwrapEvent">UnwrapEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_UpdateBalanceEvent"></a>

## Struct `UpdateBalanceEvent`

An account updates its active balance to be well-formed (e.g. after merging deposits).


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_UpdateBalanceEvent">UpdateBalanceEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_SetBalanceByIssuerEvent"></a>

## Struct `SetBalanceByIssuerEvent`

The issuer directly overwrites the balance of an account (e.g. burn/seize).
<code>new_balance</code> carries the post-write encrypted amount (the bound has been
reset to 1 by the issuer write).


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_SetBalanceByIssuerEvent">SetBalanceByIssuerEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_GlobalFreezeEvent"></a>

## Struct `GlobalFreezeEvent`

The token is frozen globally by a freeze admin.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_GlobalFreezeEvent">GlobalFreezeEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="contra_events_GlobalUnfreezeEvent"></a>

## Struct `GlobalUnfreezeEvent`

The token is unfrozen globally by the issuer.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_GlobalUnfreezeEvent">GlobalUnfreezeEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="contra_events_AccountFreezeEvent"></a>

## Struct `AccountFreezeEvent`

An account is frozen by a freeze admin.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_AccountFreezeEvent">AccountFreezeEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>account: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_AccountUnfreezeEvent"></a>

## Struct `AccountUnfreezeEvent`

An account is unfrozen by the token issuer.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_AccountUnfreezeEvent">AccountUnfreezeEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_UpdateAuditorsEvent"></a>

## Struct `UpdateAuditorsEvent`

Emitted when the auditors for a confidential token of type <code>T</code> are updated.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/events.md#contra_events_UpdateAuditorsEvent">UpdateAuditorsEvent</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>public_keys: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>recommended_min_version: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_events_emit_new_confidential_token"></a>

## Function `emit_new_confidential_token`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_new_confidential_token">emit_new_confidential_token</a>&lt;T&gt;()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_new_confidential_token">emit_new_confidential_token</a>&lt;T&gt;() {
    event::emit(<a href="../contra/events.md#contra_events_NewConfidentialTokenEvent">NewConfidentialTokenEvent</a>&lt;T&gt;());
}
</code></pre>



</details>

<a name="contra_events_emit_policy_update"></a>

## Function `emit_policy_update`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_policy_update">emit_policy_update</a>&lt;T, W&gt;(permissioned_operations: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_policy_update">emit_policy_update</a>&lt;T, W&gt;(permissioned_operations: vector&lt;u8&gt;) {
    event::emit(<a href="../contra/events.md#contra_events_PolicyUpdateEvent">PolicyUpdateEvent</a>&lt;T, W&gt;(permissioned_operations));
}
</code></pre>



</details>

<a name="contra_events_emit_new_registration"></a>

## Function `emit_new_registration`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_new_registration">emit_new_registration</a>&lt;T&gt;(owner: <b>address</b>, pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, verified_key_encryption: <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_new_registration">emit_new_registration</a>&lt;T&gt;(
    owner: <b>address</b>,
    pk: Element&lt;G&gt;,
    verified_key_encryption: VerifiedKeyEncryption,
) {
    event::emit(<a href="../contra/events.md#contra_events_NewRegistrationEvent">NewRegistrationEvent</a>&lt;T&gt; { owner, pk, verified_key_encryption });
}
</code></pre>



</details>

<a name="contra_events_emit_updated_public_key"></a>

## Function `emit_updated_public_key`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_updated_public_key">emit_updated_public_key</a>&lt;T&gt;(owner: <b>address</b>, new_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, verified_key_encryption: <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_updated_public_key">emit_updated_public_key</a>&lt;T&gt;(
    owner: <b>address</b>,
    new_pk: Element&lt;G&gt;,
    verified_key_encryption: VerifiedKeyEncryption,
) {
    event::emit(<a href="../contra/events.md#contra_events_UpdatedPublicKeyEvent">UpdatedPublicKeyEvent</a>&lt;T&gt; { owner, new_pk, verified_key_encryption });
}
</code></pre>



</details>

<a name="contra_events_emit_wrap"></a>

## Function `emit_wrap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_wrap">emit_wrap</a>&lt;T&gt;(receiver: <b>address</b>, amount: u64, memo: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_wrap">emit_wrap</a>&lt;T&gt;(receiver: <b>address</b>, amount: u64, memo: vector&lt;u8&gt;) {
    event::emit(<a href="../contra/events.md#contra_events_WrapEvent">WrapEvent</a>&lt;T&gt; { receiver, amount, memo });
}
</code></pre>



</details>

<a name="contra_events_emit_transfer"></a>

## Function `emit_transfer`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_transfer">emit_transfer</a>&lt;T&gt;(sender: <b>address</b>, sender_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, encrypted_amount_sender: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, receiver: <b>address</b>, receiver_pk: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, encrypted_amount_receiver: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, memo: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_transfer">emit_transfer</a>&lt;T&gt;(
    sender: <b>address</b>,
    sender_pk: Element&lt;G&gt;,
    encrypted_amount_sender: EncryptedAmount,
    receiver: <b>address</b>,
    receiver_pk: Element&lt;G&gt;,
    encrypted_amount_receiver: EncryptedAmount,
    memo: vector&lt;u8&gt;,
) {
    event::emit(<a href="../contra/events.md#contra_events_TransferEvent">TransferEvent</a>&lt;T&gt; {
        sender,
        sender_pk,
        encrypted_amount_sender,
        receiver,
        receiver_pk,
        encrypted_amount_receiver,
        memo,
    });
}
</code></pre>



</details>

<a name="contra_events_emit_merge_deposits"></a>

## Function `emit_merge_deposits`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_merge_deposits">emit_merge_deposits</a>&lt;T&gt;(account: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_merge_deposits">emit_merge_deposits</a>&lt;T&gt;(account: <b>address</b>) {
    event::emit(<a href="../contra/events.md#contra_events_MergeDepositsEvent">MergeDepositsEvent</a>&lt;T&gt; { account });
}
</code></pre>



</details>

<a name="contra_events_emit_try_transfer_failed"></a>

## Function `emit_try_transfer_failed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_try_transfer_failed">emit_try_transfer_failed</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_try_transfer_failed">emit_try_transfer_failed</a>() {
    event::emit(<a href="../contra/events.md#contra_events_TryTransferFailedEvent">TryTransferFailedEvent</a>());
}
</code></pre>



</details>

<a name="contra_events_emit_try_unwrap_failed"></a>

## Function `emit_try_unwrap_failed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_try_unwrap_failed">emit_try_unwrap_failed</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_try_unwrap_failed">emit_try_unwrap_failed</a>() {
    event::emit(<a href="../contra/events.md#contra_events_TryUnwrapFailedEvent">TryUnwrapFailedEvent</a>());
}
</code></pre>



</details>

<a name="contra_events_emit_try_set_public_key_failed"></a>

## Function `emit_try_set_public_key_failed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_try_set_public_key_failed">emit_try_set_public_key_failed</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_try_set_public_key_failed">emit_try_set_public_key_failed</a>() {
    event::emit(<a href="../contra/events.md#contra_events_TrySetPublicKeyFailedEvent">TrySetPublicKeyFailedEvent</a>());
}
</code></pre>



</details>

<a name="contra_events_emit_unwrap"></a>

## Function `emit_unwrap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_unwrap">emit_unwrap</a>&lt;T&gt;(sender: <b>address</b>, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_unwrap">emit_unwrap</a>&lt;T&gt;(sender: <b>address</b>, amount: u64) {
    event::emit(<a href="../contra/events.md#contra_events_UnwrapEvent">UnwrapEvent</a>&lt;T&gt; { sender, amount });
}
</code></pre>



</details>

<a name="contra_events_emit_update_balance"></a>

## Function `emit_update_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_update_balance">emit_update_balance</a>&lt;T&gt;(account: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_update_balance">emit_update_balance</a>&lt;T&gt;(account: <b>address</b>) {
    event::emit(<a href="../contra/events.md#contra_events_UpdateBalanceEvent">UpdateBalanceEvent</a>&lt;T&gt; { account });
}
</code></pre>



</details>

<a name="contra_events_emit_set_balance_by_issuer"></a>

## Function `emit_set_balance_by_issuer`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_set_balance_by_issuer">emit_set_balance_by_issuer</a>&lt;T&gt;(account: <b>address</b>, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_set_balance_by_issuer">emit_set_balance_by_issuer</a>&lt;T&gt;(account: <b>address</b>, new_balance: EncryptedAmount) {
    event::emit(<a href="../contra/events.md#contra_events_SetBalanceByIssuerEvent">SetBalanceByIssuerEvent</a>&lt;T&gt; { account, new_balance });
}
</code></pre>



</details>

<a name="contra_events_emit_global_freeze"></a>

## Function `emit_global_freeze`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_global_freeze">emit_global_freeze</a>&lt;T&gt;()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_global_freeze">emit_global_freeze</a>&lt;T&gt;() {
    event::emit(<a href="../contra/events.md#contra_events_GlobalFreezeEvent">GlobalFreezeEvent</a>&lt;T&gt;());
}
</code></pre>



</details>

<a name="contra_events_emit_global_unfreeze"></a>

## Function `emit_global_unfreeze`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_global_unfreeze">emit_global_unfreeze</a>&lt;T&gt;()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_global_unfreeze">emit_global_unfreeze</a>&lt;T&gt;() {
    event::emit(<a href="../contra/events.md#contra_events_GlobalUnfreezeEvent">GlobalUnfreezeEvent</a>&lt;T&gt;());
}
</code></pre>



</details>

<a name="contra_events_emit_account_freeze"></a>

## Function `emit_account_freeze`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_account_freeze">emit_account_freeze</a>&lt;T&gt;(admin: <b>address</b>, account: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_account_freeze">emit_account_freeze</a>&lt;T&gt;(admin: <b>address</b>, account: <b>address</b>) {
    event::emit(<a href="../contra/events.md#contra_events_AccountFreezeEvent">AccountFreezeEvent</a>&lt;T&gt; { admin, account });
}
</code></pre>



</details>

<a name="contra_events_emit_account_unfreeze"></a>

## Function `emit_account_unfreeze`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_account_unfreeze">emit_account_unfreeze</a>&lt;T&gt;(account: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_account_unfreeze">emit_account_unfreeze</a>&lt;T&gt;(account: <b>address</b>) {
    event::emit(<a href="../contra/events.md#contra_events_AccountUnfreezeEvent">AccountUnfreezeEvent</a>&lt;T&gt; { account });
}
</code></pre>



</details>

<a name="contra_events_emit_update_auditors"></a>

## Function `emit_update_auditors`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_update_auditors">emit_update_auditors</a>&lt;T&gt;(public_keys: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, version: u32, recommended_min_version: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/events.md#contra_events_emit_update_auditors">emit_update_auditors</a>&lt;T&gt;(
    public_keys: vector&lt;Element&lt;G&gt;&gt;,
    version: u32,
    recommended_min_version: u32,
) {
    event::emit(<a href="../contra/events.md#contra_events_UpdateAuditorsEvent">UpdateAuditorsEvent</a>&lt;T&gt; { public_keys, version, recommended_min_version });
}
</code></pre>



</details>
