Crate clap\_builder Copy item path

<span class="sub-heading"><a href="../src/clap_builder/lib.rs.html#6-53" class="src">Source</a>
</span>

Expand description

# <a href="#clap_builder" class="doc-anchor">§</a>`clap_builder`

Builder implementation for clap.

[docs.rs](https://docs.rs/clap)

-   [Derive
    Tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
-   [Derive
    Reference](https://docs.rs/clap/latest/clap/_derive/index.html)

## <a href="#license" class="doc-anchor">§</a>License

Licensed under either of

-   Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
    <https://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or
    <https://opensource.org/license/mit>)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms
or conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more details.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::builder::`<a href="builder/enum.ArgAction.html" class="enum"
title="enum clap_builder::builder::ArgAction"><code>ArgAction</code></a>`;`

`pub use crate::builder::`<a href="builder/struct.Command.html" class="struct"
title="struct clap_builder::builder::Command"><code>Command</code></a>`;`

`pub use crate::builder::`<a href="builder/enum.ValueHint.html" class="enum"
title="enum clap_builder::builder::ValueHint"><code>ValueHint</code></a>`;`

`pub use crate::builder::`<a href="builder/struct.Arg.html" class="struct"
title="struct clap_builder::builder::Arg"><code>Arg</code></a>`;`

`pub use crate::builder::`<a href="builder/struct.ArgGroup.html" class="struct"
title="struct clap_builder::builder::ArgGroup"><code>ArgGroup</code></a>`;`

`pub use crate::parser::`<a href="parser/struct.ArgMatches.html" class="struct"
title="struct clap_builder::parser::ArgMatches"><code>ArgMatches</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="builder/index.html" class="mod"
title="mod clap_builder::builder">builder</a>  
Define
[`Command`](builder/struct.Command.html "struct clap_builder::builder::Command")
line
[arguments](builder/struct.Arg.html "struct clap_builder::builder::Arg")

<a href="error/index.html" class="mod"
title="mod clap_builder::error">error</a>  
Error reporting

<a href="parser/index.html" class="mod"
title="mod clap_builder::parser">parser</a>  
[`Command`](builder/struct.Command.html "struct clap_builder::builder::Command")
line argument parser

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.arg.html" class="macro"
title="macro clap_builder::arg">arg</a>  
Create an
[`Arg`](builder/struct.Arg.html "struct clap_builder::builder::Arg")
from a usage string.

<a href="macro.command.html" class="macro"
title="macro clap_builder::command">command</a>  
Requires `cargo` feature flag to be enabled.

<a href="macro.value_parser.html" class="macro"
title="macro clap_builder::value_parser">value_parser</a>  
Select a
[`ValueParser`](builder/struct.ValueParser.html "struct clap_builder::builder::ValueParser")
implementation from the intended type

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Id.html" class="struct"
title="struct clap_builder::Id">Id</a>  
[`Arg`](builder/struct.Arg.html "struct clap_builder::builder::Arg") or
[`ArgGroup`](builder/struct.ArgGroup.html "struct clap_builder::builder::ArgGroup")
identifier

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ColorChoice.html" class="enum"
title="enum clap_builder::ColorChoice">ColorChoice</a>  
Represents the color preferences for program output

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Args.html" class="trait"
title="trait clap_builder::Args">Args</a>  
Parse a set of arguments into a user-defined container.

<a href="trait.CommandFactory.html" class="trait"
title="trait clap_builder::CommandFactory">CommandFactory</a>  
Create a
[`Command`](builder/struct.Command.html "struct clap_builder::builder::Command")
relevant for a user-defined container.

<a href="trait.FromArgMatches.html" class="trait"
title="trait clap_builder::FromArgMatches">FromArgMatches</a>  
Converts an instance of
[`ArgMatches`](parser/struct.ArgMatches.html "struct clap_builder::parser::ArgMatches")
to a user-defined container.

<a href="trait.Parser.html" class="trait"
title="trait clap_builder::Parser">Parser</a>  
Parse command-line arguments into `Self`.

<a href="trait.Subcommand.html" class="trait"
title="trait clap_builder::Subcommand">Subcommand</a>  
Parse a sub-command into a user-defined enum.

<a href="trait.ValueEnum.html" class="trait"
title="trait clap_builder::ValueEnum">ValueEnum</a>  
Parse arguments into enums.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Error.html" class="type"
title="type clap_builder::Error">Error</a>  
Command Line Argument Parser Error
