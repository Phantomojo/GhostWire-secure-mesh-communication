Crate archery Copy item path

<span class="sub-heading"><a href="../src/archery/lib.rs.html#1-139" class="src">Source</a>
</span>

Expand description

`archery` is a rust library that offers a way to abstraction over
[`Rc`](https://doc.rust-lang.org/1.93.1/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")
and
[`Arc`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")
smart pointers. This allows you to create data structures where the
pointer type is parameterizable, so you can [avoid the overhead of
`Arc`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html#thread-safety "struct alloc::sync::Arc")
when you don’t need to share data across threads.

In languages that supports [higher-kinded
polymorphism](https://en.wikipedia.org/wiki/Type_class#Higher-kinded_polymorphism)
this would be simple to achieve without any library, but [rust does not
support that yet](https://github.com/rust-lang/rfcs/issues/324). To
mimic higher-kinded polymorphism `archery` implements the approach
suggested by Joshua Liebow-Feeser in “[Rust has higher kinded types
already… sort
of](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/)”.
While [other approaches](#alternative-approaches) exist, they seem to
always offer poor ergonomics for the user.

# <a href="#setup" class="doc-anchor">§</a>Setup

To use `archery` add the following to your `Cargo.toml`:

``` language-toml
[dependencies]
archery = "<version>"
```

# <a href="#using-archery" class="doc-anchor">§</a>Using `archery`

`archery` defines a
[`SharedPointer`](shared_pointer/struct.SharedPointer.html "struct archery::shared_pointer::SharedPointer")
that receives the [kind of
pointer](shared_pointer/kind/trait.SharedPointerKind.html "trait archery::shared_pointer::kind::SharedPointerKind")
as a type parameter. This gives you a convenient and ergonomic way to
abstract the pointer type away.

## <a href="#example" class="doc-anchor">§</a>Example

Declare a data structure with the pointer kind as a type parameter
bounded by
[`SharedPointerKind`](shared_pointer/kind/trait.SharedPointerKind.html "trait archery::shared_pointer::kind::SharedPointerKind"):

``` {.rust .rust-example-rendered}
use archery::*;

struct KeyValuePair<K, V, P: SharedPointerKind> {
    pub key: SharedPointer<K, P>,
    pub value: SharedPointer<V, P>,
}

impl<K, V, P: SharedPointerKind> KeyValuePair<K, V, P> {
    fn new(key: K, value: V) -> KeyValuePair<K, V, P> {
        KeyValuePair {
            key: SharedPointer::new(key),
            value: SharedPointer::new(value),
        }
    }
}
```

To use it just plug-in the kind of pointer you want:

``` {.rust .rust-example-rendered}
let pair: KeyValuePair<_, _, RcK> =
    KeyValuePair::new("António Variações", 1944);

assert_eq!(*pair.value, 1944);
```

## <a href="#triomphearc" class="doc-anchor">§</a>`triomphe::Arc`

You can also use
[`triomphe::Arc`](https://docs.rs/triomphe/latest/triomphe/struct.Arc.html)
as the backing implementation of a
[`SharedPointer`](shared_pointer/struct.SharedPointer.html "struct archery::shared_pointer::SharedPointer").
This is generally faster than
[`std::sync::Arc`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc").
Read [`triomphe`’s crate
documentation](https://docs.rs/triomphe/latest/triomphe/) to learn more
about it.

To use it you need to enable the `triomphe` feature in `archery`. Use
`ArcTK` as the pointer kind in
[`SharedPointer`](shared_pointer/struct.SharedPointer.html "struct archery::shared_pointer::SharedPointer").

## <a href="#serialization" class="doc-anchor">§</a>Serialization

We support serialization through
[serde](https://crates.io/crates/serde). To use it enable the `serde`
feature. To do so change the archery dependency in your `Cargo.toml` to

``` language-toml
[dependencies]
archery = { version = "<version>", features = ["serde"] }
```

# <a href="#limitations" class="doc-anchor">§</a>Limitations

Currently it is not possible to have unsized types inside a
[`SharedPointer`](shared_pointer/struct.SharedPointer.html "struct archery::shared_pointer::SharedPointer").
As a workaround you can put the unsized type inside a
[`Box`](https://doc.rust-lang.org/1.93.1/alloc/boxed/struct.Box.html "struct alloc::boxed::Box").

# <a href="#alternative-approaches" class="doc-anchor">§</a>Alternative approaches

An alternative to the approach taken by `archery` is to use traits with
associated types to encode type-level functions. This has been suggested
[multiple](https://github.com/orium/rpds/issues/7#issuecomment-362635901)
[times](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/#comment-4160863400),
but offers ugly ergonomics (see
[here](https://github.com/Marwes/rpds/blob/e482d5abbaa6c876d7c624e497affe7299bbeece/src/sequence/vector/mod.rs#L153)
and
[here](https://github.com/Marwes/rpds/blob/e482d5abbaa6c876d7c624e497affe7299bbeece/src/sequence/vector/mod.rs#L249)).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use shared_pointer::`<a href="shared_pointer/struct.SharedPointer.html" class="struct"
title="struct archery::shared_pointer::SharedPointer"><code>SharedPointer</code></a>`;`

`pub use shared_pointer::kind::`<a href="shared_pointer/kind/trait.SharedPointerKind.html" class="trait"
title="trait archery::shared_pointer::kind::SharedPointerKind"><code>SharedPointerKind</code></a>`;`

`pub use shared_pointer::kind::`<a href="shared_pointer/kind/struct.ArcK.html" class="struct"
title="struct archery::shared_pointer::kind::ArcK"><code>ArcK</code></a>`;`

`pub use shared_pointer::kind::`<a href="shared_pointer/kind/struct.RcK.html" class="struct"
title="struct archery::shared_pointer::kind::RcK"><code>RcK</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="shared_pointer/index.html" class="mod"
title="mod archery::shared_pointer">shared_pointer</a>
