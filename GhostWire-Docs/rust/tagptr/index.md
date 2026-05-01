Crate tagptr Copy item path

<span class="sub-heading"><a href="../src/tagptr/lib.rs.html#1-225" class="src">Source</a>
</span>

Expand description

Strongly typed pointers with reserved space for storing additional bit
patterns within the same memory word.

# <a href="#motivation" class="doc-anchor">§</a>Motivation

In low-level concurrent programming (synchronization primitives,
lock-free data structures, etc) it is often required to store additional
state information (tags) alongside pointers to objects in memory, since
most atomic CPU instructions operate on pointer-wide memory words. The
marked pointer types provided by this crate encapsulate the logic and
pointer arithmetic for composing (creating), decomposing and mutating
such pointers and tag values.

# <a href="#tag-bits-and-type-alignment" class="doc-anchor">§</a>Tag Bits and Type Alignment

The possible space for storing tag bits in a pointer is determined by
the alignment of the pointed-to type, as long as the pointer is
well-aligned (e.g., not packed). For instance, pointers to types with an
alignment of 2 (2^1) bytes (e.g., `u16`) never use the first of their
lower bits (i.e., it is always zero), pointers to types with an
alignment of 8 (2^3) bytes such as `u64` never use their 3 lowest bits
and so on. Great care must be taken at all times to avoid over- or
underflows in the usually highly restricted range of valid tags for
common tag sizes when doing arithmetic operations. Any operations
resulting in tag values outside of their valid range will invariably
corrupt the bits representing the pointer and hence invoke undefined
behavior when dereferencing these pointers.

Constructing a type such as `TagPtr<u64, 4>` is hence usually a user
error, since a pointer to a `u64` has only 3 unused bits. The resulting
type would consider the first actual bit of the pointer to be part of
its tag and return a potentially corrupted pointer in methods such as
[`decompose`](struct.TagPtr.html#method.decompose "method tagptr::TagPtr::decompose").
The
[`has_sufficient_alignment`](fn.has_sufficient_alignment.html "fn tagptr::has_sufficient_alignment")
and
[`assert_alignment`](fn.assert_alignment.html "fn tagptr::assert_alignment")
functions can be used to explicitly check for or assert this property.
There is, however, one exception where using an otherwise ill-formed tag
pointer type is valid: After composing a well-formed tag pointer
instance (e.g., `TagPtr<u64, 3>`) it is valid to
[`cast`](struct.TagPtr.html#method.cast "method tagptr::TagPtr::cast")
it to a type with a smaller alignment and the same number of tag bits
such as `TagPtr<(), 3>` for the purpose of type-erasure.

# <a href="#example" class="doc-anchor">§</a>Example

Storing a boolean status flag alongside the pointer to a mutable `u64`:

``` {.rust .rust-example-rendered}
type TagPtr = tagptr::TagPtr<u64, 3>;

let mut val = 0xCAFE;
let is_ok = true;

let ptr = TagPtr::compose(&mut val, is_ok as usize);
let (reference, tag) = unsafe { ptr.decompose_mut() };
assert_eq!(reference, Some(&mut 0xCAFE));
assert_eq!(tag == 1, true);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AtomicTagPtr.html" class="struct"
title="struct tagptr::AtomicTagPtr">AtomicTagPtr</a>  
A raw pointer type which can be safely shared between threads and which
can use up to `N` of its lower bits to store additional information (the
*tag*).

<a href="struct.Null.html" class="struct"
title="struct tagptr::Null">Null</a>  
A type representing a `null` pointer with potential tag bits.

<a href="struct.TagNonNull.html" class="struct"
title="struct tagptr::TagNonNull">TagNonNull</a>  
A non-nullable tagged raw pointer type similar to
[`NonNull`](https://doc.rust-lang.org/1.93.1/core/ptr/non_null/struct.NonNull.html "struct core::ptr::non_null::NonNull")
which can use up to `N` of its lower bits to store additional
information (the *tag*).

<a href="struct.TagPtr.html" class="struct"
title="struct tagptr::TagPtr">TagPtr</a>  
A raw, unsafe pointer type like `*mut T` which can use up to `N` of its
lower bits to store additional information (the *tag*).

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.assert_alignment.html" class="fn"
title="fn tagptr::assert_alignment">assert_alignment</a>  
Asserts that the alignment of `U` is large enough so a pointer to an
instance may store `N` tag bits.

<a href="fn.has_sufficient_alignment.html" class="fn"
title="fn tagptr::has_sufficient_alignment">has_sufficient_alignment</a>  
Returns `true` if the alignment of `T` is large enough so a pointer to
an instance may store the given number of `tag_bits`.
