Crate static\_assertions Copy item path

<span class="sub-heading"><a href="../src/static_assertions/lib.rs.html#1-97"
class="src">Source</a> </span>

Expand description

[![Banner](https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Banner.png)](https://github.com/nvzqz/static-assertions-rs)

[![Downloads](https://img.shields.io/crates/d/static_assertions.svg)](https://crates.io/crates/static_assertions)
[![Build
Status](https://travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master)](https://travis-ci.org/nvzqz/static-assertions-rs)
![rustc
^1.37.0](https://img.shields.io/badge/rustc-%5E1.37.0-blue.svg)  
  

Assertions to ensure correct assumptions about constants, types, and
more.

*All* checks provided by this crate are performed at
[compile-time](https://en.wikipedia.org/wiki/Compile_time). This allows
for finding errors quickly and early when it comes to ensuring certain
features or aspects of a codebase. These macros are especially important
when exposing a public API that requires types to be the same size or
implement certain traits.

# <a href="#usage" class="doc-anchor">§</a>Usage

This crate is available [on
crates.io](https://crates.io/crates/static_assertions) and can be used
by adding the following to your project’s
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

``` language-toml
[dependencies]
static_assertions = "1.1.0"
```

and this to your crate root (`main.rs` or `lib.rs`):

``` {.rust .rust-example-rendered}
#[macro_use]
extern crate static_assertions;
```

When using [Rust 2018
edition](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#rust-2018),
the following shorthand can help if having `#[macro_use]` is
undesirable.

<a href="#" class="tooltip"
title="This example runs with edition 2018">ⓘ</a>

``` {.rust .rust-example-rendered}
extern crate static_assertions as sa;

sa::const_assert!(true);
```

# <a href="#examples" class="doc-anchor">§</a>Examples

Very thorough examples are provided in the docs for [each individual
macro](#macros). Failure case examples are also documented.

# <a href="#changes" class="doc-anchor">§</a>Changes

See
[`CHANGELOG.md`](https://github.com/nvzqz/static-assertions-rs/blob/master/CHANGELOG.md)
for an exhaustive list of what has changed from one version to another.

# <a href="#donate" class="doc-anchor">§</a>Donate

This project is made freely available (as in free beer), but
unfortunately not all beer is free! So, if you would like to buy me a
beer (or coffee or *more*), then consider supporting my work that’s
benefited your project and thousands of others.

[<img
src="https://c5.patreon.com/external/logo/become_a_patron_button.png"
height="35" alt="Become a Patron!" />](https://www.patreon.com/nvzqz)
[<img src="https://buymecoffee.intm.org/img/button-paypal-white.png"
height="35" alt="Buy me a coffee" />](https://www.paypal.me/nvzqz)

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.assert_cfg.html" class="macro"
title="macro static_assertions::assert_cfg">assert_cfg</a>  
Asserts that a given configuration is set.

<a href="macro.assert_eq_align.html" class="macro"
title="macro static_assertions::assert_eq_align">assert_eq_align</a>  
Asserts that types are equal in alignment.

<a href="macro.assert_eq_size.html" class="macro"
title="macro static_assertions::assert_eq_size">assert_eq_size</a>  
Asserts that types are equal in size.

<a href="macro.assert_eq_size_ptr.html" class="macro"
title="macro static_assertions::assert_eq_size_ptr">assert_eq_size_ptr</a>  
Asserts that values pointed to are equal in size.

<a href="macro.assert_eq_size_val.html" class="macro"
title="macro static_assertions::assert_eq_size_val">assert_eq_size_val</a>  
Asserts that values are equal in size.

<a href="macro.assert_fields.html" class="macro"
title="macro static_assertions::assert_fields">assert_fields</a>  
Asserts that the type has the given fields.

<a href="macro.assert_impl_all.html" class="macro"
title="macro static_assertions::assert_impl_all">assert_impl_all</a>  
Asserts that the type implements *all* of the given traits.

<a href="macro.assert_impl_any.html" class="macro"
title="macro static_assertions::assert_impl_any">assert_impl_any</a>  
Asserts that the type implements *any* of the given traits.

<a href="macro.assert_impl_one.html" class="macro"
title="macro static_assertions::assert_impl_one">assert_impl_one</a>  
Asserts that the type implements exactly one in a set of traits.

<a href="macro.assert_not_impl_all.html" class="macro"
title="macro static_assertions::assert_not_impl_all">assert_not_impl_all</a>  
Asserts that the type does **not** implement *all* of the given traits.

<a href="macro.assert_not_impl_any.html" class="macro"
title="macro static_assertions::assert_not_impl_any">assert_not_impl_any</a>  
Asserts that the type does **not** implement *any* of the given traits.

<a href="macro.assert_obj_safe.html" class="macro"
title="macro static_assertions::assert_obj_safe">assert_obj_safe</a>  
Asserts that the traits support dynamic dispatch
([object-safety](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects)).

<a href="macro.assert_trait_sub_all.html" class="macro"
title="macro static_assertions::assert_trait_sub_all">assert_trait_sub_all</a>  
Asserts that the trait is a child of all of the other traits.

<a href="macro.assert_trait_super_all.html" class="macro"
title="macro static_assertions::assert_trait_super_all">assert_trait_super_all</a>  
Asserts that the trait is a parent of all of the other traits.

<a href="macro.assert_type_eq_all.html" class="macro"
title="macro static_assertions::assert_type_eq_all">assert_type_eq_all</a>  
Asserts that *all* types in a list are equal to each other.

<a href="macro.assert_type_ne_all.html" class="macro"
title="macro static_assertions::assert_type_ne_all">assert_type_ne_all</a>  
Asserts that *all* types are **not** equal to each other.

<a href="macro.const_assert.html" class="macro"
title="macro static_assertions::const_assert">const_assert</a>  
Asserts that constant expressions evaluate to `true`.

<a href="macro.const_assert_eq.html" class="macro"
title="macro static_assertions::const_assert_eq">const_assert_eq</a>  
Asserts that constants are equal in value.

<a href="macro.const_assert_ne.html" class="macro"
title="macro static_assertions::const_assert_ne">const_assert_ne</a>  
Asserts that constants are **not** equal in value.
