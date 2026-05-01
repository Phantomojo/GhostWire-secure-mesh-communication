Crate unicode\_segmentation Copy item path

<span class="sub-heading"><a href="../src/unicode_segmentation/lib.rs.html#11-297"
class="src">Source</a> </span>

Expand description

Iterators which split strings on Grapheme Cluster, Word or Sentence
boundaries, according to the [Unicode Standard Annex
\#29](http://www.unicode.org/reports/tr29/) rules.

``` {.rust .rust-example-rendered}
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "a̐éö̲\r\n";
    let g = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    let b: &[_] = &["a̐", "é", "ö̲", "\r\n"];
    assert_eq!(g, b);

    let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";
    let w = s.unicode_words().collect::<Vec<&str>>();
    let b: &[_] = &["The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right"];
    assert_eq!(w, b);

    let s = "The quick (\"brown\")  fox";
    let w = s.split_word_bounds().collect::<Vec<&str>>();
    let b: &[_] = &["The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", "  ", "fox"];
    assert_eq!(w, b);
}
```

# <a href="#no_std" class="doc-anchor">§</a>no\_std

unicode-segmentation does not depend on libstd, so it can be used in
crates with the `#![no_std]` attribute.

# <a href="#cratesio" class="doc-anchor">§</a>crates.io

You can use this package in your project by adding the following to your
`Cargo.toml`:

``` language-toml
[dependencies]
unicode-segmentation = "1.9.0"
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.GraphemeCursor.html" class="struct"
title="struct unicode_segmentation::GraphemeCursor">GraphemeCursor</a>  
Cursor-based segmenter for grapheme clusters.

<a href="struct.GraphemeIndices.html" class="struct"
title="struct unicode_segmentation::GraphemeIndices">GraphemeIndices</a>  
External iterator for grapheme clusters and byte offsets.

<a href="struct.Graphemes.html" class="struct"
title="struct unicode_segmentation::Graphemes">Graphemes</a>  
External iterator for a string’s [grapheme
clusters](http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries).

<a href="struct.USentenceBoundIndices.html" class="struct"
title="struct unicode_segmentation::USentenceBoundIndices">USentenceBoundIndices</a>  
External iterator for sentence boundaries and byte offsets.

<a href="struct.USentenceBounds.html" class="struct"
title="struct unicode_segmentation::USentenceBounds">USentenceBounds</a>  
External iterator for a string’s [sentence
boundaries](http://www.unicode.org/reports/tr29/#Sentence_Boundaries).

<a href="struct.UWordBoundIndices.html" class="struct"
title="struct unicode_segmentation::UWordBoundIndices">UWordBoundIndices</a>  
External iterator for word boundaries and byte offsets.

<a href="struct.UWordBounds.html" class="struct"
title="struct unicode_segmentation::UWordBounds">UWordBounds</a>  
External iterator for a string’s [word
boundaries](http://www.unicode.org/reports/tr29/#Word_Boundaries).

<a href="struct.UnicodeSentences.html" class="struct"
title="struct unicode_segmentation::UnicodeSentences">UnicodeSentences</a>  
An iterator over the substrings of a string which, after splitting the
string on [sentence
boundaries](http://www.unicode.org/reports/tr29/#Sentence_Boundaries),
contain any characters with the
[Alphabetic](http://unicode.org/reports/tr44/#Alphabetic) property, or
with
[General\_Category=Number](http://unicode.org/reports/tr44/#General_Category_Values).

<a href="struct.UnicodeWordIndices.html" class="struct"
title="struct unicode_segmentation::UnicodeWordIndices">UnicodeWordIndices</a>  
An iterator over the substrings of a string which, after splitting the
string on [word
boundaries](http://www.unicode.org/reports/tr29/#Word_Boundaries),
contain any characters with the
[Alphabetic](http://unicode.org/reports/tr44/#Alphabetic) property, or
with
[General\_Category=Number](http://unicode.org/reports/tr44/#General_Category_Values).
This iterator also provides the byte offsets for each substring.

<a href="struct.UnicodeWords.html" class="struct"
title="struct unicode_segmentation::UnicodeWords">UnicodeWords</a>  
An iterator over the substrings of a string which, after splitting the
string on [word
boundaries](http://www.unicode.org/reports/tr29/#Word_Boundaries),
contain any characters with the
[Alphabetic](http://unicode.org/reports/tr44/#Alphabetic) property, or
with
[General\_Category=Number](http://unicode.org/reports/tr44/#General_Category_Values).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.GraphemeIncomplete.html" class="enum"
title="enum unicode_segmentation::GraphemeIncomplete">GraphemeIncomplete</a>  
An error return indicating that not enough content was available in the
provided chunk to satisfy the query, and that more content must be
provided.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.UNICODE_VERSION.html" class="constant"
title="constant unicode_segmentation::UNICODE_VERSION">UNICODE_VERSION</a>  
The version of [Unicode](http://www.unicode.org/) that this version of
unicode-segmentation is based on.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.UnicodeSegmentation.html" class="trait"
title="trait unicode_segmentation::UnicodeSegmentation">UnicodeSegmentation</a>  
Methods for segmenting strings according to [Unicode Standard Annex
\#29](http://www.unicode.org/reports/tr29/).
