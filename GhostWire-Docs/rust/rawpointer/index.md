Crate rawpointer Copy item path

<span class="sub-heading"><a href="../src/rawpointer/lib.rs.html#8-246" class="src">Source</a>
</span>

Expand description

Rawpointer adds extra utility methods to raw pointers `*const T`,
`*mut T` and `NonNull<T>`.

Features include:

-   Strided offsets -
    [`.stride_offset(stride, index)`](trait.PointerExt.html#method.stride_offset "method rawpointer::PointerExt::stride_offset")
    make it easy to compute pointer offsets where the index is unsigned
    and the stride is signed.

-   Offsetting methods in general for `NonNull`, since it does not have
    these from libcore

-   Post- and preincrement and post- and predecrement methods

    -   For `p++` use
        [`p.post_inc()`](trait.PointerExt.html#method.post_inc "method rawpointer::PointerExt::post_inc").
    -   For `++p` use
        [`p.pre_inc()`](trait.PointerExt.html#method.pre_inc "method rawpointer::PointerExt::pre_inc").
    -   For `p--` use
        [`p.post_dec()`](trait.PointerExt.html#method.post_dec "method rawpointer::PointerExt::post_dec").
    -   For `--p` use
        [`p.pre_dec()`](trait.PointerExt.html#method.pre_dec "method rawpointer::PointerExt::pre_dec").

``` {.rust .rust-example-rendered}
use rawpointer::PointerExt;

unsafe {
    // In this example:
    // Use .post_inc() to iterate and overwrite the first four
    // elements of the array.

    let mut xs = [0; 16];
    let mut ptr = xs.as_mut_ptr();
    let end = ptr.offset(4);
    let mut i = 0;
    while ptr != end {
        *ptr.post_inc() = i;
        i += 1;
    }
    assert_eq!(&xs[..8], &[0, 1, 2, 3, 0, 0, 0, 0]);
}
```

## <a href="#safety" class="doc-anchor">§</a>Safety

See the Rust
[core::ptr](https://doc.rust-lang.org/1.93.1/core/ptr/index.html "mod core::ptr")
documentation for more information.

## <a href="#rust-version" class="doc-anchor">§</a>Rust Version

This version of the crate requires Rust 1.26 or later

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.PointerExt.html" class="trait"
title="trait rawpointer::PointerExt">PointerExt</a>  
Extension methods for raw pointers

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.ptrdistance.html" class="fn"
title="fn rawpointer::ptrdistance">ptrdistance</a>  
Return the number of elements of `T` from `start` to `end`.  
Return the arithmetic difference if `T` is zero size.
