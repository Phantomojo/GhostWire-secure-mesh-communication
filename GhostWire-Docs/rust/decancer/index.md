Crate decancer Copy item path

<span class="sub-heading"><a href="../src/decancer/lib.rs.html#1-416" class="src">Source</a>
</span>

Expand description

# <a
href="#decancer-npm-cratesio-npm-downloads-cratesio-downloads-codacy-ko-fi"
class="doc-anchor">§</a>decancer [![npm](https://img.shields.io/npm/v/decancer.svg?style=flat-square)](https://npmjs.org/package/decancer) [![crates.io](https://img.shields.io/crates/v/decancer?style=flat-square)](https://crates.io/crates/decancer) [![npm downloads](https://img.shields.io/npm/dt/decancer.svg?style=flat-square)](https://npmjs.org/package/decancer) [![crates.io downloads](https://img.shields.io/crates/d/decancer?style=flat-square)](https://crates.io/crates/decancer) [![codacy](https://app.codacy.com/project/badge/Grade/d740b1aa867d42f2b37eb992ad73784a)](https://app.codacy.com/gh/null8626/decancer/dashboard) [![ko-fi](https://img.shields.io/badge/donations-ko--fi-red?color=ff5e5b&style=flat-square)](https://ko-fi.com/null8626)

A library that removes common unicode confusables/homoglyphs from
strings.

-   Its core is written in [Rust](https://www.rust-lang.org) and
    utilizes a form of [**Binary
    Search**](https://en.wikipedia.org/wiki/Binary_search_algorithm) to
    ensure speed!
-   By default, it’s capable of filtering **221,529 (19.88%) different
    unicode codepoints** like:
    -   All [whitespace
        characters](https://en.wikipedia.org/wiki/Whitespace_character)
    -   All [diacritics](https://en.wikipedia.org/wiki/Diacritic), this
        also eliminates all forms of [Zalgo
        text](https://en.wikipedia.org/wiki/Zalgo_text)
    -   Most [leetspeak characters](https://en.wikipedia.org/wiki/Leet)
    -   Most [homoglyphs](https://en.wikipedia.org/wiki/Homoglyph)
    -   Several emojis
-   Unlike other packages, this package is **[unicode
    bidi-aware](https://en.wikipedia.org/wiki/Bidirectional_text)**
    where it also interprets right-to-left characters in the same way as
    it were to be rendered by an application!
-   Its behavior is also highly customizable to your liking!

## <a href="#installation" class="doc-anchor">§</a>Installation

In your `Cargo.toml`:

``` language-toml
decancer = "3.3.3"
```

## <a href="#examples" class="doc-anchor">§</a>Examples

For more information, please read the
[documentation](https://docs.rs/decancer).

``` {.rust .rust-example-rendered}
let mut cured = decancer::cure!(r"vＥⓡ𝔂 𝔽𝕌Ňℕｙ ţ乇𝕏𝓣 wWiIiIIttHh l133t5p3/-\|<").unwrap();

assert_eq!(cured, "very funny text with leetspeak");

// WARNING: it's NOT recommended to coerce this output to a Rust string
//          and process it manually from there, as decancer has its own
//          custom comparison measures, including leetspeak matching!
assert_ne!(cured.as_str(), "very funny text with leetspeak");

assert!(cured.contains("funny"));

cured.censor("funny", '*');
assert_eq!(cured, "very ***** text with leetspeak");

cured.censor_multiple(["very", "text"], '-');
assert_eq!(cured, "---- ***** ---- with leetspeak");
```

## <a href="#donations" class="doc-anchor">§</a>Donations

If you want to support my eyes for manually looking at thousands of
unicode characters, consider donating! ❤

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/null8626)

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.cure.html" class="macro"
title="macro decancer::cure">cure</a>  
Cures a string with decancer’s default options.

<a href="macro.cure_char.html" class="macro"
title="macro decancer::cure_char">cure_char</a>  
Cures a single character/unicode codepoint with decancer’s default
options.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CuredString.html" class="struct"
title="struct decancer::CuredString">CuredString</a>  
A small wrapper around the
[`String`](https://doc.rust-lang.org/1.93.1/alloc/string/struct.String.html "struct alloc::string::String")
data type for comparison purposes.

<a href="struct.Matcher.html" class="struct"
title="struct decancer::Matcher">Matcher</a>  
A matcher iterator around a string that yields a non-inclusive
[`Range`](https://doc.rust-lang.org/1.93.1/core/ops/range/struct.Range.html "struct core::ops::range::Range")
whenever it detects a similar match.

<a href="struct.Options.html" class="struct"
title="struct decancer::Options">Options</a>  
A configuration struct where you can customize decancer’s behavior.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum decancer::Error">Error</a>  
An error enum for unicode bidi errors caused by malformed string inputs.

<a href="enum.Translation.html" class="enum"
title="enum decancer::Translation">Translation</a>  
The translation for a single character/codepoint.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.cure.html" class="fn" title="fn decancer::cure">cure</a>  
Cures a string with the specified
[`Options`](struct.Options.html "struct decancer::Options").

<a href="fn.cure_char.html" class="fn"
title="fn decancer::cure_char">cure_char</a>  
Cures a single character/unicode codepoint with the specified
[`Options`](struct.Options.html "struct decancer::Options").
