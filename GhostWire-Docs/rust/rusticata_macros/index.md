Crate rusticata\_macros Copy item path

<span class="sub-heading"><a href="../src/rusticata_macros/lib.rs.html#1-44"
class="src">Source</a> </span>

Expand description

# <a href="#rusticata-macros" class="doc-anchor">§</a>Rusticata-macros

Helper macros for the [rusticata](https://github.com/rusticata) project.

This crate contains some additions to
[nom](https://github.com/Geal/nom).

For example, the
[`combinator::cond_else`](combinator/fn.cond_else.html "fn rusticata_macros::combinator::cond_else")
function allows to apply the first parser if the condition is true, and
the second if the condition is false:

``` {.rust .rust-example-rendered}
use rusticata_macros::combinator::cond_else;
let r: IResult<_, _, ()> = cond_else(
        || s.len() > 1,
        be_u16,
        map(be_u8, u16::from)
    )(s);
```

See the documentation for more details and examples.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../nom/index.html" class="mod"
title="mod nom"><code>nom</code></a>`;`

`pub use `<a href="macros/index.html" class="mod"
title="mod rusticata_macros::macros"><code>macros</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="combinator/index.html" class="mod"
title="mod rusticata_macros::combinator">combinator</a>  
General purpose combinators

<a href="debug/index.html" class="mod"
title="mod rusticata_macros::debug">debug</a>  
Helper functions and structures for debugging purpose

<a href="macros/index.html" class="mod"
title="mod rusticata_macros::macros">macros</a>  
Helper macros

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.align32.html" class="macro"
title="macro rusticata_macros::align32">align32</a>  
Align input value to the next multiple of 4 bytes

<a href="macro.align_n2.html" class="macro"
title="macro rusticata_macros::align_n2">align_n2</a>  
Align input value to the next multiple of n bytes Valid only if n is a
power of 2

<a href="macro.custom_check.html" class="macro"
title="macro rusticata_macros::custom_check">custom_check</a>  
Helper macro for nom parsers: raise error if the condition is true

<a href="macro.empty.html" class="macro"
title="macro rusticata_macros::empty">empty</a><span class="stab deprecated" title="">Deprecated</span>  
Helper macro for nom parsers: raise error if input is not empty

<a href="macro.error_if.html" class="macro"
title="macro rusticata_macros::error_if">error_if</a>  
Helper macro for nom parsers: raise error if the condition is true

<a href="macro.flat_take.html" class="macro"
title="macro rusticata_macros::flat_take">flat_take</a>  
Combination and flat\_map! and take! as first combinator

<a href="macro.newtype_enum.html" class="macro"
title="macro rusticata_macros::newtype_enum">newtype_enum</a>  
Helper macro for newtypes: declare associated constants and implement
Display trait

<a href="macro.parse_hex_to_u64.html" class="macro"
title="macro rusticata_macros::parse_hex_to_u64">parse_hex_to_u64</a>  
Read a slice as a big-endian value.

<a href="macro.q.html" class="macro"
title="macro rusticata_macros::q">q</a>  
Nom combinator that returns the given expression unchanged

<a href="macro.upgrade_error.html" class="macro"
title="macro rusticata_macros::upgrade_error">upgrade_error</a>  
Apply combinator, trying to “upgrade” error to next error type (using
the `Into` or `From` traits).

<a href="macro.upgrade_error_to.html" class="macro"
title="macro rusticata_macros::upgrade_error_to">upgrade_error_to</a>  
Apply combinator, trying to “upgrade” error to next error type (using
the `Into` or `From` traits).

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Serialize.html" class="trait"
title="trait rusticata_macros::Serialize">Serialize</a>  
Common trait for structures serialization
