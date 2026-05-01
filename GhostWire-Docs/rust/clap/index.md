Crate clap Copy item path

<span class="sub-heading"><a href="../src/clap/lib.rs.html#6-110" class="src">Source</a>
</span>

Expand description

> **Command Line Argument Parser for Rust**

Quick Links:

-   Derive \[tutorial\]\[\_derive::\_tutorial\] and
    \[reference\]\[\_derive\]
-   Builder \[tutorial\]\[\_tutorial\] and
    [reference](struct.Command.html "struct clap::Command")
-   \[Cookbook\]\[\_cookbook\]
-   \[CLI Concepts\]\[\_concepts\]
-   \[FAQ\]\[\_faq\]
-   [Discussions](https://github.com/clap-rs/clap/discussions)
-   [CHANGELOG](https://github.com/clap-rs/clap/blob/v4.5.53/CHANGELOG.md)
    (includes major version migration guides)

## <a href="#aspirations" class="doc-anchor">§</a>Aspirations

-   Out of the box, users get a polished CLI experience
    -   Including common argument behavior, help generation, suggested
        fixes for users, colored output, [shell
        completions](https://github.com/clap-rs/clap/tree/master/clap_complete),
        etc
-   Flexible enough to port your existing CLI interface
    -   However, we won’t necessarily streamline support for each use
        case
-   Reasonable parse performance
-   Resilient maintainership, including
    -   Willing to break compatibility rather than batching up breaking
        changes in large releases
    -   Leverage feature flags to keep to one active branch
    -   Being under [WG-CLI](https://github.com/rust-cli/team/) to
        increase the bus factor
-   We follow semver and will wait about 6-9 months between major
    breaking changes
-   We will support the last two minor Rust releases (MSRV, currently
    1.74)

While these aspirations can be at odds with fast build times and low
binary size, we will still strive to keep these reasonable for the
flexibility you get. Check out the
[argparse-benchmarks](https://github.com/rust-cli/argparse-benchmarks-rs)
for CLI parsers optimized for other use cases.

## <a href="#example" class="doc-anchor">§</a>Example

Run

``` language-console
$ cargo add clap --features derive
```

*(See also \[feature flag reference\]\[\_features\])*

Then define your CLI in `main.rs`:

``` {.rust .rust-example-rendered}
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
```

And try it out:

``` language-console
$ demo --help
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: demo[EXE] [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version

$ demo --name Me
Hello Me!
```

*(version number and `.exe` extension on windows replaced by
placeholders)*

See also the derive \[tutorial\]\[\_derive::\_tutorial\] and
\[reference\]\[\_derive\]

### <a href="#related-projects" class="doc-anchor">§</a>Related Projects

Augment clap:

-   [wild](https://crates.io/crates/wild) for supporting wildcards (`*`)
    on Windows like you do Linux
-   [argfile](https://crates.io/crates/argfile) for loading additional
    arguments from a file (aka response files)
-   [shadow-rs](https://crates.io/crates/shadow-rs) for generating
    `Command::long_version`
-   [clap\_mangen](https://crates.io/crates/clap_mangen) for generating
    man page source (roff)
-   [clap\_complete](https://crates.io/crates/clap_complete) for shell
    completion support
-   [clap-i18n-richformatter](https://crates.io/crates/clap-i18n-richformatter)
    for i18n support with `clap::error::RichFormatter`

CLI Helpers

-   [clio](https://crates.io/crates/clio) for reading/writing to files
    specified as arguments
-   [clap-verbosity-flag](https://crates.io/crates/clap-verbosity-flag)
-   [clap-cargo](https://crates.io/crates/clap-cargo)
-   [colorchoice-clap](https://crates.io/crates/colorchoice-clap)

Testing

-   [`trycmd`](https://crates.io/crates/trycmd): Bulk snapshot testing
-   [`snapbox`](https://crates.io/crates/snapbox): Specialized snapshot
    testing
-   [`assert_cmd`](https://crates.io/crates/assert_cmd) and
    [`assert_fs`](https://crates.io/crates/assert_fs): Customized
    testing

Documentation:

-   [Command-line Apps for
    Rust](https://rust-cli.github.io/book/index.html) book

# Modules<a href="#modules" class="anchor">§</a>

<a href="builder/index.html" class="mod"
title="mod clap::builder">builder</a>  
Define [`Command`](struct.Command.html "struct clap::Command") line
[arguments](struct.Arg.html "struct clap::Arg")

<a href="error/index.html" class="mod" title="mod clap::error">error</a>  
Error reporting

<a href="parser/index.html" class="mod"
title="mod clap::parser">parser</a>  
[`Command`](struct.Command.html "struct clap::Command") line argument
parser

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.arg.html" class="macro" title="macro clap::arg">arg</a>  
Create an [`Arg`](struct.Arg.html "struct clap::Arg") from a usage
string.

<a href="macro.command.html" class="macro"
title="macro clap::command">command</a>  
Requires `cargo` feature flag to be enabled.

<a href="macro.value_parser.html" class="macro"
title="macro clap::value_parser">value_parser</a>  
Select a
[`ValueParser`](builder/struct.ValueParser.html "struct clap::builder::ValueParser")
implementation from the intended type

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Arg.html" class="struct"
title="struct clap::Arg">Arg</a>  
The abstract representation of a command line argument. Used to set all
the options and relationships that define a valid argument for the
program.

<a href="struct.ArgGroup.html" class="struct"
title="struct clap::ArgGroup">ArgGroup</a>  
Specifies a logical group of
[arguments](struct.Arg.html "struct clap::Arg")

<a href="struct.ArgMatches.html" class="struct"
title="struct clap::ArgMatches">ArgMatches</a>  
Container for parse results.

<a href="struct.Command.html" class="struct"
title="struct clap::Command">Command</a>  
Build a command-line interface.

<a href="struct.Id.html" class="struct" title="struct clap::Id">Id</a>  
[`Arg`](struct.Arg.html "struct clap::Arg") or
[`ArgGroup`](struct.ArgGroup.html "struct clap::ArgGroup") identifier

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ArgAction.html" class="enum"
title="enum clap::ArgAction">ArgAction</a>  
Behavior of arguments when they are encountered while parsing

<a href="enum.ColorChoice.html" class="enum"
title="enum clap::ColorChoice">ColorChoice</a>  
Represents the color preferences for program output

<a href="enum.ValueHint.html" class="enum"
title="enum clap::ValueHint">ValueHint</a>  
Provide shell with hint on how to complete an argument.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Args.html" class="trait"
title="trait clap::Args">Args</a>  
Parse a set of arguments into a user-defined container.

<a href="trait.CommandFactory.html" class="trait"
title="trait clap::CommandFactory">CommandFactory</a>  
Create a [`Command`](struct.Command.html "struct clap::Command")
relevant for a user-defined container.

<a href="trait.FromArgMatches.html" class="trait"
title="trait clap::FromArgMatches">FromArgMatches</a>  
Converts an instance of
[`ArgMatches`](struct.ArgMatches.html "struct clap::ArgMatches") to a
user-defined container.

<a href="trait.Parser.html" class="trait"
title="trait clap::Parser">Parser</a>  
Parse command-line arguments into `Self`.

<a href="trait.Subcommand.html" class="trait"
title="trait clap::Subcommand">Subcommand</a>  
Parse a sub-command into a user-defined enum.

<a href="trait.ValueEnum.html" class="trait"
title="trait clap::ValueEnum">ValueEnum</a>  
Parse arguments into enums.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Error.html" class="type"
title="type clap::Error">Error</a>  
Command Line Argument Parser Error
