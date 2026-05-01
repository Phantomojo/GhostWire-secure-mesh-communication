Crate base\_x Copy item path

<span class="sub-heading"><a href="../src/base_x/lib.rs.html#1-121" class="src">Source</a>
</span>

Expand description

# <a href="#base_x" class="doc-anchor">§</a>base\_x

Encode and decode any base alphabet.

## <a href="#installation" class="doc-anchor">§</a>Installation

Add this to `Cargo.toml` file:

``` language-toml
[dependencies]
base-x = "0.2.0"
```

## <a href="#usage" class="doc-anchor">§</a>Usage

``` {.rust .rust-example-rendered}
extern crate base_x;

fn main() {
  let decoded = base_x::decode("01", "11111111000000001111111100000000").unwrap();
  let encoded = base_x::encode("01", &decoded);
 assert_eq!(encoded, "11111111000000001111111100000000");
}
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use alphabet::`<a href="alphabet/trait.Alphabet.html" class="trait"
title="trait base_x::alphabet::Alphabet"><code>Alphabet</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="alphabet/index.html" class="mod"
title="mod base_x::alphabet">alphabet</a>

<a href="decoder/index.html" class="mod"
title="mod base_x::decoder">decoder</a>

<a href="encoder/index.html" class="mod"
title="mod base_x::encoder">encoder</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DecodeError.html" class="struct"
title="struct base_x::DecodeError">DecodeError</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode.html" class="fn" title="fn base_x::decode">decode</a>  
Decode an input vector using the given alphabet.

<a href="fn.encode.html" class="fn" title="fn base_x::encode">encode</a>  
Encode an input vector using the given alphabet.
