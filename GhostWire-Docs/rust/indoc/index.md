Crate indoc Copy item path

<span class="sub-heading"><a href="../src/indoc/lib.rs.html#1-509" class="src">Source</a>
</span>

Expand description

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/indoc) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/indoc) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/indoc)

  

This crate provides a procedural macro for indented string literals. The
`indoc!()` macro takes a multiline string literal and un-indents it at
compile time so the leftmost non-space character is in the first column.

``` language-toml
[dependencies]
indoc = "2"
```

  

# <a href="#using-indoc" class="doc-anchor">§</a>Using indoc

``` {.rust .rust-example-rendered}
use indoc::indoc;

fn main() {
    let testing = indoc! {"
        def hello():
            print('Hello, world!')

        hello()
    "};
    let expected = "def hello():\n    print('Hello, world!')\n\nhello()\n";
    assert_eq!(testing, expected);
}
```

Indoc also works with raw string literals:

``` {.rust .rust-example-rendered}
use indoc::indoc;

fn main() {
    let testing = indoc! {r#"
        def hello():
            print("Hello, world!")

        hello()
    "#};
    let expected = "def hello():\n    print(\"Hello, world!\")\n\nhello()\n";
    assert_eq!(testing, expected);
}
```

And byte string literals:

``` {.rust .rust-example-rendered}
use indoc::indoc;

fn main() {
    let testing = indoc! {b"
        def hello():
            print('Hello, world!')

        hello()
    "};
    let expected = b"def hello():\n    print('Hello, world!')\n\nhello()\n";
    assert_eq!(testing[..], expected[..]);
}
```

  
  

# <a href="#formatting-macros" class="doc-anchor">§</a>Formatting macros

The indoc crate exports five additional macros to substitute
conveniently for the standard library’s formatting macros:

-   `formatdoc!($fmt, ...)` — equivalent to `format!(indoc!($fmt), ...)`
-   `printdoc!($fmt, ...)` — equivalent to `print!(indoc!($fmt), ...)`
-   `eprintdoc!($fmt, ...)` — equivalent to `eprint!(indoc!($fmt), ...)`
-   `writedoc!($dest, $fmt, ...)` — equivalent to
    `write!($dest, indoc!($fmt), ...)`
-   `concatdoc!(...)` — equivalent to `concat!(...)` with each string
    literal wrapped in `indoc!`

``` {.rust .rust-example-rendered}
use indoc::{concatdoc, printdoc};

const HELP: &str = concatdoc! {"
    Usage: ", env!("CARGO_BIN_NAME"), " [options]

    Options:
        -h, --help
"};

fn main() {
    printdoc! {"
        GET {url}
        Accept: {mime}
        ",
        url = "http://localhost:8080",
        mime = "application/json",
    }
}
```

  
  

# <a href="#explanation" class="doc-anchor">§</a>Explanation

The following rules characterize the behavior of the `indoc!()` macro:

1.  Count the leading spaces of each line, ignoring the first line and
    any lines that are empty or contain spaces only.
2.  Take the minimum.
3.  If the first line is empty i.e. the string begins with a newline,
    remove the first line.
4.  Remove the computed number of spaces from the beginning of each
    line.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.concatdoc.html" class="macro"
title="macro indoc::concatdoc">concatdoc</a>  
Unindent and call `concat!`.

<a href="macro.eprintdoc.html" class="macro"
title="macro indoc::eprintdoc">eprintdoc</a>  
Unindent and call `eprint!`.

<a href="macro.formatdoc.html" class="macro"
title="macro indoc::formatdoc">formatdoc</a>  
Unindent and call `format!`.

<a href="macro.indoc.html" class="macro"
title="macro indoc::indoc">indoc</a>  
Unindent and produce `&'static str` or `&'static [u8]`.

<a href="macro.printdoc.html" class="macro"
title="macro indoc::printdoc">printdoc</a>  
Unindent and call `print!`.

<a href="macro.writedoc.html" class="macro"
title="macro indoc::writedoc">writedoc</a>  
Unindent and call `write!`.
