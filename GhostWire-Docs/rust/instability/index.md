Crate instability Copy item path

<span class="sub-heading"><a href="../src/instability/lib.rs.html#1-212" class="src">Source</a>
</span>

Expand description

This crate provides attribute macros for specifying API stability of
public API items of a crate.

The Rust standard library has a concept of [API
stability](https://rustc-dev-guide.rust-lang.org/stability.html) and
custom attributes for managing that on a per-item basis, but most of
these attributes are not available for normal crates to use, with the
exception of the
[`deprecated`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-deprecated-attribute)
attribute. This crate seeks to provide similar attributes on stable
Rust, though tuned more toward what the needs of normal crate authors.

For complete examples of how to use this crate, check out the source
code for the
[`instability-example`](https://github.com/ratatui/instability/tree/main/example)
crate in the repository

Currently, only the
[`unstable`](attr.unstable.html "attr instability::unstable") attribute
is available. Please see the documentation of that macro for an
explanation on what it does and how to use it.

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.stable.html" class="attr"
title="attr instability::stable">stable</a>  
Mark an API as stable.

<a href="attr.unstable.html" class="attr"
title="attr instability::unstable">unstable</a>  
Mark an API as unstable.
