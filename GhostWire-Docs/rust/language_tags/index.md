Crate language\_tags Copy item path

<span class="sub-heading"><a href="../src/language_tags/lib.rs.html#1-1119" class="src">Source</a>
</span>

Expand description

Language tags can be used identify human languages, scripts e.g. Latin
script, countries and other regions.

Language tags are defined in [BCP47](http://tools.ietf.org/html/bcp47),
an introduction is [“Language tags in HTML and
XML”](http://www.w3.org/International/articles/language-tags/) by the
W3C. They are commonly used in HTML and HTTP `Content-Language` and
`Accept-Language` header fields.

This package currently supports parsing (fully conformant parser),
validation, canonicalization, formatting and comparing language tags.

# <a href="#examples" class="doc-anchor">§</a>Examples

Create a simple language tag representing the French language as spoken
in Belgium and print it:

``` {.rust .rust-example-rendered}
use language_tags::LanguageTag;

let langtag = LanguageTag::parse("fr-BE").unwrap();
assert_eq!(langtag.to_string(), "fr-BE");
```

Parse a tag representing a special type of English specified by private
agreement:

``` {.rust .rust-example-rendered}
use language_tags::LanguageTag;
use std::iter::FromIterator;

let langtag: LanguageTag = "en-x-twain".parse().unwrap();
assert_eq!(langtag.primary_language(), "en");
assert_eq!(Vec::from_iter(langtag.private_use_subtags()), vec!["twain"]);
```

You can check for equality, but more often you should test if two tags
match. In this example we check if the resource in German language is
suitable for a user from Austria. While people speaking Austrian German
normally understand standard German the opposite is not always true. So
the resource can be presented to the user but if the resource was in
`de-AT` and a user asked for a representation in `de` the request should
be rejected.

``` {.rust .rust-example-rendered}
use language_tags::LanguageTag;

let mut langtag_server = LanguageTag::parse("de-AT").unwrap();
let mut langtag_user = LanguageTag::parse("de").unwrap();
assert!(langtag_user.matches(&langtag_server));
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.LanguageTag.html" class="struct"
title="struct language_tags::LanguageTag">LanguageTag</a>  
A language tag as described in [RFC
5646](https://tools.ietf.org/html/rfc5646).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ParseError.html" class="enum"
title="enum language_tags::ParseError">ParseError</a>  
Errors returned by `LanguageTag` parsing

<a href="enum.ValidationError.html" class="enum"
title="enum language_tags::ValidationError">ValidationError</a>  
Errors returned by the `LanguageTag` validation
