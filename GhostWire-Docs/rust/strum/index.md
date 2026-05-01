Crate strum Copy item path

<span class="sub-heading"><a href="../src/strum/lib.rs.html#1-260" class="src">Source</a>
</span>

Expand description

# <a href="#strum" class="doc-anchor">§</a>Strum

[![Build
Status](https://travis-ci.org/Peternator7/strum.svg?branch=master)](https://travis-ci.org/Peternator7/strum)
[![Latest
Version](https://img.shields.io/crates/v/strum.svg)](https://crates.io/crates/strum)
[![Rust
Documentation](https://docs.rs/strum/badge.svg)](https://docs.rs/strum)

Strum is a set of macros and traits for working with enums and strings
easier in Rust.

The full version of the README can be found on
[GitHub](https://github.com/Peternator7/strum).

# <a href="#including-strum-in-your-project" class="doc-anchor">§</a>Including Strum in Your Project

Import strum and `strum_macros` into your project by adding the
following lines to your Cargo.toml. `strum_macros` contains the macros
needed to derive all the traits in Strum.

``` language-toml
[dependencies]
strum = "0.26"
strum_macros = "0.26"

# You can also access strum_macros exports directly through strum using the "derive" feature
strum = { version = "0.26", features = ["derive"] }
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="additional_attributes/index.html" class="mod"
title="mod strum::additional_attributes">additional_attributes</a>  
Documentation for Additional Attributes

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ParseError.html" class="enum"
title="enum strum::ParseError">ParseError</a>  
The `ParseError` enum is a collection of all the possible reasons an
enum can fail to parse from a string.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsStaticRef.html" class="trait"
title="trait strum::AsStaticRef">AsStaticRef</a><span class="stab deprecated" title="">Deprecated</span>  
A cheap reference-to-reference conversion. Used to convert a value to a
reference value with `'static` lifetime within generic code.

<a href="trait.EnumCount.html" class="trait"
title="trait strum::EnumCount">EnumCount</a>  
A trait for capturing the number of variants in Enum. This trait can be
autoderived by `strum_macros`.

<a href="trait.EnumMessage.html" class="trait"
title="trait strum::EnumMessage">EnumMessage</a>  
Associates additional pieces of information with an Enum. This can be
autoimplemented by deriving `EnumMessage` and annotating your variants
with `#[strum(message="...")]`.

<a href="trait.EnumProperty.html" class="trait"
title="trait strum::EnumProperty">EnumProperty</a>  
`EnumProperty` is a trait that makes it possible to store additional
information with enum variants. This trait is designed to be used with
the macro of the same name in the `strum_macros` crate. Currently, the
only string literals are supported in attributes, the other methods will
be implemented as additional attribute types become stabilized.

<a href="trait.IntoEnumIterator.html" class="trait"
title="trait strum::IntoEnumIterator">IntoEnumIterator</a>  
This trait designates that an `Enum` can be iterated over. It can be
auto generated using the [`EnumIter`](derive.EnumIter.html) derive
macro.

<a href="trait.VariantArray.html" class="trait"
title="trait strum::VariantArray">VariantArray</a>  
A trait for retrieving a static array containing all the variants in an
Enum. This trait can be autoderived by `strum_macros`. For derived
usage, all the variants in the enumerator need to be unit-types, which
means you can’t autoderive enums with inner data in one or more
variants. Consider using it alongside
[`EnumDiscriminants`](derive.EnumDiscriminants.html "derive strum::EnumDiscriminants")
if you require inner data but still want to have an static array of
variants.

<a href="trait.VariantIterator.html" class="trait"
title="trait strum::VariantIterator">VariantIterator</a>  
<a href="trait.VariantMetadata.html" class="trait"
title="trait strum::VariantMetadata">VariantMetadata</a>  
<a href="trait.VariantNames.html" class="trait"
title="trait strum::VariantNames">VariantNames</a>  
A trait for retrieving the names of each variant in Enum. This trait can
be autoderived by `strum_macros`.

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.AsRefStr.html" class="derive"
title="derive strum::AsRefStr">AsRefStr</a>  
Converts enum variants to `&'a str`, where `'a` is the lifetime of the
input enum reference.

<a href="derive.Display.html" class="derive"
title="derive strum::Display">Display</a>  
Converts enum variants to strings.

<a href="derive.EnumCount.html" class="derive"
title="derive strum::EnumCount">EnumCount</a>  
Add a constant `usize` equal to the number of variants.

<a href="derive.EnumDiscriminants.html" class="derive"
title="derive strum::EnumDiscriminants">EnumDiscriminants</a>  
Generate a new type with only the discriminant names.

<a href="derive.EnumIs.html" class="derive"
title="derive strum::EnumIs">EnumIs</a>  
Generated `is_*()` methods for each variant. E.g. `Color.is_red()`.

<a href="derive.EnumIter.html" class="derive"
title="derive strum::EnumIter">EnumIter</a>  
Creates a new type that iterates of the variants of an enum.

<a href="derive.EnumMessage.html" class="derive"
title="derive strum::EnumMessage">EnumMessage</a>  
Add a verbose message to an enum variant.

<a href="derive.EnumProperty.html" class="derive"
title="derive strum::EnumProperty">EnumProperty</a>  
Add custom properties to enum variants.

<a href="derive.EnumString.html" class="derive"
title="derive strum::EnumString">EnumString</a>  
Converts strings to enum variants based on their name.

<a href="derive.EnumTryAs.html" class="derive"
title="derive strum::EnumTryAs">EnumTryAs</a>  
Generated `try_as_*()` methods for all tuple-style variants. E.g.
`Message.try_as_write()`.

<a href="derive.FromRepr.html" class="derive"
title="derive strum::FromRepr">FromRepr</a>  
Add a function to enum that allows accessing variants by its
discriminant

<a href="derive.IntoStaticStr.html" class="derive"
title="derive strum::IntoStaticStr">IntoStaticStr</a>  
Implements `From<MyEnum> for &'static str` on an enum.

<a href="derive.VariantArray.html" class="derive"
title="derive strum::VariantArray">VariantArray</a>  
Adds a `'static` slice with all of the Enum’s variants.

<a href="derive.VariantNames.html" class="derive"
title="derive strum::VariantNames">VariantNames</a>  
Implements `Strum::VariantNames` which adds an associated constant
`VARIANTS` which is a `'static` slice of discriminant names.
