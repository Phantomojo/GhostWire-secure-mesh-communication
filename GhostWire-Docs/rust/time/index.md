Crate time Copy item path

<span class="sub-heading"><a href="../src/time/lib.rs.html#1-151" class="src">Source</a>
</span>

Expand description

# <a href="#feature-flags" class="doc-anchor">§</a>Feature flags

This crate exposes a number of features. These can be enabled or
disabled as shown [in Cargo’s
documentation](https://doc.rust-lang.org/cargo/reference/features.html).
Features are *disabled* by default unless otherwise noted.

Reliance on a given feature is always indicated alongside the item
definition.

-   `std` (*enabled by default, implicitly enables `alloc`*)

    This enables a number of features that depend on the standard
    library.

-   `alloc` (*enabled by default via `std`*)

    Enables a number of features that require the ability to dynamically
    allocate memory.

-   `macros`

    Enables macros that provide compile-time verification of values and
    intuitive syntax.

-   `formatting` (*implicitly enables `std`*)

    Enables formatting of most structs.

-   `parsing`

    Enables parsing of most structs.

-   `local-offset` (*implicitly enables `std`*)

    This feature enables a number of methods that allow obtaining the
    system’s UTC offset.

-   `large-dates`

    By default, only years within the ±9999 range (inclusive) are
    supported. If you need support for years outside this range,
    consider enabling this feature; the supported range will be
    increased to ±999,999.

    Note that enabling this feature has some costs, as it means forgoing
    some optimizations. Ambiguities may be introduced when parsing that
    would not otherwise exist.

-   `serde`

    Enables [`serde`](https://docs.rs/serde) support for all types.

-   `serde-human-readable` (*implicitly enables `serde`, `formatting`,
    and `parsing`*)

    Allows `serde` representations to use a human-readable format. This
    is determined by the serializer, not the user. If this feature is
    not enabled or if the serializer requests a non-human-readable
    format, a format optimized for binary representation will be used.

    Libraries should never enable this feature, as the decision of what
    format to use should be up to the user.

-   `rand` (*implicitly enables `rand08` and `rand09`*)

    Previously, this would enable support for `rand` 0.8. Since the
    release of `rand` 0.9, the feature has been split into `rand08` and
    `rand09` to allow support for both versions. For backwards
    compatibility and simplicity, this feature enables support for
    *both* series.

    It is strongly recommended to enable `rand08` or `rand09` directly,
    as enabling `rand` will needlessly pull in both versions.

-   `rand08`

    Enables [`rand` 0.8](https://docs.rs/rand/0.8) support for all
    types.

-   `rand09`

    Enables [`rand` 0.9](https://docs.rs/rand/0.9) support for all
    types.

-   `quickcheck` (*implicitly enables `alloc`*)

    Enables [quickcheck](https://docs.rs/quickcheck) support for all
    types.

-   `wasm-bindgen`

    Enables [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen)
    support for converting [JavaScript
    dates](https://rustwasm.github.io/wasm-bindgen/api/js_sys/struct.Date.html),
    as well as obtaining the UTC offset from JavaScript.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::error::`<a href="error/enum.Error.html" class="enum"
title="enum time::error::Error"><code>Error</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="convert/index.html" class="mod"
title="mod time::convert">convert</a>  
Conversion between units of time.

<a href="error/index.html" class="mod" title="mod time::error">error</a>  
Various error types returned by methods in the time crate.

<a href="ext/index.html" class="mod" title="mod time::ext">ext</a>  
Extension traits.

<a href="format_description/index.html" class="mod"
title="mod time::format_description">format_description</a>  
Description of how types should be formatted and parsed.

<a href="formatting/index.html" class="mod"
title="mod time::formatting">formatting</a>  
Formatting for various types.

<a href="macros/index.html" class="mod"
title="mod time::macros">macros</a>  
Macros to construct statically known values.

<a href="parsing/index.html" class="mod"
title="mod time::parsing">parsing</a>  
Parsing for various types.

<a href="util/index.html" class="mod" title="mod time::util">util</a>  
Utility functions, including updating time zone information.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Date.html" class="struct"
title="struct time::Date">Date</a>  
Date in the proleptic Gregorian calendar.

<a href="struct.Duration.html" class="struct"
title="struct time::Duration">Duration</a>  
A span of time with nanosecond precision.

<a href="struct.OffsetDateTime.html" class="struct"
title="struct time::OffsetDateTime">OffsetDateTime</a>  
A
[`PrimitiveDateTime`](struct.PrimitiveDateTime.html "struct time::PrimitiveDateTime")
with a [`UtcOffset`](struct.UtcOffset.html "struct time::UtcOffset").

<a href="struct.PrimitiveDateTime.html" class="struct"
title="struct time::PrimitiveDateTime">PrimitiveDateTime</a>  
Combined date and time.

<a href="struct.Time.html" class="struct"
title="struct time::Time">Time</a>  
The clock time within a given date. Nanosecond precision.

<a href="struct.UtcDateTime.html" class="struct"
title="struct time::UtcDateTime">UtcDateTime</a>  
A
[`PrimitiveDateTime`](struct.PrimitiveDateTime.html "struct time::PrimitiveDateTime")
that is known to be UTC.

<a href="struct.UtcOffset.html" class="struct"
title="struct time::UtcOffset">UtcOffset</a>  
An offset from UTC.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Month.html" class="enum"
title="enum time::Month">Month</a>  
Months of the year.

<a href="enum.Weekday.html" class="enum"
title="enum time::Weekday">Weekday</a>  
Days of the week.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type time::Result">Result</a>  
An alias for
[`std::result::Result`](https://doc.rust-lang.org/1.93.1/core/result/enum.Result.html "enum core::result::Result")
with a generic error from the time crate.
