Crate serde\_xml\_rs Copy item path

<span class="sub-heading"><a href="../src/serde_xml_rs/lib.rs.html#1-17" class="src">Source</a>
</span>

Expand description

XML is a flexible markup language that is still used for sharing data
between applications or for writing configuration files.

Serde XML provides a way to convert between text and strongly-typed Rust
data structures.

# <a href="#caveats" class="doc-anchor">§</a>Caveats

The Serde framework was mainly designed with formats such as JSON or
YAML in mind. As opposed to XML, these formats have the advantage of a
stricter syntax which makes it possible to know what type a field is
without relying on an accompanying schema, and disallows repeating the
same tag multiple times in the same object.

For example, encoding the following document in YAML is not trivial.

``` language-xml
<document>
  <header>A header</header>
  <section>First section</section>
  <section>Second section</section>
  <sidenote>A sidenote</sidenote>
  <section>Third section</section>
  <sidenote>Another sidenote</sidenote>
  <section>Fourth section</section>
  <footer>The footer</footer>
</document>
```

One possibility is the following YAML document.

``` language-yaml
- header: A header
- section: First section
- section: Second section
- sidenote: A sidenote
- section: Third section
- sidenote: Another sidenote
- section: Fourth section
- footer: The footer
```

Other notable differences:

-   XML requires a named root node.
-   XML has a namespace system.
-   XML distinguishes between attributes, child tags and contents.
-   In XML, the order of nodes is sometimes important.

# <a href="#basic-example" class="doc-anchor">§</a>Basic example

``` {.rust .rust-example-rendered}
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

let src = r#"<?xml version="1.0" encoding="UTF-8"?><Item><name>Banana</name><source>Store</source></Item>"#;
let should_be = Item {
    name: "Banana".to_string(),
    source: "Store".to_string(),
};

let item: Item = from_str(src).unwrap();
assert_eq!(item, should_be);

let reserialized_item = to_string(&item).unwrap();
assert_eq!(src, reserialized_item);
```

# <a href="#correspondence-between-xml-and-rust" class="doc-anchor">§</a>Correspondence between XML and Rust

## <a href="#document-root" class="doc-anchor">§</a>Document root

As stated above, XML documents must have one and only one root element.
This puts a constraint on the range of types that can be supported at
the root, especially during serialization when a name has to be given to
the root element.

In order to support serialization and deserialization, the root Rust
type, that is the type of the value passed to `to_string` or returned by
`from_str`, must be one of:

-   a struct
-   a newtype struct
-   a unit struct (not very interesting)
-   an enum

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
    ...
&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap ignore">
<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
<div class="sourceCode" id="cb2"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span> <span class="op">...</span> <span class="op">}</span></span></code></pre></div>
</div>
<div class="example-wrap ignore">
<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
<div class="sourceCode" id="cb3"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document(<span class="op">...</span>)<span class="op">;</span></span></code></pre></div>
</div>
<div class="example-wrap ignore">
<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
<div class="sourceCode" id="cb4"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb4-1"><a href="#cb4-1" aria-hidden="true" tabindex="-1"></a><span class="kw">enum</span> Document <span class="op">{</span> <span class="op">...</span> <span class="op">}</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

Other types must be encapsulated in order to be serialized, because the
name of the struct or enum provides the name of the root element for the
XML document.

The deserializer supports more Rust types directly:

-   primitives (`bool`, `char`, integers, floats)
-   options
-   unit (`()`)

Sequences, tuples and maps are not supported as root types for the
moment, but could be in the future.

## <a href="#strings-and-byte-arrays" class="doc-anchor">§</a>Strings and byte arrays

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
    Some text
&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
    &lt;![CDATA[Some text]]&gt;
&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
    Some &lt;![CDATA[text]]&gt;
&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb4"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb4-1"><a href="#cb4-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb4-2"><a href="#cb4-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document(<span class="dt">String</span>)<span class="op">;</span></span>
<span id="cb4-3"><a href="#cb4-3" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb4-4"><a href="#cb4-4" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document(<span class="st">&quot;Some text&quot;</span><span class="op">.</span>to_string())<span class="op">;</span></span></code></pre></div>
</div>
<div class="example-wrap">
<div class="sourceCode" id="cb5"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb5-1"><a href="#cb5-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb5-2"><a href="#cb5-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document(<span class="at">#[</span>serde<span class="at">(</span>with <span class="op">=</span> <span class="st">&quot;serde_bytes&quot;</span><span class="at">)]</span> <span class="dt">Vec</span><span class="op">&lt;</span><span class="dt">u8</span><span class="op">&gt;</span>)<span class="op">;</span></span>
<span id="cb5-3"><a href="#cb5-3" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb5-4"><a href="#cb5-4" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document(<span class="st">&quot;Some text&quot;</span><span class="op">.</span>as_bytes()<span class="op">.</span>to_vec())<span class="op">;</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

Borrowed strings are not supported.

## <a href="#primitive-types" class="doc-anchor">§</a>Primitive types

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<th colspan="2">unit</th>
</tr>
&#10;<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document /&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb3"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb3-2"><a href="#cb3-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document(())<span class="op">;</span></span>
<span id="cb3-3"><a href="#cb3-3" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb3-4"><a href="#cb3-4" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document(())<span class="op">;</span></span></code></pre></div>
</div></td>
</tr>
<tr>
<td colspan="2">boolean</td>
</tr>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;true&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;1&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb6"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb6-1"><a href="#cb6-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb6-2"><a href="#cb6-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document(<span class="dt">bool</span>)<span class="op">;</span></span>
<span id="cb6-3"><a href="#cb6-3" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb6-4"><a href="#cb6-4" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document(<span class="cn">true</span>)<span class="op">;</span></span></code></pre></div>
</div></td>
</tr>
<tr>
<td colspan="2">char</td>
</tr>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;a&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb8"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb8-1"><a href="#cb8-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb8-2"><a href="#cb8-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document(<span class="dt">char</span>)<span class="op">;</span></span>
<span id="cb8-3"><a href="#cb8-3" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb8-4"><a href="#cb8-4" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document(<span class="ch">&#39;a&#39;</span>)<span class="op">;</span></span></code></pre></div>
</div></td>
</tr>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;false&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;0&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb11"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb11-1"><a href="#cb11-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb11-2"><a href="#cb11-2" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb11-3"><a href="#cb11-3" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document(<span class="cn">false</span>)<span class="op">;</span></span></code></pre></div>
</div></td>
</tr>
<tr>
<td colspan="2">integers</td>
</tr>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;123&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb13"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb13-1"><a href="#cb13-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb13-2"><a href="#cb13-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document(<span class="dt">i32</span>)<span class="op">;</span></span>
<span id="cb13-3"><a href="#cb13-3" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb13-4"><a href="#cb13-4" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document(<span class="dv">123</span>)<span class="op">;</span></span></code></pre></div>
</div></td>
</tr>
<tr>
<td colspan="2">floats</td>
</tr>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;123&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;123.0&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb16"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb16-1"><a href="#cb16-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb16-2"><a href="#cb16-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document(<span class="dt">f32</span>)<span class="op">;</span></span>
<span id="cb16-3"><a href="#cb16-3" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb16-4"><a href="#cb16-4" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document(<span class="dv">123.0</span>)<span class="op">;</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

## <a href="#child-elements" class="doc-anchor">§</a>Child elements

Rust structs can be used to (de)serialize the contents of XML elements:
child elements, attributes, and text. The name of the struct field must
match the name of the corresponding child element.

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;a&gt;abc&lt;/a&gt;
  &lt;b&gt;123&lt;/b&gt;
  &lt;c /&gt;
&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;b&gt;123&lt;/b&gt;
  &lt;c /&gt;
  &lt;a&gt;abc&lt;/a&gt;
&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb3"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb3-2"><a href="#cb3-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb3-3"><a href="#cb3-3" aria-hidden="true" tabindex="-1"></a>    a<span class="op">:</span> <span class="dt">String</span><span class="op">,</span></span>
<span id="cb3-4"><a href="#cb3-4" aria-hidden="true" tabindex="-1"></a>    b<span class="op">:</span> <span class="dt">i32</span><span class="op">,</span></span>
<span id="cb3-5"><a href="#cb3-5" aria-hidden="true" tabindex="-1"></a>    c<span class="op">:</span> ()<span class="op">,</span></span>
<span id="cb3-6"><a href="#cb3-6" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb3-7"><a href="#cb3-7" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb3-8"><a href="#cb3-8" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb3-9"><a href="#cb3-9" aria-hidden="true" tabindex="-1"></a>    a<span class="op">:</span> <span class="st">&quot;abc&quot;</span><span class="op">.</span>to_string()<span class="op">,</span></span>
<span id="cb3-10"><a href="#cb3-10" aria-hidden="true" tabindex="-1"></a>    b<span class="op">:</span> <span class="dv">123</span><span class="op">,</span></span>
<span id="cb3-11"><a href="#cb3-11" aria-hidden="true" tabindex="-1"></a>    c<span class="op">:</span> ()<span class="op">,</span></span>
<span id="cb3-12"><a href="#cb3-12" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

## <a href="#attributes" class="doc-anchor">§</a>Attributes

Fields that deserialize to and serialize from attributes must have a
name starting with `@`.

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document a=&quot;abc&quot; b=&quot;123&quot; c=&quot;&quot; /&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document c=&quot;&quot; b=&quot;123&quot; a=&quot;abc&quot; /&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb3"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb3-1"><a href="#cb3-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb3-2"><a href="#cb3-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb3-3"><a href="#cb3-3" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;@a&quot;</span><span class="at">)]</span></span>
<span id="cb3-4"><a href="#cb3-4" aria-hidden="true" tabindex="-1"></a>    a<span class="op">:</span> <span class="dt">String</span><span class="op">,</span></span>
<span id="cb3-5"><a href="#cb3-5" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;@b&quot;</span><span class="at">)]</span></span>
<span id="cb3-6"><a href="#cb3-6" aria-hidden="true" tabindex="-1"></a>    b<span class="op">:</span> <span class="dt">i32</span><span class="op">,</span></span>
<span id="cb3-7"><a href="#cb3-7" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;@c&quot;</span><span class="at">)]</span></span>
<span id="cb3-8"><a href="#cb3-8" aria-hidden="true" tabindex="-1"></a>    c<span class="op">:</span> ()<span class="op">,</span></span>
<span id="cb3-9"><a href="#cb3-9" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb3-10"><a href="#cb3-10" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb3-11"><a href="#cb3-11" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb3-12"><a href="#cb3-12" aria-hidden="true" tabindex="-1"></a>    a<span class="op">:</span> <span class="st">&quot;abc&quot;</span><span class="op">.</span>to_string()<span class="op">,</span></span>
<span id="cb3-13"><a href="#cb3-13" aria-hidden="true" tabindex="-1"></a>    b<span class="op">:</span> <span class="dv">123</span><span class="op">,</span></span>
<span id="cb3-14"><a href="#cb3-14" aria-hidden="true" tabindex="-1"></a>    c<span class="op">:</span> ()<span class="op">,</span></span>
<span id="cb3-15"><a href="#cb3-15" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

For serialization to work, all attributes must be declared before any
child elements.

``` {.rust .rust-example-rendered}
#[derive(Serialize, Deserialize)]
struct Document {
    #[serde(rename = "@a")]
    a: String,
    #[serde(rename = "b")] // This child element appears before an attribute
    b: i32,
    #[serde(rename = "@c")]
    c: (),
}

let value = Document {
    a: "abc".to_string(),
    b: 123,
    c: (),
};

assert!(serde_xml_rs::to_string(&value).is_err()); // ERROR !
```

## <a href="#elements-with-attributes-and-text-content"
class="doc-anchor">§</a>Elements with attributes and text content

When an element (root or child) that contains both attributes and text
content, the struct type must have a field named `#text`.

Currently, mixed content with child elements and text is not supported.

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document id=&quot;123&quot;&gt;abc&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb2"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-2"><a href="#cb2-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb2-3"><a href="#cb2-3" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;@id&quot;</span><span class="at">)]</span></span>
<span id="cb2-4"><a href="#cb2-4" aria-hidden="true" tabindex="-1"></a>    id<span class="op">:</span> <span class="dt">i32</span><span class="op">,</span></span>
<span id="cb2-5"><a href="#cb2-5" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;#text&quot;</span><span class="at">)]</span></span>
<span id="cb2-6"><a href="#cb2-6" aria-hidden="true" tabindex="-1"></a>    content<span class="op">:</span> <span class="dt">String</span><span class="op">,</span></span>
<span id="cb2-7"><a href="#cb2-7" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-8"><a href="#cb2-8" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-9"><a href="#cb2-9" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb2-10"><a href="#cb2-10" aria-hidden="true" tabindex="-1"></a>    id<span class="op">:</span> <span class="dv">123</span><span class="op">,</span></span>
<span id="cb2-11"><a href="#cb2-11" aria-hidden="true" tabindex="-1"></a>    content<span class="op">:</span> <span class="st">&quot;abc&quot;</span><span class="op">.</span>to_string()<span class="op">,</span></span>
<span id="cb2-12"><a href="#cb2-12" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

## <a href="#repeated-tags-and-sequences" class="doc-anchor">§</a>Repeated tags and sequences

Repeated tags are handled by fields with a type of `Vec<...>`. The name
of the field must correspond to the name of the tag that is repeated.
All of the repeated tags must be consecutive, unless the [overlapping
sequences](config/struct.SerdeXml.html#method.overlapping_sequences "method serde_xml_rs::config::SerdeXml::overlapping_sequences")
option is activated.

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;item&gt;item1&lt;/item&gt;
  &lt;item&gt;item2&lt;/item&gt;
  &lt;item&gt;item3&lt;/item&gt;
&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb2"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-2"><a href="#cb2-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb2-3"><a href="#cb2-3" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;item&quot;</span><span class="at">)]</span></span>
<span id="cb2-4"><a href="#cb2-4" aria-hidden="true" tabindex="-1"></a>    items<span class="op">:</span> <span class="dt">Vec</span><span class="op">&lt;</span><span class="dt">String</span><span class="op">&gt;,</span></span>
<span id="cb2-5"><a href="#cb2-5" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-6"><a href="#cb2-6" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-7"><a href="#cb2-7" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb2-8"><a href="#cb2-8" aria-hidden="true" tabindex="-1"></a>    items<span class="op">:</span> <span class="pp">vec!</span>[</span>
<span id="cb2-9"><a href="#cb2-9" aria-hidden="true" tabindex="-1"></a>        <span class="st">&quot;item1&quot;</span><span class="op">.</span>to_string()<span class="op">,</span></span>
<span id="cb2-10"><a href="#cb2-10" aria-hidden="true" tabindex="-1"></a>        <span class="st">&quot;item2&quot;</span><span class="op">.</span>to_string()<span class="op">,</span></span>
<span id="cb2-11"><a href="#cb2-11" aria-hidden="true" tabindex="-1"></a>        <span class="st">&quot;item3&quot;</span><span class="op">.</span>to_string()<span class="op">,</span></span>
<span id="cb2-12"><a href="#cb2-12" aria-hidden="true" tabindex="-1"></a>    ]<span class="op">,</span></span>
<span id="cb2-13"><a href="#cb2-13" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

## <a href="#choices-and-enums" class="doc-anchor">§</a>Choices and enums

Enums can be used to represent different options. The name of the
variant must match the name of the child element. Variants are handled
much like their struct counterparts (unit, newtype, struct).

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;message&gt;&lt;quit /&gt;&lt;/message&gt;
&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;message&gt;
    &lt;move&gt;
      &lt;x&gt;1&lt;/x&gt;
      &lt;y&gt;2&lt;/y&gt;
    &lt;/move&gt;
  &lt;/message&gt;
&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;message&gt;&lt;write&gt;a message&lt;/write&gt;&lt;/message&gt;
&lt;/Document&gt;</code></pre>
</div>
<div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;message&gt;&lt;change-color rgb=&quot;25 24 0&quot; /&gt;&lt;/message&gt;
&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb5"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb5-1"><a href="#cb5-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb5-2"><a href="#cb5-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb5-3"><a href="#cb5-3" aria-hidden="true" tabindex="-1"></a>    message<span class="op">:</span> Message<span class="op">,</span></span>
<span id="cb5-4"><a href="#cb5-4" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb5-5"><a href="#cb5-5" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb5-6"><a href="#cb5-6" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb5-7"><a href="#cb5-7" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>serde<span class="at">(</span>rename_all <span class="op">=</span> <span class="st">&quot;kebab-case&quot;</span><span class="at">)]</span></span>
<span id="cb5-8"><a href="#cb5-8" aria-hidden="true" tabindex="-1"></a><span class="kw">enum</span> Message <span class="op">{</span></span>
<span id="cb5-9"><a href="#cb5-9" aria-hidden="true" tabindex="-1"></a>    Quit<span class="op">,</span></span>
<span id="cb5-10"><a href="#cb5-10" aria-hidden="true" tabindex="-1"></a>    Move <span class="op">{</span> x<span class="op">:</span> <span class="dt">i32</span><span class="op">,</span> y<span class="op">:</span> <span class="dt">i32</span> <span class="op">},</span></span>
<span id="cb5-11"><a href="#cb5-11" aria-hidden="true" tabindex="-1"></a>    <span class="bu">Write</span>(<span class="dt">String</span>)<span class="op">,</span></span>
<span id="cb5-12"><a href="#cb5-12" aria-hidden="true" tabindex="-1"></a>    ChangeColor <span class="op">{</span></span>
<span id="cb5-13"><a href="#cb5-13" aria-hidden="true" tabindex="-1"></a>        <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;@rgb&quot;</span><span class="at">)]</span></span>
<span id="cb5-14"><a href="#cb5-14" aria-hidden="true" tabindex="-1"></a>        rgb<span class="op">:</span> (<span class="dt">i32</span><span class="op">,</span> <span class="dt">i32</span><span class="op">,</span> <span class="dt">i32</span>)</span>
<span id="cb5-15"><a href="#cb5-15" aria-hidden="true" tabindex="-1"></a>    <span class="op">},</span></span>
<span id="cb5-16"><a href="#cb5-16" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb5-17"><a href="#cb5-17" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb5-18"><a href="#cb5-18" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb5-19"><a href="#cb5-19" aria-hidden="true" tabindex="-1"></a>    message<span class="op">:</span> <span class="pp">Message::</span>Move <span class="op">{</span> x<span class="op">:</span> <span class="dv">1</span><span class="op">,</span> y<span class="op">:</span> <span class="dv">2</span> <span class="op">},</span></span>
<span id="cb5-20"><a href="#cb5-20" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

## <a href="#enums-in-attribute-values" class="doc-anchor">§</a>Enums in attribute values

Only unit variants can be used for attribute values.

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;&lt;card rank=&quot;K&quot; suit=&quot;♣&quot; /&gt;&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb2"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-2"><a href="#cb2-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb2-3"><a href="#cb2-3" aria-hidden="true" tabindex="-1"></a>    card<span class="op">:</span> Card</span>
<span id="cb2-4"><a href="#cb2-4" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-5"><a href="#cb2-5" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-6"><a href="#cb2-6" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-7"><a href="#cb2-7" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Card <span class="op">{</span></span>
<span id="cb2-8"><a href="#cb2-8" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;@rank&quot;</span><span class="at">)]</span></span>
<span id="cb2-9"><a href="#cb2-9" aria-hidden="true" tabindex="-1"></a>    rank<span class="op">:</span> Rank<span class="op">,</span></span>
<span id="cb2-10"><a href="#cb2-10" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;@suit&quot;</span><span class="at">)]</span></span>
<span id="cb2-11"><a href="#cb2-11" aria-hidden="true" tabindex="-1"></a>    suit<span class="op">:</span> Suit<span class="op">,</span></span>
<span id="cb2-12"><a href="#cb2-12" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-13"><a href="#cb2-13" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-14"><a href="#cb2-14" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-15"><a href="#cb2-15" aria-hidden="true" tabindex="-1"></a><span class="kw">enum</span> Rank <span class="op">{</span></span>
<span id="cb2-16"><a href="#cb2-16" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;2&quot;</span><span class="at">)]</span> _2<span class="op">,</span></span>
<span id="cb2-17"><a href="#cb2-17" aria-hidden="true" tabindex="-1"></a>    <span class="co">// ...</span></span>
<span id="cb2-18"><a href="#cb2-18" aria-hidden="true" tabindex="-1"></a>    K<span class="op">,</span></span>
<span id="cb2-19"><a href="#cb2-19" aria-hidden="true" tabindex="-1"></a>    A<span class="op">,</span></span>
<span id="cb2-20"><a href="#cb2-20" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-21"><a href="#cb2-21" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-22"><a href="#cb2-22" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-23"><a href="#cb2-23" aria-hidden="true" tabindex="-1"></a><span class="kw">enum</span> Suit <span class="op">{</span></span>
<span id="cb2-24"><a href="#cb2-24" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;♣&quot;</span><span class="at">)]</span> Clubs<span class="op">,</span></span>
<span id="cb2-25"><a href="#cb2-25" aria-hidden="true" tabindex="-1"></a>    <span class="co">// ...</span></span>
<span id="cb2-26"><a href="#cb2-26" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-27"><a href="#cb2-27" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-28"><a href="#cb2-28" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb2-29"><a href="#cb2-29" aria-hidden="true" tabindex="-1"></a>    card<span class="op">:</span> Card <span class="op">{</span> suit<span class="op">:</span> <span class="pp">Suit::</span>Clubs<span class="op">,</span> rank<span class="op">:</span> <span class="pp">Rank::</span>K <span class="op">},</span></span>
<span id="cb2-30"><a href="#cb2-30" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

## <a href="#sequences-of-choices-and-content" class="doc-anchor">§</a>Sequences of choices and `#content`

Sequences of choices can be handled various ways:

-   Without any configuration: a field named `item` with an enum type
    will be mapped to repeated element `<item>` containing a child
    element designating an enum variant and any parameters
    (`<item><variant-name>content</variant-name></item>`).
-   Using a field named `#content`: any child elements are treated as
    enum variants and are collected into the vector.
-   Container tag using an intermediate struct:
    `<items><item>...</item><item>...</item>...</items>`. The ergonomics
    of this option may be improved in the future. In the meantime, look
    at [serde-query](https://docs.rs/serde-query/latest/serde_query/).

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;message&gt;&lt;quit /&gt;&lt;/message&gt;
  &lt;message&gt;&lt;change-color rgb=&quot;0 0 255&quot; /&gt;&lt;/message&gt;
&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb2"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-2"><a href="#cb2-2" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>serde<span class="at">(</span>rename_all <span class="op">=</span> <span class="st">&quot;kebab-case&quot;</span><span class="at">)]</span></span>
<span id="cb2-3"><a href="#cb2-3" aria-hidden="true" tabindex="-1"></a><span class="kw">enum</span> Message <span class="op">{</span></span>
<span id="cb2-4"><a href="#cb2-4" aria-hidden="true" tabindex="-1"></a>    Quit<span class="op">,</span></span>
<span id="cb2-5"><a href="#cb2-5" aria-hidden="true" tabindex="-1"></a>    Move <span class="op">{</span> x<span class="op">:</span> <span class="dt">i32</span><span class="op">,</span> y<span class="op">:</span> <span class="dt">i32</span> <span class="op">},</span></span>
<span id="cb2-6"><a href="#cb2-6" aria-hidden="true" tabindex="-1"></a>    <span class="bu">Write</span>(<span class="dt">String</span>)<span class="op">,</span></span>
<span id="cb2-7"><a href="#cb2-7" aria-hidden="true" tabindex="-1"></a>    ChangeColor <span class="op">{</span></span>
<span id="cb2-8"><a href="#cb2-8" aria-hidden="true" tabindex="-1"></a>        <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;@rgb&quot;</span><span class="at">)]</span></span>
<span id="cb2-9"><a href="#cb2-9" aria-hidden="true" tabindex="-1"></a>        rgb<span class="op">:</span> (<span class="dt">i32</span><span class="op">,</span> <span class="dt">i32</span><span class="op">,</span> <span class="dt">i32</span>)</span>
<span id="cb2-10"><a href="#cb2-10" aria-hidden="true" tabindex="-1"></a>    <span class="op">},</span></span>
<span id="cb2-11"><a href="#cb2-11" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-12"><a href="#cb2-12" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-13"><a href="#cb2-13" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-14"><a href="#cb2-14" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb2-15"><a href="#cb2-15" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;message&quot;</span><span class="at">)]</span></span>
<span id="cb2-16"><a href="#cb2-16" aria-hidden="true" tabindex="-1"></a>    messages<span class="op">:</span> <span class="dt">Vec</span><span class="op">&lt;</span>Message<span class="op">&gt;,</span></span>
<span id="cb2-17"><a href="#cb2-17" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-18"><a href="#cb2-18" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-19"><a href="#cb2-19" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb2-20"><a href="#cb2-20" aria-hidden="true" tabindex="-1"></a>    messages<span class="op">:</span> <span class="pp">vec!</span>[<span class="pp">Message::</span>Quit<span class="op">,</span> <span class="pp">Message::</span>ChangeColor <span class="op">{</span> rgb<span class="op">:</span> (<span class="dv">0</span><span class="op">,</span> <span class="dv">0</span><span class="op">,</span> <span class="dv">255</span>) <span class="op">}</span>]<span class="op">,</span></span>
<span id="cb2-21"><a href="#cb2-21" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span></code></pre></div>
</div></td>
</tr>
<tr>
<td colspan="2"><p>Using <code>#content</code></p></td>
</tr>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;quit /&gt;
  &lt;change-color rgb=&quot;0 0 255&quot; /&gt;
&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb4"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb4-1"><a href="#cb4-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb4-2"><a href="#cb4-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb4-3"><a href="#cb4-3" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;#content&quot;</span><span class="at">)]</span></span>
<span id="cb4-4"><a href="#cb4-4" aria-hidden="true" tabindex="-1"></a>    messages<span class="op">:</span> <span class="dt">Vec</span><span class="op">&lt;</span>Message<span class="op">&gt;,</span></span>
<span id="cb4-5"><a href="#cb4-5" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb4-6"><a href="#cb4-6" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb4-7"><a href="#cb4-7" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb4-8"><a href="#cb4-8" aria-hidden="true" tabindex="-1"></a>    messages<span class="op">:</span> <span class="pp">vec!</span>[<span class="pp">Message::</span>Quit<span class="op">,</span> <span class="pp">Message::</span>ChangeColor <span class="op">{</span> rgb<span class="op">:</span> (<span class="dv">0</span><span class="op">,</span> <span class="dv">0</span><span class="op">,</span> <span class="dv">255</span>) <span class="op">}</span>]<span class="op">,</span></span>
<span id="cb4-9"><a href="#cb4-9" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span></code></pre></div>
</div></td>
</tr>
<tr>
<td colspan="2">Container element</td>
</tr>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document&gt;
  &lt;messages&gt;
    &lt;message&gt;&lt;quit /&gt;&lt;/message&gt;
    &lt;message&gt;&lt;change-color rgb=&quot;0 0 255&quot; /&gt;&lt;/message&gt;
  &lt;/messages&gt;
&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb6"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb6-1"><a href="#cb6-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb6-2"><a href="#cb6-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Messages <span class="op">{</span></span>
<span id="cb6-3"><a href="#cb6-3" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;message&quot;</span><span class="at">)]</span></span>
<span id="cb6-4"><a href="#cb6-4" aria-hidden="true" tabindex="-1"></a>    messages<span class="op">:</span> <span class="dt">Vec</span><span class="op">&lt;</span>Message<span class="op">&gt;</span></span>
<span id="cb6-5"><a href="#cb6-5" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb6-6"><a href="#cb6-6" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb6-7"><a href="#cb6-7" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb6-8"><a href="#cb6-8" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb6-9"><a href="#cb6-9" aria-hidden="true" tabindex="-1"></a>    messages<span class="op">:</span> Messages<span class="op">,</span></span>
<span id="cb6-10"><a href="#cb6-10" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb6-11"><a href="#cb6-11" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb6-12"><a href="#cb6-12" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb6-13"><a href="#cb6-13" aria-hidden="true" tabindex="-1"></a>    messages<span class="op">:</span> Messages <span class="op">{</span></span>
<span id="cb6-14"><a href="#cb6-14" aria-hidden="true" tabindex="-1"></a>        messages<span class="op">:</span> <span class="pp">vec!</span>[</span>
<span id="cb6-15"><a href="#cb6-15" aria-hidden="true" tabindex="-1"></a>            <span class="pp">Message::</span>Quit<span class="op">,</span></span>
<span id="cb6-16"><a href="#cb6-16" aria-hidden="true" tabindex="-1"></a>            <span class="pp">Message::</span>ChangeColor <span class="op">{</span> rgb<span class="op">:</span> (<span class="dv">0</span><span class="op">,</span> <span class="dv">0</span><span class="op">,</span> <span class="dv">255</span>) <span class="op">},</span></span>
<span id="cb6-17"><a href="#cb6-17" aria-hidden="true" tabindex="-1"></a>        ]<span class="op">,</span></span>
<span id="cb6-18"><a href="#cb6-18" aria-hidden="true" tabindex="-1"></a>    <span class="op">},</span></span>
<span id="cb6-19"><a href="#cb6-19" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

## <a href="#xml-namespaces" class="doc-anchor">§</a>XML Namespaces

Any XML namespaces declared in a document are mapped to a prefix. That
prefix can then appears in the names of attributes and elements. The
prefix must also appear in the names of the corresponding Rust fields.

-   **Deserialization:** Only prefixes matter. Any `xmlns...` attributes
    are ignored.
-   **Serialization:** The mapping between prefixes and namespace URI
    must be provided ([see
    SerdeXml::namespace](config/struct.SerdeXml.html#method.namespace "method serde_xml_rs::config::SerdeXml::namespace")).
    All namespaces are declared in the root element.

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>XML</th>
<th>Rust</th>
</tr>
</thead>
<tbody>
<tr>
<td><div class="example-wrap">
<pre class="language-xml"><code>&lt;Document xmlns=&quot;urn:example:default&quot; xmlns:a=&quot;urn:example:a&quot;&gt;
  &lt;a:a&gt;abc&lt;/a:a&gt;
  &lt;b&gt;123&lt;/b&gt;
  &lt;c a:id=&quot;456&quot; /&gt;
&lt;/Document&gt;</code></pre>
</div></td>
<td><div class="example-wrap">
<div class="sourceCode" id="cb2"><pre
class="sourceCode rust rust-example-rendered"><code class="sourceCode rust"><span id="cb2-1"><a href="#cb2-1" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-2"><a href="#cb2-2" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> Document <span class="op">{</span></span>
<span id="cb2-3"><a href="#cb2-3" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;a:a&quot;</span><span class="at">)]</span></span>
<span id="cb2-4"><a href="#cb2-4" aria-hidden="true" tabindex="-1"></a>    a<span class="op">:</span> <span class="dt">String</span><span class="op">,</span></span>
<span id="cb2-5"><a href="#cb2-5" aria-hidden="true" tabindex="-1"></a>    b<span class="op">:</span> <span class="dt">i32</span><span class="op">,</span></span>
<span id="cb2-6"><a href="#cb2-6" aria-hidden="true" tabindex="-1"></a>    c<span class="op">:</span> C<span class="op">,</span></span>
<span id="cb2-7"><a href="#cb2-7" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-8"><a href="#cb2-8" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-9"><a href="#cb2-9" aria-hidden="true" tabindex="-1"></a><span class="at">#[</span>derive<span class="at">(</span>Serialize<span class="op">,</span> Deserialize<span class="at">)]</span></span>
<span id="cb2-10"><a href="#cb2-10" aria-hidden="true" tabindex="-1"></a><span class="kw">struct</span> C <span class="op">{</span></span>
<span id="cb2-11"><a href="#cb2-11" aria-hidden="true" tabindex="-1"></a>    <span class="at">#[</span>serde<span class="at">(</span>rename <span class="op">=</span> <span class="st">&quot;@a:id&quot;</span><span class="at">)]</span></span>
<span id="cb2-12"><a href="#cb2-12" aria-hidden="true" tabindex="-1"></a>    id<span class="op">:</span> <span class="dt">i32</span><span class="op">,</span></span>
<span id="cb2-13"><a href="#cb2-13" aria-hidden="true" tabindex="-1"></a><span class="op">}</span></span>
<span id="cb2-14"><a href="#cb2-14" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-15"><a href="#cb2-15" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> value <span class="op">=</span> Document <span class="op">{</span></span>
<span id="cb2-16"><a href="#cb2-16" aria-hidden="true" tabindex="-1"></a>    a<span class="op">:</span> <span class="st">&quot;abc&quot;</span><span class="op">.</span>to_string()<span class="op">,</span></span>
<span id="cb2-17"><a href="#cb2-17" aria-hidden="true" tabindex="-1"></a>    b<span class="op">:</span> <span class="dv">123</span><span class="op">,</span></span>
<span id="cb2-18"><a href="#cb2-18" aria-hidden="true" tabindex="-1"></a>    c<span class="op">:</span> C <span class="op">{</span> id<span class="op">:</span> <span class="dv">456</span> <span class="op">},</span></span>
<span id="cb2-19"><a href="#cb2-19" aria-hidden="true" tabindex="-1"></a><span class="op">};</span></span>
<span id="cb2-20"><a href="#cb2-20" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-21"><a href="#cb2-21" aria-hidden="true" tabindex="-1"></a><span class="pp">assert_eq!</span>(<span class="pp">serde_xml_rs::from_str::</span><span class="op">&lt;</span>Document<span class="op">&gt;</span>(text)<span class="op">.</span>unwrap()<span class="op">,</span> value)<span class="op">;</span></span>
<span id="cb2-22"><a href="#cb2-22" aria-hidden="true" tabindex="-1"></a></span>
<span id="cb2-23"><a href="#cb2-23" aria-hidden="true" tabindex="-1"></a><span class="kw">let</span> config <span class="op">=</span> <span class="pp">serde_xml_rs::SerdeXml::</span>new()</span>
<span id="cb2-24"><a href="#cb2-24" aria-hidden="true" tabindex="-1"></a>    <span class="op">.</span>default_namespace(<span class="st">&quot;urn:example:default&quot;</span>)</span>
<span id="cb2-25"><a href="#cb2-25" aria-hidden="true" tabindex="-1"></a>    <span class="op">.</span>namespace(<span class="st">&quot;a&quot;</span><span class="op">,</span> <span class="st">&quot;urn:example:a&quot;</span>)<span class="op">;</span></span>
<span id="cb2-26"><a href="#cb2-26" aria-hidden="true" tabindex="-1"></a><span class="pp">assert_eq!</span>(config<span class="op">.</span>to_string(<span class="op">&amp;</span>value)<span class="op">.</span>unwrap()<span class="op">,</span> text)<span class="op">;</span></span></code></pre></div>
</div></td>
</tr>
</tbody>
</table>

# <a href="#custom-eventreader" class="doc-anchor">§</a>Custom EventReader

``` {.rust .rust-example-rendered}
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string, de::Deserializer};
use xml::reader::{EventReader, ParserConfig};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

let src = r#"<Item><name>  Banana  </name><source>Store</source></Item>"#;
let should_be = Item {
    name: "  Banana  ".to_string(),
    source: "Store".to_string(),
};

let config = ParserConfig::new()
    .trim_whitespace(false)
    .whitespace_to_characters(true);
let event_reader = EventReader::new_with_config(src.as_bytes(), config);
let item = Item::deserialize(&mut Deserializer::new(event_reader)).unwrap();
assert_eq!(item, should_be);
```

# <a href="#supported-encodings" class="doc-anchor">§</a>Supported Encodings

This crate relies on the `xml` crate for parsing XML. It therefore
supports the same encodings, namely:

-   UTF-8 and UTF-16 (minimum requirement for the XML standard)
    -   The UTF-16 file must contain a byte-order mark
-   ISO-8859-1
-   ASCII

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::config::`<a href="config/struct.SerdeXml.html" class="struct"
title="struct serde_xml_rs::config::SerdeXml"><code>SerdeXml</code></a>`;`

`pub use crate::de::`<a href="de/fn.from_reader.html" class="fn"
title="fn serde_xml_rs::de::from_reader"><code>from_reader</code></a>`;`

`pub use crate::de::`<a href="de/fn.from_str.html" class="fn"
title="fn serde_xml_rs::de::from_str"><code>from_str</code></a>`;`

`pub use crate::de::`<a href="de/struct.Deserializer.html" class="struct"
title="struct serde_xml_rs::de::Deserializer"><code>Deserializer</code></a>`;`

`pub use crate::ser::`<a href="ser/fn.to_string.html" class="fn"
title="fn serde_xml_rs::ser::to_string"><code>to_string</code></a>`;`

`pub use crate::ser::`<a href="ser/fn.to_writer.html" class="fn"
title="fn serde_xml_rs::ser::to_writer"><code>to_writer</code></a>`;`

`pub use crate::ser::`<a href="ser/struct.Serializer.html" class="struct"
title="struct serde_xml_rs::ser::Serializer"><code>Serializer</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="config/index.html" class="mod"
title="mod serde_xml_rs::config">config</a>

<a href="de/index.html" class="mod" title="mod serde_xml_rs::de">de</a>

<a href="ser/index.html" class="mod"
title="mod serde_xml_rs::ser">ser</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum serde_xml_rs::Error">Error</a>
