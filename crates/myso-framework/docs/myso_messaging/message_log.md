---
title: Module `myso_messaging::message_log`
---

Per-group **paid message escrow** only (<code>MYSO</code>). Free messaging, digests, reactions,
pins, and receipts live off-chain (relayer / clients).

Authorization is enforced in <code><a href="../myso_messaging/messaging.md#myso_messaging_messaging">messaging</a></code>; this module holds escrow state and invariants.


-  [Struct `MessageLogTag`](#myso_messaging_message_log_MessageLogTag)
-  [Struct `PaidMessageEscrow`](#myso_messaging_message_log_PaidMessageEscrow)
-  [Struct `MessageLog`](#myso_messaging_message_log_MessageLog)
-  [Struct `MessageLogCreated`](#myso_messaging_message_log_MessageLogCreated)
-  [Struct `PaidMessageSent`](#myso_messaging_message_log_PaidMessageSent)
-  [Struct `PaidMessageReplied`](#myso_messaging_message_log_PaidMessageReplied)
-  [Struct `PaymentClaimed`](#myso_messaging_message_log_PaymentClaimed)
-  [Struct `PaymentClaimedSettled`](#myso_messaging_message_log_PaymentClaimedSettled)
-  [Struct `PaymentRefunded`](#myso_messaging_message_log_PaymentRefunded)
-  [Constants](#@Constants_0)
-  [Function `new`](#myso_messaging_message_log_new)
-  [Function `group_id`](#myso_messaging_message_log_group_id)
-  [Function `uuid`](#myso_messaging_message_log_uuid)
-  [Function `next_seq`](#myso_messaging_message_log_next_seq)
-  [Function `consume_dedupe_and_nonce`](#myso_messaging_message_log_consume_dedupe_and_nonce)
-  [Function `send_paid_message`](#myso_messaging_message_log_send_paid_message)
-  [Function `reply_to_paid_message_claim_coin`](#myso_messaging_message_log_reply_to_paid_message_claim_coin)
-  [Function `reply_to_paid_message_claim_settled`](#myso_messaging_message_log_reply_to_paid_message_claim_settled)
-  [Function `refund_paid_message`](#myso_messaging_message_log_refund_paid_message)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
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
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../myso_messaging/paid_escrow_settlement.md#myso_messaging_paid_escrow_settlement">myso_messaging::paid_escrow_settlement</a>;
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



<a name="myso_messaging_message_log_MessageLogTag"></a>

## Struct `MessageLogTag`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLogTag">MessageLogTag</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>0: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_messaging_message_log_PaidMessageEscrow"></a>

## Struct `PaidMessageEscrow`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaidMessageEscrow">PaidMessageEscrow</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>payer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>escrowed_balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_messaging_message_log_MessageLog"></a>

## Struct `MessageLog`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a> <b>has</b> key, store
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
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_next_seq">next_seq</a>: u64</code>
</dt>
<dd>
 Monotonic id for each paid send (<code>seq</code> indexes <code>paid_msg_escrow</code>).
</dd>
<dt>
<code>used_dedupe: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;vector&lt;u8&gt;, bool&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>nonces: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u128, bool&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>paid_msg_escrow: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u64, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaidMessageEscrow">myso_messaging::message_log::PaidMessageEscrow</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_messaging_message_log_MessageLogCreated"></a>

## Struct `MessageLogCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLogCreated">MessageLogCreated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_log_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_messaging_message_log_PaidMessageSent"></a>

## Struct `PaidMessageSent`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaidMessageSent">PaidMessageSent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>payer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_messaging_message_log_PaidMessageReplied"></a>

## Struct `PaidMessageReplied`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaidMessageReplied">PaidMessageReplied</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>paid_msg_seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reply_char_count: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_messaging_message_log_PaymentClaimed"></a>

## Struct `PaymentClaimed`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaymentClaimed">PaymentClaimed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_messaging_message_log_PaymentClaimedSettled"></a>

## Struct `PaymentClaimedSettled`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaymentClaimedSettled">PaymentClaimedSettled</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>total_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>net_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee_recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee_recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="myso_messaging_message_log_PaymentRefunded"></a>

## Struct `PaymentRefunded`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaymentRefunded">PaymentRefunded</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>payer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>refunded_at_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_messaging_message_log_EMessageLogExists"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EMessageLogExists">EMessageLogExists</a>: u64 = 0;
</code></pre>



<a name="myso_messaging_message_log_EDedupeUsed"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EDedupeUsed">EDedupeUsed</a>: u64 = 1;
</code></pre>



<a name="myso_messaging_message_log_ENonceUsed"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_ENonceUsed">ENonceUsed</a>: u64 = 2;
</code></pre>



<a name="myso_messaging_message_log_EForbidden"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EForbidden">EForbidden</a>: u64 = 5;
</code></pre>



<a name="myso_messaging_message_log_EInsufficientPayment"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EInsufficientPayment">EInsufficientPayment</a>: u64 = 12;
</code></pre>



<a name="myso_messaging_message_log_EPaidNotFound"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaidNotFound">EPaidNotFound</a>: u64 = 6;
</code></pre>



<a name="myso_messaging_message_log_EPaymentExpired"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentExpired">EPaymentExpired</a>: u64 = 7;
</code></pre>



<a name="myso_messaging_message_log_EPaymentClaimed"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentClaimed">EPaymentClaimed</a>: u64 = 8;
</code></pre>



<a name="myso_messaging_message_log_EReplyTooShort"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EReplyTooShort">EReplyTooShort</a>: u64 = 9;
</code></pre>



<a name="myso_messaging_message_log_EDedupeKeyTooLong"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EDedupeKeyTooLong">EDedupeKeyTooLong</a>: u64 = 10;
</code></pre>



<a name="myso_messaging_message_log_EVaultEmpty"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EVaultEmpty">EVaultEmpty</a>: u64 = 11;
</code></pre>



<a name="myso_messaging_message_log_MIN_REPLY_CHARS"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MIN_REPLY_CHARS">MIN_REPLY_CHARS</a>: u32 = 6;
</code></pre>



<a name="myso_messaging_message_log_PAYMENT_EXPIRATION_MS"></a>

Paid message must be replied to within this wall-clock window (<code>Clock</code> ms).


<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PAYMENT_EXPIRATION_MS">PAYMENT_EXPIRATION_MS</a>: u64 = 2592000000;
</code></pre>



<a name="myso_messaging_message_log_MAX_DEDUPE_KEY_BYTES"></a>



<pre><code><b>const</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MAX_DEDUPE_KEY_BYTES">MAX_DEDUPE_KEY_BYTES</a>: u64 = 256;
</code></pre>



<a name="myso_messaging_message_log_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_new">new</a>(namespace_uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">myso_messaging::message_log::MessageLog</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_new">new</a>(
    namespace_uid: &<b>mut</b> UID,
    <a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>: String,
    <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: ID,
    ctx: &<b>mut</b> TxContext,
): <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a> {
    <b>assert</b>!(
        !derived_object::exists(namespace_uid, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLogTag">MessageLogTag</a>(<a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>)),
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EMessageLogExists">EMessageLogExists</a>,
    );
    <b>let</b> log = <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a> {
        id: derived_object::claim(namespace_uid, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLogTag">MessageLogTag</a>(<a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>)),
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>,
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>,
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_next_seq">next_seq</a>: 0,
        used_dedupe: table::new(ctx),
        nonces: table::new(ctx),
        paid_msg_escrow: table::new(ctx),
    };
    event::emit(<a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLogCreated">MessageLogCreated</a> {
        message_log_id: object::id(&log),
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>,
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>: log.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>,
    });
    log
}
</code></pre>



</details>

<a name="myso_messaging_message_log_group_id"></a>

## Function `group_id`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>(self: &<a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">myso_messaging::message_log::MessageLog</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>(self: &<a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a>): ID {
    self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>
}
</code></pre>



</details>

<a name="myso_messaging_message_log_uuid"></a>

## Function `uuid`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>(self: &<a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">myso_messaging::message_log::MessageLog</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>(self: &<a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a>): String {
    self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_uuid">uuid</a>
}
</code></pre>



</details>

<a name="myso_messaging_message_log_next_seq"></a>

## Function `next_seq`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_next_seq">next_seq</a>(self: &<a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">myso_messaging::message_log::MessageLog</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_next_seq">next_seq</a>(self: &<a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a>): u64 {
    self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_next_seq">next_seq</a>
}
</code></pre>



</details>

<a name="myso_messaging_message_log_consume_dedupe_and_nonce"></a>

## Function `consume_dedupe_and_nonce`



<pre><code><b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_consume_dedupe_and_nonce">consume_dedupe_and_nonce</a>(self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">myso_messaging::message_log::MessageLog</a>, sender: <b>address</b>, dedupe_key: vector&lt;u8&gt;, nonce: u128, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_consume_dedupe_and_nonce">consume_dedupe_and_nonce</a>(
    self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a>,
    sender: <b>address</b>,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(dedupe_key.length() &lt;= <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MAX_DEDUPE_KEY_BYTES">MAX_DEDUPE_KEY_BYTES</a>, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EDedupeKeyTooLong">EDedupeKeyTooLong</a>);
    <b>assert</b>!(!table::contains(&self.used_dedupe, dedupe_key), <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EDedupeUsed">EDedupeUsed</a>);
    table::add(&<b>mut</b> self.used_dedupe, dedupe_key, <b>true</b>);
    <b>if</b> (!table::contains(&self.nonces, sender)) {
        table::add(&<b>mut</b> self.nonces, sender, table::new(ctx));
    };
    <b>let</b> member_nonces = table::borrow_mut(&<b>mut</b> self.nonces, sender);
    <b>assert</b>!(!table::contains(member_nonces, nonce), <a href="../myso_messaging/message_log.md#myso_messaging_message_log_ENonceUsed">ENonceUsed</a>);
    table::add(member_nonces, nonce, <b>true</b>);
}
</code></pre>



</details>

<a name="myso_messaging_message_log_send_paid_message"></a>

## Function `send_paid_message`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_send_paid_message">send_paid_message</a>(self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">myso_messaging::message_log::MessageLog</a>, sender: <b>address</b>, recipient: <b>address</b>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, escrow_amount: u64, dedupe_key: vector&lt;u8&gt;, nonce: u128, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_send_paid_message">send_paid_message</a>(
    self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a>,
    sender: <b>address</b>,
    recipient: <b>address</b>,
    <b>mut</b> payment: Coin&lt;MYSO&gt;,
    escrow_amount: u64,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(coin::value(&payment) &gt;= escrow_amount, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EInsufficientPayment">EInsufficientPayment</a>);
    <a href="../myso_messaging/message_log.md#myso_messaging_message_log_consume_dedupe_and_nonce">consume_dedupe_and_nonce</a>(self, sender, dedupe_key, nonce, ctx);
    <b>let</b> seq = self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_next_seq">next_seq</a>;
    self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_next_seq">next_seq</a> = seq + 1;
    <b>let</b> escrow_payment = coin::split(&<b>mut</b> payment, escrow_amount, ctx);
    <b>let</b> escrow_balance = coin::into_balance(escrow_payment);
    <b>let</b> created_at_ms = clock::timestamp_ms(clock);
    <b>let</b> escrow = <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaidMessageEscrow">PaidMessageEscrow</a> {
        payer: sender,
        recipient,
        amount: escrow_amount,
        escrowed_balance: escrow_balance,
        created_at_ms,
        claimed: <b>false</b>,
    };
    table::add(&<b>mut</b> self.paid_msg_escrow, seq, escrow);
    event::emit(<a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaidMessageSent">PaidMessageSent</a> {
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>,
        seq,
        payer: sender,
        recipient,
        amount: escrow_amount,
        created_at_ms,
    });
    <b>let</b> pv = coin::value(&payment);
    <b>if</b> (pv &gt; 0) {
        transfer::public_transfer(payment, sender);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
}
</code></pre>



</details>

<a name="myso_messaging_message_log_reply_to_paid_message_claim_coin"></a>

## Function `reply_to_paid_message_claim_coin`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_reply_to_paid_message_claim_coin">reply_to_paid_message_claim_coin</a>(self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">myso_messaging::message_log::MessageLog</a>, sender: <b>address</b>, paid_msg_seq: u64, char_count: u32, dedupe_key: vector&lt;u8&gt;, nonce: u128, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_reply_to_paid_message_claim_coin">reply_to_paid_message_claim_coin</a>(
    self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a>,
    sender: <b>address</b>,
    paid_msg_seq: u64,
    char_count: u32,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;MYSO&gt; {
    <b>assert</b>!(table::contains(&self.paid_msg_escrow, paid_msg_seq), <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaidNotFound">EPaidNotFound</a>);
    <b>let</b> escrow_ref = table::borrow(&self.paid_msg_escrow, paid_msg_seq);
    <b>assert</b>!(sender == escrow_ref.recipient, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EForbidden">EForbidden</a>);
    <b>assert</b>!(!escrow_ref.claimed, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentClaimed">EPaymentClaimed</a>);
    <b>let</b> now_ms = clock::timestamp_ms(clock);
    <b>assert</b>!(now_ms - escrow_ref.created_at_ms &lt;= <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PAYMENT_EXPIRATION_MS">PAYMENT_EXPIRATION_MS</a>, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentExpired">EPaymentExpired</a>);
    <b>assert</b>!(char_count &gt;= <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MIN_REPLY_CHARS">MIN_REPLY_CHARS</a>, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EReplyTooShort">EReplyTooShort</a>);
    <a href="../myso_messaging/message_log.md#myso_messaging_message_log_consume_dedupe_and_nonce">consume_dedupe_and_nonce</a>(self, sender, dedupe_key, nonce, ctx);
    event::emit(<a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaidMessageReplied">PaidMessageReplied</a> {
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>,
        paid_msg_seq,
        recipient: sender,
        reply_char_count: char_count,
    });
    <b>let</b> escrow = table::borrow_mut(&<b>mut</b> self.paid_msg_escrow, paid_msg_seq);
    <b>assert</b>!(!escrow.claimed, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentClaimed">EPaymentClaimed</a>);
    escrow.claimed = <b>true</b>;
    <b>let</b> total_amount = escrow.amount;
    event::emit(<a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaymentClaimed">PaymentClaimed</a> {
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>,
        seq: paid_msg_seq,
        recipient: escrow.recipient,
        amount: total_amount,
        claimed_at_ms: clock::timestamp_ms(clock),
    });
    <b>let</b> coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> escrow.escrowed_balance), ctx);
    <b>assert</b>!(coin::value(&coin) &gt; 0, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EVaultEmpty">EVaultEmpty</a>);
    coin
}
</code></pre>



</details>

<a name="myso_messaging_message_log_reply_to_paid_message_claim_settled"></a>

## Function `reply_to_paid_message_claim_settled`

Same as [<code><a href="../myso_messaging/message_log.md#myso_messaging_message_log_reply_to_paid_message_claim_coin">reply_to_paid_message_claim_coin</a></code>], then splits escrow per paid-message BPS to
<code>platform_fee_recipient</code>, <code>ecosystem_fee_recipient</code>, and the original paid-message recipient.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_reply_to_paid_message_claim_settled">reply_to_paid_message_claim_settled</a>(self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">myso_messaging::message_log::MessageLog</a>, sender: <b>address</b>, paid_msg_seq: u64, char_count: u32, dedupe_key: vector&lt;u8&gt;, nonce: u128, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, platform_fee_recipient: <b>address</b>, ecosystem_fee_recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_reply_to_paid_message_claim_settled">reply_to_paid_message_claim_settled</a>(
    self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a>,
    sender: <b>address</b>,
    paid_msg_seq: u64,
    char_count: u32,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    clock: &Clock,
    platform_fee_recipient: <b>address</b>,
    ecosystem_fee_recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(table::contains(&self.paid_msg_escrow, paid_msg_seq), <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaidNotFound">EPaidNotFound</a>);
    <b>let</b> escrow_ref = table::borrow(&self.paid_msg_escrow, paid_msg_seq);
    <b>assert</b>!(sender == escrow_ref.recipient, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EForbidden">EForbidden</a>);
    <b>assert</b>!(!escrow_ref.claimed, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentClaimed">EPaymentClaimed</a>);
    <b>let</b> now_ms = clock::timestamp_ms(clock);
    <b>assert</b>!(now_ms - escrow_ref.created_at_ms &lt;= <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PAYMENT_EXPIRATION_MS">PAYMENT_EXPIRATION_MS</a>, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentExpired">EPaymentExpired</a>);
    <b>assert</b>!(char_count &gt;= <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MIN_REPLY_CHARS">MIN_REPLY_CHARS</a>, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EReplyTooShort">EReplyTooShort</a>);
    <a href="../myso_messaging/message_log.md#myso_messaging_message_log_consume_dedupe_and_nonce">consume_dedupe_and_nonce</a>(self, sender, dedupe_key, nonce, ctx);
    event::emit(<a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaidMessageReplied">PaidMessageReplied</a> {
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>,
        paid_msg_seq,
        recipient: sender,
        reply_char_count: char_count,
    });
    <b>let</b> escrow = table::borrow_mut(&<b>mut</b> self.paid_msg_escrow, paid_msg_seq);
    <b>assert</b>!(!escrow.claimed, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentClaimed">EPaymentClaimed</a>);
    escrow.claimed = <b>true</b>;
    <b>let</b> primary_recipient = escrow.recipient;
    <b>let</b> total_amount = escrow.amount;
    <b>let</b> coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> escrow.escrowed_balance), ctx);
    <b>assert</b>!(coin::value(&coin) &gt; 0, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EVaultEmpty">EVaultEmpty</a>);
    <b>let</b> totals = escrow_fees::distribute_escrow_to_recipients(
        coin,
        platform_fee_recipient,
        ecosystem_fee_recipient,
        primary_recipient,
        ctx,
    );
    event::emit(<a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaymentClaimedSettled">PaymentClaimedSettled</a> {
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>,
        seq: paid_msg_seq,
        recipient: primary_recipient,
        total_amount,
        platform_fee: escrow_fees::platform_fee(&totals),
        treasury_fee: escrow_fees::treasury_fee(&totals),
        net_amount: escrow_fees::net_amount(&totals),
        platform_fee_recipient,
        ecosystem_fee_recipient,
        claimed_at_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="myso_messaging_message_log_refund_paid_message"></a>

## Function `refund_paid_message`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_refund_paid_message">refund_paid_message</a>(self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">myso_messaging::message_log::MessageLog</a>, sender: <b>address</b>, paid_msg_seq: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_refund_paid_message">refund_paid_message</a>(
    self: &<b>mut</b> <a href="../myso_messaging/message_log.md#myso_messaging_message_log_MessageLog">MessageLog</a>,
    sender: <b>address</b>,
    paid_msg_seq: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(table::contains(&self.paid_msg_escrow, paid_msg_seq), <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaidNotFound">EPaidNotFound</a>);
    <b>let</b> escrow = table::borrow_mut(&<b>mut</b> self.paid_msg_escrow, paid_msg_seq);
    <b>assert</b>!(sender == escrow.payer, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EForbidden">EForbidden</a>);
    <b>assert</b>!(!escrow.claimed, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentClaimed">EPaymentClaimed</a>);
    <b>let</b> now_ms = clock::timestamp_ms(clock);
    <b>assert</b>!(now_ms - escrow.created_at_ms &gt;= <a href="../myso_messaging/message_log.md#myso_messaging_message_log_PAYMENT_EXPIRATION_MS">PAYMENT_EXPIRATION_MS</a>, <a href="../myso_messaging/message_log.md#myso_messaging_message_log_EPaymentExpired">EPaymentExpired</a>);
    <b>let</b> refund_amount = escrow.amount;
    <b>let</b> payer = escrow.payer;
    escrow.claimed = <b>true</b>;
    <b>let</b> refund_coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> escrow.escrowed_balance), ctx);
    event::emit(<a href="../myso_messaging/message_log.md#myso_messaging_message_log_PaymentRefunded">PaymentRefunded</a> {
        <a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>: self.<a href="../myso_messaging/message_log.md#myso_messaging_message_log_group_id">group_id</a>,
        seq: paid_msg_seq,
        payer,
        amount: refund_amount,
        refunded_at_ms: now_ms,
    });
    transfer::public_transfer(refund_coin, payer);
}
</code></pre>



</details>
