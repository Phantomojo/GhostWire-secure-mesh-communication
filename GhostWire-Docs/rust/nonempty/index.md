Crate nonempty Copy item path

<span class="sub-heading"><a href="../src/nonempty/lib.rs.html#1-885" class="src">Source</a>
</span>

Expand description

A Non-empty growable vector.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use nonempty::NonEmpty;

let mut l = NonEmpty { head: 42, tail: vec![36, 58] };

assert_eq!(l.head, 42);

l.push(9001);

assert_eq!(l.last(), &9001);

let v: Vec<i32> = l.into();
assert_eq!(v, vec![42, 36, 58, 9001]);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.NonEmpty.html" class="struct"
title="struct nonempty::NonEmpty">NonEmpty</a>
