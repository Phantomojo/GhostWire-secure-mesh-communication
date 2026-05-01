Crate clap\_lex Copy item path

<span class="sub-heading"><a href="../src/clap_lex/lib.rs.html#1-526" class="src">Source</a>
</span>

Expand description

Minimal, flexible command-line parser

As opposed to a declarative parser, this processes arguments as a stream
of tokens. As lexing a command-line is not context-free, we rely on the
caller to decide how to interpret the arguments.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use std::path::PathBuf;
use std::ffi::OsStr;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
struct Args {
    paths: Vec<PathBuf>,
    color: Color,
    verbosity: usize,
}

#[derive(Debug)]
enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    fn parse(s: Option<&OsStr>) -> Result<Self, BoxedError> {
        let s = s.map(|s| s.to_str().ok_or(s));
        match s {
            Some(Ok("always")) | Some(Ok("")) | None => {
                Ok(Color::Always)
            }
            Some(Ok("auto")) => {
                Ok(Color::Auto)
            }
            Some(Ok("never")) => {
                Ok(Color::Never)
            }
            Some(invalid) => {
                Err(format!("Invalid value for `--color`, {invalid:?}").into())
            }
        }
    }
}

fn parse_args(
    raw: impl IntoIterator<Item=impl Into<std::ffi::OsString>>
) -> Result<Args, BoxedError> {
    let mut args = Args {
        paths: Vec::new(),
        color: Color::Auto,
        verbosity: 0,
    };

    let raw = clap_lex::RawArgs::new(raw);
    let mut cursor = raw.cursor();
    raw.next(&mut cursor);  // Skip the bin
    while let Some(arg) = raw.next(&mut cursor) {
        if arg.is_escape() {
            args.paths.extend(raw.remaining(&mut cursor).map(PathBuf::from));
        } else if arg.is_stdio() {
            args.paths.push(PathBuf::from("-"));
        } else if let Some((long, value)) = arg.to_long() {
            match long {
                Ok("verbose") => {
                    if let Some(value) = value {
                        return Err(format!("`--verbose` does not take a value, got `{value:?}`").into());
                    }
                    args.verbosity += 1;
                }
                Ok("color") => {
                    args.color = Color::parse(value)?;
                }
                _ => {
                    return Err(
                        format!("Unexpected flag: --{}", arg.display()).into()
                    );
                }
            }
        } else if let Some(mut shorts) = arg.to_short() {
            while let Some(short) = shorts.next_flag() {
                match short {
                    Ok('v') => {
                        args.verbosity += 1;
                    }
                    Ok('c') => {
                        let value = shorts.next_value_os();
                        args.color = Color::parse(value)?;
                    }
                    Ok(c) => {
                        return Err(format!("Unexpected flag: -{c}").into());
                    }
                    Err(e) => {
                        return Err(format!("Unexpected flag: -{}", e.to_string_lossy()).into());
                    }
                }
            }
        } else {
            args.paths.push(PathBuf::from(arg.to_value_os().to_owned()));
        }
    }

    Ok(args)
}

let args = parse_args(["bin", "--hello", "world"]);
println!("{args:?}");
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ArgCursor.html" class="struct"
title="struct clap_lex::ArgCursor">ArgCursor</a>  
Position within
[`RawArgs`](struct.RawArgs.html "struct clap_lex::RawArgs")

<a href="struct.ParsedArg.html" class="struct"
title="struct clap_lex::ParsedArg">ParsedArg</a>  
Command-line Argument

<a href="struct.RawArgs.html" class="struct"
title="struct clap_lex::RawArgs">RawArgs</a>  
Command-line arguments

<a href="struct.ShortFlags.html" class="struct"
title="struct clap_lex::ShortFlags">ShortFlags</a>  
Walk through short flags within a
[`ParsedArg`](struct.ParsedArg.html "struct clap_lex::ParsedArg")

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.SeekFrom.html" class="enum"
title="enum clap_lex::SeekFrom">SeekFrom</a>  
Enumeration of possible methods to seek within an I/O object.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.OsStrExt.html" class="trait"
title="trait clap_lex::OsStrExt">OsStrExt</a>  
String-like methods for
[`OsStr`](https://doc.rust-lang.org/1.93.1/std/ffi/os_str/struct.OsStr.html "struct std::ffi::os_str::OsStr")
