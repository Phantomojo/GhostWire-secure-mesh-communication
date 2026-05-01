Crate icu\_properties Copy item path

<span class="sub-heading"><a href="../src/icu_properties/lib.rs.html#5-100" class="src">Source</a>
</span>

Expand description

Definitions of [Unicode
Properties](https://unicode-org.github.io/icu/userguide/strings/properties.html)
and APIs for retrieving property data in an appropriate data structure.

This module is published as its own crate
([`icu_properties`](https://docs.rs/icu_properties/latest/icu_properties/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See
the latter for more details on the ICU4X project.

APIs that return a
[`CodePointSetData`](struct.CodePointSetData.html "struct icu_properties::CodePointSetData")
exist for binary properties and certain enumerated properties.

APIs that return a
[`CodePointMapData`](struct.CodePointMapData.html "struct icu_properties::CodePointMapData")
exist for certain enumerated properties.

# <a href="#examples" class="doc-anchor">§</a>Examples

## <a href="#property-data-as-codepointsetdatas" class="doc-anchor">§</a>Property data as `CodePointSetData`s

``` {.rust .rust-example-rendered}
use icu::properties::{CodePointSetData, CodePointMapData};
use icu::properties::props::{GeneralCategory, Emoji};

// A binary property as a `CodePointSetData`

assert!(CodePointSetData::new::<Emoji>().contains('🎃')); // U+1F383 JACK-O-LANTERN
assert!(!CodePointSetData::new::<Emoji>().contains('木')); // U+6728

// An individual enumerated property value as a `CodePointSetData`

let line_sep_data = CodePointMapData::<GeneralCategory>::new()
    .get_set_for_value(GeneralCategory::LineSeparator);
let line_sep = line_sep_data.as_borrowed();

assert!(line_sep.contains('\u{2028}'));
assert!(!line_sep.contains('\u{2029}'));
```

## <a href="#property-data-as-codepointmapdatas" class="doc-anchor">§</a>Property data as `CodePointMapData`s

``` {.rust .rust-example-rendered}
use icu::properties::CodePointMapData;
use icu::properties::props::Script;

assert_eq!(CodePointMapData::<Script>::new().get('🎃'), Script::Common); // U+1F383 JACK-O-LANTERN
assert_eq!(CodePointMapData::<Script>::new().get('木'), Script::Han); // U+6728
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="props/index.html" class="mod"
title="mod icu_properties::props">props</a>  
This module defines all available properties.

<a href="provider/index.html" class="mod"
title="mod icu_properties::provider">provider</a>  
🚧 \[Unstable\] Data provider struct definitions for this ICU4X
component.

<a href="script/index.html" class="mod"
title="mod icu_properties::script">script</a>  
Data and APIs for supporting Script\_Extensions property values in an
efficient structure.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CodePointMapData.html" class="struct"
title="struct icu_properties::CodePointMapData">CodePointMapData</a>  
A wrapper around code point map data.

<a href="struct.CodePointMapDataBorrowed.html" class="struct"
title="struct icu_properties::CodePointMapDataBorrowed">CodePointMapDataBorrowed</a>  
A borrowed wrapper around code point set data, returned by
\[`CodePointSetData::as_borrowed()`\]. More efficient to query.

<a href="struct.CodePointSetData.html" class="struct"
title="struct icu_properties::CodePointSetData">CodePointSetData</a>  
A set of Unicode code points. Access its data via the borrowed version,
[`CodePointSetDataBorrowed`](struct.CodePointSetDataBorrowed.html "struct icu_properties::CodePointSetDataBorrowed").

<a href="struct.CodePointSetDataBorrowed.html" class="struct"
title="struct icu_properties::CodePointSetDataBorrowed">CodePointSetDataBorrowed</a>  
A borrowed wrapper around code point set data, returned by
[`CodePointSetData::as_borrowed()`](struct.CodePointSetData.html#method.as_borrowed "method icu_properties::CodePointSetData::as_borrowed").
More efficient to query.

<a href="struct.EmojiSetData.html" class="struct"
title="struct icu_properties::EmojiSetData">EmojiSetData</a>  
A wrapper around `UnicodeSet` data (characters and strings)

<a href="struct.EmojiSetDataBorrowed.html" class="struct"
title="struct icu_properties::EmojiSetDataBorrowed">EmojiSetDataBorrowed</a>  
A borrowed wrapper around code point set data, returned by
[`EmojiSetData::as_borrowed()`](struct.EmojiSetData.html#method.as_borrowed "method icu_properties::EmojiSetData::as_borrowed").
More efficient to query.

<a href="struct.PropertyNamesLong.html" class="struct"
title="struct icu_properties::PropertyNamesLong">PropertyNamesLong</a>  
A struct capable of looking up a property name from a value Access its
data by calling
[`Self::as_borrowed()`](struct.PropertyNamesLong.html#method.as_borrowed "method icu_properties::PropertyNamesLong::as_borrowed")
and using the methods on
[`PropertyNamesLongBorrowed`](struct.PropertyNamesLongBorrowed.html "struct icu_properties::PropertyNamesLongBorrowed").

<a href="struct.PropertyNamesLongBorrowed.html" class="struct"
title="struct icu_properties::PropertyNamesLongBorrowed">PropertyNamesLongBorrowed</a>  
A borrowed wrapper around property value name-to-enum data, returned by
[`PropertyNamesLong::as_borrowed()`](struct.PropertyNamesLong.html#method.as_borrowed "method icu_properties::PropertyNamesLong::as_borrowed").
More efficient to query.

<a href="struct.PropertyNamesShort.html" class="struct"
title="struct icu_properties::PropertyNamesShort">PropertyNamesShort</a>  
A struct capable of looking up a property name from a value Access its
data by calling
[`Self::as_borrowed()`](struct.PropertyNamesShort.html#method.as_borrowed "method icu_properties::PropertyNamesShort::as_borrowed")
and using the methods on
[`PropertyNamesShortBorrowed`](struct.PropertyNamesShortBorrowed.html "struct icu_properties::PropertyNamesShortBorrowed").

<a href="struct.PropertyNamesShortBorrowed.html" class="struct"
title="struct icu_properties::PropertyNamesShortBorrowed">PropertyNamesShortBorrowed</a>  
A borrowed wrapper around property value name-to-enum data, returned by
[`PropertyNamesShort::as_borrowed()`](struct.PropertyNamesShort.html#method.as_borrowed "method icu_properties::PropertyNamesShort::as_borrowed").
More efficient to query.

<a href="struct.PropertyParser.html" class="struct"
title="struct icu_properties::PropertyParser">PropertyParser</a>  
A struct capable of looking up a property value from a string name.
Access its data by calling
[`Self::as_borrowed()`](struct.PropertyParser.html#method.as_borrowed "method icu_properties::PropertyParser::as_borrowed")
and using the methods on
[`PropertyParserBorrowed`](struct.PropertyParserBorrowed.html "struct icu_properties::PropertyParserBorrowed").

<a href="struct.PropertyParserBorrowed.html" class="struct"
title="struct icu_properties::PropertyParserBorrowed">PropertyParserBorrowed</a>  
A borrowed wrapper around property value name-to-enum data, returned by
[`PropertyParser::as_borrowed()`](struct.PropertyParser.html#method.as_borrowed "method icu_properties::PropertyParser::as_borrowed").
More efficient to query.
