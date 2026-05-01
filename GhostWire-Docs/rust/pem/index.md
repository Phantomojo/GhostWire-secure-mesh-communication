Crate pem Copy item path

<span class="sub-heading"><a href="../src/pem/lib.rs.html#7-1061" class="src">Source</a>
</span>

Expand description

This crate provides a parser and encoder for PEM-encoded binary data.
PEM-encoded binary data is essentially a beginning and matching end tag
that encloses base64-encoded binary data (see:
https://en.wikipedia.org/wiki/Privacy-enhanced\_Electronic\_Mail).

This crate’s documentation provides a few simple examples along with
documentation on the public methods for the crate.

# <a href="#usage" class="doc-anchor">§</a>Usage

This crate is [on crates.io](https://crates.io/crates/pem) and can be
used by adding `pem` to your dependencies in your project’s
`Cargo.toml`.

``` language-toml
[dependencies]
pem = "3"
```

Using the `serde` feature will implement the serde traits for the `Pem`
struct.

# <a href="#example-parse-a-single-chunk-of-pem-encoded-text"
class="doc-anchor">§</a>Example: parse a single chunk of PEM-encoded text

Generally, PEM-encoded files contain a single chunk of PEM-encoded text.
Commonly, this is in some sort of a key file or an x.509 certificate.

``` {.rust .rust-example-rendered}
use pem::parse;

const SAMPLE: &'static str = "-----BEGIN RSA PRIVATE KEY-----
MIIBPQIBAAJBAOsfi5AGYhdRs/x6q5H7kScxA0Kzzqe6WI6gf6+tc6IvKQJo5rQc
dWWSQ0nRGt2hOPDO+35NKhQEjBQxPh/v7n0CAwEAAQJBAOGaBAyuw0ICyENy5NsO
2gkT00AWTSzM9Zns0HedY31yEabkuFvrMCHjscEF7u3Y6PB7An3IzooBHchsFDei
AAECIQD/JahddzR5K3A6rzTidmAf1PBtqi7296EnWv8WvpfAAQIhAOvowIXZI4Un
DXjgZ9ekuUjZN+GUQRAVlkEEohGLVy59AiEA90VtqDdQuWWpvJX0cM08V10tLXrT
TTGsEtITid1ogAECIQDAaFl90ZgS5cMrL3wCeatVKzVUmuJmB/VAmlLFFGzK0QIh
ANJGc7AFk4fyFD/OezhwGHbWmo/S+bfeAiIh2Ss2FxKJ
-----END RSA PRIVATE KEY-----
";

 let pem = parse(SAMPLE).unwrap();
 assert_eq!(pem.tag(), "RSA PRIVATE KEY");
```

# <a href="#example-parse-a-set-of-pem-encoded-text"
class="doc-anchor">§</a>Example: parse a set of PEM-encoded text

Sometimes, PEM-encoded files contain multiple chunks of PEM-encoded
text. You might see this if you have an x.509 certificate file that also
includes intermediate certificates.

``` {.rust .rust-example-rendered}
use pem::parse_many;

const SAMPLE: &'static str = "-----BEGIN INTERMEDIATE CERT-----
MIIBPQIBAAJBAOsfi5AGYhdRs/x6q5H7kScxA0Kzzqe6WI6gf6+tc6IvKQJo5rQc
dWWSQ0nRGt2hOPDO+35NKhQEjBQxPh/v7n0CAwEAAQJBAOGaBAyuw0ICyENy5NsO
2gkT00AWTSzM9Zns0HedY31yEabkuFvrMCHjscEF7u3Y6PB7An3IzooBHchsFDei
AAECIQD/JahddzR5K3A6rzTidmAf1PBtqi7296EnWv8WvpfAAQIhAOvowIXZI4Un
DXjgZ9ekuUjZN+GUQRAVlkEEohGLVy59AiEA90VtqDdQuWWpvJX0cM08V10tLXrT
TTGsEtITid1ogAECIQDAaFl90ZgS5cMrL3wCeatVKzVUmuJmB/VAmlLFFGzK0QIh
ANJGc7AFk4fyFD/OezhwGHbWmo/S+bfeAiIh2Ss2FxKJ
-----END INTERMEDIATE CERT-----

-----BEGIN CERTIFICATE-----
MIIBPQIBAAJBAOsfi5AGYhdRs/x6q5H7kScxA0Kzzqe6WI6gf6+tc6IvKQJo5rQc
dWWSQ0nRGt2hOPDO+35NKhQEjBQxPh/v7n0CAwEAAQJBAOGaBAyuw0ICyENy5NsO
2gkT00AWTSzM9Zns0HedY31yEabkuFvrMCHjscEF7u3Y6PB7An3IzooBHchsFDei
AAECIQD/JahddzR5K3A6rzTidmAf1PBtqi7296EnWv8WvpfAAQIhAOvowIXZI4Un
DXjgZ9ekuUjZN+GUQRAVlkEEohGLVy59AiEA90VtqDdQuWWpvJX0cM08V10tLXrT
TTGsEtITid1ogAECIQDAaFl90ZgS5cMrL3wCeatVKzVUmuJmB/VAmlLFFGzK0QIh
ANJGc7AFk4fyFD/OezhwGHbWmo/S+bfeAiIh2Ss2FxKJ
-----END CERTIFICATE-----
";

 let pems = parse_many(SAMPLE).unwrap();
 assert_eq!(pems.len(), 2);
 assert_eq!(pems[0].tag(), "INTERMEDIATE CERT");
 assert_eq!(pems[1].tag(), "CERTIFICATE");
```

# <a href="#features" class="doc-anchor">§</a>Features

This crate supports two features: `std` and `serde`.

The `std` feature is enabled by default. If you specify
`default-features = false` to disable `std`, be aware that this crate
still needs an allocator.

The `serde` feature implements `serde::{Deserialize, Serialize}` for
this crate’s `Pem` struct.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.ensure.html" class="macro"
title="macro pem::ensure">ensure</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.EncodeConfig.html" class="struct"
title="struct pem::EncodeConfig">EncodeConfig</a>  
Configuration for Pem encoding

<a href="struct.HeaderMap.html" class="struct"
title="struct pem::HeaderMap">HeaderMap</a>  
Provides access to the headers that might be found in a Pem-encoded file

<a href="struct.HeadersIter.html" class="struct"
title="struct pem::HeadersIter">HeadersIter</a>  
Iterator across all headers in the Pem-encoded data

<a href="struct.Pem.html" class="struct" title="struct pem::Pem">Pem</a>  
A representation of Pem-encoded data

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.LineEnding.html" class="enum"
title="enum pem::LineEnding">LineEnding</a>  
Enum describing line endings

<a href="enum.PemError.html" class="enum"
title="enum pem::PemError">PemError</a>  
The `pem` error type.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.encode.html" class="fn" title="fn pem::encode">encode</a>  
Encode a PEM struct into a PEM-encoded data string

<a href="fn.encode_config.html" class="fn"
title="fn pem::encode_config">encode_config</a>  
Encode a PEM struct into a PEM-encoded data string with additional
configuration options

<a href="fn.encode_many.html" class="fn"
title="fn pem::encode_many">encode_many</a>  
Encode multiple PEM structs into a PEM-encoded data string

<a href="fn.encode_many_config.html" class="fn"
title="fn pem::encode_many_config">encode_many_config</a>  
Encode multiple PEM structs into a PEM-encoded data string with
additional configuration options

<a href="fn.parse.html" class="fn" title="fn pem::parse">parse</a>  
Parses a single PEM-encoded data from a data-type that can be
dereferenced as a
[u8](https://doc.rust-lang.org/1.93.1/std/primitive.u8.html "primitive u8").

<a href="fn.parse_many.html" class="fn"
title="fn pem::parse_many">parse_many</a>  
Parses a set of PEM-encoded data from a data-type that can be
dereferenced as a
[u8](https://doc.rust-lang.org/1.93.1/std/primitive.u8.html "primitive u8").

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type pem::Result">Result</a>  
The `pem` result type.
