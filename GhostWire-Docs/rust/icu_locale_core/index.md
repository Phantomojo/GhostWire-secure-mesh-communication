Crate icu\_locale\_core Copy item path

<span class="sub-heading"><a href="../src/icu_locale_core/lib.rs.html#5-96" class="src">Source</a>
</span>

Expand description

Parsing, manipulating, and serializing Unicode Language and Locale
Identifiers.

This module is published as its own crate
([`icu_locale_core`](https://docs.rs/icu_locale_core/latest/icu_locale_core/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See
the latter for more details on the ICU4X project.

The module provides algorithms for parsing a string into a well-formed
language or locale identifier as defined by
[`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`](https://unicode.org/reports/tr35/tr35.html#Unicode_Language_and_Locale_Identifiers).
Additionally the module provides
[`preferences`](preferences/index.html "mod icu_locale_core::preferences")
interface for operations on locale preferences and conversions from and
to locale unicode extensions.

[`Locale`](struct.Locale.html "struct icu_locale_core::Locale") is the
most common structure to use for storing information about a language,
script, region, variants and extensions. In almost all cases, this
struct should be used as the base unit for all locale management
operations.

[`LanguageIdentifier`](struct.LanguageIdentifier.html "struct icu_locale_core::LanguageIdentifier")
is a strict subset of
[`Locale`](struct.Locale.html "struct icu_locale_core::Locale") which
can be useful in a narrow range of cases where
[`Unicode Extensions`](extensions/index.html "mod icu_locale_core::extensions")
are not relevant.

If in doubt, use
[`Locale`](struct.Locale.html "struct icu_locale_core::Locale").

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use icu::locale::Locale;
use icu::locale::{
    locale,
    subtags::{language, region},
};

let mut loc: Locale = locale!("en-US");

assert_eq!(loc.id.language, language!("en"));
assert_eq!(loc.id.script, None);
assert_eq!(loc.id.region, Some(region!("US")));
assert_eq!(loc.id.variants.len(), 0);

loc.id.region = Some(region!("GB"));

assert_eq!(loc, locale!("en-GB"));
```

For more details, see
[`Locale`](struct.Locale.html "struct icu_locale_core::Locale") and
[`LanguageIdentifier`](struct.LanguageIdentifier.html "struct icu_locale_core::LanguageIdentifier").

# Modules<a href="#modules" class="anchor">§</a>

<a href="extensions/index.html" class="mod"
title="mod icu_locale_core::extensions">extensions</a>  
Unicode Extensions provide a mechanism to extend the
[`LanguageIdentifier`](struct.LanguageIdentifier.html "struct icu_locale_core::LanguageIdentifier")
with additional bits of information - a combination of a
[`LanguageIdentifier`](struct.LanguageIdentifier.html "struct icu_locale_core::LanguageIdentifier")
and
[`Extensions`](extensions/struct.Extensions.html "struct icu_locale_core::extensions::Extensions")
is called
[`Locale`](struct.Locale.html "struct icu_locale_core::Locale").

<a href="preferences/index.html" class="mod"
title="mod icu_locale_core::preferences">preferences</a>  
This API provides necessary functionality for building user preferences
structs.

<a href="subtags/index.html" class="mod"
title="mod icu_locale_core::subtags">subtags</a>  
Language Identifier and Locale contains a set of subtags which represent
different fields of the structure.

<a href="zerovec/index.html" class="mod"
title="mod icu_locale_core::zerovec">zerovec</a>  
Documentation on zero-copy deserialization of locale types.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.langid.html" class="macro"
title="macro icu_locale_core::langid">langid</a>  
A macro allowing for compile-time construction of valid
[`LanguageIdentifier`](struct.LanguageIdentifier.html "struct icu_locale_core::LanguageIdentifier")s.

<a href="macro.locale.html" class="macro"
title="macro icu_locale_core::locale">locale</a>  
A macro allowing for compile-time construction of valid
[`Locale`](struct.Locale.html "struct icu_locale_core::Locale")s.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DataLocale.html" class="struct"
title="struct icu_locale_core::DataLocale">DataLocale</a>  
A locale type optimized for use in fallbacking and the ICU4X data
pipeline.

<a href="struct.LanguageIdentifier.html" class="struct"
title="struct icu_locale_core::LanguageIdentifier">LanguageIdentifier</a>  
A core struct representing a
[`Unicode BCP47 Language Identifier`](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier).

<a href="struct.Locale.html" class="struct"
title="struct icu_locale_core::Locale">Locale</a>  
A core struct representing a
[`Unicode Locale Identifier`](https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ParseError.html" class="enum"
title="enum icu_locale_core::ParseError">ParseError</a>  
List of parser errors that can be generated while parsing
[`LanguageIdentifier`](struct.LanguageIdentifier.html "struct icu_locale_core::LanguageIdentifier"),
[`Locale`](struct.Locale.html "struct icu_locale_core::Locale"),
[`subtags`](subtags/index.html "mod icu_locale_core::subtags") or
[`extensions`](extensions/index.html "mod icu_locale_core::extensions").
