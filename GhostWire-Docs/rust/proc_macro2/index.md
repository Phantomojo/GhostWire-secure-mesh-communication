Crate proc\_macro2 Copy item path

<span class="sub-heading"><a href="../src/proc_macro2/lib.rs.html#1-1521" class="src">Source</a>
</span>

Expand description

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/proc-macro2) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/proc-macro2) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](index.html "mod proc_macro2")

  

A wrapper around the procedural macro API of the compiler’s
[`proc_macro`](https://doc.rust-lang.org/1.93.1/proc_macro/index.html "mod proc_macro")
crate. This library serves two purposes:

-   **Bring proc-macro-like functionality to other contexts like
    build.rs and main.rs.** Types from `proc_macro` are entirely
    specific to procedural macros and cannot ever exist in code outside
    of a procedural macro. Meanwhile `proc_macro2` types may exist
    anywhere including non-macro code. By developing foundational
    libraries like [syn](https://github.com/dtolnay/syn) and
    [quote](https://github.com/dtolnay/quote) against `proc_macro2`
    rather than `proc_macro`, the procedural macro ecosystem becomes
    easily applicable to many other use cases and we avoid
    reimplementing non-macro equivalents of those libraries.

-   **Make procedural macros unit testable.** As a consequence of being
    specific to procedural macros, nothing that uses `proc_macro` can be
    executed from a unit test. In order for helper libraries or
    components of a macro to be testable in isolation, they must be
    implemented using `proc_macro2`.

# <a href="#usage" class="doc-anchor">§</a>Usage

The skeleton of a typical procedural macro typically looks like this:

``` {.rust .rust-example-rendered}
extern crate proc_macro;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = {
        /* transform input */
    };

    proc_macro::TokenStream::from(output)
}
```

If parsing with [Syn](https://github.com/dtolnay/syn), you’ll use
[`parse_macro_input!`](https://docs.rs/syn/2.0/syn/macro.parse_macro_input.html)
instead to propagate parse errors correctly back to the compiler when
parsing fails.

# <a href="#unstable-features" class="doc-anchor">§</a>Unstable features

The default feature set of proc-macro2 tracks the most recent stable
compiler API. Functionality in `proc_macro` that is not yet stable is
not exposed by proc-macro2 by default.

To opt into the additional APIs available in the most recent nightly
compiler, the `procmacro2_semver_exempt` config flag must be passed to
rustc. We will polyfill those nightly-only APIs back to Rust 1.68.0. As
these are unstable APIs that track the nightly compiler, minor versions
of proc-macro2 may make breaking changes to them at any time.

``` language-sh
RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build
```

Note that this must not only be done for your crate, but for any crate
that depends on your crate. This infectious nature is intentional, as it
serves as a reminder that you are outside of the normal semver
guarantees.

Semver exempt methods are marked as such in the proc-macro2
documentation.

# <a href="#thread-safety" class="doc-anchor">§</a>Thread-Safety

Most types in this crate are `!Sync` because the underlying compiler
types make use of thread-local memory, meaning they cannot be accessed
from a different thread.

# Modules<a href="#modules" class="anchor">§</a>

<a href="extra/index.html" class="mod"
title="mod proc_macro2::extra">extra</a>  
Items which do not have a correspondence to any API in the proc\_macro
crate, but are necessary to include in proc-macro2.

<a href="token_stream/index.html" class="mod"
title="mod proc_macro2::token_stream">token_stream</a>  
Public implementation details for the `TokenStream` type, such as
iterators.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Group.html" class="struct"
title="struct proc_macro2::Group">Group</a>  
A delimited token stream.

<a href="struct.Ident.html" class="struct"
title="struct proc_macro2::Ident">Ident</a>  
A word of Rust code, which may be a keyword or legal variable name.

<a href="struct.LexError.html" class="struct"
title="struct proc_macro2::LexError">LexError</a>  
Error returned from `TokenStream::from_str`.

<a href="struct.Literal.html" class="struct"
title="struct proc_macro2::Literal">Literal</a>  
A literal string (`"hello"`), byte string (`b"hello"`), character
(`'a'`), byte character (`b'a'`), an integer or floating point number
with or without a suffix (`1`, `1u8`, `2.3`, `2.3f32`).

<a href="struct.Punct.html" class="struct"
title="struct proc_macro2::Punct">Punct</a>  
A `Punct` is a single punctuation character like `+`, `-` or `#`.

<a href="struct.Span.html" class="struct"
title="struct proc_macro2::Span">Span</a>  
A region of source code, along with macro expansion information.

<a href="struct.TokenStream.html" class="struct"
title="struct proc_macro2::TokenStream">TokenStream</a>  
An abstract stream of tokens, or more concretely a sequence of token
trees.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Delimiter.html" class="enum"
title="enum proc_macro2::Delimiter">Delimiter</a>  
Describes how a sequence of token trees is delimited.

<a href="enum.Spacing.html" class="enum"
title="enum proc_macro2::Spacing">Spacing</a>  
Whether a `Punct` is followed immediately by another `Punct` or followed
by another token or whitespace.

<a href="enum.TokenTree.html" class="enum"
title="enum proc_macro2::TokenTree">TokenTree</a>  
A single token or a delimited sequence of token trees (e.g.
`[1, (), ..]`).
