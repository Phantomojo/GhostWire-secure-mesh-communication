Crate idna Copy item path

<span class="sub-heading"><a href="../src/idna/lib.rs.html#9-198" class="src">Source</a>
</span>

Expand description

This Rust crate implements IDNA [per the WHATWG URL
Standard](https://url.spec.whatwg.org/#idna).

It also exposes the underlying algorithms from [*Unicode IDNA
Compatibility Processing* (Unicode Technical Standard
\#46)](http://www.unicode.org/reports/tr46/) and [Punycode (RFC
3492)](https://tools.ietf.org/html/rfc3492).

Quoting from [UTS \#46’s
introduction](http://www.unicode.org/reports/tr46/#Introduction):

> Initially, domain names were restricted to ASCII characters. A system
> was introduced in 2003 for internationalized domain names (IDN). This
> system is called Internationalizing Domain Names for Applications, or
> IDNA2003 for short. This mechanism supports IDNs by means of a client
> software transformation into a format known as Punycode. A revision of
> IDNA was approved in 2010 (IDNA2008). This revision has a number of
> incompatibilities with IDNA2003.
>
> The incompatibilities force implementers of client software, such as
> browsers and emailers, to face difficult choices during the transition
> period as registries shift from IDNA2003 to IDNA2008. This document
> specifies a mechanism that minimizes the impact of this transition for
> client software, allowing client software to access domains that are
> valid under either system.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use uts46::`<a href="uts46/struct.AsciiDenyList.html" class="struct"
title="struct idna::uts46::AsciiDenyList"><code>AsciiDenyList</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="punycode/index.html" class="mod"
title="mod idna::punycode">punycode</a>  
Punycode ([RFC 3492](http://tools.ietf.org/html/rfc3492))
implementation.

<a href="uts46/index.html" class="mod" title="mod idna::uts46">uts46</a>  
This module provides the lower-level API for UTS 46.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Config.html" class="struct"
title="struct idna::Config">Config</a><span class="stab deprecated" title="">Deprecated</span>  
Deprecated configuration API.

<a href="struct.Errors.html" class="struct"
title="struct idna::Errors">Errors</a>  
Type indicating that there were errors during UTS \#46 processing.

<a href="struct.Idna.html" class="struct"
title="struct idna::Idna">Idna</a><span class="stab deprecated" title="">Deprecated</span>  
Deprecated. Use the crate-top-level functions or
[`Uts46`](uts46/struct.Uts46.html "struct idna::uts46::Uts46").

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.domain_to_ascii.html" class="fn"
title="fn idna::domain_to_ascii">domain_to_ascii</a>  
The [domain to
ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;
version returning `String` and no ASCII deny list (i.e.
*UseSTD3ASCIIRules=false*).

<a href="fn.domain_to_ascii_cow.html" class="fn"
title="fn idna::domain_to_ascii_cow">domain_to_ascii_cow</a>  
The [domain to
ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;
version returning a `Cow`.

<a href="fn.domain_to_ascii_from_cow.html" class="fn"
title="fn idna::domain_to_ascii_from_cow">domain_to_ascii_from_cow</a>  
The [domain to
ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;
version accepting and returning a `Cow`.

<a href="fn.domain_to_ascii_strict.html" class="fn"
title="fn idna::domain_to_ascii_strict">domain_to_ascii_strict</a>  
The [domain to
ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm,
with the `beStrict` flag set.

<a href="fn.domain_to_unicode.html" class="fn"
title="fn idna::domain_to_unicode">domain_to_unicode</a>  
The [domain to
Unicode](https://url.spec.whatwg.org/#concept-domain-to-unicode)
algorithm; version returning `String` and no ASCII deny list (i.e.
*UseSTD3ASCIIRules=false*).
