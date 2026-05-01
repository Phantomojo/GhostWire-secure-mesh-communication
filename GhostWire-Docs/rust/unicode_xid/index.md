Crate unicode\_xid Copy item path

<span class="sub-heading"><a href="../src/unicode_xid/lib.rs.html#11-90" class="src">Source</a>
</span>

Expand description

Determine if a `char` is a valid identifier for a parser and/or lexer
according to [Unicode Standard Annex
\#31](http://www.unicode.org/reports/tr31/) rules.

``` {.rust .rust-example-rendered}
use unicode_xid::UnicodeXID;

fn main() {
    assert_eq!(UnicodeXID::is_xid_start('a'), true); // 'a' is a valid start of an identifier
    assert_eq!(UnicodeXID::is_xid_start('△'), false); // '△' is a NOT valid start of an identifier
}
```

# <a href="#features" class="doc-anchor">§</a>features

unicode-xid supports a `no_std` feature. This eliminates dependence on
std, and instead uses equivalent functions from core.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.UNICODE_VERSION.html" class="constant"
title="constant unicode_xid::UNICODE_VERSION">UNICODE_VERSION</a>  
The version of [Unicode](http://www.unicode.org/) that this version of
unicode-xid is based on.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.UnicodeXID.html" class="trait"
title="trait unicode_xid::UnicodeXID">UnicodeXID</a>  
Methods for determining if a character is a valid identifier character.
