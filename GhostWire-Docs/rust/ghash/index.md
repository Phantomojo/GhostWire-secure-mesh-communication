Crate ghash Copy item path

<span class="sub-heading"><a href="../src/ghash/lib.rs.html#1-143" class="src">Source</a>
</span>

Expand description

**GHASH**: universal hash over GF(2^128) used by AES-GCM for message
authentication (i.e. GMAC).

## <a href="#implementation-notes" class="doc-anchor">§</a>Implementation Notes

The implementation of GHASH found in this crate internally uses the
[`polyval`](https://github.com/RustCrypto/universal-hashes/tree/master/polyval)
crate, which provides a similar universal hash function used by
AES-GCM-SIV (RFC 8452).

By implementing GHASH in terms of POLYVAL, the two universal hash
functions can share a common core, meaning any optimization work (e.g.
CPU-specific SIMD implementations) which happens upstream in the
`polyval` crate benefits GHASH as well.

From RFC 8452 Appendix A:
<https://tools.ietf.org/html/rfc8452#appendix-A>

> GHASH and POLYVAL both operate in GF(2^128), although with different
> irreducible polynomials: POLYVAL works modulo x^128 + x^127 + x^126 +
> x^121 + 1 and GHASH works modulo x^128 + x^7 + x^2 + x + 1. Note that
> these irreducible polynomials are the “reverse” of each other.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use polyval::`<a href="../universal_hash/index.html" class="mod"
title="mod universal_hash"><code>universal_hash</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.GHash.html" class="struct"
title="struct ghash::GHash">GHash</a>  
**GHASH**: universal hash over GF(2^128) used by AES-GCM.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Block.html" class="type"
title="type ghash::Block">Block</a>  
GHASH blocks (16-bytes)

<a href="type.Key.html" class="type" title="type ghash::Key">Key</a>  
GHASH keys (16-bytes)

<a href="type.Tag.html" class="type" title="type ghash::Tag">Tag</a>  
GHASH tags (16-bytes)
