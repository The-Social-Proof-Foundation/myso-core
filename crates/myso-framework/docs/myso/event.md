---
title: Module `myso::event`
---

Events module. Defines the <code><a href="../myso/event.md#myso_event_emit">myso::event::emit</a></code> function which
creates and sends a custom MoveEvent as a part of the effects
certificate of the transaction.

Every MoveEvent has the following properties:
- sender
- type signature (<code>T</code>)
- event data (the value of <code>T</code>)
- timestamp (local to a node)
- transaction digest

Example:
```
module my::marketplace {
use myso::event;
/* ... */
struct ItemPurchased has copy, drop {
item_id: ID, buyer: address
}
entry fun buy(/* .... */) {
/* ... */
event::emit(ItemPurchased { item_id: ..., buyer: .... })
}
}
```


-  [Function `emit`](#myso_event_emit)
-  [Function `emit_authenticated`](#myso_event_emit_authenticated)
-  [Function `emit_authenticated_impl`](#myso_event_emit_authenticated_impl)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
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



<a name="myso_event_emit"></a>

## Function `emit`

Emit a custom Move event, sending the data offchain.

Used for creating custom indexes and tracking onchain
activity in a way that mysots a specific application the most.

The type <code>T</code> is the main way to index the event, and can contain
phantom parameters, eg <code><a href="../myso/event.md#myso_event_emit">emit</a>(MyEvent&lt;<b>phantom</b> T&gt;)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/event.md#myso_event_emit">emit</a>&lt;T: <b>copy</b>, drop&gt;(<a href="../myso/event.md#myso_event">event</a>: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../myso/event.md#myso_event_emit">emit</a>&lt;T: <b>copy</b> + drop&gt;(<a href="../myso/event.md#myso_event">event</a>: T);
</code></pre>



</details>

<a name="myso_event_emit_authenticated"></a>

## Function `emit_authenticated`

Emits a custom Move event which can be authenticated by a light client.

This method emits the authenticated event to the event stream for the Move package that
defines the event type <code>T</code>.
Only the package that defines the type <code>T</code> can emit authenticated events to this stream.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/event.md#myso_event_emit_authenticated">emit_authenticated</a>&lt;T: <b>copy</b>, drop&gt;(<a href="../myso/event.md#myso_event">event</a>: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/event.md#myso_event_emit_authenticated">emit_authenticated</a>&lt;T: <b>copy</b> + drop&gt;(<a href="../myso/event.md#myso_event">event</a>: T) {
    <b>let</b> stream_id = type_name::original_id&lt;T&gt;();
    <b>let</b> accumulator_addr = <a href="../myso/accumulator.md#myso_accumulator_accumulator_address">accumulator::accumulator_address</a>&lt;EventStreamHead&gt;(stream_id);
    <a href="../myso/event.md#myso_event_emit_authenticated_impl">emit_authenticated_impl</a>&lt;EventStreamHead, T&gt;(accumulator_addr, stream_id, <a href="../myso/event.md#myso_event">event</a>);
}
</code></pre>



</details>

<a name="myso_event_emit_authenticated_impl"></a>

## Function `emit_authenticated_impl`



<pre><code><b>fun</b> <a href="../myso/event.md#myso_event_emit_authenticated_impl">emit_authenticated_impl</a>&lt;StreamHeadT, T: <b>copy</b>, drop&gt;(accumulator_id: <b>address</b>, stream: <b>address</b>, <a href="../myso/event.md#myso_event">event</a>: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../myso/event.md#myso_event_emit_authenticated_impl">emit_authenticated_impl</a>&lt;StreamHeadT, T: <b>copy</b> + drop&gt;(
    accumulator_id: <b>address</b>,
    stream: <b>address</b>,
    <a href="../myso/event.md#myso_event">event</a>: T,
);
</code></pre>



</details>
