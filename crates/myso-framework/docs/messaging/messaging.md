---
title: Module `messaging::messaging`
---

Module: messaging

Public-facing module for the messaging package. All external interactions
should go through this module.

Wraps <code>permissions_group</code> to provide messaging-specific permission management,
<code><a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a></code> for key rotation, and <code><a href="../messaging/message_log.md#messaging_message_log">message_log</a></code> for **paid** <code>MYSO</code> escrow only.


<a name="@Permissions_0"></a>

### Permissions


From groups (auto-granted to creator):
- <code>PermissionsAdmin</code>: Manages core permissions (from permissioned_groups package)
- <code>ExtensionPermissionsAdmin</code>: Manages extension permissions (from other packages)

Messaging-specific:
- <code><a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a></code>: Send messages
- <code><a href="../messaging/messaging.md#messaging_messaging_MessagingReader">MessagingReader</a></code>: Read/decrypt messages
- <code><a href="../messaging/messaging.md#messaging_messaging_MessagingEditor">MessagingEditor</a></code>: Edit messages
- <code><a href="../messaging/messaging.md#messaging_messaging_MessagingDeleter">MessagingDeleter</a></code>: Delete messages
- <code>EncryptionKeyRotator</code>: Rotate encryption keys
- <code><a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a></code>: Register or clear this group's handle in [<code><a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">group_handle_registry::GroupHandleRegistry</a></code>]
- <code><a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a></code>: Edit group metadata (name, data)


<a name="@Security_1"></a>

### Security


- Membership is defined by having at least one permission
- Granting a permission implicitly adds the member if they don't exist
- Revoking the last permission automatically removes the member


    -  [Permissions](#@Permissions_0)
    -  [Security](#@Security_1)
-  [Struct `MESSAGING`](#messaging_messaging_MESSAGING)
-  [Struct `Messaging`](#messaging_messaging_Messaging)
-  [Struct `MessagingSender`](#messaging_messaging_MessagingSender)
-  [Struct `MessagingReader`](#messaging_messaging_MessagingReader)
-  [Struct `MessagingDeleter`](#messaging_messaging_MessagingDeleter)
-  [Struct `MessagingEditor`](#messaging_messaging_MessagingEditor)
-  [Struct `GroupHandleAdmin`](#messaging_messaging_GroupHandleAdmin)
-  [Struct `MetadataAdmin`](#messaging_messaging_MetadataAdmin)
-  [Struct `MessagingNamespace`](#messaging_messaging_MessagingNamespace)
-  [Constants](#@Constants_2)
-  [Function `init`](#messaging_messaging_init)
-  [Function `create_group`](#messaging_messaging_create_group)
    -  [Parameters](#@Parameters_3)
    -  [Returns](#@Returns_4)
    -  [Note](#@Note_5)
    -  [Aborts](#@Aborts_6)
-  [Function `create_and_share_group`](#messaging_messaging_create_and_share_group)
    -  [Parameters](#@Parameters_7)
    -  [Note](#@Note_8)
-  [Function `rotate_encryption_key`](#messaging_messaging_rotate_encryption_key)
    -  [Parameters](#@Parameters_9)
    -  [Aborts](#@Aborts_10)
-  [Function `leave`](#messaging_messaging_leave)
    -  [Parameters](#@Parameters_11)
    -  [Aborts](#@Aborts_12)
-  [Function `archive_group`](#messaging_messaging_archive_group)
    -  [Aborts](#@Aborts_13)
    -  [Note](#@Note_14)
-  [Function `set_group_handle`](#messaging_messaging_set_group_handle)
    -  [Aborts](#@Aborts_15)
-  [Function `clear_group_handle`](#messaging_messaging_clear_group_handle)
    -  [Aborts](#@Aborts_16)
-  [Function `lookup_group_by_handle`](#messaging_messaging_lookup_group_by_handle)
-  [Function `set_group_name`](#messaging_messaging_set_group_name)
    -  [Aborts](#@Aborts_17)
-  [Function `insert_group_data`](#messaging_messaging_insert_group_data)
    -  [Aborts](#@Aborts_18)
-  [Function `remove_group_data`](#messaging_messaging_remove_group_data)
    -  [Returns](#@Returns_19)
    -  [Aborts](#@Aborts_20)
-  [Function `assert_message_log_matches_group`](#messaging_messaging_assert_message_log_matches_group)
-  [Function `assert_group_not_archived`](#messaging_messaging_assert_group_not_archived)
-  [Function `send_paid_message_digest`](#messaging_messaging_send_paid_message_digest)
-  [Function `reply_to_paid_message_claim_coin`](#messaging_messaging_reply_to_paid_message_claim_coin)
-  [Function `reply_to_paid_message_claim_settled`](#messaging_messaging_reply_to_paid_message_claim_settled)
-  [Function `refund_paid_escrow`](#messaging_messaging_refund_paid_escrow)
-  [Function `grant_all_messaging_permissions`](#messaging_messaging_grant_all_messaging_permissions)
-  [Function `assert_peers_not_blocked`](#messaging_messaging_assert_peers_not_blocked)
-  [Function `count_non_creator_peers`](#messaging_messaging_count_non_creator_peers)
-  [Function `is_direct_message_group`](#messaging_messaging_is_direct_message_group)
-  [Function `assert_paid_open_allowed`](#messaging_messaging_assert_paid_open_allowed)
-  [Function `assert_paid_parties_not_blocked`](#messaging_messaging_assert_paid_parties_not_blocked)


<pre><code><b>use</b> <a href="../messaging/encryption_history.md#messaging_encryption_history">messaging::encryption_history</a>;
<b>use</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry">messaging::group_handle_registry</a>;
<b>use</b> <a href="../messaging/group_leaver.md#messaging_group_leaver">messaging::group_leaver</a>;
<b>use</b> <a href="../messaging/group_manager.md#messaging_group_manager">messaging::group_manager</a>;
<b>use</b> <a href="../messaging/message_log.md#messaging_message_log">messaging::message_log</a>;
<b>use</b> <a href="../messaging/metadata.md#messaging_metadata">messaging::metadata</a>;
<b>use</b> <a href="../messaging/paid_escrow_settlement.md#messaging_paid_escrow_settlement">messaging::paid_escrow_settlement</a>;
<b>use</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy">messaging::paid_messaging_policy</a>;
<b>use</b> <a href="../messaging/version.md#messaging_version">messaging::version</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/myso.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/permissioned_group.md#myso_permissioned_group">myso::permissioned_group</a>;
<b>use</b> <a href="../myso/permissions_table.md#myso_permissions_table">myso::permissions_table</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/table_vec.md#myso_table_vec">myso::table_vec</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/unpause_cap.md#myso_unpause_cap">myso::unpause_cap</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
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



<a name="messaging_messaging_MESSAGING"></a>

## Struct `MESSAGING`

One-Time Witness for claiming Publisher.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging.md#messaging_messaging_MESSAGING">MESSAGING</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="messaging_messaging_Messaging"></a>

## Struct `Messaging`

Package witness for <code>PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="messaging_messaging_MessagingSender"></a>

## Struct `MessagingSender`

Permission to send messages to the group.
Separate from <code><a href="../messaging/messaging.md#messaging_messaging_MessagingReader">MessagingReader</a></code> to enable mute functionality.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="messaging_messaging_MessagingReader"></a>

## Struct `MessagingReader`

Permission to read/decrypt messages from the group.
Separate from <code><a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a></code> to enable read-only or write-only access.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging.md#messaging_messaging_MessagingReader">MessagingReader</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="messaging_messaging_MessagingDeleter"></a>

## Struct `MessagingDeleter`

Permission to delete messages in the group.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging.md#messaging_messaging_MessagingDeleter">MessagingDeleter</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="messaging_messaging_MessagingEditor"></a>

## Struct `MessagingEditor`

Permission to edit messages in the group.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging.md#messaging_messaging_MessagingEditor">MessagingEditor</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="messaging_messaging_GroupHandleAdmin"></a>

## Struct `GroupHandleAdmin`

Permission to set or clear this group's handle in the package [<code>GroupHandleRegistry</code>].


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="messaging_messaging_MetadataAdmin"></a>

## Struct `MetadataAdmin`

Permission to edit group metadata (name, data).


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="messaging_messaging_MessagingNamespace"></a>

## Struct `MessagingNamespace`

Shared object used as namespace for deriving group and encryption history addresses.
One per package deployment.


<pre><code><b>public</b> <b>struct</b> <a href="../messaging/messaging.md#messaging_messaging_MessagingNamespace">MessagingNamespace</a> <b>has</b> key
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

<a name="@Constants_2"></a>

## Constants


<a name="messaging_messaging_ENotPermitted"></a>

Caller lacks the required permission for the operation.


<pre><code><b>const</b> <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>: u64 = 0;
</code></pre>



<a name="messaging_messaging_EGroupArchived"></a>

The group is archived (paused) and cannot be mutated.


<pre><code><b>const</b> <a href="../messaging/messaging.md#messaging_messaging_EGroupArchived">EGroupArchived</a>: u64 = 1;
</code></pre>



<a name="messaging_messaging_EEncryptionHistoryMismatch"></a>

The provided <code>EncryptionHistory</code> does not belong to the given group.


<pre><code><b>const</b> <a href="../messaging/messaging.md#messaging_messaging_EEncryptionHistoryMismatch">EEncryptionHistoryMismatch</a>: u64 = 2;
</code></pre>



<a name="messaging_messaging_EPermissionsAdminCannotLeave"></a>

<code>PermissionsAdmin</code> holders cannot use <code><a href="../messaging/messaging.md#messaging_messaging_leave">leave</a>()</code>. They should use
<code>permissioned_group::remove_member()</code> for their own address instead,
which has a best-effort guard against removing the last <code>PermissionsAdmin</code>
(see <code>ELastPermissionsAdmin</code> — note that this count includes actor-object admins).


<pre><code><b>const</b> <a href="../messaging/messaging.md#messaging_messaging_EPermissionsAdminCannotLeave">EPermissionsAdminCannotLeave</a>: u64 = 3;
</code></pre>



<a name="messaging_messaging_EMessageLogMismatch"></a>

The <code>MessageLog</code> object does not belong to the given group.


<pre><code><b>const</b> <a href="../messaging/messaging.md#messaging_messaging_EMessageLogMismatch">EMessageLogMismatch</a>: u64 = 4;
</code></pre>



<a name="messaging_messaging_EPaidNotRequiredForFollower"></a>

Sender follows recipient on a new 1:1 DM; paid open is not required.


<pre><code><b>const</b> <a href="../messaging/messaging.md#messaging_messaging_EPaidNotRequiredForFollower">EPaidNotRequiredForFollower</a>: u64 = 5;
</code></pre>



<a name="messaging_messaging_EBelowMinMessageCost"></a>

Escrow is below the recipient's configured minimum for stranger paid DMs.


<pre><code><b>const</b> <a href="../messaging/messaging.md#messaging_messaging_EBelowMinMessageCost">EBelowMinMessageCost</a>: u64 = 6;
</code></pre>



<a name="messaging_messaging_CONVERSATION_KIND_KEY"></a>



<pre><code><b>const</b> <a href="../messaging/messaging.md#messaging_messaging_CONVERSATION_KIND_KEY">CONVERSATION_KIND_KEY</a>: vector&lt;u8&gt; = vector[99, 111, 110, 118, 101, 114, 115, 97, 116, 105, 111, 110, 95, 107, 105, 110, 100];
</code></pre>



<a name="messaging_messaging_CONVERSATION_KIND_DM"></a>



<pre><code><b>const</b> <a href="../messaging/messaging.md#messaging_messaging_CONVERSATION_KIND_DM">CONVERSATION_KIND_DM</a>: vector&lt;u8&gt; = vector[100, 109];
</code></pre>



<a name="messaging_messaging_init"></a>

## Function `init`



<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_init">init</a>(otw: <a href="../messaging/messaging.md#messaging_messaging_MESSAGING">messaging::messaging::MESSAGING</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_init">init</a>(otw: <a href="../messaging/messaging.md#messaging_messaging_MESSAGING">MESSAGING</a>, ctx: &<b>mut</b> TxContext) {
    package::claim_and_keep(otw, ctx);
    <b>let</b> <b>mut</b> namespace = <a href="../messaging/messaging.md#messaging_messaging_MessagingNamespace">MessagingNamespace</a> {
        id: object::new(ctx),
    };
    <b>let</b> <a href="../messaging/group_leaver.md#messaging_group_leaver">group_leaver</a> = <a href="../messaging/group_leaver.md#messaging_group_leaver_new">group_leaver::new</a>(&<b>mut</b> namespace.id);
    <b>let</b> <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a> = <a href="../messaging/group_manager.md#messaging_group_manager_new">group_manager::new</a>(&<b>mut</b> namespace.id);
    <b>let</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry">group_handle_registry</a> = <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_new">group_handle_registry::new</a>(&<b>mut</b> namespace.id, ctx);
    <b>let</b> paid_messaging_registry = <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_new">paid_messaging_policy::new</a>(&<b>mut</b> namespace.id, ctx);
    transfer::share_object(namespace);
    <a href="../messaging/group_leaver.md#messaging_group_leaver">group_leaver</a>.share();
    <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>.share();
    <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry">group_handle_registry</a>.share();
    paid_messaging_registry.share();
}
</code></pre>



</details>

<a name="messaging_messaging_create_group"></a>

## Function `create_group`

Creates a new messaging group with encryption.
The transaction sender (<code>ctx.sender()</code>) automatically becomes the creator with all permissions.


<a name="@Parameters_3"></a>

### Parameters

- <code><a href="../messaging/version.md#messaging_version">version</a></code>: Reference to the Version shared object
- <code>namespace</code>: Mutable reference to the MessagingNamespace
- <code><a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a></code>: Reference to the shared GroupManager actor
- <code>name</code>: Human-readable group name
- <code>uuid</code>: Client-provided UUID for deterministic address derivation
- <code>initial_encrypted_dek</code>: Initial MyData-encrypted DEK bytes
- <code>initial_members</code>: Addresses to grant <code><a href="../messaging/messaging.md#messaging_messaging_MessagingReader">MessagingReader</a></code> permission (should not include
creator)
- <code>ctx</code>: Transaction context


<a name="@Returns_4"></a>

### Returns

Tuple of <code>(PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;, EncryptionHistory, MessageLog)</code>.


<a name="@Note_5"></a>

### Note

If <code>initial_members</code> contains the creator's address, it is silently skipped (no abort).
This handles the common case where the creator might be mistakenly included in the initial
members list.


<a name="@Aborts_6"></a>

### Aborts

- <code>EInvalidVersion</code> (from <code><a href="../messaging/version.md#messaging_version">version</a></code>): if package version doesn't match
- If the UUID has already been used (duplicate derivation)


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_create_group">create_group</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, namespace: &<b>mut</b> <a href="../messaging/messaging.md#messaging_messaging_MessagingNamespace">messaging::messaging::MessagingNamespace</a>, <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, block_list: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, uuid: <a href="../std/string.md#std_string_String">std::string::String</a>, initial_encrypted_dek: vector&lt;u8&gt;, initial_members: <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, <a href="../messaging/encryption_history.md#messaging_encryption_history_EncryptionHistory">messaging::encryption_history::EncryptionHistory</a>, <a href="../messaging/message_log.md#messaging_message_log_MessageLog">messaging::message_log::MessageLog</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_create_group">create_group</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    namespace: &<b>mut</b> <a href="../messaging/messaging.md#messaging_messaging_MessagingNamespace">MessagingNamespace</a>,
    <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &GroupManager,
    block_list: &BlockListRegistry,
    name: String,
    uuid: String,
    initial_encrypted_dek: vector&lt;u8&gt;,
    initial_members: VecSet&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext,
): (PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;, EncryptionHistory, MessageLog) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <b>let</b> creator = ctx.sender();
    <a href="../messaging/messaging.md#messaging_messaging_assert_peers_not_blocked">assert_peers_not_blocked</a>(block_list, creator, &initial_members);
    <b>let</b> <b>mut</b> group: PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt; = permissioned_group::new_derived&lt;
        <a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>,
        <a href="../messaging/encryption_history.md#messaging_encryption_history_PermissionedGroupTag">encryption_history::PermissionedGroupTag</a>,
    &gt;(
        <a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>(),
        &<b>mut</b> namespace.id,
        <a href="../messaging/encryption_history.md#messaging_encryption_history_permissions_group_tag">encryption_history::permissions_group_tag</a>(uuid),
        ctx,
    );
    <a href="../messaging/messaging.md#messaging_messaging_grant_all_messaging_permissions">grant_all_messaging_permissions</a>(&<b>mut</b> group, creator, ctx);
    // Grant PermissionsAdmin to the GroupLeaver actor so it can remove members on behalf of
    // callers.
    // The <b>address</b> is derived deterministically from the namespace — no need to pass the object.
    <b>let</b> group_leaver_address = derived_object::derive_address(
        object::id(namespace),
        <a href="../messaging/group_leaver.md#messaging_group_leaver_derivation_key">group_leaver::derivation_key</a>(),
    );
    group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, PermissionsAdmin&gt;(group_leaver_address, ctx);
    // Grant ObjectAdmin to the GroupManager actor so it can access the group UID
    // <b>for</b> <a href="../messaging/metadata.md#messaging_metadata">metadata</a> management (dynamic field on the group UID).
    group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, ObjectAdmin&gt;(
        object::id(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>).to_address(),
        ctx,
    );
    // Attach Metadata via GroupManager
    <b>let</b> m = <a href="../messaging/metadata.md#messaging_metadata_new">metadata::new</a>(name, uuid, creator);
    <a href="../messaging/group_manager.md#messaging_group_manager_attach_metadata">group_manager::attach_metadata</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>, &<b>mut</b> group, m);
    <b>if</b> (<a href="../messaging/messaging.md#messaging_messaging_count_non_creator_peers">count_non_creator_peers</a>(&initial_members, creator) == 1) {
        <b>let</b> m = <a href="../messaging/group_manager.md#messaging_group_manager_borrow_metadata_mut">group_manager::borrow_metadata_mut</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>, &<b>mut</b> group);
        m.insert_data(
            string::utf8(<a href="../messaging/messaging.md#messaging_messaging_CONVERSATION_KIND_KEY">CONVERSATION_KIND_KEY</a>),
            string::utf8(<a href="../messaging/messaging.md#messaging_messaging_CONVERSATION_KIND_DM">CONVERSATION_KIND_DM</a>),
        );
    };
    // Grant <a href="../messaging/messaging.md#messaging_messaging_MessagingReader">MessagingReader</a> permission to initial members (skip creator)
    initial_members.into_keys().do!(|member| {
        <b>if</b> (member != creator) {
            group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MessagingReader">MessagingReader</a>&gt;(member, ctx);
        };
    });
    <b>let</b> <a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a> = <a href="../messaging/encryption_history.md#messaging_encryption_history_new">encryption_history::new</a>(
        &<b>mut</b> namespace.id,
        uuid,
        object::id(&group),
        initial_encrypted_dek,
        ctx,
    );
    <b>let</b> <a href="../messaging/message_log.md#messaging_message_log">message_log</a> = <a href="../messaging/message_log.md#messaging_message_log_new">message_log::new</a>(&<b>mut</b> namespace.id, uuid, object::id(&group), ctx);
    (group, <a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a>, <a href="../messaging/message_log.md#messaging_message_log">message_log</a>)
}
</code></pre>



</details>

<a name="messaging_messaging_create_and_share_group"></a>

## Function `create_and_share_group`

Creates a new messaging group and shares both objects.


<a name="@Parameters_7"></a>

### Parameters

- <code><a href="../messaging/version.md#messaging_version">version</a></code>: Reference to the Version shared object
- <code>namespace</code>: Mutable reference to the MessagingNamespace
- <code><a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a></code>: Reference to the shared GroupManager actor
- <code>name</code>: Human-readable group name
- <code>uuid</code>: Client-provided UUID for deterministic address derivation
- <code>initial_encrypted_dek</code>: Initial MyData-encrypted DEK bytes
- <code>initial_members</code>: Set of addresses to grant <code><a href="../messaging/messaging.md#messaging_messaging_MessagingReader">MessagingReader</a></code> permission
- <code>ctx</code>: Transaction context


<a name="@Note_8"></a>

### Note

See <code><a href="../messaging/messaging.md#messaging_messaging_create_group">create_group</a></code> for details on creator permissions and initial member handling.


<pre><code><b>entry</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_create_and_share_group">create_and_share_group</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, namespace: &<b>mut</b> <a href="../messaging/messaging.md#messaging_messaging_MessagingNamespace">messaging::messaging::MessagingNamespace</a>, <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, block_list: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, uuid: <a href="../std/string.md#std_string_String">std::string::String</a>, initial_encrypted_dek: vector&lt;u8&gt;, initial_members: vector&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_create_and_share_group">create_and_share_group</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    namespace: &<b>mut</b> <a href="../messaging/messaging.md#messaging_messaging_MessagingNamespace">MessagingNamespace</a>,
    <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &GroupManager,
    block_list: &BlockListRegistry,
    name: String,
    uuid: String,
    initial_encrypted_dek: vector&lt;u8&gt;,
    initial_members: vector&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (group, <a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a>, <a href="../messaging/message_log.md#messaging_message_log">message_log</a>) = <a href="../messaging/messaging.md#messaging_messaging_create_group">create_group</a>(
        <a href="../messaging/version.md#messaging_version">version</a>,
        namespace,
        <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>,
        block_list,
        name,
        uuid,
        initial_encrypted_dek,
        vec_set::from_keys(initial_members),
        ctx,
    );
    transfer::public_share_object(group);
    transfer::public_share_object(<a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a>);
    transfer::public_share_object(<a href="../messaging/message_log.md#messaging_message_log">message_log</a>);
}
</code></pre>



</details>

<a name="messaging_messaging_rotate_encryption_key"></a>

## Function `rotate_encryption_key`

Rotates the encryption key for a group.


<a name="@Parameters_9"></a>

### Parameters

- <code><a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a></code>: Mutable reference to the group's EncryptionHistory
- <code>group</code>: Reference to the PermissionedGroup<Messaging>
- <code>new_encrypted_dek</code>: New MyData-encrypted DEK bytes
- <code>ctx</code>: Transaction context


<a name="@Aborts_10"></a>

### Aborts

- <code>EInvalidVersion</code> (from <code><a href="../messaging/version.md#messaging_version">version</a></code>): if package version doesn't match
- <code><a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a></code>: if caller doesn't have <code>EncryptionKeyRotator</code> permission


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_rotate_encryption_key">rotate_encryption_key</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, <a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a>: &<b>mut</b> <a href="../messaging/encryption_history.md#messaging_encryption_history_EncryptionHistory">messaging::encryption_history::EncryptionHistory</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, new_encrypted_dek: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_rotate_encryption_key">rotate_encryption_key</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    <a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a>: &<b>mut</b> EncryptionHistory,
    group: &PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    new_encrypted_dek: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <b>assert</b>!(!group.is_paused(), <a href="../messaging/messaging.md#messaging_messaging_EGroupArchived">EGroupArchived</a>);
    <b>assert</b>!(<a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a>.group_id() == object::id(group), <a href="../messaging/messaging.md#messaging_messaging_EEncryptionHistoryMismatch">EEncryptionHistoryMismatch</a>);
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, EncryptionKeyRotator&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <a href="../messaging/encryption_history.md#messaging_encryption_history">encryption_history</a>.rotate_key(new_encrypted_dek);
}
</code></pre>



</details>

<a name="messaging_messaging_leave"></a>

## Function `leave`

Removes the caller from a messaging group.
The <code>GroupLeaver</code> actor holds <code>PermissionsAdmin</code> on all groups and calls
<code>object_remove_member</code> on behalf of the caller.

<code>PermissionsAdmin</code> holders cannot use this function. Since they already have
<code>PermissionsAdmin</code>, they can call <code>permissioned_group::remove_member()</code> for
their own address instead. Alternatively, they can first revoke their own
<code>PermissionsAdmin</code> and then call <code><a href="../messaging/messaging.md#messaging_messaging_leave">leave</a>()</code>.

**Why**: <code><a href="../messaging/messaging.md#messaging_messaging_leave">leave</a>()</code> is a self-service action via the <code>GroupLeaver</code> actor object.
Since <code>permissions_admin_count</code> includes both human and actor-object admins,
there is no reliable way to determine whether removing the caller would leave
the group without a human admin. Blocking <code>PermissionsAdmin</code> holders from
<code><a href="../messaging/messaging.md#messaging_messaging_leave">leave</a>()</code> makes this a deliberate admin decision rather than a casual action.

**Limitation**: Note that <code>permissions_admin_count</code> is a best-effort invariant.
Even via <code>remove_member()</code>, a group could end up with only actor-object admins
if the caller removes themselves when they are the last human admin. The count
cannot distinguish human from actor-object holders.


<a name="@Parameters_11"></a>

### Parameters

- <code><a href="../messaging/group_leaver.md#messaging_group_leaver">group_leaver</a></code>: Reference to the shared <code>GroupLeaver</code> object
- <code>group</code>: Mutable reference to the <code>PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;</code>
- <code>ctx</code>: Transaction context


<a name="@Aborts_12"></a>

### Aborts

- <code><a href="../messaging/messaging.md#messaging_messaging_EPermissionsAdminCannotLeave">EPermissionsAdminCannotLeave</a></code>: if the caller holds <code>PermissionsAdmin</code>
- <code>EMemberNotFound</code> (from <code>permissioned_group</code>): if the caller is not a member


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_leave">leave</a>(<a href="../messaging/group_leaver.md#messaging_group_leaver">group_leaver</a>: &<a href="../messaging/group_leaver.md#messaging_group_leaver_GroupLeaver">messaging::group_leaver::GroupLeaver</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_leave">leave</a>(
    <a href="../messaging/group_leaver.md#messaging_group_leaver">group_leaver</a>: &GroupLeaver,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    ctx: &TxContext,
) {
    <b>assert</b>!(
        !group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, PermissionsAdmin&gt;(ctx.sender()),
        <a href="../messaging/messaging.md#messaging_messaging_EPermissionsAdminCannotLeave">EPermissionsAdminCannotLeave</a>,
    );
    <a href="../messaging/group_leaver.md#messaging_group_leaver_leave">group_leaver::leave</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;(<a href="../messaging/group_leaver.md#messaging_group_leaver">group_leaver</a>, group, ctx);
}
</code></pre>



</details>

<a name="messaging_messaging_archive_group"></a>

## Function `archive_group`

Permanently archives a messaging group.

Pauses the group and burns the <code>UnpauseCap</code>, making it impossible to unpause.
After this call, <code>is_paused()</code> returns <code><b>true</b></code> and all mutations are blocked.

The caller must have <code>PermissionsAdmin</code> permission (enforced by <code>pause()</code>).


<a name="@Aborts_13"></a>

### Aborts

- <code><a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a></code> (from <code>pause</code>): if caller doesn't have <code>PermissionsAdmin</code>
- <code>EAlreadyPaused</code> (from <code>pause</code>): if the group is already paused


<a name="@Note_14"></a>

### Note

Alternative to burning: <code>transfer::public_freeze_object(cap)</code> makes the cap immutable
and un-passable by value, also preventing unpause without destroying the object.


<pre><code><b>entry</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_archive_group">archive_group</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_archive_group">archive_group</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <b>let</b> cap = group.pause&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;(ctx);
    cap.burn();
}
</code></pre>



</details>

<a name="messaging_messaging_set_group_handle"></a>

## Function `set_group_handle`

Registers or replaces the canonical handle for this group in the shared [<code>GroupHandleRegistry</code>].

The caller must have <code><a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a></code>. See <code><a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_set_handle">group_handle_registry::set_handle</a></code> for handle rules.


<a name="@Aborts_15"></a>

### Aborts

- <code><a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a></code>: if caller doesn't have <code><a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a></code>
- <code><a href="../messaging/messaging.md#messaging_messaging_EGroupArchived">EGroupArchived</a></code>: if the group is paused
- <code><a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_EHandleTaken">group_handle_registry::EHandleTaken</a></code> / <code>EInvalidHandle</code>: from the registry


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_set_group_handle">set_group_handle</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, registry: &<b>mut</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">messaging::group_handle_registry::GroupHandleRegistry</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, handle: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_set_group_handle">set_group_handle</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    registry: &<b>mut</b> GroupHandleRegistry,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    handle: String,
    ctx: &TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <a href="../messaging/messaging.md#messaging_messaging_assert_group_not_archived">assert_group_not_archived</a>(group);
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a>&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_set_handle">group_handle_registry::set_handle</a>(registry, object::id(group), handle);
}
</code></pre>



</details>

<a name="messaging_messaging_clear_group_handle"></a>

## Function `clear_group_handle`

Removes this group's handle from the registry, if any.


<a name="@Aborts_16"></a>

### Aborts

- <code><a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a></code>: if caller doesn't have <code><a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a></code>
- <code><a href="../messaging/messaging.md#messaging_messaging_EGroupArchived">EGroupArchived</a></code>: if the group is paused


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_clear_group_handle">clear_group_handle</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, registry: &<b>mut</b> <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">messaging::group_handle_registry::GroupHandleRegistry</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_clear_group_handle">clear_group_handle</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    registry: &<b>mut</b> GroupHandleRegistry,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    ctx: &TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <a href="../messaging/messaging.md#messaging_messaging_assert_group_not_archived">assert_group_not_archived</a>(group);
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a>&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_clear_handle">group_handle_registry::clear_handle</a>(registry, object::id(group));
}
</code></pre>



</details>

<a name="messaging_messaging_lookup_group_by_handle"></a>

## Function `lookup_group_by_handle`

Read-only: resolve a handle to a group object ID. Does not require <code><a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_lookup_group_by_handle">lookup_group_by_handle</a>(registry: &<a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_GroupHandleRegistry">messaging::group_handle_registry::GroupHandleRegistry</a>, handle: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_lookup_group_by_handle">lookup_group_by_handle</a>(registry: &GroupHandleRegistry, handle: String): Option&lt;ID&gt; {
    <a href="../messaging/group_handle_registry.md#messaging_group_handle_registry_lookup_group_by_handle">group_handle_registry::lookup_group_by_handle</a>(registry, handle)
}
</code></pre>



</details>

<a name="messaging_messaging_set_group_name"></a>

## Function `set_group_name`

Sets the group name.
Caller must have <code><a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a></code> permission.


<a name="@Aborts_17"></a>

### Aborts

- <code><a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a></code>: if caller doesn't have <code><a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a></code>
- <code>ENameTooLong</code> (from <code><a href="../messaging/metadata.md#messaging_metadata">metadata</a></code>): if name exceeds limit


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_set_group_name">set_group_name</a>(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_set_group_name">set_group_name</a>(
    <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &GroupManager,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    name: String,
    ctx: &TxContext,
) {
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a>&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <b>let</b> m = <a href="../messaging/group_manager.md#messaging_group_manager_borrow_metadata_mut">group_manager::borrow_metadata_mut</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>, group);
    m.set_name(name);
}
</code></pre>



</details>

<a name="messaging_messaging_insert_group_data"></a>

## Function `insert_group_data`

Inserts a key-value pair into the group's metadata data map.
Caller must have <code><a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a></code> permission.


<a name="@Aborts_18"></a>

### Aborts

- <code><a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a></code>: if caller doesn't have <code><a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a></code>
- <code>EDataKeyTooLong</code> (from <code><a href="../messaging/metadata.md#messaging_metadata">metadata</a></code>): if key exceeds limit
- <code>EDataValueTooLong</code> (from <code><a href="../messaging/metadata.md#messaging_metadata">metadata</a></code>): if value exceeds limit


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_insert_group_data">insert_group_data</a>(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, key: <a href="../std/string.md#std_string_String">std::string::String</a>, value: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_insert_group_data">insert_group_data</a>(
    <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &GroupManager,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    key: String,
    value: String,
    ctx: &TxContext,
) {
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a>&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <b>let</b> m = <a href="../messaging/group_manager.md#messaging_group_manager_borrow_metadata_mut">group_manager::borrow_metadata_mut</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>, group);
    m.insert_data(key, value);
}
</code></pre>



</details>

<a name="messaging_messaging_remove_group_data"></a>

## Function `remove_group_data`

Removes a key-value pair from the group's metadata data map.
Caller must have <code><a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a></code> permission.


<a name="@Returns_19"></a>

### Returns

The removed (key, value) tuple.


<a name="@Aborts_20"></a>

### Aborts

- <code><a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a></code>: if caller doesn't have <code><a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_remove_group_data">remove_group_data</a>(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, key: &<a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_remove_group_data">remove_group_data</a>(
    <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &GroupManager,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    key: &String,
    ctx: &TxContext,
): (String, String) {
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a>&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <b>let</b> m = <a href="../messaging/group_manager.md#messaging_group_manager_borrow_metadata_mut">group_manager::borrow_metadata_mut</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>, group);
    m.remove_data(key)
}
</code></pre>



</details>

<a name="messaging_messaging_assert_message_log_matches_group"></a>

## Function `assert_message_log_matches_group`



<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_message_log_matches_group">assert_message_log_matches_group</a>(log: &<a href="../messaging/message_log.md#messaging_message_log_MessageLog">messaging::message_log::MessageLog</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_message_log_matches_group">assert_message_log_matches_group</a>(log: &MessageLog, group: &PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;) {
    <b>assert</b>!(<a href="../messaging/message_log.md#messaging_message_log_group_id">message_log::group_id</a>(log) == object::id(group), <a href="../messaging/messaging.md#messaging_messaging_EMessageLogMismatch">EMessageLogMismatch</a>);
}
</code></pre>



</details>

<a name="messaging_messaging_assert_group_not_archived"></a>

## Function `assert_group_not_archived`



<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_group_not_archived">assert_group_not_archived</a>(group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_group_not_archived">assert_group_not_archived</a>(group: &PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;) {
    <b>assert</b>!(!group.is_paused(), <a href="../messaging/messaging.md#messaging_messaging_EGroupArchived">EGroupArchived</a>);
}
</code></pre>



</details>

<a name="messaging_messaging_send_paid_message_digest"></a>

## Function `send_paid_message_digest`

Escrow <code>escrow_amount</code> from <code>payment</code> for a paid message. Requires <code><a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a></code>.
Excess coin returns to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_send_paid_message_digest">send_paid_message_digest</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, log: &<b>mut</b> <a href="../messaging/message_log.md#messaging_message_log_MessageLog">messaging::message_log::MessageLog</a>, paid_registry: &<a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">messaging::paid_messaging_policy::PaidMessagingRegistry</a>, social_graph: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, block_list: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, recipient: <b>address</b>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, escrow_amount: u64, dedupe_key: vector&lt;u8&gt;, nonce: u128, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_send_paid_message_digest">send_paid_message_digest</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    group: &PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    log: &<b>mut</b> MessageLog,
    paid_registry: &PaidMessagingRegistry,
    social_graph: &SocialGraph,
    block_list: &BlockListRegistry,
    <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &GroupManager,
    recipient: <b>address</b>,
    payment: Coin&lt;MYSO&gt;,
    escrow_amount: u64,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <a href="../messaging/messaging.md#messaging_messaging_assert_group_not_archived">assert_group_not_archived</a>(group);
    <a href="../messaging/messaging.md#messaging_messaging_assert_message_log_matches_group">assert_message_log_matches_group</a>(log, group);
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a>&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <b>let</b> sender = ctx.sender();
    <a href="../messaging/messaging.md#messaging_messaging_assert_paid_open_allowed">assert_paid_open_allowed</a>(
        paid_registry,
        social_graph,
        block_list,
        <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>,
        group,
        log,
        sender,
        recipient,
        escrow_amount,
    );
    <a href="../messaging/message_log.md#messaging_message_log_send_paid_message">message_log::send_paid_message</a>(
        log,
        sender,
        recipient,
        payment,
        escrow_amount,
        dedupe_key,
        nonce,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="messaging_messaging_reply_to_paid_message_claim_coin"></a>

## Function `reply_to_paid_message_claim_coin`

Reply to a paid message and take full escrow as coin. Caller may split fees (e.g. via
[<code><a href="../messaging/messaging.md#messaging_messaging_reply_to_paid_message_claim_settled">reply_to_paid_message_claim_settled</a></code>]) or use this entry for custom routing.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_reply_to_paid_message_claim_coin">reply_to_paid_message_claim_coin</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, log: &<b>mut</b> <a href="../messaging/message_log.md#messaging_message_log_MessageLog">messaging::message_log::MessageLog</a>, block_list: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, paid_msg_seq: u64, char_count: u32, dedupe_key: vector&lt;u8&gt;, nonce: u128, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_reply_to_paid_message_claim_coin">reply_to_paid_message_claim_coin</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    group: &PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    log: &<b>mut</b> MessageLog,
    block_list: &BlockListRegistry,
    paid_msg_seq: u64,
    char_count: u32,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;MYSO&gt; {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <a href="../messaging/messaging.md#messaging_messaging_assert_group_not_archived">assert_group_not_archived</a>(group);
    <a href="../messaging/messaging.md#messaging_messaging_assert_message_log_matches_group">assert_message_log_matches_group</a>(log, group);
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a>&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <a href="../messaging/messaging.md#messaging_messaging_assert_paid_parties_not_blocked">assert_paid_parties_not_blocked</a>(block_list, ctx.sender(), log, paid_msg_seq);
    <a href="../messaging/message_log.md#messaging_message_log_reply_to_paid_message_claim_coin">message_log::reply_to_paid_message_claim_coin</a>(
        log,
        ctx.sender(),
        paid_msg_seq,
        char_count,
        dedupe_key,
        nonce,
        clock,
        ctx,
    )
}
</code></pre>



</details>

<a name="messaging_messaging_reply_to_paid_message_claim_settled"></a>

## Function `reply_to_paid_message_claim_settled`

Reply and settle: same validation as [<code><a href="../messaging/messaging.md#messaging_messaging_reply_to_paid_message_claim_coin">reply_to_paid_message_claim_coin</a></code>], then split escrow per
paid-message BPS to <code>platform_fee_recipient</code> and <code>ecosystem_fee_recipient</code> (typically addresses
matching <code>Platform</code> treasury policy and <code>EcosystemTreasury</code>), with net to the paid-message recipient.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_reply_to_paid_message_claim_settled">reply_to_paid_message_claim_settled</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, log: &<b>mut</b> <a href="../messaging/message_log.md#messaging_message_log_MessageLog">messaging::message_log::MessageLog</a>, block_list: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, paid_msg_seq: u64, char_count: u32, dedupe_key: vector&lt;u8&gt;, nonce: u128, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, platform_fee_recipient: <b>address</b>, ecosystem_fee_recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_reply_to_paid_message_claim_settled">reply_to_paid_message_claim_settled</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    group: &PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    log: &<b>mut</b> MessageLog,
    block_list: &BlockListRegistry,
    paid_msg_seq: u64,
    char_count: u32,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    clock: &Clock,
    platform_fee_recipient: <b>address</b>,
    ecosystem_fee_recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <a href="../messaging/messaging.md#messaging_messaging_assert_group_not_archived">assert_group_not_archived</a>(group);
    <a href="../messaging/messaging.md#messaging_messaging_assert_message_log_matches_group">assert_message_log_matches_group</a>(log, group);
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a>&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <a href="../messaging/messaging.md#messaging_messaging_assert_paid_parties_not_blocked">assert_paid_parties_not_blocked</a>(block_list, ctx.sender(), log, paid_msg_seq);
    <a href="../messaging/message_log.md#messaging_message_log_reply_to_paid_message_claim_settled">message_log::reply_to_paid_message_claim_settled</a>(
        log,
        ctx.sender(),
        paid_msg_seq,
        char_count,
        dedupe_key,
        nonce,
        clock,
        platform_fee_recipient,
        ecosystem_fee_recipient,
        ctx,
    );
}
</code></pre>



</details>

<a name="messaging_messaging_refund_paid_escrow"></a>

## Function `refund_paid_escrow`

Refund expired paid escrow to the payer. Requires <code><a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a></code> (payer must be a member).


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_refund_paid_escrow">refund_paid_escrow</a>(<a href="../messaging/version.md#messaging_version">version</a>: &<a href="../messaging/version.md#messaging_version_Version">messaging::version::Version</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, log: &<b>mut</b> <a href="../messaging/message_log.md#messaging_message_log_MessageLog">messaging::message_log::MessageLog</a>, block_list: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, paid_msg_seq: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_refund_paid_escrow">refund_paid_escrow</a>(
    <a href="../messaging/version.md#messaging_version">version</a>: &Version,
    group: &PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    log: &<b>mut</b> MessageLog,
    block_list: &BlockListRegistry,
    paid_msg_seq: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../messaging/version.md#messaging_version">version</a>.validate_version();
    <a href="../messaging/messaging.md#messaging_messaging_assert_group_not_archived">assert_group_not_archived</a>(group);
    <a href="../messaging/messaging.md#messaging_messaging_assert_message_log_matches_group">assert_message_log_matches_group</a>(log, group);
    <b>assert</b>!(group.has_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a>&gt;(ctx.sender()), <a href="../messaging/messaging.md#messaging_messaging_ENotPermitted">ENotPermitted</a>);
    <b>let</b> (payer, recipient) = <a href="../messaging/message_log.md#messaging_message_log_paid_message_parties">message_log::paid_message_parties</a>(log, paid_msg_seq);
    block_list::assert_not_blocked(block_list, payer, recipient);
    <a href="../messaging/message_log.md#messaging_message_log_refund_paid_message">message_log::refund_paid_message</a>(log, ctx.sender(), paid_msg_seq, clock, ctx);
}
</code></pre>



</details>

<a name="messaging_messaging_grant_all_messaging_permissions"></a>

## Function `grant_all_messaging_permissions`

Grants all messaging permissions to a member.
<code><a href="../messaging/messaging.md#messaging_messaging_MessagingDeleter">MessagingDeleter</a></code>, <code>EncryptionKeyRotator</code>, <code><a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a></code>, <code><a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a></code>.


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_grant_all_messaging_permissions">grant_all_messaging_permissions</a>(group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, member: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_grant_all_messaging_permissions">grant_all_messaging_permissions</a>(
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    member: <b>address</b>,
    ctx: &TxContext,
) {
    group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MessagingSender">MessagingSender</a>&gt;(member, ctx);
    group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MessagingReader">MessagingReader</a>&gt;(member, ctx);
    group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MessagingEditor">MessagingEditor</a>&gt;(member, ctx);
    group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MessagingDeleter">MessagingDeleter</a>&gt;(member, ctx);
    group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, EncryptionKeyRotator&gt;(member, ctx);
    group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_GroupHandleAdmin">GroupHandleAdmin</a>&gt;(member, ctx);
    group.grant_permission&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>, <a href="../messaging/messaging.md#messaging_messaging_MetadataAdmin">MetadataAdmin</a>&gt;(member, ctx);
}
</code></pre>



</details>

<a name="messaging_messaging_assert_peers_not_blocked"></a>

## Function `assert_peers_not_blocked`



<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_peers_not_blocked">assert_peers_not_blocked</a>(block_list: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, creator: <b>address</b>, members: &<a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_peers_not_blocked">assert_peers_not_blocked</a>(
    block_list: &BlockListRegistry,
    creator: <b>address</b>,
    members: &VecSet&lt;<b>address</b>&gt;,
) {
    <b>let</b> keys = members.keys();
    <b>let</b> len = vector::length(keys);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> member = *vector::borrow(keys, i);
        <b>if</b> (member != creator) {
            block_list::assert_not_blocked(block_list, creator, member);
        };
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="messaging_messaging_count_non_creator_peers"></a>

## Function `count_non_creator_peers`



<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_count_non_creator_peers">count_non_creator_peers</a>(members: &<a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;<b>address</b>&gt;, creator: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_count_non_creator_peers">count_non_creator_peers</a>(members: &VecSet&lt;<b>address</b>&gt;, creator: <b>address</b>): u64 {
    <b>let</b> keys = members.keys();
    <b>let</b> len = vector::length(keys);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> count = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> member = *vector::borrow(keys, i);
        <b>if</b> (member != creator) {
            count = count + 1;
        };
        i = i + 1;
    };
    count
}
</code></pre>



</details>

<a name="messaging_messaging_is_direct_message_group"></a>

## Function `is_direct_message_group`



<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_is_direct_message_group">is_direct_message_group</a>(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_is_direct_message_group">is_direct_message_group</a>(
    <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &GroupManager,
    group: &PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
): bool {
    <b>let</b> m = <a href="../messaging/group_manager.md#messaging_group_manager_borrow_metadata">group_manager::borrow_metadata</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>, group);
    <b>let</b> key = string::utf8(<a href="../messaging/messaging.md#messaging_messaging_CONVERSATION_KIND_KEY">CONVERSATION_KIND_KEY</a>);
    <b>let</b> maybe_value = <a href="../messaging/metadata.md#messaging_metadata_get_data_value">metadata::get_data_value</a>(m, &key);
    <b>if</b> (option::is_some(&maybe_value)) {
        *option::borrow(&maybe_value) == string::utf8(<a href="../messaging/messaging.md#messaging_messaging_CONVERSATION_KIND_DM">CONVERSATION_KIND_DM</a>)
    } <b>else</b> {
        <b>false</b>
    }
}
</code></pre>



</details>

<a name="messaging_messaging_assert_paid_open_allowed"></a>

## Function `assert_paid_open_allowed`



<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_paid_open_allowed">assert_paid_open_allowed</a>(paid_registry: &<a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">messaging::paid_messaging_policy::PaidMessagingRegistry</a>, social_graph: &<a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, block_list: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &<a href="../messaging/group_manager.md#messaging_group_manager_GroupManager">messaging::group_manager::GroupManager</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">messaging::messaging::Messaging</a>&gt;, log: &<a href="../messaging/message_log.md#messaging_message_log_MessageLog">messaging::message_log::MessageLog</a>, sender: <b>address</b>, recipient: <b>address</b>, escrow_amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_paid_open_allowed">assert_paid_open_allowed</a>(
    paid_registry: &PaidMessagingRegistry,
    social_graph: &SocialGraph,
    block_list: &BlockListRegistry,
    <a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>: &GroupManager,
    group: &PermissionedGroup&lt;<a href="../messaging/messaging.md#messaging_messaging_Messaging">Messaging</a>&gt;,
    log: &MessageLog,
    sender: <b>address</b>,
    recipient: <b>address</b>,
    escrow_amount: u64,
) {
    block_list::assert_not_blocked(block_list, sender, recipient);
    <b>if</b> (!<a href="../messaging/messaging.md#messaging_messaging_is_direct_message_group">is_direct_message_group</a>(<a href="../messaging/group_manager.md#messaging_group_manager">group_manager</a>, group)) {
        <b>return</b>
    };
    <b>if</b> (<a href="../messaging/message_log.md#messaging_message_log_next_seq">message_log::next_seq</a>(log) != 0) {
        <b>return</b>
    };
    <b>if</b> (social_graph::is_following(social_graph, sender, recipient)) {
        <b>abort</b> <a href="../messaging/messaging.md#messaging_messaging_EPaidNotRequiredForFollower">EPaidNotRequiredForFollower</a>
    };
    <b>let</b> min_cost = <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_requires_payment_from">paid_messaging_policy::requires_payment_from</a>(paid_registry, recipient);
    <b>if</b> (option::is_none(&min_cost)) {
        <b>return</b>
    };
    <b>assert</b>!(
        escrow_amount &gt;= *option::borrow(&min_cost),
        <a href="../messaging/messaging.md#messaging_messaging_EBelowMinMessageCost">EBelowMinMessageCost</a>,
    );
}
</code></pre>



</details>

<a name="messaging_messaging_assert_paid_parties_not_blocked"></a>

## Function `assert_paid_parties_not_blocked`



<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_paid_parties_not_blocked">assert_paid_parties_not_blocked</a>(block_list: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, caller: <b>address</b>, log: &<a href="../messaging/message_log.md#messaging_message_log_MessageLog">messaging::message_log::MessageLog</a>, paid_msg_seq: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../messaging/messaging.md#messaging_messaging_assert_paid_parties_not_blocked">assert_paid_parties_not_blocked</a>(
    block_list: &BlockListRegistry,
    caller: <b>address</b>,
    log: &MessageLog,
    paid_msg_seq: u64,
) {
    <b>let</b> (payer, recipient) = <a href="../messaging/message_log.md#messaging_message_log_paid_message_parties">message_log::paid_message_parties</a>(log, paid_msg_seq);
    <b>if</b> (caller == payer) {
        block_list::assert_not_blocked(block_list, caller, recipient);
    } <b>else</b> <b>if</b> (caller == recipient) {
        block_list::assert_not_blocked(block_list, caller, payer);
    } <b>else</b> {
        block_list::assert_not_blocked(block_list, caller, payer);
        block_list::assert_not_blocked(block_list, caller, recipient);
    };
}
</code></pre>



</details>
