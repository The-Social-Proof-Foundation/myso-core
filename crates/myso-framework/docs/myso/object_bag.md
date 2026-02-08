---
title: Module `myso::object_bag`
---

Similar to <code><a href="../myso/bag.md#myso_bag">myso::bag</a></code>, an <code><a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a></code> is a heterogeneous map-like collection. But unlike
<code><a href="../myso/bag.md#myso_bag">myso::bag</a></code>, the values bound to these dynamic fields _must_ be objects themselves. This allows
for the objects to still exist in storage, which may be important for external tools.
The difference is otherwise not observable from within Move.


-  [Struct `ObjectBag`](#myso_object_bag_ObjectBag)
-  [Constants](#@Constants_0)
-  [Function `new`](#myso_object_bag_new)
-  [Function `add`](#myso_object_bag_add)
-  [Function `borrow`](#myso_object_bag_borrow)
-  [Function `borrow_mut`](#myso_object_bag_borrow_mut)
-  [Function `remove`](#myso_object_bag_remove)
-  [Function `contains`](#myso_object_bag_contains)
-  [Function `contains_with_type`](#myso_object_bag_contains_with_type)
-  [Function `length`](#myso_object_bag_length)
-  [Function `is_empty`](#myso_object_bag_is_empty)
-  [Function `destroy_empty`](#myso_object_bag_destroy_empty)
-  [Function `value_id`](#myso_object_bag_value_id)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="myso_object_bag_ObjectBag"></a>

## Struct `ObjectBag`



<pre><code><b>public</b> <b>struct</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
 the ID of this bag
</dd>
<dt>
<code>size: u64</code>
</dt>
<dd>
 the number of key-value pairs in the bag
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_object_bag_EBagNotEmpty"></a>



<pre><code><b>const</b> <a href="../myso/object_bag.md#myso_object_bag_EBagNotEmpty">EBagNotEmpty</a>: u64 = 0;
</code></pre>



<a name="myso_object_bag_new"></a>

## Function `new`

Creates a new, empty bag


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_new">new</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a> {
    <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a> {
        id: <a href="../myso/object.md#myso_object_new">object::new</a>(ctx),
        size: 0,
    }
}
</code></pre>



</details>

<a name="myso_object_bag_add"></a>

## Function `add`

Adds a key-value pair to the bag <code><a href="../myso/bag.md#myso_bag">bag</a>: &<b>mut</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a></code>
Aborts with <code><a href="../myso/dynamic_field.md#myso_dynamic_field_EFieldAlreadyExists">myso::dynamic_field::EFieldAlreadyExists</a></code> if the bag already has an entry with
that key <code>k: K</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_add">add</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<b>mut</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>, k: K, v: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_add">add</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<b>mut</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>, k: K, v: V) {
    ofield::add(&<b>mut</b> <a href="../myso/bag.md#myso_bag">bag</a>.id, k, v);
    <a href="../myso/bag.md#myso_bag">bag</a>.size = <a href="../myso/bag.md#myso_bag">bag</a>.size + 1;
}
</code></pre>



</details>

<a name="myso_object_bag_borrow"></a>

## Function `borrow`

Immutably borrows the value associated with the key in the bag <code><a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a></code>.
Aborts with <code><a href="../myso/dynamic_field.md#myso_dynamic_field_EFieldDoesNotExist">myso::dynamic_field::EFieldDoesNotExist</a></code> if the bag does not have an entry with
that key <code>k: K</code>.
Aborts with <code><a href="../myso/dynamic_field.md#myso_dynamic_field_EFieldTypeMismatch">myso::dynamic_field::EFieldTypeMismatch</a></code> if the bag has an entry for the key, but
the value does not have the specified type.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/borrow.md#myso_borrow">borrow</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>, k: K): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/borrow.md#myso_borrow">borrow</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>, k: K): &V {
    ofield::borrow(&<a href="../myso/bag.md#myso_bag">bag</a>.id, k)
}
</code></pre>



</details>

<a name="myso_object_bag_borrow_mut"></a>

## Function `borrow_mut`

Mutably borrows the value associated with the key in the bag <code><a href="../myso/bag.md#myso_bag">bag</a>: &<b>mut</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a></code>.
Aborts with <code><a href="../myso/dynamic_field.md#myso_dynamic_field_EFieldDoesNotExist">myso::dynamic_field::EFieldDoesNotExist</a></code> if the bag does not have an entry with
that key <code>k: K</code>.
Aborts with <code><a href="../myso/dynamic_field.md#myso_dynamic_field_EFieldTypeMismatch">myso::dynamic_field::EFieldTypeMismatch</a></code> if the bag has an entry for the key, but
the value does not have the specified type.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_borrow_mut">borrow_mut</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<b>mut</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>, k: K): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_borrow_mut">borrow_mut</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<b>mut</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>, k: K): &<b>mut</b> V {
    ofield::borrow_mut(&<b>mut</b> <a href="../myso/bag.md#myso_bag">bag</a>.id, k)
}
</code></pre>



</details>

<a name="myso_object_bag_remove"></a>

## Function `remove`

Mutably borrows the key-value pair in the bag <code><a href="../myso/bag.md#myso_bag">bag</a>: &<b>mut</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a></code> and returns the value.
Aborts with <code><a href="../myso/dynamic_field.md#myso_dynamic_field_EFieldDoesNotExist">myso::dynamic_field::EFieldDoesNotExist</a></code> if the bag does not have an entry with
that key <code>k: K</code>.
Aborts with <code><a href="../myso/dynamic_field.md#myso_dynamic_field_EFieldTypeMismatch">myso::dynamic_field::EFieldTypeMismatch</a></code> if the bag has an entry for the key, but
the value does not have the specified type.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_remove">remove</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<b>mut</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>, k: K): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_remove">remove</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<b>mut</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>, k: K): V {
    <b>let</b> v = ofield::remove(&<b>mut</b> <a href="../myso/bag.md#myso_bag">bag</a>.id, k);
    <a href="../myso/bag.md#myso_bag">bag</a>.size = <a href="../myso/bag.md#myso_bag">bag</a>.size - 1;
    v
}
</code></pre>



</details>

<a name="myso_object_bag_contains"></a>

## Function `contains`

Returns true iff there is an value associated with the key <code>k: K</code> in the bag <code><a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_contains">contains</a>&lt;K: <b>copy</b>, drop, store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>, k: K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_contains">contains</a>&lt;K: <b>copy</b> + drop + store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>, k: K): bool {
    ofield::exists_&lt;K&gt;(&<a href="../myso/bag.md#myso_bag">bag</a>.id, k)
}
</code></pre>



</details>

<a name="myso_object_bag_contains_with_type"></a>

## Function `contains_with_type`

Returns true iff there is an value associated with the key <code>k: K</code> in the bag <code><a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a></code>
with an assigned value of type <code>V</code>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_contains_with_type">contains_with_type</a>&lt;K: <b>copy</b>, drop, store, V: key, store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>, k: K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_contains_with_type">contains_with_type</a>&lt;K: <b>copy</b> + drop + store, V: key + store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>, k: K): bool {
    ofield::exists_with_type&lt;K, V&gt;(&<a href="../myso/bag.md#myso_bag">bag</a>.id, k)
}
</code></pre>



</details>

<a name="myso_object_bag_length"></a>

## Function `length`

Returns the size of the bag, the number of key-value pairs


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_length">length</a>(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_length">length</a>(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>): u64 {
    <a href="../myso/bag.md#myso_bag">bag</a>.size
}
</code></pre>



</details>

<a name="myso_object_bag_is_empty"></a>

## Function `is_empty`

Returns true iff the bag is empty (if <code><a href="../myso/object_bag.md#myso_object_bag_length">length</a></code> returns <code>0</code>)


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_is_empty">is_empty</a>(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_is_empty">is_empty</a>(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>): bool {
    <a href="../myso/bag.md#myso_bag">bag</a>.size == 0
}
</code></pre>



</details>

<a name="myso_object_bag_destroy_empty"></a>

## Function `destroy_empty`

Destroys an empty bag
Aborts with <code><a href="../myso/object_bag.md#myso_object_bag_EBagNotEmpty">EBagNotEmpty</a></code> if the bag still contains values


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_destroy_empty">destroy_empty</a>(<a href="../myso/bag.md#myso_bag">bag</a>: <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_destroy_empty">destroy_empty</a>(<a href="../myso/bag.md#myso_bag">bag</a>: <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>) {
    <b>let</b> <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a> { id, size } = <a href="../myso/bag.md#myso_bag">bag</a>;
    <b>assert</b>!(size == 0, <a href="../myso/object_bag.md#myso_object_bag_EBagNotEmpty">EBagNotEmpty</a>);
    id.delete()
}
</code></pre>



</details>

<a name="myso_object_bag_value_id"></a>

## Function `value_id`

Returns the ID of the object associated with the key if the bag has an entry with key <code>k: K</code>
Returns none otherwise


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_value_id">value_id</a>&lt;K: <b>copy</b>, drop, store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a>, k: K): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/object_bag.md#myso_object_bag_value_id">value_id</a>&lt;K: <b>copy</b> + drop + store&gt;(<a href="../myso/bag.md#myso_bag">bag</a>: &<a href="../myso/object_bag.md#myso_object_bag_ObjectBag">ObjectBag</a>, k: K): Option&lt;ID&gt; {
    ofield::id(&<a href="../myso/bag.md#myso_bag">bag</a>.id, k)
}
</code></pre>



</details>
