Crate heck Copy item path

<span class="sub-heading"><a href="../src/heck/lib.rs.html#1-192" class="src">Source</a>
</span>

Expand description

**heck** is a case conversion library.

This library exists to provide case conversion between common cases like
CamelCase and snake\_case. It is intended to be unicode aware,
internally consistent, and reasonably well performing.

## <a href="#definition-of-a-word-boundary" class="doc-anchor">§</a>Definition of a word boundary

Word boundaries are defined by non-alphanumeric characters, as well as
within those words in this manner:

1.  If an uppercase character is followed by lowercase letters, a word
    boundary is considered to be just prior to that uppercase character.
2.  If multiple uppercase characters are consecutive, they are
    considered to be within a single word, except that the last will be
    part of the next word if it is followed by lowercase characters (see
    rule 1).

That is, “HelloWorld” is segmented `Hello|World` whereas
“XMLHttpRequest” is segmented `XML|Http|Request`.

Characters not within words (such as spaces, punctuations, and
underscores) are not included in the output string except as they are a
part of the case being converted to. Multiple adjacent word boundaries
(such as a series of underscores) are folded into one. (“hello\_\_world”
in snake case is therefore “hello\_world”, not the exact same string).
Leading or trailing word boundary indicators are dropped, except insofar
as CamelCase capitalizes the first word.

### <a href="#cases-contained-in-this-library" class="doc-anchor">§</a>Cases contained in this library:

1.  UpperCamelCase
2.  lowerCamelCase
3.  snake\_case
4.  kebab-case
5.  SHOUTY\_SNAKE\_CASE
6.  Title Case
7.  SHOUTY-KEBAB-CASE
8.  Train-Case

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AsKebabCase.html" class="struct"
title="struct heck::AsKebabCase">AsKebabCase</a>  
This wrapper performs a kebab case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsLowerCamelCase.html" class="struct"
title="struct heck::AsLowerCamelCase">AsLowerCamelCase</a>  
This wrapper performs a lower camel case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsPascalCase.html" class="struct"
title="struct heck::AsPascalCase">AsPascalCase</a>  
This wrapper performs a upper camel case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsShoutyKebabCase.html" class="struct"
title="struct heck::AsShoutyKebabCase">AsShoutyKebabCase</a>  
This wrapper performs a kebab case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsShoutySnakeCase.html" class="struct"
title="struct heck::AsShoutySnakeCase">AsShoutySnakeCase</a>  
This wrapper performs a shouty snake case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsShoutySnekCase.html" class="struct"
title="struct heck::AsShoutySnekCase">AsShoutySnekCase</a>  
This wrapper performs a shouty snake case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsSnakeCase.html" class="struct"
title="struct heck::AsSnakeCase">AsSnakeCase</a>  
This wrapper performs a snake case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsSnekCase.html" class="struct"
title="struct heck::AsSnekCase">AsSnekCase</a>  
This wrapper performs a snake case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsTitleCase.html" class="struct"
title="struct heck::AsTitleCase">AsTitleCase</a>  
This wrapper performs a title case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsTrainCase.html" class="struct"
title="struct heck::AsTrainCase">AsTrainCase</a>  
This wrapper performs a train case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

<a href="struct.AsUpperCamelCase.html" class="struct"
title="struct heck::AsUpperCamelCase">AsUpperCamelCase</a>  
This wrapper performs a upper camel case conversion in
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display").

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ToKebabCase.html" class="trait"
title="trait heck::ToKebabCase">ToKebabCase</a>  
This trait defines a kebab case conversion.

<a href="trait.ToLowerCamelCase.html" class="trait"
title="trait heck::ToLowerCamelCase">ToLowerCamelCase</a>  
This trait defines a lower camel case conversion.

<a href="trait.ToPascalCase.html" class="trait"
title="trait heck::ToPascalCase">ToPascalCase</a>  
`ToPascalCase` is an alias for
[`ToUpperCamelCase`](trait.ToUpperCamelCase.html "trait heck::ToUpperCamelCase").
See ToUpperCamelCase for more documentation.

<a href="trait.ToShoutyKebabCase.html" class="trait"
title="trait heck::ToShoutyKebabCase">ToShoutyKebabCase</a>  
This trait defines a shouty kebab case conversion.

<a href="trait.ToShoutySnakeCase.html" class="trait"
title="trait heck::ToShoutySnakeCase">ToShoutySnakeCase</a>  
This trait defines a shouty snake case conversion.

<a href="trait.ToShoutySnekCase.html" class="trait"
title="trait heck::ToShoutySnekCase">ToShoutySnekCase</a>  
Oh heck, `ToShoutySnekCase` is an alias for
[`ToShoutySnakeCase`](trait.ToShoutySnakeCase.html "trait heck::ToShoutySnakeCase").
See ToShoutySnakeCase for more documentation.

<a href="trait.ToSnakeCase.html" class="trait"
title="trait heck::ToSnakeCase">ToSnakeCase</a>  
This trait defines a snake case conversion.

<a href="trait.ToSnekCase.html" class="trait"
title="trait heck::ToSnekCase">ToSnekCase</a>  
Oh heck, `SnekCase` is an alias for
[`ToSnakeCase`](trait.ToSnakeCase.html "trait heck::ToSnakeCase"). See
ToSnakeCase for more documentation.

<a href="trait.ToTitleCase.html" class="trait"
title="trait heck::ToTitleCase">ToTitleCase</a>  
This trait defines a title case conversion.

<a href="trait.ToTrainCase.html" class="trait"
title="trait heck::ToTrainCase">ToTrainCase</a>  
This trait defines a train case conversion.

<a href="trait.ToUpperCamelCase.html" class="trait"
title="trait heck::ToUpperCamelCase">ToUpperCamelCase</a>  
This trait defines an upper camel case conversion.
