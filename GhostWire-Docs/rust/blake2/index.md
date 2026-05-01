Crate blake2 Copy item path

<span class="sub-heading"><a href="../src/blake2/lib.rs.html#1-172" class="src">Source</a>
</span>

Expand description

An implementation of the
[BLAKE2](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2)
hash functions.

# <a href="#usage" class="doc-anchor">§</a>Usage

[`Blake2b512`](type.Blake2b512.html "type blake2::Blake2b512") and
[`Blake2s256`](type.Blake2s256.html "type blake2::Blake2s256") can be
used in the following way:

``` {.rust .rust-example-rendered}
use blake2::{Blake2b512, Blake2s256, Digest};
use hex_literal::hex;

// create a Blake2b512 object
let mut hasher = Blake2b512::new();

// write input message
hasher.update(b"hello world");

// read hash digest and consume hasher
let res = hasher.finalize();
assert_eq!(res[..], hex!("
    021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbc
    c05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0
")[..]);

// same example for Blake2s256:
let mut hasher = Blake2s256::new();
hasher.update(b"hello world");
let res = hasher.finalize();
assert_eq!(res[..], hex!("
    9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b
")[..]);
```

Also see [RustCrypto/hashes](https://github.com/RustCrypto/hashes)
readme.

## <a href="#variable-output-size" class="doc-anchor">§</a>Variable output size

This implementation supports run and compile time variable sizes.

Run time variable output example:

``` {.rust .rust-example-rendered}
use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use hex_literal::hex;

let mut hasher = Blake2bVar::new(10).unwrap();
hasher.update(b"my_input");
let mut buf = [0u8; 10];
hasher.finalize_variable(&mut buf).unwrap();
assert_eq!(buf, hex!("2cc55c84e416924e6400"));
```

Compile time variable output example:

``` {.rust .rust-example-rendered}
use blake2::{Blake2b, Digest, digest::consts::U10};
use hex_literal::hex;

type Blake2b80 = Blake2b<U10>;

let mut hasher = Blake2b80::new();
hasher.update(b"my_input");
let res = hasher.finalize();
assert_eq!(res[..], hex!("2cc55c84e416924e6400")[..]);
```

# <a href="#acknowledgment" class="doc-anchor">§</a>Acknowledgment

Based on the [blake2-rfc](https://github.com/cesarb/blake2-rfc) crate.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../digest/index.html" class="mod"
title="mod digest"><code>digest</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Blake2bMac.html" class="struct"
title="struct blake2::Blake2bMac">Blake2bMac</a>  
Blake2b MAC function

<a href="struct.Blake2bVarCore.html" class="struct"
title="struct blake2::Blake2bVarCore">Blake2bVarCore</a>  
Blake2b instance with a variable output.

<a href="struct.Blake2sMac.html" class="struct"
title="struct blake2::Blake2sMac">Blake2sMac</a>  
Blake2s MAC function

<a href="struct.Blake2sVarCore.html" class="struct"
title="struct blake2::Blake2sVarCore">Blake2sVarCore</a>  
Blake2s instance with a variable output.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Digest.html" class="trait"
title="trait blake2::Digest">Digest</a>  
Convenience wrapper trait covering functionality of cryptographic hash
functions with fixed output size.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Blake2b.html" class="type"
title="type blake2::Blake2b">Blake2b</a>  
BLAKE2b generic over output size.

<a href="type.Blake2b512.html" class="type"
title="type blake2::Blake2b512">Blake2b512</a>  
BLAKE2b-512 hasher state.

<a href="type.Blake2bCore.html" class="type"
title="type blake2::Blake2bCore">Blake2bCore</a>  
Core hasher state of BLAKE2b generic over output size.

<a href="type.Blake2bMac512.html" class="type"
title="type blake2::Blake2bMac512">Blake2bMac512</a>  
BLAKE2b-512 MAC state.

<a href="type.Blake2bVar.html" class="type"
title="type blake2::Blake2bVar">Blake2bVar</a>  
BLAKE2b which allows to choose output size at runtime.

<a href="type.Blake2s.html" class="type"
title="type blake2::Blake2s">Blake2s</a>  
BLAKE2s generic over output size.

<a href="type.Blake2s256.html" class="type"
title="type blake2::Blake2s256">Blake2s256</a>  
BLAKE2s-256 hasher state.

<a href="type.Blake2sCore.html" class="type"
title="type blake2::Blake2sCore">Blake2sCore</a>  
Core hasher state of BLAKE2s generic over output size.

<a href="type.Blake2sMac256.html" class="type"
title="type blake2::Blake2sMac256">Blake2sMac256</a>  
BLAKE2s-256 MAC state.

<a href="type.Blake2sVar.html" class="type"
title="type blake2::Blake2sVar">Blake2sVar</a>  
BLAKE2s which allows to choose output size at runtime.
