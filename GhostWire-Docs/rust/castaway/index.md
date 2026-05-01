Crate castaway Copy item path

<span class="sub-heading"><a href="../src/castaway/lib.rs.html#1-550" class="src">Source</a>
</span>

Expand description

Safe, zero-cost downcasting for limited compile-time specialization.

This crate works fully on stable Rust, and also does not require the
standard library. To disable references to the standard library, you
must opt-out of the `std` feature using `default-features = false` in
your `Cargo.toml` file. When in no-std mode, a separate `alloc` feature
flag is available to support casting to several
[`alloc`](https://doc.rust-lang.org/1.93.1/alloc/index.html "mod alloc")
types not included in
[`core`](https://doc.rust-lang.org/1.93.1/core/index.html "mod core").

Castaway provides the following key macros:

-   [`cast`](macro.cast.html "macro castaway::cast"): Attempt to cast
    the result of an expression into a given concrete type.
-   [`match_type`](macro.match_type.html "macro castaway::match_type"):
    Match the result of an expression against multiple concrete types.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.cast.html" class="macro"
title="macro castaway::cast">cast</a>  
Attempt to cast the result of an expression into a given concrete type.

<a href="macro.match_type.html" class="macro"
title="macro castaway::match_type">match_type</a>  
Match the result of an expression against multiple concrete types.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.LifetimeFree.html" class="trait"
title="trait castaway::LifetimeFree">LifetimeFree</a>  
Marker trait for types that do not contain any lifetime parameters. Such
types are safe to cast from non-static type parameters if their types
are equal.
