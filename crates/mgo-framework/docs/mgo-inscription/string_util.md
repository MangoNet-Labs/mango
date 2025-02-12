
<a name="0x4_string_util"></a>

# Module `0x4::string_util`



-  [Function `is_empty_str`](#0x4_string_util_is_empty_str)
-  [Function `is_empty`](#0x4_string_util_is_empty)
-  [Function `is_tick_valid`](#0x4_string_util_is_tick_valid)
-  [Function `to_uppercase`](#0x4_string_util_to_uppercase)
-  [Function `to_lowercase`](#0x4_string_util_to_lowercase)
-  [Function `is_number`](#0x4_string_util_is_number)
-  [Function `is_lowercase`](#0x4_string_util_is_lowercase)
-  [Function `is_uppercase`](#0x4_string_util_is_uppercase)
-  [Function `starts_with`](#0x4_string_util_starts_with)
-  [Function `index_of`](#0x4_string_util_index_of)
-  [Function `last_index_of`](#0x4_string_util_last_index_of)
-  [Function `substring`](#0x4_string_util_substring)
-  [Function `contains_any`](#0x4_string_util_contains_any)


<pre><code><b>use</b> <a href="dependencies/move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="dependencies/move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a name="0x4_string_util_is_empty_str"></a>

## Function `is_empty_str`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_empty_str">is_empty_str</a>(input: &<a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_empty_str">is_empty_str</a>(input: &String): bool {
    <a href="string_util.md#0x4_string_util_is_empty">is_empty</a>(<a href="dependencies/move-stdlib/string.md#0x1_string_bytes">string::bytes</a>(input))
}
</code></pre>



</details>

<a name="0x4_string_util_is_empty"></a>

## Function `is_empty`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_empty">is_empty</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_empty">is_empty</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool {
    <a href="dependencies/move-stdlib/vector.md#0x1_vector_is_empty">vector::is_empty</a>(input)
}
</code></pre>



</details>

<a name="0x4_string_util_is_tick_valid"></a>

## Function `is_tick_valid`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_tick_valid">is_tick_valid</a>(input: &<a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_tick_valid">is_tick_valid</a>(input: &String): bool {
    <b>let</b> bytes = <a href="dependencies/move-stdlib/string.md#0x1_string_bytes">string::bytes</a>(input);
    <b>let</b> len = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(bytes);
    <b>let</b> i = 0;
    <b>let</b> res = len &gt; 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> b = *<a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(bytes, i);
        res = <a href="string_util.md#0x4_string_util_is_number">is_number</a>(b) || <a href="string_util.md#0x4_string_util_is_uppercase">is_uppercase</a>(b);
        <b>if</b> (!res) {
            <b>break</b>
        };
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a name="0x4_string_util_to_uppercase"></a>

## Function `to_uppercase`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_to_uppercase">to_uppercase</a>(input: &<b>mut</b> <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_to_uppercase">to_uppercase</a>(input: &<b>mut</b> <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <b>let</b> length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(input);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; length) {
        <b>let</b> letter = <a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(input, i);
        <b>if</b> (<a href="string_util.md#0x4_string_util_is_lowercase">is_lowercase</a>(*letter)) {
            *letter = *letter - 32;
        };
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="0x4_string_util_to_lowercase"></a>

## Function `to_lowercase`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_to_lowercase">to_lowercase</a>(input: &<b>mut</b> <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_to_lowercase">to_lowercase</a>(input: &<b>mut</b> <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <b>let</b> length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(input);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; length) {
        <b>let</b> letter = <a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(input, i);
        <b>if</b> (<a href="string_util.md#0x4_string_util_is_uppercase">is_uppercase</a>(*letter)) {
            *letter = *letter + 32;
        };
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="0x4_string_util_is_number"></a>

## Function `is_number`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_number">is_number</a>(letter: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_number">is_number</a>(letter: u8): bool {
    letter &gt;= 48 && letter &lt;= 57
}
</code></pre>



</details>

<a name="0x4_string_util_is_lowercase"></a>

## Function `is_lowercase`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_lowercase">is_lowercase</a>(letter: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_lowercase">is_lowercase</a>(letter: u8): bool {
    letter &gt;= 97 && letter &lt;= 122
}
</code></pre>



</details>

<a name="0x4_string_util_is_uppercase"></a>

## Function `is_uppercase`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_uppercase">is_uppercase</a>(letter: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_is_uppercase">is_uppercase</a>(letter: u8): bool {
    letter &gt;= 65 && letter &lt;= 90
}
</code></pre>



</details>

<a name="0x4_string_util_starts_with"></a>

## Function `starts_with`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_starts_with">starts_with</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, prefix: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_starts_with">starts_with</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, prefix: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool {
    <b>let</b> input_length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(input);
    <b>let</b> prefix_length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(prefix);
    <b>if</b> (input_length &lt; prefix_length) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> i = 0;
    <b>while</b> (i &lt; prefix_length) {
        <b>if</b> (<a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(input, i) != <a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(prefix, i)) {
            <b>return</b> <b>false</b>
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="0x4_string_util_index_of"></a>

## Function `index_of`

Returns if the input contains the search string and the index of the first match


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_index_of">index_of</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, search: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_index_of">index_of</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, search: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): (bool, u64) {
    <b>let</b> input_length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(input);
    <b>let</b> search_length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(search);
    <b>if</b> (input_length &lt; search_length) {
        <b>return</b> (<b>false</b>, 0)
    };
    <b>if</b> (input_length == 0 && input_length == search_length) {
        <b>return</b> (<b>true</b>, 0)
    };
    <b>let</b> i = 0;
    <b>while</b> (i &lt; input_length) {
        <b>let</b> j = 0;
        <b>while</b> (j &lt; search_length) {
            <b>let</b> idx = i + j;
            <b>if</b> (idx &gt;= input_length) {
                <b>break</b>
            };
            <b>if</b> (<a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(input, idx) != <a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(search, j)) {
                <b>break</b>
            };
            j = j + 1;
        };
        <b>if</b> (j == search_length) {
            <b>return</b> (<b>true</b>, i)
        };
        i = i + 1;
    };
    (<b>false</b>, 0)
}
</code></pre>



</details>

<a name="0x4_string_util_last_index_of"></a>

## Function `last_index_of`

Returns if the input contains the search string and the index of the last match


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_last_index_of">last_index_of</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, search: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_last_index_of">last_index_of</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, search: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): (bool, u64) {
    <b>let</b> input_length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(input);
    <b>let</b> search_length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(search);
    <b>if</b> (input_length &lt; search_length) {
        <b>return</b> (<b>false</b>, 0)
    };
    <b>if</b> (input_length == 0 && input_length == search_length) {
        <b>return</b> (<b>true</b>, 0)
    };
    <b>let</b> i = input_length - search_length;
    <b>while</b> (i &gt;= 0) {
        <b>let</b> j = 0;
        <b>while</b> (j &lt; search_length) {
            <b>let</b> idx = i + j;
            <b>if</b> (idx &gt;= input_length) {
                <b>break</b>
            };
            <b>if</b> (<a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(input, idx) != <a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(search, j)) {
                <b>break</b>
            };
            j = j + 1;
        };
        <b>if</b> (j == search_length) {
            <b>return</b> (<b>true</b>, i)
        };
        <b>if</b> (i == 0) {
            <b>break</b>
        };
        i = i - 1;
    };
    (<b>false</b>, 0)
}
</code></pre>



</details>

<a name="0x4_string_util_substring"></a>

## Function `substring`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_substring">substring</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, start: u64, end: u64): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_substring">substring</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, start: u64, end: u64): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(input);
    <b>if</b> (start &gt;= length) {
        <b>return</b> <a href="dependencies/move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>()
    };
    <b>let</b> end = <b>if</b> (end &gt; length) {
        length
    } <b>else</b> {
        end
    };
    <b>let</b> result = <a href="dependencies/move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>();
    <b>let</b> i = start;
    <b>while</b> (i &lt; end) {
        <a href="dependencies/move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> result, *<a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(input, i));
        i = i + 1;
    };
    result
}
</code></pre>



</details>

<a name="0x4_string_util_contains_any"></a>

## Function `contains_any`

Returns if the input contains any of the chars


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_contains_any">contains_any</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, chars: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="string_util.md#0x4_string_util_contains_any">contains_any</a>(input: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, chars: &<a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool {
    <b>let</b> length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(input);
    <b>let</b> chars_length = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(chars);
    <b>if</b> (length == 0 && length == chars_length) {
        <b>return</b> <b>true</b>
    };
    <b>let</b> i = 0;
    <b>while</b> (i &lt; length) {
        <b>let</b> j = 0;
        <b>while</b> (j &lt; chars_length) {
            <b>if</b> (<a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(input, i) == <a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(chars, j)) {
                <b>return</b> <b>true</b>
            };
            j = j + 1;
        };
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>
