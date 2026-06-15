---
title: Module `messaging::paid_messaging_policy`
---

Per-wallet paid DM policy for the messaging package.

Stored separately from social profiles: keyed by wallet address, sparse table
(only wallets that opt in have a row).


-  [Struct `PaidMessagingPolicy`](#messaging_paid_messaging_policy_PaidMessagingPolicy)
-  [Struct `PaidMessagingRegistry`](#messaging_paid_messaging_policy_PaidMessagingRegistry)
-  [Struct `PaidMessagingPolicyUpdated`](#messaging_paid_messaging_policy_PaidMessagingPolicyUpdated)
-  [Constants](#@Constants_0)
-  [Function `new`](#messaging_paid_messaging_policy_new)
-  [Function `share`](#messaging_paid_messaging_policy_share)
-  [Function `set_paid_messaging_policy`](#messaging_paid_messaging_policy_set_paid_messaging_policy)
-  [Function `requires_payment_from`](#messaging_paid_messaging_policy_requires_payment_from)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
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



<a name="messaging_paid_messaging_policy_PaidMessagingPolicy"></a>

## Struct `PaidMessagingPolicy`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingPolicy">PaidMessagingPolicy</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>min_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="messaging_paid_messaging_policy_PaidMessagingRegistry"></a>

## Struct `PaidMessagingRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">PaidMessagingRegistry</a> <b>has</b> key
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
<code>policies: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingPolicy">messaging::paid_messaging_policy::PaidMessagingPolicy</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="messaging_paid_messaging_policy_PaidMessagingPolicyUpdated"></a>

## Struct `PaidMessagingPolicyUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingPolicyUpdated">PaidMessagingPolicyUpdated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>wallet: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>enabled: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>min_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="messaging_paid_messaging_policy_EInvalidPolicy"></a>



<pre><code><b>const</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_EInvalidPolicy">EInvalidPolicy</a>: u64 = 0;
</code></pre>



<a name="messaging_paid_messaging_policy_PAID_MESSAGING_REGISTRY_DERIVATION_KEY"></a>



<pre><code><b>const</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PAID_MESSAGING_REGISTRY_DERIVATION_KEY">PAID_MESSAGING_REGISTRY_DERIVATION_KEY</a>: vector&lt;u8&gt; = vector[112, 97, 105, 100, 95, 109, 101, 115, 115, 97, 103, 105, 110, 103, 95, 114, 101, 103, 105, 115, 116, 114, 121];
</code></pre>



<a name="messaging_paid_messaging_policy_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_new">new</a>(namespace_uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">messaging::paid_messaging_policy::PaidMessagingRegistry</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_new">new</a>(namespace_uid: &<b>mut</b> UID, ctx: &<b>mut</b> TxContext): <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">PaidMessagingRegistry</a> {
    <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">PaidMessagingRegistry</a> {
        id: derived_object::claim(
            namespace_uid,
            <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PAID_MESSAGING_REGISTRY_DERIVATION_KEY">PAID_MESSAGING_REGISTRY_DERIVATION_KEY</a>.to_string(),
        ),
        policies: table::new(ctx),
    }
}
</code></pre>



</details>

<a name="messaging_paid_messaging_policy_share"></a>

## Function `share`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_share">share</a>(self: <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">messaging::paid_messaging_policy::PaidMessagingRegistry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_share">share</a>(self: <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">PaidMessagingRegistry</a>) {
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="messaging_paid_messaging_policy_set_paid_messaging_policy"></a>

## Function `set_paid_messaging_policy`

Sets paid DM policy for the transaction sender's wallet.

When <code>enabled</code> is true, <code>min_cost</code> must be set (enforced on stranger 1:1 paid opens).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_set_paid_messaging_policy">set_paid_messaging_policy</a>(registry: &<b>mut</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">messaging::paid_messaging_policy::PaidMessagingRegistry</a>, enabled: bool, min_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_set_paid_messaging_policy">set_paid_messaging_policy</a>(
    registry: &<b>mut</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">PaidMessagingRegistry</a>,
    enabled: bool,
    min_cost: Option&lt;u64&gt;,
    ctx: &TxContext,
) {
    <b>let</b> wallet = ctx.sender();
    <b>if</b> (enabled) {
        <b>assert</b>!(option::is_some(&min_cost), <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_EInvalidPolicy">EInvalidPolicy</a>);
    };
    <b>let</b> policy = <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingPolicy">PaidMessagingPolicy</a> { enabled, min_cost };
    <b>if</b> (table::contains(&registry.policies, wallet)) {
        *table::borrow_mut(&<b>mut</b> registry.policies, wallet) = policy;
    } <b>else</b> {
        table::add(&<b>mut</b> registry.policies, wallet, policy);
    };
    event::emit(<a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingPolicyUpdated">PaidMessagingPolicyUpdated</a> { wallet, enabled, min_cost });
}
</code></pre>



</details>

<a name="messaging_paid_messaging_policy_requires_payment_from"></a>

## Function `requires_payment_from`

Returns <code>Some(min_cost)</code> when the recipient requires paid stranger DMs.


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_requires_payment_from">requires_payment_from</a>(registry: &<a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">messaging::paid_messaging_policy::PaidMessagingRegistry</a>, recipient: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_requires_payment_from">requires_payment_from</a>(
    registry: &<a href="../messaging/paid_messaging_policy.md#messaging_paid_messaging_policy_PaidMessagingRegistry">PaidMessagingRegistry</a>,
    recipient: <b>address</b>,
): Option&lt;u64&gt; {
    <b>if</b> (!table::contains(&registry.policies, recipient)) {
        <b>return</b> option::none()
    };
    <b>let</b> policy = table::borrow(&registry.policies, recipient);
    <b>if</b> (policy.enabled && option::is_some(&policy.min_cost)) {
        policy.min_cost
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>
