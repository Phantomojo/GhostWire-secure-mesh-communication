Crate unicode\_normalization Copy item path

<span class="sub-heading"><a href="../src/unicode_normalization/lib.rs.html#11-232"
class="src">Source</a> </span>

Expand description

Unicode character composition and decomposition utilities as described
in [Unicode Standard Annex \#15](http://www.unicode.org/reports/tr15/).

``` {.rust .rust-example-rendered}
extern crate unicode_normalization;

use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;

fn main() {
    assert_eq!(compose('A','\u{30a}'), Some('Å'));

    let s = "ÅΩ";
    let c = s.nfc().collect::<String>();
    assert_eq!(c, "ÅΩ");
}
```

# <a href="#cratesio" class="doc-anchor">§</a>crates.io

You can use this package in your project by adding the following to your
`Cargo.toml`:

``` language-toml
[dependencies]
unicode-normalization = "0.1.20"
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="char/index.html" class="mod"
title="mod unicode_normalization::char">char</a>  
Methods for composing and decomposing characters.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Decompositions.html" class="struct"
title="struct unicode_normalization::Decompositions">Decompositions</a>  
External iterator for a string decomposition’s characters.

<a href="struct.Recompositions.html" class="struct"
title="struct unicode_normalization::Recompositions">Recompositions</a>  
External iterator for a string recomposition’s characters.

<a href="struct.Replacements.html" class="struct"
title="struct unicode_normalization::Replacements">Replacements</a>  
External iterator for replacements for a string’s characters.

<a href="struct.StreamSafe.html" class="struct"
title="struct unicode_normalization::StreamSafe">StreamSafe</a>  
[UAX15-D4](https://www.unicode.org/reports/tr15/#UAX15-D4): This
iterator keeps track of how many non-starters there have been since the
last starter in *NFKD* and will emit a Combining Grapheme Joiner
(U+034F) if the count exceeds 30.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.IsNormalized.html" class="enum"
title="enum unicode_normalization::IsNormalized">IsNormalized</a>  
QuickCheck quickly determines if a string is normalized, it can return
`Maybe`

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.UNICODE_VERSION.html" class="constant"
title="constant unicode_normalization::UNICODE_VERSION">UNICODE_VERSION</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.UnicodeNormalization.html" class="trait"
title="trait unicode_normalization::UnicodeNormalization">UnicodeNormalization</a>  
Methods for iterating over strings while applying Unicode normalizations
as described in [Unicode Standard Annex
\#15](http://www.unicode.org/reports/tr15/).

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.is_nfc.html" class="fn"
title="fn unicode_normalization::is_nfc">is_nfc</a>  
Authoritatively check if a string is in NFC.

<a href="fn.is_nfc_quick.html" class="fn"
title="fn unicode_normalization::is_nfc_quick">is_nfc_quick</a>  
Quickly check if a string is in NFC, potentially returning
`IsNormalized::Maybe` if further checks are necessary. In this case a
check like `s.chars().nfc().eq(s.chars())` should suffice.

<a href="fn.is_nfc_stream_safe.html" class="fn"
title="fn unicode_normalization::is_nfc_stream_safe">is_nfc_stream_safe</a>  
Authoritatively check if a string is Stream-Safe NFC.

<a href="fn.is_nfc_stream_safe_quick.html" class="fn"
title="fn unicode_normalization::is_nfc_stream_safe_quick">is_nfc_stream_safe_quick</a>  
Quickly check if a string is Stream-Safe NFC.

<a href="fn.is_nfd.html" class="fn"
title="fn unicode_normalization::is_nfd">is_nfd</a>  
Authoritatively check if a string is in NFD.

<a href="fn.is_nfd_quick.html" class="fn"
title="fn unicode_normalization::is_nfd_quick">is_nfd_quick</a>  
Quickly check if a string is in NFD.

<a href="fn.is_nfd_stream_safe.html" class="fn"
title="fn unicode_normalization::is_nfd_stream_safe">is_nfd_stream_safe</a>  
Authoritatively check if a string is Stream-Safe NFD.

<a href="fn.is_nfd_stream_safe_quick.html" class="fn"
title="fn unicode_normalization::is_nfd_stream_safe_quick">is_nfd_stream_safe_quick</a>  
Quickly check if a string is Stream-Safe NFD.

<a href="fn.is_nfkc.html" class="fn"
title="fn unicode_normalization::is_nfkc">is_nfkc</a>  
Authoritatively check if a string is in NFKC.

<a href="fn.is_nfkc_quick.html" class="fn"
title="fn unicode_normalization::is_nfkc_quick">is_nfkc_quick</a>  
Quickly check if a string is in NFKC.

<a href="fn.is_nfkd.html" class="fn"
title="fn unicode_normalization::is_nfkd">is_nfkd</a>  
Authoritatively check if a string is in NFKD.

<a href="fn.is_nfkd_quick.html" class="fn"
title="fn unicode_normalization::is_nfkd_quick">is_nfkd_quick</a>  
Quickly check if a string is in NFKD.
