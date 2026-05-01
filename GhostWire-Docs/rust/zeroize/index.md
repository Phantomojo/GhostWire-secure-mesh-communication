Crate zeroize Copy item path

<span class="sub-heading"><a href="../src/zeroize/lib.rs.html#1-875" class="src">Source</a>
</span>

Expand description

Securely zero memory with a simple trait
([`Zeroize`](trait.Zeroize.html "trait zeroize::Zeroize")) built on
stable Rust primitives which guarantee the operation will not be
“optimized away”.

## <a href="#about" class="doc-anchor">§</a>About

[Zeroing memory securely is
hard](http://www.daemonology.net/blog/2014-09-04-how-to-zero-a-buffer.html) -
compilers optimize for performance, and in doing so they love to
“optimize away” unnecessary zeroing calls. There are many documented
“tricks” to attempt to avoid these optimizations and ensure that a
zeroing routine is performed reliably.

This crate isn’t about tricks: it uses
[`core::ptr::write_volatile`](https://doc.rust-lang.org/1.93.1/core/ptr/fn.write_volatile.html "fn core::ptr::write_volatile")
and
[`core::sync::atomic`](https://doc.rust-lang.org/1.93.1/core/sync/atomic/index.html "mod core::sync::atomic")
memory fences to provide easy-to-use, portable zeroing behavior which
works on all of Rust’s core number types and slices thereof, implemented
in pure Rust with no usage of FFI or assembly.

-   No insecure fallbacks!
-   No dependencies!
-   No FFI or inline assembly! **WASM friendly** (and tested)!
-   `#![no_std]` i.e. **embedded-friendly**!
-   No functionality besides securely zeroing memory!
-   (Optional) Custom derive support for zeroing complex structures

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

Requires Rust **1.72** or newer.

In the future, we reserve the right to change MSRV (i.e. MSRV is
out-of-scope for this crate’s SemVer guarantees), however when we do it
will be accompanied by a minor version bump.

## <a href="#usage" class="doc-anchor">§</a>Usage

``` {.rust .rust-example-rendered}
use zeroize::Zeroize;

// Protip: don't embed secrets in your source code.
// This is just an example.
let mut secret = b"Air shield password: 1,2,3,4,5".to_vec();
// [ ... ] open the air shield here

// Now that we're done using the secret, zero it out.
secret.zeroize();
```

The [`Zeroize`](trait.Zeroize.html "trait zeroize::Zeroize") trait is
impl’d on all of Rust’s core scalar types including integers, floats,
`bool`, and `char`.

Additionally, it’s implemented on slices and `IterMut`s of the above
types.

When the `alloc` feature is enabled (which it is by default), it’s also
impl’d for `Vec<T>` for the above types as well as `String`, where it
provides
[`Vec::clear`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html#method.clear "method alloc::vec::Vec::clear")
/
[`String::clear`](https://doc.rust-lang.org/1.93.1/alloc/string/struct.String.html#method.clear "method alloc::string::String::clear")-like
behavior (truncating to zero-length) but ensures the backing memory is
securely zeroed with some caveats.

With the `std` feature enabled (which it is **not** by default),
[`Zeroize`](trait.Zeroize.html "trait zeroize::Zeroize") is also
implemented for
[`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).
After calling `zeroize()` on a `CString`, its internal buffer will
contain exactly one nul byte. The backing memory is zeroed by converting
it to a `Vec<u8>` and back into a `CString`. (NOTE: see “Stack/Heap
Zeroing Notes” for important `Vec`/`String`/`CString` details)

The
[`DefaultIsZeroes`](trait.DefaultIsZeroes.html "trait zeroize::DefaultIsZeroes")
marker trait can be impl’d on types which also impl
[`Default`](https://doc.rust-lang.org/1.93.1/core/default/trait.Default.html "trait core::default::Default"),
which implements
[`Zeroize`](trait.Zeroize.html "trait zeroize::Zeroize") by overwriting
a value with the default value.

## <a href="#custom-derive-support" class="doc-anchor">§</a>Custom Derive Support

This crate has custom derive support for the `Zeroize` trait, gated
under the `zeroize` crate’s `zeroize_derive` Cargo feature, which
automatically calls `zeroize()` on all members of a struct or tuple
struct.

Attributes supported for `Zeroize`:

On the item level:

-   `#[zeroize(drop)]`: *deprecated* use `ZeroizeOnDrop` instead
-   `#[zeroize(bound = "T: MyTrait")]`: this replaces any trait bounds
    inferred by zeroize

On the field level:

-   `#[zeroize(skip)]`: skips this field or variant when calling
    `zeroize()`

Attributes supported for `ZeroizeOnDrop`:

On the field level:

-   `#[zeroize(skip)]`: skips this field or variant when calling
    `zeroize()`

Example which derives `Drop`:

``` {.rust .rust-example-rendered}
use zeroize::{Zeroize, ZeroizeOnDrop};

// This struct will be zeroized on drop
#[derive(Zeroize, ZeroizeOnDrop)]
struct MyStruct([u8; 32]);
```

Example which does not derive `Drop` (useful for e.g. `Copy` types)

``` {.rust .rust-example-rendered}
#[cfg(feature = "zeroize_derive")]
use zeroize::Zeroize;

// This struct will *NOT* be zeroized on drop
#[derive(Copy, Clone, Zeroize)]
struct MyStruct([u8; 32]);
```

Example which only derives `Drop`:

``` {.rust .rust-example-rendered}
use zeroize::ZeroizeOnDrop;

// This struct will be zeroized on drop
#[derive(ZeroizeOnDrop)]
struct MyStruct([u8; 32]);
```

## <a href="#zeroizingz-wrapper-for-zeroizing-arbitrary-values-on-drop"
class="doc-anchor">§</a>`Zeroizing<Z>`: wrapper for zeroizing arbitrary values on drop

`Zeroizing<Z: Zeroize>` is a generic wrapper type that impls `Deref` and
`DerefMut`, allowing access to an inner value of type `Z`, and also
impls a `Drop` handler which calls `zeroize()` on its contents:

``` {.rust .rust-example-rendered}
use zeroize::Zeroizing;

fn use_secret() {
    let mut secret = Zeroizing::new([0u8; 5]);

    // Set the air shield password
    // Protip (again): don't embed secrets in your source code.
    secret.copy_from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(secret.as_ref(), &[1, 2, 3, 4, 5]);

    // The contents of `secret` will be automatically zeroized on drop
}
```

## <a href="#what-guarantees-does-this-crate-provide"
class="doc-anchor">§</a>What guarantees does this crate provide?

This crate guarantees the following:

1.  The zeroing operation can’t be “optimized away” by the compiler.
2.  All subsequent reads to memory will see “zeroized” values.

LLVM’s volatile semantics ensure \#1 is true.

Additionally, thanks to work by the [Unsafe Code Guidelines Working
Group](https://github.com/rust-lang/unsafe-code-guidelines), we can now
fairly confidently say \#2 is true as well. Previously there were
worries that the approach used by this crate (mixing volatile and
non-volatile accesses) was undefined behavior due to language contained
in the documentation for `write_volatile`, however after some discussion
[these remarks have been
removed](https://github.com/rust-lang/rust/pull/60972) and the specific
usage pattern in this crate is considered to be well-defined.

Additionally this crate leverages
[`core::sync::atomic::compiler_fence`](https://doc.rust-lang.org/1.93.1/core/sync/atomic/fn.compiler_fence.html "fn core::sync::atomic::compiler_fence")
with the strictest ordering
([`Ordering::SeqCst`](https://doc.rust-lang.org/1.93.1/core/sync/atomic/enum.Ordering.html#variant.SeqCst "variant core::sync::atomic::Ordering::SeqCst"))
as a precaution to help ensure reads are not reordered before memory has
been zeroed.

All of that said, there is still potential for microarchitectural
attacks (ala Spectre/Meltdown) to leak “zeroized” secrets through covert
channels. This crate makes no guarantees that zeroized values cannot be
leaked through such channels, as they represent flaws in the underlying
hardware.

## <a href="#stackheap-zeroing-notes" class="doc-anchor">§</a>Stack/Heap Zeroing Notes

This crate can be used to zero values from either the stack or the heap.

However, be aware several operations in Rust can unintentionally leave
copies of data in memory. This includes but is not limited to:

-   Moves and
    [`Copy`](https://doc.rust-lang.org/1.93.1/core/marker/trait.Copy.html "trait core::marker::Copy")
-   Heap reallocation when using
    [`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")
    and
    [`String`](https://doc.rust-lang.org/1.93.1/alloc/string/struct.String.html "struct alloc::string::String")
-   Borrowers of a reference making copies of the data

[`Pin`](https://doc.rust-lang.org/1.93.1/core/pin/struct.Pin.html "struct core::pin::Pin")
can be leveraged in conjunction with this crate to ensure data kept on
the stack isn’t moved.

The `Zeroize` impls for `Vec`, `String` and `CString` zeroize the entire
capacity of their backing buffer, but cannot guarantee copies of the
data were not previously made by buffer reallocation. It’s therefore
important when attempting to zeroize such buffers to initialize them to
the correct capacity, and take care to prevent subsequent reallocation.

The `secrecy` crate provides higher-level abstractions for eliminating
usage patterns which can cause reallocations:

<https://crates.io/crates/secrecy>

## <a href="#what-about-clearing-registers-mlock-mprotect-etc"
class="doc-anchor">§</a>What about: clearing registers, mlock, mprotect, etc?

This crate is focused on providing simple, unobtrusive support for
reliably zeroing memory using the best approach possible on stable Rust.

Clearing registers is a difficult problem that can’t easily be solved by
something like a crate, and requires either inline ASM or rustc support.
See <https://github.com/rust-lang/rust/issues/17046> for background on
this particular problem.

Other memory protection mechanisms are interesting and useful, but often
overkill (e.g. defending against RAM scraping or attackers with swap
access). In as much as there may be merit to these approaches, there are
also many other crates that already implement more sophisticated memory
protections. Such protections are explicitly out-of-scope for this
crate.

Zeroing memory is [good cryptographic
hygiene](https://github.com/veorq/cryptocoding#clean-memory-of-secret-data)
and this crate seeks to promote it in the most unobtrusive manner
possible. This includes omitting complex `unsafe` memory protection
systems and just trying to make the best memory zeroing crate available.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Zeroizing.html" class="struct"
title="struct zeroize::Zeroizing">Zeroizing</a>  
`Zeroizing` is a a wrapper for any `Z: Zeroize` type which implements a
`Drop` handler which zeroizes dropped values.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.DefaultIsZeroes.html" class="trait"
title="trait zeroize::DefaultIsZeroes">DefaultIsZeroes</a>  
Marker trait for types whose
[`Default`](https://doc.rust-lang.org/1.93.1/core/default/trait.Default.html "trait core::default::Default")
is the desired zeroization result

<a href="trait.TryZeroize.html" class="trait"
title="trait zeroize::TryZeroize">TryZeroize</a>  
Fallible trait for representing cases where zeroization may or may not
be possible.

<a href="trait.Zeroize.html" class="trait"
title="trait zeroize::Zeroize">Zeroize</a>  
Trait for securely erasing values from memory.

<a href="trait.ZeroizeOnDrop.html" class="trait"
title="trait zeroize::ZeroizeOnDrop">ZeroizeOnDrop</a>  
Marker trait signifying that this type will
[`Zeroize::zeroize`](trait.Zeroize.html#tymethod.zeroize "method zeroize::Zeroize::zeroize")
itself on
[`Drop`](https://doc.rust-lang.org/1.93.1/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop").

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.zeroize_flat_type.html" class="fn"
title="fn zeroize::zeroize_flat_type">zeroize_flat_type</a><sup>⚠</sup>  
Zeroizes a flat type/struct. Only zeroizes the values that it owns, and
it does not work on dynamically sized values or trait objects. It would
be inefficient to use this function on a type that already implements
`ZeroizeOnDrop`.

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.Zeroize.html" class="derive"
title="derive zeroize::Zeroize">Zeroize</a>  
Derive the `Zeroize` trait.

<a href="derive.ZeroizeOnDrop.html" class="derive"
title="derive zeroize::ZeroizeOnDrop">ZeroizeOnDrop</a>  
Derive the `ZeroizeOnDrop` trait.
