---
title: Module `mydata::polynomial`
---



-  [Struct `Polynomial`](#mydata_polynomial_Polynomial)
-  [Constants](#@Constants_0)
-  [Function `evaluate`](#mydata_polynomial_evaluate)
-  [Function `get_constant_term`](#mydata_polynomial_get_constant_term)
-  [Function `div_by_monic_linear`](#mydata_polynomial_div_by_monic_linear)
-  [Function `interpolate_with_numerators`](#mydata_polynomial_interpolate_with_numerators)
-  [Function `compute_numerators`](#mydata_polynomial_compute_numerators)
-  [Function `interpolate_all`](#mydata_polynomial_interpolate_all)
-  [Function `add`](#mydata_polynomial_add)
-  [Function `mul`](#mydata_polynomial_mul)
-  [Function `scale`](#mydata_polynomial_scale)
-  [Function `monic_linear`](#mydata_polynomial_monic_linear)


<pre><code><b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="mydata_polynomial_Polynomial"></a>

## Struct `Polynomial`

This represents a polynomial over GF(2^8).
The first coefficient is the constant term.


<pre><code><b>public</b> <b>struct</b> <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coefficients: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mydata_polynomial_EIncompatibleInputLengths"></a>



<pre><code><b>const</b> <a href="../mydata/polynomial.md#mydata_polynomial_EIncompatibleInputLengths">EIncompatibleInputLengths</a>: u64 = 1;
</code></pre>



<a name="mydata_polynomial_evaluate"></a>

## Function `evaluate`

Evaluate a polynomial at a given point.


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_evaluate">evaluate</a>(p: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>, x: u8): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_evaluate">evaluate</a>(p: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>, x: u8): u8 {
    <b>if</b> (p.coefficients.is_empty()) {
        <b>return</b> 0
    };
    <b>let</b> n = p.coefficients.length();
    <b>let</b> <b>mut</b> result = p.coefficients[n - 1];
    (n - 1).do!(|i| {
        result = <a href="../mydata/gf256.md#mydata_gf256_add">gf256::add</a>(<a href="../mydata/gf256.md#mydata_gf256_mul">gf256::mul</a>(result, x), p.coefficients[n - i - 2]);
    });
    result
}
</code></pre>



</details>

<a name="mydata_polynomial_get_constant_term"></a>

## Function `get_constant_term`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_get_constant_term">get_constant_term</a>(p: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_get_constant_term">get_constant_term</a>(p: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>): u8 {
    <b>if</b> (p.coefficients.is_empty()) 0 // zero <a href="../mydata/polynomial.md#mydata_polynomial">polynomial</a>
    <b>else</b> p.coefficients[0]
}
</code></pre>



</details>

<a name="mydata_polynomial_div_by_monic_linear"></a>

## Function `div_by_monic_linear`



<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_div_by_monic_linear">div_by_monic_linear</a>(x: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>, c: u8): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_div_by_monic_linear">div_by_monic_linear</a>(x: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>, c: u8): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> {
    <b>let</b> n = x.coefficients.length();
    <b>let</b> <b>mut</b> coefficients = vector::empty();
    <b>if</b> (n &gt; 1) {
        <b>let</b> <b>mut</b> previous = x.coefficients[n - 1];
        coefficients.push_back(previous);
        range_do_eq!(1, n - 2, |i| {
            previous = <a href="../mydata/gf256.md#mydata_gf256_sub">gf256::sub</a>(x.coefficients[n - i - 1], <a href="../mydata/gf256.md#mydata_gf256_mul">gf256::mul</a>(previous, c));
            coefficients.push_back(previous);
        });
        coefficients.reverse();
    };
    <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> { coefficients }
}
</code></pre>



</details>

<a name="mydata_polynomial_interpolate_with_numerators"></a>

## Function `interpolate_with_numerators`

Same as interpolate, but the numerator product, \prod_i (x - x_i), is precomputed.


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_interpolate_with_numerators">interpolate_with_numerators</a>(x: &vector&lt;u8&gt;, y: &vector&lt;u8&gt;, numerators: &vector&lt;<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>&gt;): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_interpolate_with_numerators">interpolate_with_numerators</a>(
    x: &vector&lt;u8&gt;,
    y: &vector&lt;u8&gt;,
    numerators: &vector&lt;<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>&gt;,
): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> {
    <b>assert</b>!(x.length() == y.length(), <a href="../mydata/polynomial.md#mydata_polynomial_EIncompatibleInputLengths">EIncompatibleInputLengths</a>);
    <b>let</b> n = x.length();
    <b>let</b> <b>mut</b> sum = <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> { coefficients: vector[] };
    n.do!(|j| {
        <b>let</b> <b>mut</b> denominator = 1;
        n.do!(|i| {
            <b>if</b> (i != j) {
                denominator = <a href="../mydata/gf256.md#mydata_gf256_mul">gf256::mul</a>(denominator, <a href="../mydata/gf256.md#mydata_gf256_sub">gf256::sub</a>(x[j], x[i]));
            };
        });
        sum =
            <a href="../mydata/polynomial.md#mydata_polynomial_add">add</a>(
                &sum,
                &numerators[j].<a href="../mydata/polynomial.md#mydata_polynomial_scale">scale</a>(
                    <a href="../mydata/gf256.md#mydata_gf256_div">gf256::div</a>(y[j], denominator),
                ),
            );
    });
    sum
}
</code></pre>



</details>

<a name="mydata_polynomial_compute_numerators"></a>

## Function `compute_numerators`

Compute the numerators of the Lagrange polynomials for the given x values.


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_compute_numerators">compute_numerators</a>(x: vector&lt;u8&gt;): vector&lt;<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_compute_numerators">compute_numerators</a>(x: vector&lt;u8&gt;): vector&lt;<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>&gt; {
    // The full numerator depends only on x, so we can compute it here
    <b>let</b> full_numerator = x.fold!(<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> { coefficients: vector[1] }, |product, x_j| {
        product.<a href="../mydata/polynomial.md#mydata_polynomial_mul">mul</a>(&<a href="../mydata/polynomial.md#mydata_polynomial_monic_linear">monic_linear</a>(&x_j))
    });
    x.map_ref!(|x_j| <a href="../mydata/polynomial.md#mydata_polynomial_div_by_monic_linear">div_by_monic_linear</a>(&full_numerator, *x_j))
}
</code></pre>



</details>

<a name="mydata_polynomial_interpolate_all"></a>

## Function `interpolate_all`

Interpolate l polynomials p_1, ..., p_l such that p_i(x_j) = y[j][i] for all i, j.
The length of the input vectors must be the same.
The length of each vector in y must be the same (equal to the l above).
Aborts if the input lengths are not compatible or if the vectors are empty.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_interpolate_all">interpolate_all</a>(x: &vector&lt;u8&gt;, y: &vector&lt;vector&lt;u8&gt;&gt;): vector&lt;<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_interpolate_all">interpolate_all</a>(x: &vector&lt;u8&gt;, y: &vector&lt;vector&lt;u8&gt;&gt;): vector&lt;<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>&gt; {
    <b>assert</b>!(x.length() == y.length(), <a href="../mydata/polynomial.md#mydata_polynomial_EIncompatibleInputLengths">EIncompatibleInputLengths</a>);
    <b>let</b> l = y[0].length();
    <b>assert</b>!(y.all!(|yi| yi.length() == l), <a href="../mydata/polynomial.md#mydata_polynomial_EIncompatibleInputLengths">EIncompatibleInputLengths</a>);
    // The numerators depend only on x, so we can compute them here
    <b>let</b> numerators = <a href="../mydata/polynomial.md#mydata_polynomial_compute_numerators">compute_numerators</a>(*x);
    vector::tabulate!(l, |i| {
        <b>let</b> yi = y.map_ref!(|yj| yj[i]);
        <a href="../mydata/polynomial.md#mydata_polynomial_interpolate_with_numerators">interpolate_with_numerators</a>(x, &yi, &numerators)
    })
}
</code></pre>



</details>

<a name="mydata_polynomial_add"></a>

## Function `add`



<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_add">add</a>(x: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>, y: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_add">add</a>(x: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>, y: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> {
    <b>let</b> x_length: u64 = x.coefficients.length();
    <b>let</b> y_length: u64 = y.coefficients.length();
    <b>if</b> (x_length &lt; y_length) {
        // We assume that x is the longer vector
        <b>return</b> y.<a href="../mydata/polynomial.md#mydata_polynomial_add">add</a>(x)
    };
    <b>let</b> coefficients = vector::tabulate!(x_length, |i| {
        <b>if</b> (i &lt; y_length) {
            <a href="../mydata/gf256.md#mydata_gf256_add">gf256::add</a>(x.coefficients[i], y.coefficients[i])
        } <b>else</b> {
            x.coefficients[i]
        }
    });
    <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> { coefficients }
}
</code></pre>



</details>

<a name="mydata_polynomial_mul"></a>

## Function `mul`



<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_mul">mul</a>(x: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>, y: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_mul">mul</a>(x: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>, y: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> {
    <b>if</b> (x.coefficients.is_empty() || y.coefficients.is_empty()) {
        <b>return</b> <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> { coefficients: vector[] }
    };
    <b>let</b> coefficients = vector::tabulate!(
        x.coefficients.length() + y.coefficients.length() -  1,
        |i| {
            <b>let</b> <b>mut</b> sum = 0;
            i.do_eq!(|j| {
                <b>if</b> (j &lt; x.coefficients.length() && i - j &lt; y.coefficients.length()) {
                    sum = <a href="../mydata/gf256.md#mydata_gf256_add">gf256::add</a>(sum, <a href="../mydata/gf256.md#mydata_gf256_mul">gf256::mul</a>(x.coefficients[j], y.coefficients[i - j]));
                }
            });
            sum
        },
    );
    <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> { coefficients }
}
</code></pre>



</details>

<a name="mydata_polynomial_scale"></a>

## Function `scale`



<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_scale">scale</a>(x: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>, s: u8): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_scale">scale</a>(x: &<a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a>, s: u8): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> {
    <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> { coefficients: x.coefficients.map_ref!(|c| <a href="../mydata/gf256.md#mydata_gf256_mul">gf256::mul</a>(*c, s)) }
}
</code></pre>



</details>

<a name="mydata_polynomial_monic_linear"></a>

## Function `monic_linear`

Return x - c (same as x + c since GF256 is a binary field)


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_monic_linear">monic_linear</a>(c: &u8): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">mydata::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/polynomial.md#mydata_polynomial_monic_linear">monic_linear</a>(c: &u8): <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> {
    <a href="../mydata/polynomial.md#mydata_polynomial_Polynomial">Polynomial</a> { coefficients: vector[*c, 1] }
}
</code></pre>



</details>
