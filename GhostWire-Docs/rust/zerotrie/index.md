Crate zerotrie Copy item path

<span class="sub-heading"><a href="../src/zerotrie/lib.rs.html#5-87" class="src">Source</a>
</span>

Expand description

A data structure offering zero-copy storage and retrieval of byte
strings, with a focus on the efficient storage of ASCII strings. Strings
are mapped to `usize` values.

[`ZeroTrie`](struct.ZeroTrie.html "struct zerotrie::ZeroTrie") does not
support mutation because doing so would require recomputing the entire
data structure. Instead, it supports conversion to and from
[`LiteMap`](litemap::LiteMap) and
[`BTreeMap`](alloc::collections::BTreeMap).

There are multiple variants of
[`ZeroTrie`](struct.ZeroTrie.html "struct zerotrie::ZeroTrie") optimized
for different use cases.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use zerotrie::ZeroTrie;

let data: &[(&str, usize)] = &[("abc", 11), ("xyz", 22), ("axyb", 33)];

let trie: ZeroTrie<Vec<u8>> = data.iter().copied().collect();

assert_eq!(trie.get("axyb"), Some(33));
assert_eq!(trie.byte_len(), 18);
```

# <a href="#internal-structure" class="doc-anchor">§</a>Internal Structure

To read about the internal structure of
[`ZeroTrie`](struct.ZeroTrie.html "struct zerotrie::ZeroTrie"), build
the docs with private modules:

``` language-bash
cargo doc --document-private-items --all-features --no-deps --open
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="cursor/index.html" class="mod"
title="mod zerotrie::cursor">cursor</a>  
Types for walking stepwise through a trie.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ZeroAsciiIgnoreCaseTrie.html" class="struct"
title="struct zerotrie::ZeroAsciiIgnoreCaseTrie">ZeroAsciiIgnoreCaseTrie</a>  
A data structure that compactly maps from ASCII strings to integers in a
case-insensitive way.

<a href="struct.ZeroTrie.html" class="struct"
title="struct zerotrie::ZeroTrie">ZeroTrie</a>  
A data structure that compactly maps from byte sequences to integers.

<a href="struct.ZeroTrieExtendedCapacity.html" class="struct"
title="struct zerotrie::ZeroTrieExtendedCapacity">ZeroTrieExtendedCapacity</a>  
A data structure that maps from a large number of byte strings to
integers.

<a href="struct.ZeroTriePerfectHash.html" class="struct"
title="struct zerotrie::ZeroTriePerfectHash">ZeroTriePerfectHash</a>  
A data structure that compactly maps from byte strings to integers.

<a href="struct.ZeroTrieSimpleAscii.html" class="struct"
title="struct zerotrie::ZeroTrieSimpleAscii">ZeroTrieSimpleAscii</a>  
A data structure that compactly maps from ASCII strings to integers.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ZeroTrieBuildError.html" class="enum"
title="enum zerotrie::ZeroTrieBuildError">ZeroTrieBuildError</a>  
Error types for the `zerotrie` crate.
