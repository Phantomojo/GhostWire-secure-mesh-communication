Crate quote Copy item path

<span class="sub-heading"><a href="../src/quote/lib.rs.html#1-1455" class="src">Source</a>
</span>

Expand description

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/quote) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/quote) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/quote)

  

This crate provides the
[`quote!`](macro.quote.html "macro quote::quote") macro for turning Rust
syntax tree data structures into tokens of source code.

Procedural macros in Rust receive a stream of tokens as input, execute
arbitrary Rust code to determine how to manipulate those tokens, and
produce a stream of tokens to hand back to the compiler to compile into
the caller’s crate. Quasi-quoting is a solution to one piece of that —
producing tokens to return to the compiler.

The idea of quasi-quoting is that we write *code* that we treat as
*data*. Within the `quote!` macro, we can write what looks like code to
our text editor or IDE. We get all the benefits of the editor’s brace
matching, syntax highlighting, indentation, and maybe autocompletion.
But rather than compiling that as code into the current crate, we can
treat it as data, pass it around, mutate it, and eventually hand it back
to the compiler as tokens to compile into the macro caller’s crate.

This crate is motivated by the procedural macro use case, but is a
general-purpose Rust quasi-quoting library and is not specific to
procedural macros.

``` language-toml
[dependencies]
quote = "1.0"
```

  

# <a href="#example" class="doc-anchor">§</a>Example

The following quasi-quoted block of code is something you might find in
[a](https://serde.rs/) procedural macro having to do with data structure
serialization. The `#var` syntax performs interpolation of runtime
variables into the quoted tokens. Check out the documentation of the
[`quote!`](macro.quote.html "macro quote::quote") macro for more detail
about the syntax. See also the
[`quote_spanned!`](macro.quote_spanned.html "macro quote::quote_spanned")
macro which is important for implementing hygienic procedural macros.

``` {.rust .rust-example-rendered}
let tokens = quote! {
    struct SerializeWith #generics #where_clause {
        value: &'a #field_ty,
        phantom: core::marker::PhantomData<#item_ty>,
    }

    impl #generics serde::Serialize for SerializeWith #generics #where_clause {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            #path(self.value, serializer)
        }
    }

    SerializeWith {
        value: #value,
        phantom: core::marker::PhantomData::<#item_ty>,
    }
};
```

  

# <a href="#non-macro-code-generators" class="doc-anchor">§</a>Non-macro code generators

When using `quote` in a build.rs or main.rs and writing the output out
to a file, consider having the code generator pass the tokens through
[prettyplease](https://github.com/dtolnay/prettyplease) before writing.
This way if an error occurs in the generated code it is convenient for a
human to read and debug.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.format_ident.html" class="macro"
title="macro quote::format_ident">format_ident</a>  
Formatting macro for constructing `Ident`s.

<a href="macro.quote.html" class="macro"
title="macro quote::quote">quote</a>  
The whole point.

<a href="macro.quote_spanned.html" class="macro"
title="macro quote::quote_spanned">quote_spanned</a>  
Same as `quote!`, but applies a given span to all tokens originating
within the macro invocation.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.IdentFragment.html" class="trait"
title="trait quote::IdentFragment">IdentFragment</a>  
Specialized formatting trait used by `format_ident!`.

<a href="trait.ToTokens.html" class="trait"
title="trait quote::ToTokens">ToTokens</a>  
Types that can be interpolated inside a `quote!` invocation.

<a href="trait.TokenStreamExt.html" class="trait"
title="trait quote::TokenStreamExt">TokenStreamExt</a>  
TokenStream extension trait with methods for appending tokens.
