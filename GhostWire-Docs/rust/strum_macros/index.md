Crate strum\_macros Copy item path

<span class="sub-heading"><a href="../src/strum_macros/lib.rs.html#1-956" class="src">Source</a>
</span>

Expand description

# <a href="#strum" class="doc-anchor">§</a>Strum

Strum is a set of macros and traits for working with enums and strings
easier in Rust.

This crate only contains derive macros for use with the
[`strum`](https://docs.rs/strum) crate. The macros provied by this crate
are also available by enabling the `derive` feature in aforementioned
`strum` crate.

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.AsRefStr.html" class="derive"
title="derive strum_macros::AsRefStr">AsRefStr</a>  
Converts enum variants to `&'a str`, where `'a` is the lifetime of the
input enum reference.

<a href="derive.Display.html" class="derive"
title="derive strum_macros::Display">Display</a>  
Converts enum variants to strings.

<a href="derive.EnumCount.html" class="derive"
title="derive strum_macros::EnumCount">EnumCount</a>  
Add a constant `usize` equal to the number of variants.

<a href="derive.EnumDiscriminants.html" class="derive"
title="derive strum_macros::EnumDiscriminants">EnumDiscriminants</a>  
Generate a new type with only the discriminant names.

<a href="derive.EnumIs.html" class="derive"
title="derive strum_macros::EnumIs">EnumIs</a>  
Generated `is_*()` methods for each variant. E.g. `Color.is_red()`.

<a href="derive.EnumIter.html" class="derive"
title="derive strum_macros::EnumIter">EnumIter</a>  
Creates a new type that iterates of the variants of an enum.

<a href="derive.EnumMessage.html" class="derive"
title="derive strum_macros::EnumMessage">EnumMessage</a>  
Add a verbose message to an enum variant.

<a href="derive.EnumProperty.html" class="derive"
title="derive strum_macros::EnumProperty">EnumProperty</a>  
Add custom properties to enum variants.

<a href="derive.EnumString.html" class="derive"
title="derive strum_macros::EnumString">EnumString</a>  
Converts strings to enum variants based on their name.

<a href="derive.EnumTryAs.html" class="derive"
title="derive strum_macros::EnumTryAs">EnumTryAs</a>  
Generated `try_as_*()` methods for all tuple-style variants. E.g.
`Message.try_as_write()`.

<a href="derive.FromRepr.html" class="derive"
title="derive strum_macros::FromRepr">FromRepr</a>  
Add a function to enum that allows accessing variants by its
discriminant

<a href="derive.IntoStaticStr.html" class="derive"
title="derive strum_macros::IntoStaticStr">IntoStaticStr</a>  
Implements `From<MyEnum> for &'static str` on an enum.

<a href="derive.VariantArray.html" class="derive"
title="derive strum_macros::VariantArray">VariantArray</a>  
Adds a `'static` slice with all of the Enum’s variants.

<a href="derive.VariantNames.html" class="derive"
title="derive strum_macros::VariantNames">VariantNames</a>  
Implements `Strum::VariantNames` which adds an associated constant
`VARIANTS` which is a `'static` slice of discriminant names.
