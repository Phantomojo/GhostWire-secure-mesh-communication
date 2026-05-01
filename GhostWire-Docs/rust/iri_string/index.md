Crate iri\_string Copy item path

<span class="sub-heading"><a href="../src/iri_string/lib.rs.html#1-159" class="src">Source</a>
</span>

Expand description

String types for [RFC 3987 Internationalized Resource Identifiers
(IRIs)](https://www.rfc-editor.org/rfc/rfc3987.html) and [RFC 3986
Uniform Resource Identifiers
(URIs)](https://www.rfc-editor.org/rfc/rfc3986.html).

Note that this crate does not have any extra knowledge about protocols.
Comparisons between IRI strings by `PartialEq` and `Eq` is implemented
as [simple string
comparison](https://www.rfc-editor.org/rfc/rfc3986.html#section-6.2.1).
You should implement by yourself or use another crate to use such extra
knowledge to compare IRIs / URIs.

# <a href="#capability" class="doc-anchor">§</a>Capability

This crate provides many features for IRIs / URIs.

## <a href="#string-types" class="doc-anchor">§</a>String types

[`types` module](types/index.html "mod iri_string::types") module
provides various string types for IRIs and URIs. The borrowed string
types are unsized slice types (such as `[u8]` and `str`) and not a sized
struct, so they are highly interoperable with for example `Cow` and
`Rc`. Conversions between `&str` and borrwed IRI string types are easy.

## <a href="#resolvers" class="doc-anchor">§</a>Resolvers

[`resolve` module](resolve/index.html "mod iri_string::resolve")
provides IRI / URI references resolver. However, you are recommended to
use methods of string types such as
[`RiReferenceStr::resolve_against()`](types/struct.RiReferenceStr.html#method.resolve_against "method iri_string::types::RiReferenceStr::resolve_against")
or
[`RiRelativeStr::resolve_against()`](types/struct.RiRelativeStr.html#method.resolve_against "method iri_string::types::RiRelativeStr::resolve_against")
if you don’t intend to resolve multiple IRIs against the same base.

## <a href="#validators" class="doc-anchor">§</a>Validators

Validator functions are provided from [`validate`
module](validate/index.html "mod iri_string::validate").

## <a href="#percent-encoding" class="doc-anchor">§</a>Percent encoding

[`percent_encode`
module](percent_encode/index.html "mod iri_string::percent_encode")
provides a converter to encode user-provided string into percent-encoded
one (if syntax requires so).

## <a href="#iri-builder" class="doc-anchor">§</a>IRI builder

[`build` module](build/index.html "mod iri_string::build") provides IRI
builder.

## <a href="#uri-template-rfc-6570" class="doc-anchor">§</a>URI template (RFC 6570)

[`template` module](template/index.html "mod iri_string::template")
provides an RFC 6570 URI Template processor.

# <a href="#feature-flags" class="doc-anchor">§</a>Feature flags

## <a href="#std-and-alloc-support" class="doc-anchor">§</a>`std` and `alloc` support

This crate supports `no_std` usage.

-   `alloc` feature:
    -   Std library or `alloc` crate is required.
    -   This feature enables types and functions which require memory
        allocation, e.g. `types::IriString` and
        `types::IriRelativeStr::resolve_against()`.
-   `std` feature (**enabled by default**):
    -   Std library is required.
    -   This automatically enables `alloc` feature.
    -   The feature let the crate utilize std-specific stuff, such as
        `std::error::Error` trait.
-   With neither of them:
    -   The crate can be used in `no_std` environment.

## <a href="#other-features" class="doc-anchor">§</a>Other features

-   `serde`
    -   Enables serde support.
    -   Implement `Serailize` and `Deserialize` traits for IRI / URI
        types.
-   `memchr`
    -   Enables faster internal character search.

# <a href="#rationale" class="doc-anchor">§</a>Rationale

## <a href="#foo-foo-foo-foo-foo--are-valid-iris" class="doc-anchor">§</a>`foo:`, `foo:/`, `foo://`, `foo:///`, `foo:////`, … are valid IRIs

All of these are valid IRIs. (On the other hand, all of them are invalid
as relative IRI reference, because they don’t match `relative-part`
rule, especially `path-noscheme`, as the first path component of the
relative path contains a colon.)

-   `foo:`
    -   Decomposed to `<scheme="foo">:<path-empty="">`.
-   `foo:/`
    -   Decomposed to `<scheme="foo">:<path-absolute="/">`.
-   `foo://`
    -   Decomposed to
        `<scheme="foo">://<authority=""><path-absolute="">`.
-   `foo:///`
    -   Decomposed to
        `<scheme="foo">://<authority=""><path-absolute="/">`.
-   `foo:////`
    -   Decomposed to
        `<scheme="foo">://<authority=""><path-absolute="//">`.
-   `foo://///`
    -   Decomposed to
        `<scheme="foo">://<authority=""><path-absolute="///">`.

RFC 3986 says that “if authority is absent, path cannot start with
`//`”.

> When authority is present, the path must either be empty or begin with
> a slash (“/”) character. When authority is not present, the path
> cannot begin with two slash characters (“//”).
>
> — [RFC 3986, section 3. Syntax
> Components](https://www.rfc-editor.org/rfc/rfc3986.html#section-3).

> If a URI contains an authority component, then the path component must
> either be empty or begin with a slash (“/”) character. If a URI does
> not contain an authority component, then the path cannot begin with
> two slash characters (“//”).
>
> — [RFC 3986, section 3.3.
> Path](https://www.rfc-editor.org/rfc/rfc3986.html#section-3.3)

We should interpret them as “if `authority` rule is completely unused
(i.e. does not match any strings **including empty string**), path
cannot start with `//`”. In other words, we should consider this as
**explaining the ABNF of `hier-part` rule** (especially why it does not
use `path` rule), but **not adding extra restriction to the rule written
in ABNF**.

This restriction is necessary to remove ambiguity in decomposition of
some strings. For example, it is natural to decompose `foo://` to
`<scheme="foo">:<path="//">` or
`<scheme="foo">://<authority=""><path="">`. The restriction, **which is
already encoded to the ABNF rule**, tells us to always decompose to the
latter form, rather than the former one.

Readers of the spec might be confused by “when authority is **present**”
and “if a URI **contains** an authority component, which is unclear.
However, based on the interpretation above, we should consider authority
part with empty string as satisfying the condition “authority is
**present**”.

## <a href="#iri-resolution-can-fail" class="doc-anchor">§</a>IRI resolution can fail

For some inputs, resulting string of IRI normalization and resolution
can be syntactically correct but semantically wrong. In such cases, the
normalizer and resolver provided by this crate do not silently “fix” the
IRI by non-standard processing, but just fail by returning `Err(_)`.

For details, see the documentation of
[`normalize`](normalize/index.html "mod iri_string::normalize") module.

# Modules<a href="#modules" class="anchor">§</a>

<a href="build/index.html" class="mod"
title="mod iri_string::build">build</a>  
URI/IRI builder.

<a href="components/index.html" class="mod"
title="mod iri_string::components">components</a>  
Components of IRIs.

<a href="convert/index.html" class="mod"
title="mod iri_string::convert">convert</a>  
Conversion between URI/IRI types.

<a href="format/index.html" class="mod"
title="mod iri_string::format">format</a>  
Utilities for formatting (especially `Display` trait).

<a href="mask_password/index.html" class="mod"
title="mod iri_string::mask_password">mask_password</a>  
Password masker.

<a href="normalize/index.html" class="mod"
title="mod iri_string::normalize">normalize</a>  
Normalization.

<a href="percent_encode/index.html" class="mod"
title="mod iri_string::percent_encode">percent_encode</a>  
Percent encoding.

<a href="resolve/index.html" class="mod"
title="mod iri_string::resolve">resolve</a>  
URI and IRI resolvers.

<a href="spec/index.html" class="mod"
title="mod iri_string::spec">spec</a>  
IRI specs.

<a href="template/index.html" class="mod"
title="mod iri_string::template">template</a>  
Processor for [RFC 6570](https://www.rfc-editor.org/rfc/rfc6570.html)
URI Template.

<a href="types/index.html" class="mod"
title="mod iri_string::types">types</a>  
URI and IRI types.

<a href="validate/index.html" class="mod"
title="mod iri_string::validate">validate</a>  
Validators.
