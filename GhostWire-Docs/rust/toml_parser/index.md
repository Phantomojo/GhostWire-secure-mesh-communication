Crate toml\_parser Copy item path

<span class="sub-heading"><a href="../src/toml_parser/lib.rs.html#1-49" class="src">Source</a>
</span>

Expand description

TOML lexer and parser

Characteristics:

-   Error recovery
-   Lazy validation
-   `forbid(unsafe)` by default, requiring the `unsafe` feature
    otherwise
-   `no_std` support, including putting users in charge of allocation
    choices (including not allocating)

Full parsing is broken into three phases:

1.  [Lexing tokens](lexer/index.html "mod toml_parser::lexer")
2.  [Parsing tokens](parser/index.html "mod toml_parser::parser") (push
    parser)
3.  Organizing the physical layout into the logical layout, including
    [decoding keys and
    values](decoder/index.html "mod toml_parser::decoder")

# Modules<a href="#modules" class="anchor">§</a>

<a href="decoder/index.html" class="mod"
title="mod toml_parser::decoder">decoder</a>  
Decode [raw](struct.Raw.html "struct toml_parser::Raw") TOML values into
Rust native types

<a href="lexer/index.html" class="mod"
title="mod toml_parser::lexer">lexer</a>  
Lex TOML tokens

<a href="parser/index.html" class="mod"
title="mod toml_parser::parser">parser</a>  
A TOML push
[parser](parser/fn.parse_document.html "fn toml_parser::parser::parse_document")

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ParseError.html" class="struct"
title="struct toml_parser::ParseError">ParseError</a>  
<a href="struct.Raw.html" class="struct"
title="struct toml_parser::Raw">Raw</a>  
A slice of [`Source`](struct.Source.html "struct toml_parser::Source")

<a href="struct.Source.html" class="struct"
title="struct toml_parser::Source">Source</a>  
Data encoded as TOML

<a href="struct.Span.html" class="struct"
title="struct toml_parser::Span">Span</a>  
Location within the
[`Source`](struct.Source.html "struct toml_parser::Source")

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Expected.html" class="enum"
title="enum toml_parser::Expected">Expected</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ErrorSink.html" class="trait"
title="trait toml_parser::ErrorSink">ErrorSink</a>  
<a href="trait.SourceIndex.html" class="trait"
title="trait toml_parser::SourceIndex">SourceIndex</a>  
A helper trait used for indexing operations on
[`Source`](struct.Source.html "struct toml_parser::Source")
