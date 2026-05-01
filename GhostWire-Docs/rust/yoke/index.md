Crate yoke Copy item path

<span class="sub-heading"><a href="../src/yoke/lib.rs.html#5-68" class="src">Source</a>
</span>

Expand description

This crate provides
[`Yoke<Y, C>`](struct.Yoke.html "struct yoke::Yoke"), which allows one
to “yoke” (attach) a zero-copy deserialized object (say, a
[`Cow<'a, str>`](alloc::borrow::Cow)) to the source it was deserialized
from, (say, an [`Rc<[u8]>`](alloc::rc::Rc)), known in this crate as a
“cart”, producing a type that looks like
`Yoke<Cow<'static, str>, Rc<[u8]>>` and can be moved around with
impunity.

Succinctly, this allows one to “erase” static lifetimes and turn them
into dynamic ones, similarly to how `dyn` allows one to “erase” static
types and turn them into dynamic ones.

Most of the time the yokeable `Y` type will be some kind of zero-copy
deserializable abstraction, potentially with an owned variant (like
[`Cow`](alloc::borrow::Cow), [`ZeroVec`](https://docs.rs/zerovec), or an
aggregate containing such types), and the cart `C` will be some smart
pointer like [`Box<T>`](alloc::boxed::Box), [`Rc<T>`](alloc::rc::Rc), or
[`Arc<T>`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc"),
potentially wrapped in an
[`Option<T>`](https://doc.rust-lang.org/1.93.1/core/option/enum.Option.html "enum core::option::Option").

The key behind this crate is
[`Yoke::get()`](struct.Yoke.html#method.get "method yoke::Yoke::get"),
where calling
[`.get()`](struct.Yoke.html#method.get "method yoke::Yoke::get") on a
type like `Yoke<Cow<'static, str>, _>` will get you a short-lived
`&'a Cow<'a, str>`, restricted to the lifetime of the borrow used during
[`.get()`](struct.Yoke.html#method.get "method yoke::Yoke::get"). This
is entirely safe since the `Cow` borrows from the cart type `C`, which
cannot be interfered with as long as the `Yoke` is borrowed by
[`.get()`](struct.Yoke.html#method.get "method yoke::Yoke::get").
[`.get()`](struct.Yoke.html#method.get "method yoke::Yoke::get")
protects access by essentially reifying the erased lifetime to a safe
local one when necessary.

See the documentation of [`Yoke`](struct.Yoke.html "struct yoke::Yoke")
for more details.

# Modules<a href="#modules" class="anchor">§</a>

<a href="cartable_ptr/index.html" class="mod"
title="mod yoke::cartable_ptr">cartable_ptr</a>  
Types for optional pointers with niche optimization.

<a href="either/index.html" class="mod"
title="mod yoke::either">either</a>  
Types to enable polymorphic carts.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Yoke.html" class="struct"
title="struct yoke::Yoke">Yoke</a>  
A Cow-like borrowed object “yoked” to its backing data.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.CloneableCart.html" class="trait"
title="trait yoke::CloneableCart">CloneableCart</a>  
This trait marks cart types that do not change source on cloning

<a href="trait.Yokeable.html" class="trait"
title="trait yoke::Yokeable">Yokeable</a>  
The `Yokeable<'a>` trait is implemented on the `'static` version of any
zero-copy type; for example, `Cow<'static, T>` implements `Yokeable<'a>`
(for all `'a`).

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.Yokeable.html" class="derive"
title="derive yoke::Yokeable">Yokeable</a>  
Custom derive for `yoke::Yokeable`,
