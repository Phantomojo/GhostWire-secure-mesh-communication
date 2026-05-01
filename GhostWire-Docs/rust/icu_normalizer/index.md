Crate icu\_normalizer Copy item path

<span class="sub-heading"><a href="../src/icu_normalizer/lib.rs.html#6-3097"
class="src">Source</a> </span>

Expand description

Normalizing text into Unicode Normalization Forms.

This module is published as its own crate
([`icu_normalizer`](https://docs.rs/icu_normalizer/latest/icu_normalizer/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See
the latter for more details on the ICU4X project.

# <a href="#functionality" class="doc-anchor">§</a>Functionality

The top level of the crate provides normalization of input into the four
normalization forms defined in [UAX \#15: Unicode Normalization
Forms](https://www.unicode.org/reports/tr15/): NFC, NFD, NFKC, and NFKD.

Three kinds of contiguous inputs are supported: known-well-formed UTF-8
(`&str`), potentially-not-well-formed UTF-8, and
potentially-not-well-formed UTF-16. Additionally, an iterator over
`char` can be wrapped in a normalizing iterator.

The `uts46` module provides the combination of mapping and normalization
operations for [UTS \#46: Unicode IDNA Compatibility
Processing](https://www.unicode.org/reports/tr46/). This functionality
is not meant to be used by applications directly. Instead, it is meant
as a building block for a full implementation of UTS \#46, such as the
[`idna`](https://docs.rs/idna/latest/idna/) crate.

The `properties` module provides the non-recursive canonical
decomposition operation on a per `char` basis and the canonical
compositon operation given two `char`s. It also provides access to the
Canonical Combining Class property. These operations are primarily meant
for [HarfBuzz](https://harfbuzz.github.io/) via the
[`icu_harfbuzz`](https://docs.rs/icu_harfbuzz/latest/icu_harfbuzz/)
crate.

Notably, this normalizer does *not* provide the normalization “quick
check” that can result in “maybe” in addition to “yes” and “no”. The
normalization checks provided by this crate always give a definitive
non-“maybe” answer.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
let nfc = icu_normalizer::ComposingNormalizerBorrowed::new_nfc();
assert_eq!(nfc.normalize("a\u{0308}"), "ä");
assert!(nfc.is_normalized("ä"));

let nfd = icu_normalizer::DecomposingNormalizerBorrowed::new_nfd();
assert_eq!(nfd.normalize("ä"), "a\u{0308}");
assert!(!nfd.is_normalized("ä"));
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="properties/index.html" class="mod"
title="mod icu_normalizer::properties">properties</a>  
Access to the Unicode properties or property-based operations that are
required for NFC and NFD.

<a href="provider/index.html" class="mod"
title="mod icu_normalizer::provider">provider</a>  
🚧 \[Unstable\] Data provider struct definitions for this ICU4X
component.

<a href="uts46/index.html" class="mod"
title="mod icu_normalizer::uts46">uts46</a>  
Bundles the part of UTS 46 that makes sense to implement as a
normalization.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ComposingNormalizer.html" class="struct"
title="struct icu_normalizer::ComposingNormalizer">ComposingNormalizer</a>  
A normalizer for performing composing normalization.

<a href="struct.ComposingNormalizerBorrowed.html" class="struct"
title="struct icu_normalizer::ComposingNormalizerBorrowed">ComposingNormalizerBorrowed</a>  
Borrowed version of a normalizer for performing composing normalization.

<a href="struct.Composition.html" class="struct"
title="struct icu_normalizer::Composition">Composition</a>  
An iterator adaptor that turns an `Iterator` over `char` into a
lazily-decomposed and then canonically composed `char` sequence.

<a href="struct.DecomposingNormalizer.html" class="struct"
title="struct icu_normalizer::DecomposingNormalizer">DecomposingNormalizer</a>  
A normalizer for performing decomposing normalization.

<a href="struct.DecomposingNormalizerBorrowed.html" class="struct"
title="struct icu_normalizer::DecomposingNormalizerBorrowed">DecomposingNormalizerBorrowed</a>  
Borrowed version of a normalizer for performing decomposing
normalization.

<a href="struct.Decomposition.html" class="struct"
title="struct icu_normalizer::Decomposition">Decomposition</a>  
An iterator adaptor that turns an `Iterator` over `char` into a
lazily-decomposed `char` sequence.
