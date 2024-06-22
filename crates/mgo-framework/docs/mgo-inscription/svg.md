
<a name="0x4_svg"></a>

# Module `0x4::svg`



-  [Constants](#@Constants_0)
-  [Function `generate_coinscription_svg`](#0x4_svg_generate_coinscription_svg)
-  [Function `generate_singlescription_svg`](#0x4_svg_generate_singlescription_svg)


<pre><code><b>use</b> <a href="dependencies/move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x4_svg_SVG_PATH_COIN_1"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_COIN_1">SVG_PATH_COIN_1</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [100, 97, 116, 97, 58, 105, 109, 97, 103, 101, 47, 115, 118, 103, 43, 120, 109, 108, 44, 37, 51, 67, 115, 118, 103, 37, 50, 48, 119, 105, 100, 116, 104, 37, 51, 68, 37, 50, 50, 49, 50, 48, 37, 50, 50, 37, 50, 48, 104, 101, 105, 103, 104, 116, 37, 51, 68, 37, 50, 50, 49, 50, 48, 37, 50, 50, 37, 50, 48, 118, 105, 101, 119, 66, 111, 120, 37, 51, 68, 37, 50, 50, 48, 37, 50, 48, 48, 37, 50, 48, 49, 50, 48, 37, 50, 48, 49, 50, 48, 37, 50, 50, 37, 50, 48, 102, 105, 108, 108, 37, 51, 68, 37, 50, 50, 110, 111, 110, 101, 37, 50, 50, 37, 50, 48, 120, 109, 108, 110, 115, 37, 51, 68, 37, 50, 50, 104, 116, 116, 112, 37, 51, 65, 37, 50, 70, 37, 50, 70, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 37, 50, 70, 50, 48, 48, 48, 37, 50, 70, 115, 118, 103, 37, 50, 50, 37, 51, 69, 37, 51, 67, 114, 101, 99, 116, 37, 50, 48, 119, 105, 100, 116, 104, 37, 51, 68, 37, 50, 50, 49, 50, 48, 37, 50, 50, 37, 50, 48, 104, 101, 105, 103, 104, 116, 37, 51, 68, 37, 50, 50, 49, 50, 48, 37, 50, 50, 37, 50, 48, 102, 105, 108, 108, 37, 51, 68, 37, 50, 50, 37, 50, 51, 52, 54, 65, 68, 66, 51, 37, 50, 50, 37, 50, 70, 37, 51, 69, 37, 51, 67, 116, 101, 120, 116, 37, 50, 48, 102, 105, 108, 108, 37, 51, 68, 37, 50, 50, 37, 50, 51, 69, 65, 70, 55, 70, 70, 37, 50, 50, 37, 50, 48, 120, 109, 108, 37, 51, 65, 115, 112, 97, 99, 101, 37, 51, 68, 37, 50, 50, 112, 114, 101, 115, 101, 114, 118, 101, 37, 50, 50, 37, 50, 48, 115, 116, 121, 108, 101, 37, 51, 68, 37, 50, 50, 119, 104, 105, 116, 101, 45, 115, 112, 97, 99, 101, 37, 51, 65, 37, 50, 48, 112, 114, 101, 37, 50, 50, 37, 50, 48, 102, 111, 110, 116, 45, 102, 97, 109, 105, 108, 121, 37, 51, 68, 37, 50, 50, 73, 110, 116, 101, 114, 37, 50, 50, 37, 50, 48, 102, 111, 110, 116, 45, 115, 105, 122, 101, 37, 51, 68, 37, 50, 50, 49, 48, 37, 50, 50, 37, 50, 48, 108, 101, 116, 116, 101, 114, 45, 115, 112, 97, 99, 105, 110, 103, 37, 51, 68, 37, 50, 50, 48, 101, 109, 37, 50, 50, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 50, 54, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 55, 66, 37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 52, 49, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 112, 37, 51, 65, 37, 50, 48, 37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66];
</code></pre>



<a name="0x4_svg_SVG_PATH_COIN_2"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_COIN_2">SVG_PATH_COIN_2</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66, 37, 50, 67, 37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 53, 54, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 111, 112, 37, 51, 65, 37, 50, 48, 37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66];
</code></pre>



<a name="0x4_svg_SVG_PATH_COIN_3"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_COIN_3">SVG_PATH_COIN_3</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66, 37, 50, 67, 37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 55, 49, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 116, 105, 99, 107, 37, 51, 65, 37, 50, 48, 37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66];
</code></pre>



<a name="0x4_svg_SVG_PATH_COIN_4"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_COIN_4">SVG_PATH_COIN_4</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66, 37, 50, 67, 37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 56, 54, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 97, 109, 116, 37, 51, 65, 37, 50, 48];
</code></pre>



<a name="0x4_svg_SVG_PATH_COIN_5"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_COIN_5">SVG_PATH_COIN_5</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 49, 48, 49, 46, 56, 54, 52, 37, 50, 50, 37, 51, 69, 37, 55, 68, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 37, 50, 70, 116, 101, 120, 116, 37, 51, 69, 37, 51, 67, 37, 50, 70, 115, 118, 103, 37, 51, 69];
</code></pre>



<a name="0x4_svg_SVG_PATH_SINGLE_1"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_1">SVG_PATH_SINGLE_1</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [100, 97, 116, 97, 58, 105, 109, 97, 103, 101, 47, 115, 118, 103, 43, 120, 109, 108, 44, 37, 51, 67, 115, 118, 103, 37, 50, 48, 119, 105, 100, 116, 104, 37, 51, 68, 37, 50, 50, 49, 50, 48, 37, 50, 50, 37, 50, 48, 104, 101, 105, 103, 104, 116, 37, 51, 68, 37, 50, 50, 49, 50, 48, 37, 50, 50, 37, 50, 48, 118, 105, 101, 119, 66, 111, 120, 37, 51, 68, 37, 50, 50, 48, 37, 50, 48, 48, 37, 50, 48, 49, 50, 48, 37, 50, 48, 49, 50, 48, 37, 50, 50, 37, 50, 48, 102, 105, 108, 108, 37, 51, 68, 37, 50, 50, 110, 111, 110, 101, 37, 50, 50, 37, 50, 48, 120, 109, 108, 110, 115, 37, 51, 68, 37, 50, 50, 104, 116, 116, 112, 37, 51, 65, 37, 50, 70, 37, 50, 70, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 37, 50, 70, 50, 48, 48, 48, 37, 50, 70, 115, 118, 103, 37, 50, 50, 37, 51, 69, 37, 51, 67, 114, 101, 99, 116, 37, 50, 48, 119, 105, 100, 116, 104, 37, 51, 68, 37, 50, 50, 49, 50, 48, 37, 50, 50, 37, 50, 48, 104, 101, 105, 103, 104, 116, 37, 51, 68, 37, 50, 50, 49, 50, 48, 37, 50, 50, 37, 50, 48, 102, 105, 108, 108, 37, 51, 68, 37, 50, 50, 37, 50, 51, 52, 54, 65, 68, 66, 51, 37, 50, 50, 37, 50, 70, 37, 51, 69, 37, 51, 67, 116, 101, 120, 116, 37, 50, 48, 102, 105, 108, 108, 37, 51, 68, 37, 50, 50, 37, 50, 51, 69, 65, 70, 55, 70, 70, 37, 50, 50, 37, 50, 48, 120, 109, 108, 37, 51, 65, 115, 112, 97, 99, 101, 37, 51, 68, 37, 50, 50, 112, 114, 101, 115, 101, 114, 118, 101, 37, 50, 50, 37, 50, 48, 115, 116, 121, 108, 101, 37, 51, 68, 37, 50, 50, 119, 104, 105, 116, 101, 45, 115, 112, 97, 99, 101, 37, 51, 65, 37, 50, 48, 112, 114, 101, 37, 50, 50, 37, 50, 48, 102, 111, 110, 116, 45, 102, 97, 109, 105, 108, 121, 37, 51, 68, 37, 50, 50, 73, 110, 116, 101, 114, 37, 50, 50, 37, 50, 48, 102, 111, 110, 116, 45, 115, 105, 122, 101, 37, 51, 68, 37, 50, 50, 49, 48, 37, 50, 50, 37, 50, 48, 108, 101, 116, 116, 101, 114, 45, 115, 112, 97, 99, 105, 110, 103, 37, 51, 68, 37, 50, 50, 48, 101, 109, 37, 50, 50, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 50, 54, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 55, 66, 37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 52, 49, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 112, 37, 51, 65, 37, 50, 48, 37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66];
</code></pre>



<a name="0x4_svg_SVG_PATH_SINGLE_2"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_2">SVG_PATH_SINGLE_2</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66, 37, 50, 67, 37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 53, 54, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 110, 97, 37, 51, 65, 37, 50, 48, 37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66];
</code></pre>



<a name="0x4_svg_SVG_PATH_SINGLE_3"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_3">SVG_PATH_SINGLE_3</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66, 37, 50, 67, 37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 55, 49, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 116, 121, 112, 37, 51, 65, 37, 50, 48, 37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66];
</code></pre>



<a name="0x4_svg_SVG_PATH_SINGLE_4"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_4">SVG_PATH_SINGLE_4</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66, 37, 50, 67, 37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 56, 54, 46, 56, 54, 51, 54, 37, 50, 50, 37, 51, 69, 37, 50, 48, 37, 50, 48, 37, 50, 48, 37, 50, 48, 99, 111, 112, 114, 37, 51, 65, 37, 50, 48, 37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66, 37, 67, 50, 37, 65, 57];
</code></pre>



<a name="0x4_svg_SVG_PATH_SINGLE_5"></a>



<pre><code><b>const</b> <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_5">SVG_PATH_SINGLE_5</a>: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [37, 50, 54, 37, 50, 51, 51, 57, 37, 51, 66, 37, 50, 54, 37, 50, 51, 49, 48, 37, 51, 66, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 116, 115, 112, 97, 110, 37, 50, 48, 120, 37, 51, 68, 37, 50, 50, 49, 53, 37, 50, 50, 37, 50, 48, 121, 37, 51, 68, 37, 50, 50, 49, 48, 49, 46, 56, 54, 52, 37, 50, 50, 37, 51, 69, 37, 55, 68, 37, 51, 67, 37, 50, 70, 116, 115, 112, 97, 110, 37, 51, 69, 37, 51, 67, 37, 50, 70, 116, 101, 120, 116, 37, 51, 69, 37, 51, 67, 37, 50, 70, 115, 118, 103, 37, 51, 69];
</code></pre>



<a name="0x4_svg_generate_coinscription_svg"></a>

## Function `generate_coinscription_svg`



<pre><code><b>public</b> <b>fun</b> <a href="svg.md#0x4_svg_generate_coinscription_svg">generate_coinscription_svg</a>(p: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, op: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, tick: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, amt: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="svg.md#0x4_svg_generate_coinscription_svg">generate_coinscription_svg</a>(
    p: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    op: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    tick: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    amt: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> metadata = <a href="svg.md#0x4_svg_SVG_PATH_COIN_1">SVG_PATH_COIN_1</a>;
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, p);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, <a href="svg.md#0x4_svg_SVG_PATH_COIN_2">SVG_PATH_COIN_2</a>);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, op);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, <a href="svg.md#0x4_svg_SVG_PATH_COIN_3">SVG_PATH_COIN_3</a>);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, tick);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, <a href="svg.md#0x4_svg_SVG_PATH_COIN_4">SVG_PATH_COIN_4</a>);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, amt);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, <a href="svg.md#0x4_svg_SVG_PATH_COIN_5">SVG_PATH_COIN_5</a>);

    metadata
}
</code></pre>



</details>

<a name="0x4_svg_generate_singlescription_svg"></a>

## Function `generate_singlescription_svg`



<pre><code><b>public</b> <b>fun</b> <a href="svg.md#0x4_svg_generate_singlescription_svg">generate_singlescription_svg</a>(p: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, na: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, typ: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, copr: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="svg.md#0x4_svg_generate_singlescription_svg">generate_singlescription_svg</a>(
    p: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    na: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    typ: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    copr: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> metadata = <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_1">SVG_PATH_SINGLE_1</a>;
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, p);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_2">SVG_PATH_SINGLE_2</a>);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, na);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_3">SVG_PATH_SINGLE_3</a>);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, typ);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_4">SVG_PATH_SINGLE_4</a>);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, copr);
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> metadata, <a href="svg.md#0x4_svg_SVG_PATH_SINGLE_5">SVG_PATH_SINGLE_5</a>);

    metadata
}
</code></pre>



</details>
