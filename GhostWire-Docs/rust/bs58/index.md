Crate bs58 Copy item path

<span class="sub-heading"><a href="../src/bs58/lib.rs.html#1-221" class="src">Source</a>
</span>

Expand description

Another [Base58](https://en.wikipedia.org/wiki/Base58) codec
implementation.

Compared to [`base58`](https://github.com/debris/base58) this is
significantly faster at decoding (about 2.4x as fast when decoding 32
bytes), almost the same speed for encoding (about 3% slower when
encoding 32 bytes) and doesn’t have the 128 byte limitation.

Compared to [`rust-base58`](https://github.com/nham/rust-base58) this is
massively faster (over ten times as fast when decoding 32 bytes, almost
40 times as fast when encoding 32 bytes) and has no external
dependencies.

Compared to both this supports a configurable alphabet and user provided
buffers for zero-allocation {en,de}coding.

# <a href="#features" class="doc-anchor">§</a>Features

| Feature | Activation | Effect |
|----|----|----|
| `std` | **on**-by-default | Implement [`Error`](https://doc.rust-lang.org/1.93.1/core/error/trait.Error.html "trait core::error::Error") for error types |
| `alloc` | implied by `std` | Support encoding/decoding to [`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") and [`String`](https://doc.rust-lang.org/1.93.1/alloc/string/struct.String.html "struct alloc::string::String") as appropriate |
| `check` | **off**-by-default | Integrated support for [Base58Check](https://en.bitcoin.it/wiki/Base58Check_encoding) |
| `cb58` | **off**-by-default | Integrated support for [CB58](https://support.avax.network/en/articles/4587395-what-is-cb58) |

# <a href="#examples" class="doc-anchor">§</a>Examples

## <a href="#basic-example" class="doc-anchor">§</a>Basic example

``` {.rust .rust-example-rendered}
let decoded = bs58::decode("he11owor1d").into_vec()?;
let encoded = bs58::encode(decoded).into_string();
assert_eq!("he11owor1d", encoded);
```

## <a href="#changing-the-alphabet" class="doc-anchor">§</a>Changing the alphabet

``` {.rust .rust-example-rendered}
let decoded = bs58::decode("he11owor1d")
    .with_alphabet(bs58::Alphabet::RIPPLE)
    .into_vec()?;
let encoded = bs58::encode(decoded)
    .with_alphabet(bs58::Alphabet::FLICKR)
    .into_string();
assert_eq!("4DSSNaN1SC", encoded);
```

## <a href="#decoding-into-an-existing-buffer" class="doc-anchor">§</a>Decoding into an existing buffer

``` {.rust .rust-example-rendered}
let (mut decoded, mut encoded) = ([0xFF; 8], String::with_capacity(10));
bs58::decode("he11owor1d").onto(&mut decoded)?;
bs58::encode(decoded).onto(&mut encoded)?;
assert_eq!("he11owor1d", encoded);
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="alphabet/index.html" class="mod"
title="mod bs58::alphabet">alphabet</a>  
Support for configurable alphabets

<a href="decode/index.html" class="mod"
title="mod bs58::decode">decode</a>  
Functions for decoding Base58 encoded strings.

<a href="encode/index.html" class="mod"
title="mod bs58::encode">encode</a>  
Functions for encoding into Base58 encoded strings.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Alphabet.html" class="struct"
title="struct bs58::Alphabet">Alphabet</a>  
Prepared Alphabet for
[`EncodeBuilder::with_alphabet`](encode/struct.EncodeBuilder.html#method.with_alphabet "method bs58::encode::EncodeBuilder::with_alphabet")
and
[`DecodeBuilder::with_alphabet`](decode/struct.DecodeBuilder.html#method.with_alphabet "method bs58::decode::DecodeBuilder::with_alphabet").

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode.html" class="fn" title="fn bs58::decode">decode</a>  
Setup decoder for the given string using the [default
alphabet](struct.Alphabet.html#associatedconstant.DEFAULT "associated constant bs58::Alphabet::DEFAULT").

<a href="fn.encode.html" class="fn" title="fn bs58::encode">encode</a>  
Setup encoder for the given bytes using the [default
alphabet](struct.Alphabet.html#associatedconstant.DEFAULT "associated constant bs58::Alphabet::DEFAULT").
