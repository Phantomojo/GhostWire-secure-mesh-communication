Crate regex\_syntax Copy item path

<span class="sub-heading"><a href="../src/regex_syntax/lib.rs.html#1-433" class="src">Source</a>
</span>

Expand description

This crate provides a robust regular expression parser.

This crate defines two primary types:

-   [`Ast`](ast/enum.Ast.html "enum regex_syntax::ast::Ast") is the
    abstract syntax of a regular expression. An abstract syntax
    corresponds to a *structured representation* of the concrete syntax
    of a regular expression, where the concrete syntax is the pattern
    string itself (e.g., `foo(bar)+`). Given some abstract syntax, it
    can be converted back to the original concrete syntax (modulo some
    details, like whitespace). To a first approximation, the abstract
    syntax is complex and difficult to analyze.
-   [`Hir`](hir/struct.Hir.html "struct regex_syntax::hir::Hir") is the
    high-level intermediate representation (“HIR” or “high-level IR” for
    short) of regular expression. It corresponds to an intermediate
    state of a regular expression that sits between the abstract syntax
    and the low level compiled opcodes that are eventually responsible
    for executing a regular expression search. Given some high-level IR,
    it is not possible to produce the original concrete syntax (although
    it is possible to produce an equivalent concrete syntax, but it will
    likely scarcely resemble the original pattern). To a first
    approximation, the high-level IR is simple and easy to analyze.

These two types come with conversion routines:

-   An
    [`ast::parse::Parser`](ast/parse/struct.Parser.html "struct regex_syntax::ast::parse::Parser")
    converts concrete syntax (a `&str`) to an
    [`Ast`](ast/enum.Ast.html "enum regex_syntax::ast::Ast").
-   A
    [`hir::translate::Translator`](hir/translate/struct.Translator.html "struct regex_syntax::hir::translate::Translator")
    converts an [`Ast`](ast/enum.Ast.html "enum regex_syntax::ast::Ast")
    to a [`Hir`](hir/struct.Hir.html "struct regex_syntax::hir::Hir").

As a convenience, the above two conversion routines are combined into
one via the top-level
[`Parser`](struct.Parser.html "struct regex_syntax::Parser") type. This
`Parser` will first convert your pattern to an `Ast` and then convert
the `Ast` to an `Hir`. It’s also exposed as top-level
[`parse`](fn.parse.html "fn regex_syntax::parse") free function.

# <a href="#example" class="doc-anchor">§</a>Example

This example shows how to parse a pattern string into its HIR:

``` {.rust .rust-example-rendered}
use regex_syntax::{hir::Hir, parse};

let hir = parse("a|b")?;
assert_eq!(hir, Hir::alternation(vec![
    Hir::literal("a".as_bytes()),
    Hir::literal("b".as_bytes()),
]));
```

# <a href="#concrete-syntax-supported" class="doc-anchor">§</a>Concrete syntax supported

The concrete syntax is documented as part of the public API of the
[`regex` crate](https://docs.rs/regex/%2A/regex/#syntax).

# <a href="#input-safety" class="doc-anchor">§</a>Input safety

A key feature of this library is that it is safe to use with end user
facing input. This plays a significant role in the internal
implementation. In particular:

1.  Parsers provide a `nest_limit` option that permits callers to
    control how deeply nested a regular expression is allowed to be.
    This makes it possible to do case analysis over an `Ast` or an `Hir`
    using recursion without worrying about stack overflow.
2.  Since relying on a particular stack size is brittle, this crate goes
    to great lengths to ensure that all interactions with both the `Ast`
    and the `Hir` do not use recursion. Namely, they use constant stack
    space and heap space proportional to the size of the original
    pattern string (in bytes). This includes the type’s corresponding
    destructors. (One exception to this is literal extraction, but this
    will eventually get fixed.)

# <a href="#error-reporting" class="doc-anchor">§</a>Error reporting

The `Display` implementations on all `Error` types exposed in this
library provide nice human readable errors that are suitable for showing
to end users in a monospace font.

# <a href="#literal-extraction" class="doc-anchor">§</a>Literal extraction

This crate provides limited support for [literal extraction from `Hir`
values](hir/literal/index.html "mod regex_syntax::hir::literal"). Be
warned that literal extraction uses recursion, and therefore, stack size
proportional to the size of the `Hir`.

The purpose of literal extraction is to speed up searches. That is, if
you know a regular expression must match a prefix or suffix literal,
then it is often quicker to search for instances of that literal, and
then confirm or deny the match using the full regular expression engine.
These optimizations are done automatically in the `regex` crate.

# <a href="#crate-features" class="doc-anchor">§</a>Crate features

An important feature provided by this crate is its Unicode support. This
includes things like case folding, boolean properties, general
categories, scripts and Unicode-aware support for the Perl classes `\w`,
`\s` and `\d`. However, a downside of this support is that it requires
bundling several Unicode data tables that are substantial in size.

A fair number of use cases do not require full Unicode support. For this
reason, this crate exposes a number of features to control which Unicode
data is available.

If a regular expression attempts to use a Unicode feature that is not
available because the corresponding crate feature was disabled, then
translating that regular expression to an `Hir` will return an error.
(It is still possible construct an `Ast` for such a regular expression,
since Unicode data is not used until translation to an `Hir`.) Stated
differently, enabling or disabling any of the features below can only
add or subtract from the total set of valid regular expressions.
Enabling or disabling a feature will never modify the match semantics of
a regular expression.

The following features are available:

-   **std** - Enables support for the standard library. This feature is
    enabled by default. When disabled, only `core` and `alloc` are used.
    Otherwise, enabling `std` generally just enables `std::error::Error`
    trait impls for the various error types.
-   **unicode** - Enables all Unicode features. This feature is enabled
    by default, and will always cover all Unicode features, even if more
    are added in the future.
-   **unicode-age** - Provide the data for the [Unicode `Age`
    property](https://www.unicode.org/reports/tr44/tr44-24.html#Character_Age).
    This makes it possible to use classes like `\p{Age:6.0}` to refer to
    all codepoints first introduced in Unicode 6.0
-   **unicode-bool** - Provide the data for numerous Unicode boolean
    properties. The full list is not included here, but contains
    properties like `Alphabetic`, `Emoji`, `Lowercase`, `Math`,
    `Uppercase` and `White_Space`.
-   **unicode-case** - Provide the data for case insensitive matching
    using [Unicode’s “simple loose matches”
    specification](https://www.unicode.org/reports/tr18/#Simple_Loose_Matches).
-   **unicode-gencat** - Provide the data for [Unicode general
    categories](https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values).
    This includes, but is not limited to, `Decimal_Number`, `Letter`,
    `Math_Symbol`, `Number` and `Punctuation`.
-   **unicode-perl** - Provide the data for supporting the Unicode-aware
    Perl character classes, corresponding to `\w`, `\s` and `\d`. This
    is also necessary for using Unicode-aware word boundary assertions.
    Note that if this feature is disabled, the `\s` and `\d` character
    classes are still available if the `unicode-bool` and
    `unicode-gencat` features are enabled, respectively.
-   **unicode-script** - Provide the data for [Unicode scripts and
    script extensions](https://www.unicode.org/reports/tr24/). This
    includes, but is not limited to, `Arabic`, `Cyrillic`, `Hebrew`,
    `Latin` and `Thai`.
-   **unicode-segment** - Provide the data necessary to provide the
    properties used to implement the [Unicode text segmentation
    algorithms](https://www.unicode.org/reports/tr29/). This enables
    using classes like `\p{gcb=Extend}`, `\p{wb=Katakana}` and
    `\p{sb=ATerm}`.
-   **arbitrary** - Enabling this feature introduces a public dependency
    on the [`arbitrary`](https://crates.io/crates/arbitrary) crate.
    Namely, it implements the `Arbitrary` trait from that crate for the
    [`Ast`](ast/enum.Ast.html "enum regex_syntax::ast::Ast") type. This
    feature is disabled by default.

# Modules<a href="#modules" class="anchor">§</a>

<a href="ast/index.html" class="mod"
title="mod regex_syntax::ast">ast</a>  
Defines an abstract syntax for regular expressions.

<a href="hir/index.html" class="mod"
title="mod regex_syntax::hir">hir</a>  
Defines a high-level intermediate (HIR) representation for regular
expressions.

<a href="utf8/index.html" class="mod"
title="mod regex_syntax::utf8">utf8</a>  
Converts ranges of Unicode scalar values to equivalent ranges of UTF-8
bytes.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Parser.html" class="struct"
title="struct regex_syntax::Parser">Parser</a>  
A convenience parser for regular expressions.

<a href="struct.ParserBuilder.html" class="struct"
title="struct regex_syntax::ParserBuilder">ParserBuilder</a>  
A builder for a regular expression parser.

<a href="struct.UnicodeWordError.html" class="struct"
title="struct regex_syntax::UnicodeWordError">UnicodeWordError</a>  
An error that occurs when the Unicode-aware `\w` class is unavailable.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum regex_syntax::Error">Error</a>  
This error type encompasses any error that can be returned by this
crate.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.escape.html" class="fn"
title="fn regex_syntax::escape">escape</a>  
Escapes all regular expression meta characters in `text`.

<a href="fn.escape_into.html" class="fn"
title="fn regex_syntax::escape_into">escape_into</a>  
Escapes all meta characters in `text` and writes the result into `buf`.

<a href="fn.is_escapeable_character.html" class="fn"
title="fn regex_syntax::is_escapeable_character">is_escapeable_character</a>  
Returns true if the given character can be escaped in a regex.

<a href="fn.is_meta_character.html" class="fn"
title="fn regex_syntax::is_meta_character">is_meta_character</a>  
Returns true if the given character has significance in a regex.

<a href="fn.is_word_byte.html" class="fn"
title="fn regex_syntax::is_word_byte">is_word_byte</a>  
Returns true if and only if the given character is an ASCII word
character.

<a href="fn.is_word_character.html" class="fn"
title="fn regex_syntax::is_word_character">is_word_character</a>  
Returns true if and only if the given character is a Unicode word
character.

<a href="fn.parse.html" class="fn"
title="fn regex_syntax::parse">parse</a>  
A convenience routine for parsing a regex using default options.

<a href="fn.try_is_word_character.html" class="fn"
title="fn regex_syntax::try_is_word_character">try_is_word_character</a>  
Returns true if and only if the given character is a Unicode word
character.
