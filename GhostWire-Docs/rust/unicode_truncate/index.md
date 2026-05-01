Crate unicode\_truncate Copy item path

<span class="sub-heading"><a href="../src/unicode_truncate/lib.rs.html#10-591"
class="src">Source</a> </span>

Expand description

Unicode-aware algorithm to pad or truncate `str` in terms of displayed
width.

See the
[`UnicodeTruncateStr`](trait.UnicodeTruncateStr.html "trait unicode_truncate::UnicodeTruncateStr")
trait for new methods available on `str`.

# <a href="#examples" class="doc-anchor">§</a>Examples

Safely truncate string to display width even not at character
boundaries.

``` {.rust .rust-example-rendered}
use unicode_truncate::UnicodeTruncateStr;
assert_eq!("你好吗".unicode_truncate(5), ("你好", 4));
```

Making sure the string is displayed in exactly number of columns by
combining padding and truncating.

``` {.rust .rust-example-rendered}
use unicode_truncate::UnicodeTruncateStr;
use unicode_truncate::Alignment;
use unicode_width::UnicodeWidthStr;

let str = "你好吗".unicode_pad(5, Alignment::Left, true);
assert_eq!(str, "你好 ");
assert_eq!(str.width(), 5);
```

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Alignment.html" class="enum"
title="enum unicode_truncate::Alignment">Alignment</a>  
Defines the alignment for truncation and padding.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.UnicodeTruncateStr.html" class="trait"
title="trait unicode_truncate::UnicodeTruncateStr">UnicodeTruncateStr</a>  
Methods for padding or truncating using displayed width of Unicode
strings.
