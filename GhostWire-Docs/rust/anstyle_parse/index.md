Crate anstyle\_parse Copy item path

<span class="sub-heading"><a href="../src/anstyle_parse/lib.rs.html#1-442" class="src">Source</a>
</span>

Expand description

Parser for implementing virtual terminal emulators

[`Parser`](struct.Parser.html "struct anstyle_parse::Parser") is
implemented according to [Paul Williams’ ANSI parser state
machine](https://vt100.net/emu/dec_ansi_parser). The state machine
doesn’t assign meaning to the parsed data and is thus not itself
sufficient for writing a terminal emulator. Instead, it is expected that
an implementation of
[`Perform`](trait.Perform.html "trait anstyle_parse::Perform") is
provided which does something useful with the parsed data. The
[`Parser`](struct.Parser.html "struct anstyle_parse::Parser") handles
the book keeping, and the
[`Perform`](trait.Perform.html "trait anstyle_parse::Perform") gets to
simply handle actions.

# <a href="#examples" class="doc-anchor">§</a>Examples

For an example of using the
[`Parser`](struct.Parser.html "struct anstyle_parse::Parser") please see
the examples folder. The example included there simply logs all the
actions [`Perform`](trait.Perform.html "trait anstyle_parse::Perform")
does. One quick thing to see it in action is to pipe `vim` into it

``` language-sh
cargo build --release --example parselog
vim | target/release/examples/parselog
```

Just type `:q` to exit.

# <a href="#differences-from-original-state-machine-description"
class="doc-anchor">§</a>Differences from original state machine description

-   UTF-8 Support for Input
-   OSC Strings can be terminated by 0x07
-   Only supports 7-bit codes. Some 8-bit codes are still supported, but
    they no longer work in all states.

# Modules<a href="#modules" class="anchor">§</a>

<a href="state/index.html" class="mod"
title="mod anstyle_parse::state">state</a>  
ANSI escape code parsing state machine

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AsciiParser.html" class="struct"
title="struct anstyle_parse::AsciiParser">AsciiParser</a>  
Only allow parsing 7-bit ASCII

<a href="struct.Params.html" class="struct"
title="struct anstyle_parse::Params">Params</a>  
<a href="struct.ParamsIter.html" class="struct"
title="struct anstyle_parse::ParamsIter">ParamsIter</a>  
Immutable subparameter iterator.

<a href="struct.Parser.html" class="struct"
title="struct anstyle_parse::Parser">Parser</a>  
Parser for raw *VTE* protocol which delegates actions to a
[`Perform`](trait.Perform.html "trait anstyle_parse::Perform")

<a href="struct.Utf8Parser.html" class="struct"
title="struct anstyle_parse::Utf8Parser">Utf8Parser</a>  
Allow parsing UTF-8

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.CharAccumulator.html" class="trait"
title="trait anstyle_parse::CharAccumulator">CharAccumulator</a>  
Build a `char` out of bytes

<a href="trait.Perform.html" class="trait"
title="trait anstyle_parse::Perform">Perform</a>  
Performs actions requested by the
[`Parser`](struct.Parser.html "struct anstyle_parse::Parser")

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.DefaultCharAccumulator.html" class="type"
title="type anstyle_parse::DefaultCharAccumulator">DefaultCharAccumulator</a>  
Most flexible
[`CharAccumulator`](trait.CharAccumulator.html "trait anstyle_parse::CharAccumulator")
for [`Parser`](struct.Parser.html "struct anstyle_parse::Parser") based
on active features
