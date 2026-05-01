Crate cfg\_if Copy item path

<span class="sub-heading"><a href="../src/cfg_if/lib.rs.html#1-212" class="src">Source</a>
</span>

Expand description

A macro for defining `#[cfg]` if-else statements.

The macro provided by this crate, `cfg_if`, is similar to the `if/elif`
C preprocessor macro by allowing definition of a cascade of `#[cfg]`
cases, emitting the implementation which matches first.

This allows you to conveniently provide a long list `#[cfg]`’d blocks of
code without having to rewrite each clause multiple times.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
cfg_if::cfg_if! {
    if #[cfg(unix)] {
        fn foo() { /* unix specific functionality */ }
    } else if #[cfg(target_pointer_width = "32")] {
        fn foo() { /* non-unix, 32-bit functionality */ }
    } else {
        fn foo() { /* fallback implementation */ }
    }
}
```

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.cfg_if.html" class="macro"
title="macro cfg_if::cfg_if">cfg_if</a>  
The main macro provided by this crate. See crate documentation for more
information.
