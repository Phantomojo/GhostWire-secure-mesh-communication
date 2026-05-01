Crate opaque\_debug Copy item path

<span class="sub-heading"><a href="../src/opaque_debug/lib.rs.html#1-81" class="src">Source</a>
</span>

Expand description

Macro for opaque
[`Debug`](https://doc.rust-lang.org/1.93.1/core/fmt/macros/derive.Debug.html "derive core::fmt::macros::Debug")
trait implementation.

In many cases it’s convenient to have `Debug` implementation for all
crate types, e.g. to allow deriving of `Debug` in user-defined structs.
But at the same time, using the default derive macro can be a security
hazard since it cause leaking of sensitive information, for example,
through uncareful logging.

This crate introduces the
[`implement!`](macro.implement.html "macro opaque_debug::implement")
macro which creates an opaque
[`Debug`](https://doc.rust-lang.org/1.93.1/core/fmt/macros/derive.Debug.html "derive core::fmt::macros::Debug")
implementation, which does not expose any internal type data.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
pub struct CryptoStuff {
    key: [u8; 16],
}

opaque_debug::implement!(CryptoStuff);

let val = CryptoStuff { key: [42; 16] };
assert_eq!(format!("{:?}", val), "CryptoStuff { ... }")
```

The macro also support generic paramters:

``` {.rust .rust-example-rendered}
pub struct GenricCryptoStuff<K> {
    key: K,
}

opaque_debug::implement!(GenricCryptoStuff<K>);

let val = GenricCryptoStuff { key: [42u8; 16] };
assert_eq!(format!("{:?}", val), "GenricCryptoStuff<[u8; 16]> { ... }")
```

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.implement.html" class="macro"
title="macro opaque_debug::implement">implement</a>  
Macro for implementing an opaque `Debug` implementation.
