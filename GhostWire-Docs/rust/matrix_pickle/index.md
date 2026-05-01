Crate matrix\_pickle Copy item path

<span class="sub-heading"><a href="../src/matrix_pickle/lib.rs.html#15-172" class="src">Source</a>
</span>

Expand description

![Build
Status](https://img.shields.io/github/actions/workflow/status/matrix-org/matrix-pickle/ci.yml?branch=main&style=flat-square)
[![codecov](https://img.shields.io/codecov/c/github/matrix-org/matrix-pickle/main.svg?style=flat-square)](https://codecov.io/gh/matrix-org/matrix-pickle)
[![License](https://img.shields.io/badge/License-MIT-yellowgreen.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Docs](https://img.shields.io/crates/v/matrix-pickle?color=blue&label=docs&style=flat-square)](https://docs.rs/matrix-pickle)

A simple binary encoding format used in the Matrix world.

The `matrix-pickle` binary encoding format is used in the
[libolm](https://gitlab.matrix.org/matrix-org/olm/) and
[vodozemac](https://github.com/matrix-org/vodozemac/) cryptographic
libraries.

# <a href="#how-to-use" class="doc-anchor">§</a>How to use

The simplest way to use `matrix-pickle` is using the derive macros:

``` {.rust .rust-example-rendered}
use anyhow::Result;
use matrix_pickle::{Encode, Decode};

fn main() -> Result<()> {
    #[derive(Clone, Debug, Decode, Encode, PartialEq, Eq)]
    struct MyStruct {
        public_key: [u8; 32],
        data: Vec<u8>,
    }
    
    let data = MyStruct {
        public_key: [5u8; 32],
        data: vec![1, 2, 3],
    };
    
    let encoded = data.encode_to_vec()?;
    let decoded = MyStruct::decode_from_slice(&encoded)?;
    
    assert_eq!(data, decoded);

    Ok(())
}
```

# <a href="#format-definition" class="doc-anchor">§</a>Format definition

`matrix-pickle` encodes most values without any metadata, the bytes that
are part of the struct in most cases get encoded verbatim.

The table bellow defines how common types are encoded.

| Type | Example value | Encoded value | Comment |
|:--:|:--:|:--:|----|
| `u8` | `255` | `[FF]` | Encoded verbatim |
| `bool` | `true` | `[01]` | Converted to an `u8` before encoding |
| `[u8; N]` | `[1u8, 2u8]` | `[01, 02]` | Encoded verbatim |
| `u32` | `16` | `[00, 00, 00, 10]` | Encoded as a byte array in big endian form |
| `usize` | `32` | `[00, 00, 00, 20]` | Converted to an `u32` before encoding |
| `&[T]` | `&[3u8, 4u8]` | `[00, 00, 00, 02, 03, 04]` | The length gets encoded first, then each element |

# <a href="#derive-support" class="doc-anchor">§</a>Derive support

The crate supports deriving `Encode` and `Decode` implementations for
structs and enums as long as the types inside them implement `Encode`
and `Decode` as well.

## <a href="#structs" class="doc-anchor">§</a>Structs

The derive support for structs simply encodes each field of a struct in
the order they are defined, for example:

``` {.rust .rust-example-rendered}
use std::io::Write;
use matrix_pickle::{Encode, EncodeError};

struct Foo {
    first: [u8; 32],
    second: Vec<u8>,
}

impl Encode for Foo {
    fn encode(&self, writer: &mut impl Write) -> Result<usize, EncodeError> {
        let mut ret = 0;

        // Encode the first struct field.
        ret += self.first.encode(writer)?;
        // Now encode the second struct field.
        ret += self.second.encode(writer)?;

        Ok(ret)
    }
}
```

## <a href="#enums" class="doc-anchor">§</a>Enums

Enums on the other hand first encode the number of the variant as an
`u8`, then the value of the enum.

Only enums with variants that contain a single associated data value are
supported.

``` {.rust .rust-example-rendered}
use std::io::Write;
use matrix_pickle::{Encode, EncodeError};

enum Bar {
    First(u32),
    Second(u32),
}

impl Encode for Bar {
    fn encode(&self, writer: &mut impl Write) -> Result<usize, EncodeError> {
        let mut ret = 0;

        match self {
            Bar::First(value) => {
                // This is our first variant, encode a 0u8 first.
                ret += 0u8.encode(writer)?;
                // Now encode the associated value.
                ret += value.encode(writer)?;
            },
            Bar::Second(value) => {
                // This is our second variant, encode a 1u8 first.
                ret += 1u8.encode(writer)?;
                // Now encode the associated value.
                ret += value.encode(writer)?;
            },
        }

        Ok(ret)
    }
}
```

## <a href="#encoding-and-decoding-secrets" class="doc-anchor">§</a>Encoding and decoding secrets

For decoding values which are meant to be secret, make sure to box the
array. We have a helper attribute that reminds you that values that are
meant to be kept secret should be boxed.

Simply annotate any struct field using the `#[secret]` attribute.

If a value that is meant to be a secret is not boxed a compiler error
will be thrown. For example, this snippet won’t compile.

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
use matrix_pickle::{Encode, Decode};

#[derive(Encode, Decode)]
struct Key {
    #[secret]
    private: [u8; 32],
    public: [u8; 32],
}
```

This example on the other hand compiles.

``` {.rust .rust-example-rendered}
use matrix_pickle::{Encode, Decode};

#[derive(Encode, Decode)]
struct Key {
    #[secret]
    private: Box<[u8; 32]>,
    public: [u8; 32],
}
```

# <a href="#comparison-to-bincode" class="doc-anchor">§</a>Comparison to bincode

The binary format is similar to what the
[bincode](https://github.com/bincode-org/bincode/) crate provides with
the following config:

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
let config = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .skip_fixed_array_length();
```

The two major differences to the format are:

-   `bincode` uses `u64` to encode slice lengths
-   `matrix-pickle` uses `u32` to encode slice lengths

Other differences are:

-   No support to configure the encoding format, if you need to tweak
    the format, use bincode.
-   No unsafe code. Optimized for simplicity, not for pure performance

# Enums<a href="#enums-1" class="anchor">§</a>

<a href="enum.DecodeError.html" class="enum"
title="enum matrix_pickle::DecodeError">DecodeError</a>  
Error type describing failure modes for libolm pickle decoding.

<a href="enum.EncodeError.html" class="enum"
title="enum matrix_pickle::EncodeError">EncodeError</a>  
Error type describing failure modes for libolm pickle decoding.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Decode.html" class="trait"
title="trait matrix_pickle::Decode">Decode</a>  
A trait for decoding values that were encoded using the `matrix-pickle`
binary format.

<a href="trait.Encode.html" class="trait"
title="trait matrix_pickle::Encode">Encode</a>  
A trait for encoding values into the `matrix-pickle` binary format.

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.Decode.html" class="derive"
title="derive matrix_pickle::Decode">Decode</a>  
Derive an `Decode` implementation for a struct or enum.

<a href="derive.Encode.html" class="derive"
title="derive matrix_pickle::Encode">Encode</a>  
Derive an `Encode` implementation for a struct or enum.
